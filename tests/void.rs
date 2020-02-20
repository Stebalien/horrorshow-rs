#![cfg(feature = "alloc")]

#[macro_use]
extern crate horrorshow;

use horrorshow::Template;

#[test]
fn test_void() {
    assert_eq!(
        html! {
            div;
            br;
            input(foo="bar");
            img
        }
        .into_string()
        .unwrap(),
        "<div></div><br><input foo=\"bar\"><img>"
    );
}
