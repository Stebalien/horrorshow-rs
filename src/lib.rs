//! # Horrorshow
//!
//! An html templating library.
//!
//! ## Example:
//!
//! ```
//! # #[macro_use] extern crate horrorshow;
//! # fn main() {
//! let actual = html! {
//!     html {
//!         head {
//!             title { : "Hello world!" }
//!         }
//!         body {
//!             // attributes
//!             h1(id="heading") { : "Hello!" }
//!             p {
//!                 // Insert text (actually, anything that defines Display)
//!                 : "Let's count to 10!"
//!             }
//!             ol(id="count") {
//!                 // run some inline code...
//!                 @ for i in 0..10 {
//!                     // append to the current template.
//!                     append_html! {
//!                         li {
//!                             // format some text
//!                             #{"{}", i+1 }
//!                         }
//!                     }
//!                 }
//!             }
//!             // You need semi-colons for tags without children.
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
//! Inside an html template, the following expressions are valid:
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
//! `append_html! { html_template... }` to append to the template at the current position. That's how
//! the for loop works in the example above.
//!
//! ## Notes:
//!
//! 1. This library does no escaping, sanitization. You have to do that yourself!
//! 2. There are bugs.
use std::cell::RefCell;

#[macro_use]
mod html;

// TODO: Escape?

thread_local!(static __TEMPLATE: RefCell<Option<String>> = RefCell::new(None));

/// Call `f` with a new template scope.
///
/// Returns the evaluated template.
#[doc(hidden)]
#[inline]
pub fn __with_template_scope<F: FnMut()>(mut f: F) -> String {
    // The scoped variant is unstable so we do this ourselves...
    __TEMPLATE.with(|current| {
        let mut stash = Some(String::new());
        ::std::mem::swap(&mut *current.borrow_mut(), &mut stash);
        (f)();
        ::std::mem::swap(&mut *current.borrow_mut(), &mut stash);
        stash.unwrap()
    })
}

/// Call `f` with a mutable reference to the current template string.
///
/// Returns the evaluated template.
#[doc(hidden)]
#[inline]
pub fn __with_template<F: FnMut(&mut String)>(mut f: F) {
    // The scoped variant is unstable so we do this ourselves...
    __TEMPLATE.with(|template| (f)(template.borrow_mut().as_mut().unwrap()));
}

#[macro_export]
macro_rules! append {
    ($($tok:tt)+) => {{
        use ::std::fmt::Write;
        $crate::__with_template(|template| {
            // TODO: Handle errors?
            write!(template, $($tok)+).unwrap();
        });
    }}
}
