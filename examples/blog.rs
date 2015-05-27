#[macro_use]
extern crate horrorshow;

fn main() {
    fn post<'a>(title: &'a str) -> Box<horrorshow::RenderBox + 'a> {
        box_html! {
            article {
                title { h1 : title }
                p : "This is one paragraph.";
                p : "This is a second.";
            }
        }
    }
    println!("{}", html! {
        html {
            body {
                : post("First Post");
                |t| (0..10).fold(t, |t, i| {
                    // Waiting for non-lexical borrows!!!!
                    let tmp = format!("Spam post {}", i);
                    let post = post(&tmp);
                    t << post
                })
            }
        }
    });
}
