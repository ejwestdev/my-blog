use include_dir::{include_dir, Dir};
use pulldown_cmark::{html, Parser};
use serde::Deserialize;
use std::sync::OnceLock;

/// All blog posts and projects are embedded at compile time.
/// Post ids are derived from the filename prefix before the first dash
/// (e.g. `1-my-first-post.md` -> 1); project slugs come from the file stem
/// (e.g. `greppy.md` -> "greppy").
static BLOG_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/content/blog");
static PROJECTS_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/content/projects");

#[derive(Debug, Clone, Deserialize)]
struct PostFrontmatter {
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

#[derive(Debug, Clone, Deserialize)]
struct ProjectFrontmatter {
    title: String,
    #[serde(default)]
    link: Option<String>,
    order: i32,
    description: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Project {
    pub slug: &'static str,
    pub title: String,
    pub link: Option<String>,
    pub order: i32,
    pub description: String,
    pub html: String,
}

fn split_frontmatter(source: &str) -> (&str, &str) {
    let parts: Vec<&str> = source.splitn(3, "---").collect();
    (parts[1].trim(), parts[2].trim())
}

fn render_markdown(md: &str) -> String {
    let mut html_output = String::new();
    html::push_html(&mut html_output, Parser::new(md));
    html_output
}

fn parse_post(id: i32, source: &str) -> BlogPost {
    let (fm_str, md_body) = split_frontmatter(source);
    let fm: PostFrontmatter = toml::from_str(fm_str)
        .unwrap_or_else(|e| panic!("Invalid frontmatter in post {}: {}", id, e));

    BlogPost {
        id,
        title: fm.title,
        date: fm.date,
        html: render_markdown(md_body),
    }
}

fn parse_project(slug: &'static str, source: &str) -> Project {
    let (fm_str, md_body) = split_frontmatter(source);
    let fm: ProjectFrontmatter = toml::from_str(fm_str)
        .unwrap_or_else(|e| panic!("Invalid frontmatter in project {}: {}", slug, e));

    Project {
        slug,
        title: fm.title,
        link: fm.link,
        order: fm.order,
        description: fm.description,
        html: render_markdown(md_body),
    }
}

fn all_posts_impl() -> &'static Vec<BlogPost> {
    static POSTS: OnceLock<Vec<BlogPost>> = OnceLock::new();
    POSTS.get_or_init(|| {
        let mut posts: Vec<BlogPost> = BLOG_DIR
            .files()
            .filter_map(|file| {
                let name = file.path().file_name()?.to_str()?;
                let (id_str, _) = name.split_once('-')?;
                let id: i32 = id_str.parse().ok()?;
                Some(parse_post(id, file.contents_utf8()?))
            })
            .collect();
        posts.sort_by_key(|p| p.id);
        posts
    })
}

fn all_projects_impl() -> &'static Vec<Project> {
    static PROJECTS: OnceLock<Vec<Project>> = OnceLock::new();
    PROJECTS.get_or_init(|| {
        let mut projects: Vec<Project> = PROJECTS_DIR
            .files()
            .filter_map(|file| {
                let slug = file.path().file_stem()?.to_str()?;
                Some(parse_project(slug, file.contents_utf8()?))
            })
            .collect();
        projects.sort_by_key(|p| p.order);
        projects
    })
}

pub fn all_posts() -> Vec<BlogPost> {
    all_posts_impl().clone()
}

pub fn get_post(id: i32) -> Option<BlogPost> {
    all_posts_impl().iter().find(|p| p.id == id).cloned()
}

pub fn all_projects() -> Vec<Project> {
    all_projects_impl().clone()
}

pub fn get_project(slug: &str) -> Option<Project> {
    all_projects_impl().iter().find(|p| p.slug == slug).cloned()
}
