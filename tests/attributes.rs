#[macro_use]
extern crate horrorshow;

use horrorshow::Template;


#[test]
fn test_dash() {
    assert_eq!(html! {
        my_tag {
            inner(data-test="abcde");
        }
    }.into_string(), "<my_tag><inner data-test=\"abcde\" /></my_tag>");
}

#[test]
fn test_no_value() {
    assert_eq!(html! {
        my_tag {
            inner(a="test", some_tag?, other="1");
        }
    }.into_string(), "<my_tag><inner a=\"test\" some_tag other=\"1\" /></my_tag>");
}

#[test]
fn test_boolean() {
    assert_eq!(html! {
        tag(flag?=true);
    }.into_string(), "<tag flag />");

    assert_eq!(html! {
        tag(flag?=false);
    }.into_string(), "<tag />");
}

#[test]
fn test_fmt() {
    assert_eq!(html! {
        tag(attr = #{"{}", 1});
    }.into_string(), "<tag attr=\"1\" />");
}
