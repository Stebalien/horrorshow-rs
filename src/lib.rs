//! # Horrorshow
//!
//! An xml templating library.
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
//!             // attributes
//!             h1(id="heading") {
//!                 // Insert escaped text
//!                 : "Hello! This is <html />"
//!             }
//!             p {
//!                 // Insert raw text (unescaped)
//!                 : raw!("Let's <i>count</i> to 10!")
//!             }
//!             ol(id="count") {
//!                 // run some inline code...
//!                 |mut tmpl| for i in 0..10 {
//!                     // append to the current template.
//!                     // store output because rust bug #25753
//!                     tmpl = tmpl << xml! {
//!                         li {
//!                             // format some text
//!                             #{"{}", i+1 }
//!                         }
//!                     };
//!                 }
//!             }
//!             // You need semi-colons for tags without children.
//!             br; br;
//!             p {
//!                 : "Easy!"
//!             }
//!         }
//!     }
//! }.render();
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
//! * `: rust_expression`, `: { rust_code }` -- Evaluate the expression or block and insert result
//! current position. To insert literal xml, mark it as raw with the `raw!` macro.
//!
//! * `#{"format_str", rust_expressions... }` -- Format the arguments according to `format_str` and insert the
//! result at the current position.
//!
//! * `|tmpl| rust_expression`, `|tmpl| { rust_code }` -- Evaluate the expression or block. This is
//! actually a closure so the block/expression can append to the current template through `tmpl`
//! (of type `&mut Template`).
use std::fmt;
use std::fmt::Write;

#[macro_use]
mod xml;


/// A component that can be appended to a template.
///
/// In a perfect world, I'd just use the Display but the string format system is REALLY slow.
pub trait TemplateComponent {
    fn render_into(self, tmpl: &mut Template);
}

/// A template renderer.
pub struct Renderer<F> {
    renderer: F,
    expected_size: usize,
}


impl<F> Renderer<F> where F: FnOnce(&mut Template) {
    /// Render this template into a string.
    pub fn render(self) -> String {
        let mut tmpl = Template::with_capacity(self.expected_size);
        self.render_into(&mut tmpl);
        tmpl.0
    }
}

impl<F> TemplateComponent for Renderer<F> where F: FnOnce(&mut Template) {
    fn render_into(self, tmpl: &mut Template) {
        (self.renderer)(tmpl);
    }
}

/// Raw content.
///
/// When rendered, raw content will not be escaped.
pub struct Raw<S: AsRef<str>>(S);

impl<S> Raw<S> where S: AsRef<str> {
    /// Mark as raw.
    pub fn new(content: S) -> Raw<S> {
        Raw(content)
    }
}

/// Mark a string as a raw. The string will not be rendered.
#[macro_export]
macro_rules! raw {
    ($e:expr) => { $crate::Raw::new($e) }
}

impl<S> TemplateComponent for Raw<S> where S: AsRef<str> {
    #[inline]
    fn render_into(self, tmpl: &mut Template) {
        tmpl.write_raw(self.0.as_ref());
    }
}


impl<'a> TemplateComponent for &'a str {
    #[inline]
    fn render_into(self, tmpl: &mut Template) {
        tmpl.write_str(self).unwrap();
    }
}

impl<'a> TemplateComponent for &'a String {
    #[inline]
    fn render_into(self, tmpl: &mut Template) {
        tmpl.write_str(&self).unwrap();
    }
}

impl TemplateComponent for String {
    #[inline]
    fn render_into(self, tmpl: &mut Template) {
        tmpl.write_str(&self).unwrap();
    }
}

impl<'a, T> std::ops::Shl<T> for &'a mut Template where T: TemplateComponent {
    type Output = &'a mut Template ;
    #[inline]
    fn shl(self, component: T) -> &'a mut Template {
        component.render_into(self);
        self
    }
}

/// Template builder.
pub struct Template(String);

#[doc(hidden)]
pub fn __new_renderer<F: FnOnce(&mut Template)>(expected_size: usize, f: F) -> Renderer<F> {
    Renderer {
        renderer: f,
        expected_size: expected_size,
    }
}

impl Template {
    /// Create a new template builder.
    #[inline]
    pub fn new() -> Template {
        Template(String::new())
    }
    /// Create a new template builder with the given initial capacity.
    #[inline]
    pub fn with_capacity(capacity: usize) -> Template {
        Template(String::with_capacity(capacity))
    }

    /// Append a raw string to the template.
    #[inline]
    pub fn write_raw(&mut self, text: &str) {
        self.0.push_str(text);
    }
}

impl std::ops::Deref for Template {
    type Target = str;
    #[inline]
    fn deref(&self) -> &str {
        &self.0
    }
}

impl AsRef<str> for Template {
    #[inline]
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl From<Template> for String {
    #[inline]
    fn from(from: Template) -> String {
        from.0
    }
}

impl fmt::Write for Template {
    /// Escape and write a string to the template.
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

// We shouldn't need this but without it I get the folloowing error:
// error: unexpected token: `an interpolated tt`
#[macro_export]
#[doc(hidden)]
macro_rules! __horrorshow_block_identity {
    ($b:block) => { $b };
}
