#[macro_use]
extern crate horrorshow;

use horrorshow::{RenderBox, Template};

#[test]
#[allow(unused_variables)]
fn test_box_render_once_send() {
    let x: Box<RenderBox + Send> = Box::new(html!{});
    let mut v = Vec::new();
    x.write_to_io(&mut v).unwrap();
}
