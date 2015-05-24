#[macro_use]
extern crate horrorshow;

#[test]
fn test_reentrant() {
    assert_eq!(&xml! {
        p {
            #{"{}", xml! { a(href="abcde") }.render()}
        }
    }.render(), "<p>&lt;a href=&quot;abcde&quot; /&gt;</p>");

    assert_eq!(&xml! {
        p {
            |tmpl| tmpl << xml! { a(href="abcde") }.render();
        }
    }.render(), "<p>&lt;a href=&quot;abcde&quot; /&gt;</p>");

    assert_eq!(&xml! {
        p {
            ! xml! { a(href="abcde") }.render();
        }
    }.render(), "<p><a href=\"abcde\" /></p>");
}

#[test]
fn test_namespace() {
    assert_eq!(xml! {
        a:first {
            b:inner;
        }
        c:second(e:id="testing");
        d:last;
    }.render(), "<a:first><b:inner /></a:first><c:second e:id=\"testing\" /><d:last />");
}


#[test]
fn test_dash() {
    assert_eq!(xml! {
        my_tag {
            inner(data-test="abcde");
        }
    }.render(), "<my_tag><inner data-test=\"abcde\" /></my_tag>");
}
