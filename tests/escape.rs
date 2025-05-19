#![cfg(feature = "alloc")]

#[macro_use]
extern crate horrorshow;

use horrorshow::Escape;

#[test]
fn test_escape() {
    let html = html! {
        span : Escape(html! {
            b : "some <span>text</span>";
        })
    }
    .to_string();
    assert_eq!(
        html,
        "<span>&lt;b&gt;some &amp;lt;span&amp;gt;text&amp;lt;/span&amp;gt;&lt;/b&gt;</span>"
    );
}
