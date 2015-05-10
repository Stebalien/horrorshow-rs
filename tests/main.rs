#[macro_use]
extern crate horrorshow;

#[test]
fn test_reentrant() {
    assert_eq!(&html! {
        p {
            #{"{}", html! { a(href="abcde") }}
        }
    }, "<p>&lt;a href=&quot;abcde&quot; /&gt;</p>");

    assert_eq!(&html! {
        p {
            @ append!(html! { a(href="abcde") });
        }
    }, "<p>&lt;a href=&quot;abcde&quot; /&gt;</p>");

    assert_eq!(&html! {
        p {
            ! html! { a(href="abcde") };
        }
    }, "<p><a href=\"abcde\" /></p>");
}
