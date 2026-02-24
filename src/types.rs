use std::fmt;

#[derive(Debug)]
pub enum PostType {
    Text,
    Link,
    Gallery,
    Video,
}

impl fmt::Display for PostType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PostType::Text => write!(f, "text"),
            PostType::Link => write!(f, "link"),
            PostType::Gallery => write!(f, "gallery"),
            PostType::Video => write!(f, "video"),
        }
    }
}

#[derive(Debug)]
pub struct Post {
    pub id: String,
    pub title: String,
    pub score: i64,
    pub num_comments: i64,
    pub author: String,
    pub post_type: PostType,
    pub content: String,
    pub permalink: String,
}

#[derive(Debug)]
pub struct Comment {
    pub author: String,
    pub score: i64,
    pub body: String,
    pub replies: Vec<Comment>,
}

#[derive(Debug, Clone)]
pub struct FieldFilter {
    pub show_score: bool,
    pub show_author: bool,
    pub show_comments_count: bool,
    pub show_type: bool,
    pub show_link: bool,
    pub show_content: bool,
    pub show_id: bool,
    pub show_comments: bool,
}

impl Default for FieldFilter {
    fn default() -> Self {
        Self {
            show_score: true,
            show_author: true,
            show_comments_count: true,
            show_type: true,
            show_link: true,
            show_content: true,
            show_id: true,
            show_comments: true,
        }
    }
}
