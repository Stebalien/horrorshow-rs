#[macro_use]
extern crate horrorshow;

use horrorshow::Template;

#[test]
fn test_if() {
    assert_eq!(html! {
        @ if true {
            span : "test"
        }
    }.into_string().unwrap(), "<span>test</span>");
}

#[test]
fn test_if_let() {
    assert_eq!(html! {
        @ if let Some(v) = Some(1) {
            span : v
        }
    }.into_string().unwrap(), "<span>1</span>");
}

#[test]
fn test_if_else() {
    assert_eq!(html! {
        @ if true {
            span : "yes"
        } else {
            span : "no"
        }
    }.into_string().unwrap(), "<span>yes</span>");

    assert_eq!(html! {
        @ if false {
            span : "no"
        } else if true {
            span : "yes"
        }
    }.into_string().unwrap(), "<span>yes</span>");

    assert_eq!(html! {
        @ if false {
            span : "no"
        } else if false {
            span : "no"
        } else {
            span : "yes"
        }
    }.into_string().unwrap(), "<span>yes</span>");

    assert_eq!(html! {
        @ if false {
            span : "no"
        } else if let Some(v) = Some(1) {
            span : v
        } else {
            span : "yes"
        }
    }.into_string().unwrap(), "<span>1</span>");
}

/*
#[test]
fn test_for() {
    assert_eq!(html! {
        ol {
            @ for a in 0..2 {
                li : a
            }
        }
    }.into_string().unwrap(), "<ol><li>0</li><li>1</li></ol>");
}
*/

#[test]
fn test_while() {
    let mut i = 2;
    assert_eq!(html! {
        ol {
            @ while i > 0 {
                li : {
                    i -= 1;
                    i
                };
            }
        }
    }.into_string().unwrap(), "<ol><li>1</li><li>0</li></ol>");
}

#[test]
fn test_while_let() {
    let mut iter = 0..2;
    assert_eq!(html! {
        ol {
            @ while let Some(v) = iter.next() {
                li : v
            }
        }
    }.into_string().unwrap(), "<ol><li>0</li><li>1</li></ol>");
}
