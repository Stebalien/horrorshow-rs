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
//!                 // Insert escaped text
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
//! In rust code embedded inside of a template, you can append text with any of the following
//! macros:
//!
//! * `append_fmt!("format_str", args...)` -- format, escape, and append arguments
//! * `append_raw!(text)` -- append text without escaping
//! * `append!(text)` -- escape and append text
//! * `append_html! { html_template... }` -- append an html template.
use std::cell::RefCell;
use std::fmt;

#[macro_use]
mod html;

thread_local!(static __TEMPLATE: RefCell<Option<Template>> = RefCell::new(None));

/// Private helper for storing template output. We need this to do escaping.
#[doc(hidden)]
pub struct Template(String);

impl Template {
    #[inline]
    fn with_capacity(n: usize) -> Template {
        Template(String::with_capacity(n))
    }
    #[inline]
    pub fn write_raw(&mut self, text: &str) {
        self.0.push_str(text);
    }
}

impl fmt::Write for Template {
    #[inline]
    fn write_str(&mut self, text: &str) -> fmt::Result {
        for b in text.bytes() {
            match b {
                b'&' => self.0.push_str("&amp;"),
                b'"' => self.0.push_str("&quot;"),
                b'<' => self.0.push_str("&lt;"),
                b'>' => self.0.push_str("&gt;"),
                // This is safe because we're working bytewise.
                _ => unsafe { self.0.as_mut_vec() }.push(b)
            }
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
        stash.unwrap().0
    })
}

/// Call `f` with a mutable reference to the current template string.
/// NOT REENTRANT!
/// Returns the evaluated template.
#[doc(hidden)]
#[inline]
pub fn __with_template<F: FnMut(&mut Template)>(mut f: F) {
    // The scoped variant is unstable so we do this ourselves...
    __TEMPLATE.with(|template| {
        (f)(template.borrow_mut().as_mut().unwrap());
    });
}

/// Call `f` with a mutable reference to the current template string.
/// This is the slower reentrant version.
///
/// Returns the evaluated template.
#[doc(hidden)]
#[inline]
pub fn __with_template_reentrant<F: FnMut(&mut Template)>(mut f: F) {
    // The scoped variant is unstable so we do this ourselves...
    __TEMPLATE.with(|template| {
        let mut local_template = None;
        ::std::mem::swap(&mut *template.borrow_mut(), &mut local_template);
        (f)(local_template.as_mut().unwrap());
        ::std::mem::swap(&mut *template.borrow_mut(), &mut local_template);
    });
}

/// Append text without escaping.
#[macro_export]
macro_rules! append_raw {
    ($s:expr) => {{
        let output = $s;
        let s: &str = &output;
        $crate::__with_template(|template| {
            template.write_raw(s);
        });
    }}
}

/// Format, escape, and append arguments.
#[macro_export]
macro_rules! append_fmt {
    ($($tok:tt)+) => {{
        use ::std::fmt::Write;
        $crate::__with_template_reentrant(|template| {
            write!(template, $($tok)+).unwrap();
        });
    }}
}

/// Escape and append text. 
#[macro_export]
macro_rules! append {
    ($s:expr) => {{
        use ::std::fmt::Write;
        let output = $s;
        let s: &str = &output;
        $crate::__with_template(|template| {
            template.write_str(s).unwrap();
        });
    }}
}

// We shouldn't need this but without it I get the folloowing error:
// error: unexpected token: `an interpolated tt`
#[macro_export]
#[doc(hidden)]
macro_rules! __horrorshow_block_identity {
    ($b:block) => { $b };
}
