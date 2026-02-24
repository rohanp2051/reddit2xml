use crate::auth::USER_AGENT;
use crate::types::{Comment, Post, PostType};
use serde_json::Value;
use std::io;

const OAUTH_API_BASE: &str = "https://oauth.reddit.com";
const MAX_RETRIES: u32 = 3;

fn fetch_json(endpoint: &str, token: &str) -> Result<Value, io::Error> {
    let url = format!("{OAUTH_API_BASE}{endpoint}");

    for attempt in 0..=MAX_RETRIES {
        let result = ureq::get(&url)
            .set("Authorization", &format!("Bearer {token}"))
            .set("User-Agent", USER_AGENT)
            .call();

        match result {
            Ok(resp) => {
                return resp
                    .into_json()
                    .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e.to_string()));
            }
            Err(ureq::Error::Status(429, _)) if attempt < MAX_RETRIES => {
                let wait = 5 * (1 << attempt);
                std::thread::sleep(std::time::Duration::from_secs(wait));
                continue;
            }
            Err(e) => {
                return Err(io::Error::new(io::ErrorKind::Other, e.to_string()));
            }
        }
    }

    unreachable!()
}

fn get_post_type(data: &Value) -> PostType {
    if data["is_self"].as_bool().unwrap_or(false) {
        PostType::Text
    } else if data["is_gallery"].as_bool().unwrap_or(false) {
        PostType::Gallery
    } else if data["is_video"].as_bool().unwrap_or(false) {
        PostType::Video
    } else {
        PostType::Link
    }
}

fn get_content(data: &Value) -> String {
    if data["is_self"].as_bool().unwrap_or(false) {
        let text = data["selftext"].as_str().unwrap_or("");
        if text.is_empty() {
            "(no text)".to_string()
        } else {
            text.to_string()
        }
    } else {
        data["url"].as_str().unwrap_or("").to_string()
    }
}

fn str_field(data: &Value, key: &str) -> String {
    data[key].as_str().unwrap_or("").to_string()
}

fn i64_field(data: &Value, key: &str) -> i64 {
    data[key].as_i64().unwrap_or(0)
}

pub fn fetch_hot(subreddit: &str, limit: u32, token: &str) -> Result<Vec<Post>, io::Error> {
    let endpoint = format!("/r/{subreddit}/hot?limit={limit}");
    let json = fetch_json(&endpoint, token)?;

    let children = json["data"]["children"]
        .as_array()
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "missing data.children"))?;

    let posts = children
        .iter()
        .map(|child| {
            let data = &child["data"];
            Post {
                id: str_field(data, "id"),
                title: str_field(data, "title"),
                score: i64_field(data, "score"),
                num_comments: i64_field(data, "num_comments"),
                author: data["author"].as_str().unwrap_or("[deleted]").to_string(),
                post_type: get_post_type(data),
                content: get_content(data),
                permalink: str_field(data, "permalink"),
            }
        })
        .collect();

    Ok(posts)
}

fn parse_comment(data: &Value) -> Comment {
    let replies_val = &data["replies"];
    let replies = if replies_val.is_object() {
        replies_val["data"]["children"]
            .as_array()
            .map(|arr| {
                arr.iter()
                    .filter(|c| c["kind"].as_str() == Some("t1"))
                    .map(|c| parse_comment(&c["data"]))
                    .collect()
            })
            .unwrap_or_default()
    } else {
        Vec::new()
    };

    Comment {
        author: data["author"].as_str().unwrap_or("[deleted]").to_string(),
        score: i64_field(data, "score"),
        body: data["body"].as_str().unwrap_or("[removed]").to_string(),
        replies,
    }
}

pub fn fetch_post(
    post_id: &str,
    comment_id: Option<&str>,
    context: Option<u32>,
    comment_limit: u32,
    comment_depth: u32,
    token: &str,
) -> Result<(Post, Vec<Comment>), io::Error> {
    let mut endpoint = format!(
        "/comments/{post_id}?limit={comment_limit}&depth={comment_depth}&sort=top"
    );
    if let Some(cid) = comment_id {
        endpoint.push_str(&format!("&comment={cid}"));
    }
    if let Some(ctx) = context {
        endpoint.push_str(&format!("&context={ctx}"));
    }
    let json = fetch_json(&endpoint, token)?;

    let arr = json
        .as_array()
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "expected array response"))?;

    if arr.len() < 2 {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "expected at least 2 listings",
        ));
    }

    // Parse submission
    let sub_data = &arr[0]["data"]["children"][0]["data"];
    let post = Post {
        id: str_field(sub_data, "id"),
        title: str_field(sub_data, "title"),
        score: i64_field(sub_data, "score"),
        num_comments: i64_field(sub_data, "num_comments"),
        author: sub_data["author"]
            .as_str()
            .unwrap_or("[deleted]")
            .to_string(),
        post_type: get_post_type(sub_data),
        content: get_content(sub_data),
        permalink: str_field(sub_data, "permalink"),
    };

    // Parse comments
    let comments = arr[1]["data"]["children"]
        .as_array()
        .map(|arr| {
            arr.iter()
                .filter(|c| c["kind"].as_str() == Some("t1"))
                .map(|c| parse_comment(&c["data"]))
                .collect()
        })
        .unwrap_or_default();

    Ok((post, comments))
}
