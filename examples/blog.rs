#[macro_use]
extern crate horrorshow;

use horrorshow::{RenderBox, Template};

fn render_post(post: Post) -> Box<RenderBox> {
    let Post { title, body, tags } = post;
    box_html! {
        article {
            header(class="post-header") {
                h1 : title;
                ul {
                    @ for tag in tags {
                        li : tag
                    }
                }
            }
            section(class="post-body") : body;
        }
    }
}

fn render<I: Iterator<Item=Post>>(title: &str, posts: I) -> String {
    (html! {
        : raw!("<!DOCTYPE html>");
        html {
            head {
                title : title
            }
            body {
                main {
                    header { h1 : title }
                    section(id="posts") {
                        @ for post in posts {
                            : render_post(post)
                        }
                    }
                }
            }
        }
    }).into_string().unwrap()
}

struct Post {
    title: String,
    tags: Vec<String>,
    body: String,
}

fn main() {
    let posts = vec![
        Post {
            title: String::from("First Post"),
            tags: vec![String::from("first post")],
            body: String::from("My Test Post"),
        },
        Post {
            title: String::from("Second Post"),
            tags: vec![],
            body: String::from("My Second Test Post"),
        },
    ];
    println!("{}", render("my blog", posts.into_iter()));
}
