#[macro_use]
extern crate horrorshow;

use horrorshow::{Concat, Template};

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
