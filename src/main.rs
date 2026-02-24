mod api;
mod auth;
mod format;
mod types;

use clap::{Parser, Subcommand};
use std::io::{self, BufWriter, Write};
use std::process;
use types::FieldFilter;

#[derive(Parser)]
#[command(name = "reddit2xml", about = "Token-efficient Reddit CLI outputting compact XML")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Fetch hot posts from a subreddit
    Hot {
        /// Subreddit name or URL (e.g. "rust" or "https://reddit.com/r/rust")
        subreddit: String,

        /// Number of posts (default: 10, max: 100)
        #[arg(short = 'n', long = "limit", default_value_t = 10)]
        limit: u32,

        /// Omit score
        #[arg(long)]
        no_score: bool,

        /// Omit author
        #[arg(long)]
        no_author: bool,

        /// Omit comment count
        #[arg(long)]
        no_comments_count: bool,

        /// Omit post type
        #[arg(long)]
        no_type: bool,

        /// Omit permalink
        #[arg(long)]
        no_link: bool,

        /// Omit content/URL
        #[arg(long)]
        no_content: bool,

        /// Omit post ID
        #[arg(long)]
        no_id: bool,

        /// Shorthand: --no-score --no-author --no-type --no-link
        #[arg(long)]
        minimal: bool,

        /// Only titles + IDs
        #[arg(long)]
        titles_only: bool,
    },
    /// Fetch a post and its comments
    Post {
        /// Reddit post ID or URL (e.g. "1r8yi06" or "https://reddit.com/r/NixOS/comments/1r8yi06/...")
        post_id: String,

        /// Top-level comments (default: 20, max: 100)
        #[arg(short = 'c', long = "comment-limit", default_value_t = 20)]
        comment_limit: u32,

        /// Reply depth (default: 3, max: 10)
        #[arg(short = 'd', long = "comment-depth", default_value_t = 3)]
        comment_depth: u32,

        /// Omit scores (post + comments)
        #[arg(long)]
        no_score: bool,

        /// Omit authors (post + comments)
        #[arg(long)]
        no_author: bool,

        /// Omit post type
        #[arg(long)]
        no_type: bool,

        /// Omit comments entirely
        #[arg(long)]
        no_comments: bool,

        /// Shorthand: --no-score --no-author --no-type
        #[arg(long)]
        minimal: bool,
    },
}

/// Extract subreddit name from a URL or return the input as-is.
/// Accepts: "rust", "r/rust", "https://www.reddit.com/r/rust/...", etc.
fn parse_subreddit(input: &str) -> Result<String, String> {
    let name = if input.contains('/') {
        // Try to extract from URL or r/name path
        input
            .split('/')
            .skip_while(|s| *s != "r")
            .nth(1)
            .map(|s| s.to_string())
            .ok_or_else(|| format!("could not extract subreddit from: {input}"))?
    } else {
        input.to_string()
    };

    if name.is_empty() || name.len() > 21 {
        return Err("subreddit name must be 1-21 characters".into());
    }
    if !name
        .chars()
        .all(|c| c.is_ascii_alphanumeric() || c == '_')
    {
        return Err("subreddit name must be alphanumeric or underscores".into());
    }
    Ok(name)
}

/// Extract post ID from a URL or return the input as-is.
/// Accepts: "1r8yi06", "https://www.reddit.com/r/NixOS/comments/1r8yi06/...", etc.
fn parse_post_id(input: &str) -> Result<String, String> {
    let id = if input.contains('/') {
        // Try to extract from URL: .../comments/<id>/...
        input
            .split('/')
            .skip_while(|s| *s != "comments")
            .nth(1)
            .map(|s| s.to_string())
            .ok_or_else(|| format!("could not extract post ID from: {input}"))?
    } else {
        input.to_string()
    };

    if id.is_empty() || id.len() > 10 {
        return Err("post ID must be 1-10 characters".into());
    }
    if !id.chars().all(|c| c.is_ascii_lowercase() || c.is_ascii_digit()) {
        return Err("post ID must be lowercase alphanumeric".into());
    }
    Ok(id)
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match cli.command {
        Command::Hot {
            subreddit,
            limit,
            no_score,
            no_author,
            no_comments_count,
            no_type,
            no_link,
            no_content,
            no_id,
            minimal,
            titles_only,
        } => {
            let subreddit = parse_subreddit(&subreddit)?;
            let limit = limit.clamp(1, 100);

            let mut filter = FieldFilter::default();
            if minimal {
                filter.show_score = false;
                filter.show_author = false;
                filter.show_type = false;
                filter.show_link = false;
            }
            if titles_only {
                filter.show_score = false;
                filter.show_author = false;
                filter.show_comments_count = false;
                filter.show_type = false;
                filter.show_link = false;
                filter.show_content = false;
            }
            if no_score {
                filter.show_score = false;
            }
            if no_author {
                filter.show_author = false;
            }
            if no_comments_count {
                filter.show_comments_count = false;
            }
            if no_type {
                filter.show_type = false;
            }
            if no_link {
                filter.show_link = false;
            }
            if no_content {
                filter.show_content = false;
            }
            if no_id {
                filter.show_id = false;
            }

            let token = auth::get_access_token()?;
            let posts = api::fetch_hot(&subreddit, limit, &token)?;

            let stdout = io::stdout();
            let mut w = BufWriter::new(stdout.lock());
            format::write_hot_xml(&mut w, &subreddit, &posts, &filter)?;
            w.flush()?;
        }
        Command::Post {
            post_id,
            comment_limit,
            comment_depth,
            no_score,
            no_author,
            no_type,
            no_comments,
            minimal,
        } => {
            let post_id = parse_post_id(&post_id)?;
            let comment_limit = comment_limit.clamp(1, 100);
            let comment_depth = comment_depth.clamp(1, 10);

            let mut filter = FieldFilter::default();
            if minimal {
                filter.show_score = false;
                filter.show_author = false;
                filter.show_type = false;
            }
            if no_score {
                filter.show_score = false;
            }
            if no_author {
                filter.show_author = false;
            }
            if no_type {
                filter.show_type = false;
            }
            if no_comments {
                filter.show_comments = false;
            }

            let token = auth::get_access_token()?;
            let (post, comments) = api::fetch_post(&post_id, comment_limit, comment_depth, &token)?;

            let stdout = io::stdout();
            let mut w = BufWriter::new(stdout.lock());
            format::write_post_xml(&mut w, &post, &comments, &filter)?;
            w.flush()?;
        }
    }

    Ok(())
}

fn main() {
    if let Err(e) = run() {
        eprintln!("error: {e}");
        process::exit(1);
    }
}
