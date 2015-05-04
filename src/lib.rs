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
//!             h1(id="heading") {
//!                 // Insert escaped text (actually, anything that defines Display)
//!                 : "Hello! This is <html />"
//!             }
//!             p {
//!                 // Insert raw text (unescaped)
//!                 ! "Let's <i>count</i> to 10!"
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
//! let expected = "<html><head><title>Hello world!</title></head><body><h1 id=\"heading\">Hello! This is &lt;html /&gt;</h1><p>Let's <i>count</i> to 10!</p><ol id=\"count\"><li>1</li><li>2</li><li>3</li><li>4</li><li>5</li><li>6</li><li>7</li><li>8</li><li>9</li><li>10</li></ol><br /><br /><p>Easy!</p></body></html>";
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
use std::fmt;

#[macro_use]
mod html;

thread_local!(static __TEMPLATE: RefCell<Option<Template>> = RefCell::new(None));

/// Private helper for storing template output. We need this to do escaping.
#[doc(hidden)]
struct Template {
    data: String,
    escape: bool,
}

impl Template {
    #[inline]
    fn with_capacity(n: usize) -> Template {
        Template {
            data: String::with_capacity(n),
            escape: false,
        }
    }
}

impl fmt::Write for Template {
    #[inline]
    fn write_str(&mut self, text: &str) -> fmt::Result {
        if self.escape {
            self.data.reserve(text.len());
            for c in text.chars() {
                let _ = self.write_char(c);
            }
        } else {
            self.data.push_str(text);
        }
        Ok(())
    }
    #[inline]
    fn write_char(&mut self, c: char) -> fmt::Result {
        match c {
            '&' => self.data.push_str("&amp;"),
            '"' => self.data.push_str("&quot;"),
            '<' => self.data.push_str("&lt;"),
            '>' => self.data.push_str("&gt;"),
            _ => self.data.push(c),
        }
        Ok(())
    }
}

/// Call `f` with a new template scope.
///
/// Returns the evaluated template.
#[doc(hidden)]
#[inline]
pub fn __with_template_scope<F: FnMut()>(n: usize, mut f: F) -> String {
    // The scoped variant is unstable so we do this ourselves...
    __TEMPLATE.with(|current| {
        let mut stash = Some(Template::with_capacity(n));
        ::std::mem::swap(&mut *current.borrow_mut(), &mut stash);
        (f)();
        ::std::mem::swap(&mut *current.borrow_mut(), &mut stash);
        stash.unwrap().data
    })
}

/// Call `f` with a mutable reference to the current template string.
///
/// Returns the evaluated template.
#[doc(hidden)]
#[inline]
pub fn __with_template<F: FnMut(&mut Template)>(escape: bool, mut f: F) {
    // The scoped variant is unstable so we do this ourselves...
    __TEMPLATE.with(|template| {
        // Make the borrow checker happy...
        let mut borrow = template.borrow_mut();
        let inner = borrow.as_mut().unwrap();
        let old_escape = inner.escape;
        inner.escape = escape;
        (f)(inner);
        inner.escape = old_escape;
    });
}

#[macro_export]
macro_rules! append_raw {
    ($($tok:tt)+) => {{
        use ::std::fmt::Write;
        $crate::__with_template(false, |template| {
            // TODO: Handle errors?
            write!(template, $($tok)+).unwrap();
        });
    }}
}

#[macro_export]
macro_rules! append {
    ($($tok:tt)+) => {{
        use ::std::fmt::Write;
        $crate::__with_template(true, |template| {
            // TODO: Handle errors?
            write!(template, $($tok)+).unwrap();
        });
    }}
}
