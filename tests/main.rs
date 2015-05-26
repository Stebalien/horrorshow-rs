#[macro_use]
extern crate horrorshow;

#[test]
fn test_reentrant() {
    assert_eq!(&html! {
        p {
            #{"{}", html! { a(href="abcde") }.render()}
        }
    }.render(), "<p>&lt;a href=&quot;abcde&quot; /&gt;</p>");

    assert_eq!(&html! {
        p {
            |tmpl| tmpl << (html! { a(href="abcde") }).render();
        }
    }.render(), "<p>&lt;a href=&quot;abcde&quot; /&gt;</p>");

    assert_eq!(&html! {
        p {
            : raw!(html! { a(href="abcde") }.render());
        }
    }.render(), "<p><a href=\"abcde\" /></p>");
}

#[test]
fn test_dash() {
    assert_eq!(html! {
        my_tag {
            inner(data-test="abcde");
        }
    }.render(), "<my_tag><inner data-test=\"abcde\" /></my_tag>");
}


#[test]
fn test_render_by_ref() {
    let r = html! {
        |tmpl| tmpl << "test";
    };
    assert_eq!((&r).render(), "test");
    assert_eq!((&r).render(), "test");
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
