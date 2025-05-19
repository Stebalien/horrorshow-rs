#![cfg(feature = "alloc")]

#[macro_use]
extern crate horrorshow;

use horrorshow::{Concat, Join, Template};

#[test]
fn test_cat() {
    assert_eq!(
        html! {
            : Concat(&["a", "b", "c"])
        }
        .into_string()
        .unwrap(),
        "abc"
    );
}

#[test]
fn test_join() {
    assert_eq!(
        html! {
            : Join(" ", &["a", "b", "c"])
        }
        .into_string()
        .unwrap(),
        "a b c"
    );
}
