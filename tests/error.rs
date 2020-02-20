#![cfg(feature = "alloc")]

#[macro_use]
extern crate horrorshow;
use horrorshow::Template;

#[test]
#[cfg(feature = "std")]
fn test_result() {
    let mut s = String::new();
    let e = (html! {
        : Ok::<&str, &str>("pass");
        : Err::<&str, &str>("fail");
        : Ok::<&str, &str>("hidden");
        : Err::<&str, &str>("fail2");
    })
    .write_to_string(&mut s)
    .err()
    .unwrap();
    assert_eq!(s, "pass");
    assert!(e.write.is_none());
    assert_eq!(e.render.len(), 2);
    assert_eq!(&e.render[0].to_string(), "fail");
    assert_eq!(&e.render[1].to_string(), "fail2");
}

#[test]
fn test_record() {
    let e = (html! {
        tag {
            |tmpl| tmpl.record_error("test");
        }
    })
    .into_string()
    .err()
    .unwrap();
    assert!(e.write.is_none());
    assert_eq!(e.render.len(), 1);
    assert_eq!(&e.render[0].to_string(), "test");
}
