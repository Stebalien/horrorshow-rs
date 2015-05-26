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
//!                 : raw!("Let's <i>count</i> to 10!")
//!             }
//!             ol(id="count") {
//!                 // run some inline code...
//!                 |mut tmpl| for i in 0..10 {
//!                     // append to the current template.
//!                     // store output because rust bug #25753
//!                     tmpl = try!(tmpl << html! {
//!                         li {
//!                             // format some text
//!                             #{"{}", i+1 }
//!                         }
//!                     });
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
//! * `: rust_expression`, `: { rust_code }` -- Evaluate the expression or block and insert result
//! current position. To insert literal html, mark it as raw with the `raw!` macro.
//!
//! * `#{"format_str", rust_expressions... }` -- Format the arguments according to `format_str` and insert the
//! result at the current position.
//!
//! * `|tmpl| rust_expression`, `|tmpl| { rust_code }` -- Evaluate the expression or block. This is
//! actually a closure so the block/expression can append to the current template through `tmpl`
//! (of type `&mut TemplateBuilder`).
use std::fmt;
use std::io;

#[macro_use]
mod html;


/// A component that can be appended to a template.
///
/// In a perfect world, I'd just use the Display but the string format system is REALLY slow.
pub trait TemplateComponent {
    #[inline]
    fn render_into<'a>(self, tmpl: &mut TemplateBuilder<'a>) -> Result<()>;
}

pub enum Error {
    Render(Box<std::error::Error>),
    Write(io::Error),
}

impl<E> From<E> for Error where E: Into<Box<std::error::Error>> {
    #[inline]
    fn from(e: E) -> Error {
        Error::Render(e.into())
    }
}

pub type Result<T> = std::result::Result<T, Error>;

/// A template renderer.
pub struct Renderer<F> {
    renderer: F,
    expected_size: usize,
}

impl<F> Renderer<F> where F: FnOnce(&mut TemplateBuilder) -> Result<()> {
    /// Render this template into a string.
    #[inline]
    pub fn render(self) -> Result<String> {
        let mut writer = String::with_capacity(self.expected_size);
        self.render_fmt(&mut writer).and(Ok(writer))
    }

    #[inline]
    pub fn render_fmt(self, writer: &mut fmt::Write) -> Result<()> {
        let mut tmpl = TemplateBuilder::new_fmt(writer);
        self.render_into(&mut tmpl)
    }

    #[inline]
    pub fn render_io(self, writer: &mut io::Write) -> Result<()> {
        let mut tmpl = TemplateBuilder::new_io(writer);
        self.render_into(&mut tmpl)
    }
}

impl<F> TemplateComponent for Renderer<F> where F: FnOnce(&mut TemplateBuilder) -> Result<()> {
    #[inline]
    fn render_into(self, tmpl: &mut TemplateBuilder) -> Result<()> {
        (self.renderer)(tmpl)
    }
}

/// Raw content.
///
/// When rendered, raw content will not be escaped.
pub struct Raw<S: AsRef<str>>(S);

impl<S> Raw<S> where S: AsRef<str> {
    /// Mark as raw.
    #[inline]
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
    fn render_into(self, tmpl: &mut TemplateBuilder) -> Result<()> {
        tmpl.write_raw(self.0.as_ref())
    }
}

impl<T, E> TemplateComponent for std::result::Result<T, E>
    where T: TemplateComponent,
          E: Into<Box<std::error::Error>>
{
    #[inline]
    fn render_into(self, tmpl: &mut TemplateBuilder) -> Result<()> {
        try!(self).render_into(tmpl)
    }
}


impl<'a> TemplateComponent for &'a str {
    #[inline]
    fn render_into(self, tmpl: &mut TemplateBuilder) -> Result<()> {
        tmpl.write_str(self)
    }
}

impl<'a> TemplateComponent for &'a String {
    #[inline]
    fn render_into(self, tmpl: &mut TemplateBuilder) -> Result<()> {
        tmpl.write_str(&self)
    }
}

impl TemplateComponent for String {
    #[inline]
    fn render_into(self, tmpl: &mut TemplateBuilder) -> Result<()> {
        tmpl.write_str(&self)
    }
}

impl<'a, 'b, T> std::ops::Shl<T> for &'a mut TemplateBuilder<'b> where T: TemplateComponent {
    type Output = Result<&'a mut TemplateBuilder<'b>>;
    /// Render the component into the template.
    ///
    /// Note: If writing to the template fails, this method will neither panic nor return errors.
    /// Instead, no more data will be written to the template and horrorshow abort template
    /// rendering (return an error) when it re-gains control.
    #[inline]
    fn shl(self, component: T) -> Result<&'a mut TemplateBuilder<'b>> {
        component.render_into(self).and(Ok(self))
    }
}

/// TemplateBuilder builder.
pub struct TemplateBuilder<'a>(TemplateWriter<'a>);

enum TemplateWriter<'a> {
    Io {
        writer: &'a mut io::Write,
        error: Option<io::Error>,
    },
    Fmt {
        writer: &'a mut fmt::Write,
    }
}

impl<'a> TemplateWriter<'a> {
    #[inline]
    fn take_error(&mut self) -> io::Error {
        use TemplateWriter::*;
        match self {
            &mut Io { ref mut error, .. } => {
                error.take().unwrap()
            },
            &mut Fmt { .. } => {
                // TODO: error::Error not implemented for fmt::Error
                io::Error::new(io::ErrorKind::Other, "format error")
            },
        }
    }
}


#[doc(hidden)]
pub fn __new_renderer<F: FnOnce(&mut TemplateBuilder) -> Result<()>>(expected_size: usize, f: F) -> Renderer<F> {
    Renderer {
        renderer: f,
        expected_size: expected_size,
    }
}

impl<'a> TemplateBuilder<'a> {
    #[inline]
    fn new_fmt(w: &mut fmt::Write) -> TemplateBuilder {
        TemplateBuilder(TemplateWriter::Fmt { writer: w })
    }
    #[inline]
    fn new_io(w: &mut io::Write) -> TemplateBuilder {
        TemplateBuilder(TemplateWriter::Io { writer: w, error: None })
    }
    /// Append a raw string to the template.
    #[inline]
    pub fn write_raw(&mut self, text: &str) -> Result<()> {
        use TemplateWriter::*;
        use std::fmt::Write;
        match self.0 {
            Io { ref mut writer, .. } => {
                writer.write_all(text.as_bytes()).map_err(|e| Error::Write(e))
            },
            Fmt {ref mut writer, .. } => {
                // TODO: error::Error not implemented for fmt::Error
                writer.write_str(text).map_err(|_| Error::Write(io::Error::new(io::ErrorKind::Other, "format error")))
            },
        }
    }

    #[inline]
    pub fn write_fmt(&mut self, args: fmt::Arguments) -> Result<()> {
        use std::fmt::Write;
        match self.0.write_fmt(args) {
            Err(_) => Err(Error::Write(self.0.take_error())),
            Ok(()) => Ok(()),
        }
    }

    #[inline]
    pub fn write_str(&mut self, text: &str) -> Result<()> {
        use std::fmt::Write;
        match self.0.write_str(text) {
            Err(_) => Err(Error::Write(self.0.take_error())),
            Ok(()) => Ok(()),
        }
    }
}

impl<'a> fmt::Write for TemplateWriter<'a> {
    /// Escape and write a string to the template.
    #[inline]
    fn write_str(&mut self, text: &str) -> fmt::Result {
        use TemplateWriter::*;
        match self {
            &mut Io { ref mut writer, ref mut error } => {
                for b in text.bytes() {
                    if let Err(e) = match b {
                        b'&' => writer.write_all("&amp;".as_bytes()),
                        b'"' => writer.write_all("&quot;".as_bytes()),
                        b'<' => writer.write_all("&lt;".as_bytes()),
                        b'>' => writer.write_all("&gt;".as_bytes()),
                        _ => writer.write_all(&[b] as &[u8]),
                    } {
                        *error = Some(e);
                        return Err(fmt::Error);
                    }
                }
            },
            &mut Fmt { ref mut writer } => {
                for c in text.chars() {
                    try!(match c {
                        '&' => writer.write_str("&amp;"),
                        '"' => writer.write_str("&quot;"),
                        '<' => writer.write_str("&lt;"),
                        '>' => writer.write_str("&gt;"),
                        _ => writer.write_char(c),
                    });
                }
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
