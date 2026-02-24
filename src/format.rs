use crate::types::{Comment, FieldFilter, Post};
use std::io::{self, Write};

pub fn xml_escape(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for c in s.chars() {
        match c {
            '&' => out.push_str("&amp;"),
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            '"' => out.push_str("&quot;"),
            _ => out.push(c),
        }
    }
    out
}

pub fn write_hot_xml<W: Write>(
    w: &mut W,
    subreddit: &str,
    posts: &[Post],
    filter: &FieldFilter,
) -> io::Result<()> {
    writeln!(w, "<r sub=\"{}\">", xml_escape(subreddit))?;
    for post in posts {
        write_post_element(w, post, filter)?;
    }
    writeln!(w, "</r>")?;
    Ok(())
}

fn write_post_element<W: Write>(w: &mut W, post: &Post, filter: &FieldFilter) -> io::Result<()> {
    write!(w, "<p")?;
    if filter.show_id {
        write!(w, " id=\"{}\"", xml_escape(&post.id))?;
    }
    if filter.show_score {
        write!(w, " s=\"{}\"", post.score)?;
    }
    if filter.show_comments_count {
        write!(w, " nc=\"{}\"", post.num_comments)?;
    }
    if filter.show_author {
        write!(w, " by=\"{}\"", xml_escape(&post.author))?;
    }
    if filter.show_type {
        write!(w, " type=\"{}\"", post.post_type)?;
    }
    writeln!(w, ">")?;

    writeln!(w, "<t>{}</t>", xml_escape(&post.title))?;

    if filter.show_content && !post.content.is_empty() {
        writeln!(w, "<c>{}</c>", xml_escape(&post.content))?;
    }
    if filter.show_link && !post.permalink.is_empty() {
        writeln!(w, "<l>{}</l>", xml_escape(&post.permalink))?;
    }

    writeln!(w, "</p>")?;
    Ok(())
}

pub fn write_post_xml<W: Write>(
    w: &mut W,
    post: &Post,
    comments: &[Comment],
    filter: &FieldFilter,
) -> io::Result<()> {
    writeln!(w, "<r>")?;

    // Post element
    write!(w, "<p")?;
    if filter.show_id {
        write!(w, " id=\"{}\"", xml_escape(&post.id))?;
    }
    if filter.show_score {
        write!(w, " s=\"{}\"", post.score)?;
    }
    if filter.show_author {
        write!(w, " by=\"{}\"", xml_escape(&post.author))?;
    }
    if filter.show_type {
        write!(w, " type=\"{}\"", post.post_type)?;
    }
    writeln!(w, ">")?;

    writeln!(w, "<t>{}</t>", xml_escape(&post.title))?;

    if filter.show_content && !post.content.is_empty() {
        writeln!(w, "<c>{}</c>", xml_escape(&post.content))?;
    }

    writeln!(w, "</p>")?;

    // Comments
    if filter.show_comments && !comments.is_empty() {
        writeln!(w, "<comments>")?;
        for comment in comments {
            write_comment_xml(w, comment, filter, 0)?;
        }
        writeln!(w, "</comments>")?;
    }

    writeln!(w, "</r>")?;
    Ok(())
}

fn write_comment_xml<W: Write>(
    w: &mut W,
    comment: &Comment,
    filter: &FieldFilter,
    depth: usize,
) -> io::Result<()> {
    let indent = "  ".repeat(depth);

    write!(w, "{indent}<cm")?;
    if filter.show_author {
        write!(w, " by=\"{}\"", xml_escape(&comment.author))?;
    }
    if filter.show_score {
        write!(w, " s=\"{}\"", comment.score)?;
    }
    writeln!(w, ">")?;

    // Write body, indented
    let body = xml_escape(&comment.body);
    for line in body.lines() {
        writeln!(w, "{indent}  {line}")?;
    }

    // Recurse into replies
    for reply in &comment.replies {
        write_comment_xml(w, reply, filter, depth + 1)?;
    }

    writeln!(w, "{indent}</cm>")?;
    Ok(())
}
