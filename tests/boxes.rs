#![cfg(feature = "std")]

#[macro_use]
extern crate horrorshow;

use horrorshow::{RenderBox, RenderOnce, Template};

#[test]
fn test_box_render_once_send() {
    let x: Box<dyn RenderBox + Send> = Box::new(html! {});
    let mut v = String::new();
    x.write_to_fmt(&mut v).unwrap();
}

#[test]
fn test_box_render_size() {
    let x: Box<dyn RenderBox> = Box::new(html! {});
    assert_eq!(x.size_hint(), 0);

    let x: Box<dyn RenderBox + Send> = Box::new(html! {});
    assert_eq!(x.size_hint(), 0);
}
