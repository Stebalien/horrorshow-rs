#![feature(scoped_tls)]
use std::cell::RefCell;
use std::fmt::Write;

// TODO: Escape?

scoped_thread_local!(pub static __TEMPLATE: RefCell<String>);

#[macro_export]
macro_rules! xml {
    ($($inner:tt)*) => {{
        let text = RefCell::new(String::new());
        __TEMPLATE.set(&text, || {
            append_xml!($($inner)*);
        });
        text.into_inner()
    }}
}

#[macro_export]
macro_rules! append {
    ($($tok:tt)+) => {{
        __TEMPLATE.with(|value| {
            // TODO: Handle errors?
            write!(value.borrow_mut(), $($tok)+).unwrap();
        });
    }}
}

#[macro_export]
macro_rules! append_xml {
    (: {$($code:expr);+} $($next:tt)*) => {{
        append!("{}", {$($code);+});
        append_xml!($($next)*);
    }};
    (: $code:expr; $($next:tt)* ) => {{
        append!("{}", $code);
        append_xml!($($next)*);
    }};
    (: $code:expr ) => {{
        append!("{}", $code);
    }};
    (@ {$($code:expr);+} $($next:tt)*) => {{
        $($code);+
        append_xml!($($next)*);
    }};
    (@ $code:expr; $($next:tt)* ) => {{
        $code;
        append_xml!($($next)*);
    }};
    (@ $code:expr ) => {{
        append_xml!(@ {$code});
    }};
    (#{$($tok:tt)+} $($next:tt)*) => {{
        append!($($tok)+);
        append_xml!($($next)*);
    }};
    ($tag:ident($($attr:ident=$value:expr),+) { $($children:tt)* } $($next:tt)* ) => {{
        append!(concat!("<", stringify!($tag), $(concat!(" ", stringify!($attr), "=\"{}\"")),+, ">"), $($value),+);
        append_xml!($($children)*);
        append!(concat!("</", stringify!($tag), ">"));
        append_xml!($($next)*);
    }};
    ($tag:ident($($attr:ident=$value:expr),+); $($next:tt)*) => {{
        append!(concat!("<", stringify!($tag), $(concat!(" ", stringify!($attr), "=\"{}\"")),+, " />"), $($value),+);
        append_xml!($($next)*);
    }};
    ($tag:ident($($attr:ident=$value:expr),+)) => {{
        append!(concat!("<", stringify!($tag), $(concat!(" ", stringify!($attr), "=\"{}\"")),+, "/>"), $($value),+);
    }};
    ($tag:ident { $($children:tt)* } $($next:tt)* ) => {{
        append!(concat!("<", stringify!($tag), ">"));
        append_xml!($($children)*);
        append!(concat!("</", stringify!($tag), ">"));
        append_xml!($($next)*);
    }};
    ($tag:ident; $($next:tt)*) => {{
        append!(concat!("<", stringify!($tag), " />"));
        append_xml!($($next)*);
    }};
    ($tag:ident) => {{
        append!(concat!("<", stringify!($tag), "/>"))
    }};
    () => {""};
}

#[test]
fn it_works() {
    let s = xml! {
        html {
            head {
                title { : "Hello world!" }
            }
            body {
                h1(id="heading") { : "Hello!" }
                p { : "Let's count to 10!" }
                ol(id="count") {
                    @ for i in 0..10 {
                        append_xml! {
                            li {
                                #{"{}", i+1 }
                            }
                        }
                    }
                }
                br; br;
                p {
                    : "Easy!"
                }
            }
        }
    };
    println!("{}", &s);
}
