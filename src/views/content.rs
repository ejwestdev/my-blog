use pulldown_cmark::{html, Parser};
use serde::Deserialize;
use std::sync::OnceLock;

#[derive(Debug, Clone, Deserialize)]
struct Frontmatter {
    title: String,
    date: String,
}

#[derive(Debug, Clone)]
pub struct BlogPost {
    pub id: i32,
    pub title: String,
    pub date: String,
    pub html: String,
}

fn parse_post(id: i32, source: &str) -> BlogPost {
    let parts: Vec<&str> = source.splitn(3, "---").collect();
    let fm: Frontmatter = toml::from_str(parts[1].trim())
        .unwrap_or_else(|e| panic!("Invalid frontmatter in post {}: {}", id, e));
    let md_body = parts[2].trim();

    let parser = Parser::new(md_body);
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);

    BlogPost {
        id,
        title: fm.title,
        date: fm.date,
        html: html_output,
    }
}

fn all_posts_impl() -> &'static Vec<BlogPost> {
    static POSTS: OnceLock<Vec<BlogPost>> = OnceLock::new();
    POSTS.get_or_init(|| {
        vec![
            parse_post(1, include_str!("../../content/blog/1-my-first-post.md")),
            parse_post(2, include_str!("../../content/blog/2-second-post.md")),
        ]
    })
}

pub fn all_posts() -> Vec<BlogPost> {
    all_posts_impl().clone()
}

pub fn get_post(id: i32) -> Option<BlogPost> {
    all_posts_impl().iter().find(|p| p.id == id).cloned()
}
