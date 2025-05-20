#![cfg(feature = "alloc")]

use horrorshow::Template;

#[macro_use]
extern crate horrorshow;

#[test]
fn test_xml_close() {
    assert_eq!(
        xml! {
            root {
                link(href = "foobar");
            }
        }
        .into_string()
        .unwrap(),
        "<root><link href=\"foobar\"/></root>",
    );
}

#[test]
fn test_xml_bool_attr() {
    assert_eq!(
        xml! {
            root {
                first(attr);
                second(attr ?= true);
                second(attr ?= false);
            }
        }
        .into_string()
        .unwrap(),
        "<root><first attr=\"attr\"/><second attr=\"attr\"/><second/></root>",
    );
}
