#[macro_use]
extern crate horrorshow;

use horrorshow::Template;

#[test]
fn test_multi() {
    assert_eq!(&html! {
        html body p : "Some content";
    }.into_string(), "<html><body><p>Some content</p></body></html>");

    assert_eq!(&html! {
        html(lang="en") body p : "Some content";
    }.into_string(), "<html lang=\"en\"><body><p>Some content</p></body></html>");

    assert_eq!(&html! {
        html(lang="en") body p(id="test") : "Some content";
    }.into_string(), "<html lang=\"en\"><body><p id=\"test\">Some content</p></body></html>");

    assert_eq!(&html! {
        html(lang="en") body(class="body") p(id="test") { : "Some content" }
    }.into_string(), "<html lang=\"en\"><body class=\"body\"><p id=\"test\">Some content</p></body></html>");
}

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
fn test_option() {
    assert_eq!(html! {
        tag : Some("testing")
    }.into_string(), "<tag>testing</tag>");

    assert_eq!(html! {
        tag : None::<&str>
    }.into_string(), "<tag></tag>");
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
fn test_embed_twice() {
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
