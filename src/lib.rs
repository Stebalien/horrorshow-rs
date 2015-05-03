//! # Horrorshow
//!
//! An xml (mostly just the html subset) templating library.
//!
//! ## Example:
//!
//! ```
//! # #[macro_use] extern crate horrorshow;
//! # fn main() {
//! let actual = xml! {
//!     html {
//!         head {
//!             title { : "Hello world!" }
//!         }
//!         body {
//!             h1(id="heading") { : "Hello!" }
//!             p { : "Let's count to 10!" }
//!             ol(id="count") {
//!                 @ for i in 0..10 {
//!                     append_xml! {
//!                         li {
//!                             #{"{}", i+1 }
//!                         }
//!                     }
//!                 }
//!             }
//!             br; br;
//!             p {
//!                 : "Easy!"
//!             }
//!         }
//!     }
//! };
//!
//! let expected = "<html><head><title>Hello world!</title></head><body><h1 id=\"heading\">Hello!</h1><p>Let's count to 10!</p><ol id=\"count\"><li>1</li><li>2</li><li>3</li><li>4</li><li>5</li><li>6</li><li>7</li><li>8</li><li>9</li><li>10</li></ol><br /><br /><p>Easy!</p></body></html>";
//! assert_eq!(expected, actual);
//!
//! # }
//! ```
//!
//! Or cleaned up:
//!
//! ```html
//! <html>
//!   <head>
//!     <title>Hello world!</title>
//!   </head>
//!   <body>
//!     <h1 id="heading">Hello!</h1>
//!     <p>Let's count to 10!</p>
//!     <ol id="count">
//!       <li>1</li>
//!       <li>2</li>
//!       <li>3</li>
//!       <li>4</li>
//!       <li>5</li>
//!       <li>6</li>
//!       <li>7</li>
//!       <li>8</li>
//!       <li>9</li>
//!       <li>10</li>
//!     </ol>
//!     <br /><br />
//!     <p>Easy!</p>
//!   </body>
//! </html>
//! ```
//!
//! ## Usage
//!
//!
//! Inside an xml template, the following expressions are valid:
//!
//! * `some_tag;` -- Insert a the tag `some_tag`.
//!
//! * `some_tag(attr=rust_expresion,...);` -- Insert a the tag `some_tag` with the specified
//!    attributes. The attribute values will be evaluated as rust expressions at runtime.
//!
//! * `some_tag { ... }` -- Insert a the tag `some_tag` and recursivly evaluate the `...`.
//!
//! * `some_tag(...) { ... }` -- Same as above but with custom attributes.
//!
//! * `: rust_expression`, `: { rust_code }` -- Evaluate the expression or block and insert result current position.
//!
//! * `#{"format_str", rust_expressions... }` -- Format the arguments according to `format_str` and insert the
//! result at the current position.
//!
//! * `@ rust_expression`, `@ { rust_code }` -- Evaluate the expression or block.
//!
//! In rust code embedded inside of a template, you can invoke `append!("format_str", args...)` or
//! `append_xml! { xml_template... }` to append to the template at the current position. That's how
//! the for loop works in the example above.
//!
//! ## Notes:
//!
//! 1. This library does no escaping, sanitization. You have to do that yourself!
//! 2. There are bugs.
use std::cell::RefCell;

// TODO: Escape?

#[doc(hidden)]
thread_local!(pub static __TEMPLATE: RefCell<Option<String>> = RefCell::new(None));

#[macro_export]
macro_rules! xml {
    ($($inner:tt)*) => {{
        // The scoped variant is unstable
        use ::std::cell::RefCell;
        $crate::__TEMPLATE.with(|current| {
            let mut stash = Some(String::new());
            ::std::mem::swap(&mut *current.borrow_mut(), &mut stash);
            append_xml!($($inner)*);
            ::std::mem::swap(&mut *current.borrow_mut(), &mut stash);
            stash.unwrap()
        })
    }}
}

#[macro_export]
macro_rules! append {
    ($($tok:tt)+) => {{
        use ::std::fmt::Write;
        $crate::__TEMPLATE.with(|value| {
            // TODO: Handle errors?
            write!(value.borrow_mut().as_mut().unwrap(), $($tok)+).unwrap();
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
