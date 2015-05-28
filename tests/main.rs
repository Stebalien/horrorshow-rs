#[macro_use]
extern crate horrorshow;

use horrorshow::Template;

#[test]
fn test_reentrant() {
    assert_eq!(&html! {
        p {
            #{"{}", html! { a(href="abcde") }.into_string()}
        }
    }.into_string(), "<p>&lt;a href=&quot;abcde&quot; /&gt;</p>");

    assert_eq!(&html! {
        p {
            |tmpl| tmpl << (html! { a(href="abcde") }).into_string();
        }
    }.into_string(), "<p>&lt;a href=&quot;abcde&quot; /&gt;</p>");

    assert_eq!(&html! {
        p {
            : raw!(html! { a(href="abcde") }.into_string());
        }
    }.into_string(), "<p><a href=\"abcde\" /></p>");
}

#[test]
fn test_dash() {
    assert_eq!(html! {
        my_tag {
            inner(data-test="abcde");
        }
    }.into_string(), "<my_tag><inner data-test=\"abcde\" /></my_tag>");
}

#[test]
fn test_attr_no_value() {
    assert_eq!(html! {
        my_tag {
            inner(a="test", some_tag, other="1");
        }
    }.into_string(), "<my_tag><inner a=\"test\" some_tag other=\"1\" /></my_tag>");
}

#[test]
fn test_attr_fmt() {
    assert_eq!(html! {
        tag(attr = #{"{}", 1});
    }.into_string(), "<tag attr=\"1\" />");
}


#[test]
fn test_into_string_by_ref() {
    let r = html! {
        |tmpl| tmpl << "test";
    };
    assert_eq!((&r).into_string(), "test");
    assert_eq!((&r).into_string(), "test");
}

#[test]
fn test_enbed_twice() {
    let r = html! {
        |tmpl| {
            let sub = html! { : "abcde" };
            tmpl << &sub << &sub;
        }
    };
    assert_eq!(r.into_string(), "abcdeabcde");
}

#[test]
fn test_display() {
    use std::fmt::Write;
    let r = html! {
        |tmpl| tmpl << "test";
    };
    let mut s = String::new();
    write!(s, "{}", r).unwrap();
    assert_eq!(&s, "test");
}
