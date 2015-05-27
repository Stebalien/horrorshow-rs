//! # Horrorshow
//!
//! An html templating library.
//!
//! ## Example:
//!
//! ```
//! # #[macro_use] extern crate horrorshow;
//! # fn main() {
//! use horrorshow::RenderOnce;
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
//!                     tmpl = tmpl << html! {
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
//! Inside an html template, the following expressions are valid:
//!
//! * `some_tag;` -- Insert a the tag `some_tag`.
//!
//! * `some_tag(attr=rust_expresion,...);` -- Insert a the tag `some_tag` with the specified
//!    attributes. The attribute values will be evaluated as rust expressions at runtime.
//!
//! * `some_tag { ... }` -- Insert the tag `some_tag` and recursively evaluate the `...`.
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
//!
//! ## Traits, traits oh-my!
//!
//! You will likely notice that there are three(!) render traits:
//!
//! 1. `RenderOnce`
//! 2. `RenderMut`
//! 3. `Render`
//!
//! These three traits map to the three `Fn` traits and reflect the fact that some templates need
//! exclusive access (`RenderMut`) in order to be rendered and others might even consume their
//! environment (`RenderOnce`).
//!
//! In general, just import `RenderOnce` into your environment. `RenderOnce` is implemented on
//! `&mut T where T: RenderMut` and `&T where T: Render` so you don't need the other traits
//! in-scope to render. As a matter of fact, having the other traits in-scope is a bad idea because
//! rust will have a hard time picking the right method.
//!
//! ## Error Handling
//!
//! IO errors (writing to the buffer) are handled in the background. If an io (or fmt) error
//! occurs, template rendering will continue but no more data will be written and the original
//! `render_fmt`/`render_io` call will return the error when rendering terminates.
//!
//! There is no way to abort template rendering other than panicing. Try to do everything that can
//! fail before rendering a template.
use std::fmt;
use std::io;

#[macro_use]
mod html;


/// Something that can be rendered once.
pub trait RenderOnce: Sized {
    /// Render this into a new String.
    fn render(self) -> String {
        let mut string = String::with_capacity(self.size_hint());
        self.render_string(&mut string);
        string
    }

    /// Render this into an existing String.
    ///
    /// Note: You could also use render_into_fmt but this is noticeably faster.
    fn render_string(self, string: &mut String) {
        let mut tmpl = TemplateBuilder::new_str(string);
        self.render_tmpl(&mut tmpl);
    }

    /// Render this into something that implements fmt::Write.
    /// 
    /// Renderer also implements Display but that's about twice as slow...
    fn render_fmt(self, writer: &mut fmt::Write) -> Result<(), fmt::Error> {
        let mut tmpl = TemplateBuilder::new_fmt(writer);
        self.render_tmpl(&mut tmpl);
        if let Some(e) = tmpl.0.fmt_error() { Err(e) } else { Ok(()) }
    }

    /// Render this into something that implements io::Write.
    ///
    /// Note: If you're writing directly to a file/socket etc., you should *seriously* consider
    /// wrapping your writer in a BufWriter. Otherwise, you'll end up making quite a few unnecessary
    /// system calls.
    fn render_io(self, writer: &mut io::Write) -> Result<(), io::Error> {
        let mut tmpl = TemplateBuilder::new_io(writer);
        self.render_tmpl(&mut tmpl);
        if let Some(e) = tmpl.0.io_error() { Err(e) } else { Ok(()) }
    }

    /// Render this into a template builder.
    fn render_tmpl<'a>(self, tmpl: &mut TemplateBuilder<'a>);

    /// Yields a hint at how many bytes this component use.
    fn size_hint<'a>(&self) -> usize { 0 }
}

/// Something that can be rendered by mutable reference.
pub trait RenderMut: RenderOnce {
    /// Render this into a new String.
    fn render(&mut self) -> String {
        let mut string = String::with_capacity(self.size_hint());
        self.render_string(&mut string);
        string
    }

    /// Render this into an existing String.
    ///
    /// Note: You could also use render_into_fmt but this is noticeably faster.
    fn render_string(&mut self, string: &mut String) {
        let mut tmpl = TemplateBuilder::new_str(string);
        self.render_tmpl(&mut tmpl);
    }

    /// Render this into something that implements fmt::Write.
    /// 
    /// Renderer also implements Display but that's about twice as slow...
    fn render_fmt(&mut self, writer: &mut fmt::Write) -> Result<(), fmt::Error> {
        let mut tmpl = TemplateBuilder::new_fmt(writer);
        self.render_tmpl(&mut tmpl);
        if let Some(e) = tmpl.0.fmt_error() { Err(e) } else { Ok(()) }
    }

    /// Render this into something that implements io::Write.
    ///
    /// Note: If you're writing directly to a file/socket etc., you should *seriously* consider
    /// wrapping your writer in a BufWriter. Otherwise, you'll end up making quite a few unnecessary
    /// system calls.
    fn render_io(&mut self, writer: &mut io::Write) -> Result<(), io::Error> {
        let mut tmpl = TemplateBuilder::new_io(writer);
        self.render_tmpl(&mut tmpl);
        if let Some(e) = tmpl.0.io_error() { Err(e) } else { Ok(()) }
    }

    /// Render this into a template builder.
    fn render_tmpl<'a>(&mut self, tmpl: &mut TemplateBuilder<'a>);
}

/// Something that can be rendered by reference.
pub trait Render: RenderMut {
    /// Render this into a new String.
    fn render(&self) -> String {
        let mut string = String::with_capacity(self.size_hint());
        self.render_string(&mut string);
        string
    }

    /// Render this into an existing String.
    ///
    /// Note: You could also use render_into_fmt but this is noticeably faster.
    fn render_string(&self, string: &mut String) {
        let mut tmpl = TemplateBuilder::new_str(string);
        self.render_tmpl(&mut tmpl);
    }

    /// Render this into something that implements fmt::Write.
    /// 
    /// Renderer also implements Display but that's about twice as slow...
    fn render_fmt(&self, writer: &mut fmt::Write) -> Result<(), fmt::Error> {
        let mut tmpl = TemplateBuilder::new_fmt(writer);
        self.render_tmpl(&mut tmpl);
        if let Some(e) = tmpl.0.fmt_error() { Err(e) } else { Ok(()) }
    }

    /// Render this into something that implements io::Write.
    ///
    /// Note: If you're writing directly to a file/socket etc., you should *seriously* consider
    /// wrapping your writer in a BufWriter. Otherwise, you'll end up making quite a few unnecessary
    /// system calls.
    fn render_io(&self, writer: &mut io::Write) -> Result<(), io::Error> {
        let mut tmpl = TemplateBuilder::new_io(writer);
        self.render_tmpl(&mut tmpl);
        if let Some(e) = tmpl.0.io_error() { Err(e) } else { Ok(()) }
    }

    /// Render this into a template builder.
    fn render_tmpl<'a>(&self, tmpl: &mut TemplateBuilder<'a>);
}


/// A template renderer. The `html! {}` macro returns a `Renderer`.
pub struct Renderer<F> {
    renderer: F,
    expected_size: usize,
}

impl<F> RenderOnce for Renderer<F> where F: FnOnce(&mut TemplateBuilder) {
    fn render_tmpl(self, tmpl: &mut TemplateBuilder) {
        (self.renderer)(tmpl)
    }

    fn size_hint(&self) -> usize {
        self.expected_size
    }
}

impl<F> RenderMut for Renderer<F> where F: FnMut(&mut TemplateBuilder) {
    fn render_tmpl(&mut self, tmpl: &mut TemplateBuilder) {
        (self.renderer)(tmpl)
    }
}

impl<F> Render for Renderer<F> where F: Fn(&mut TemplateBuilder) {
    fn render_tmpl(&self, tmpl: &mut TemplateBuilder) {
        (self.renderer)(tmpl)
    }
}

// I'd like to be able to say impl Display for T where T: Render but coherence.
impl<F> fmt::Display for Renderer<F> where Renderer<F>: Render {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        struct Adapter<'a, 'b>(&'a mut fmt::Formatter<'b>) where 'b: 'a;
        impl<'a, 'b> fmt::Write for Adapter<'a, 'b> {
            #[inline]
            fn write_str(&mut self, text: &str) -> fmt::Result {
                self.0.write_str(text)
            }
            #[inline]
            fn write_fmt(&mut self, args: fmt::Arguments) -> fmt::Result {
                self.0.write_fmt(args)
            }
        }
        Render::render_fmt(self, &mut Adapter(f))
    }
}

/// Raw content marker.
///
/// When rendered, raw content will not be escaped.
pub struct Raw<S: AsRef<str>>(S);

impl<S> Raw<S> where S: AsRef<str> {
    /// Mark as raw.
    pub fn new(content: S) -> Raw<S> {
        Raw(content)
    }
}

impl<'a, T> RenderOnce for &'a mut T where T: RenderMut {
    fn render_tmpl(self, tmpl: &mut TemplateBuilder) {
        RenderMut::render_tmpl(self, tmpl)
    }
    fn size_hint(&self) -> usize {
        (**self).size_hint()
    }
}

impl<'a, T> RenderOnce for &'a T where T: Render {
    fn render_tmpl(self, tmpl: &mut TemplateBuilder) {
        Render::render_tmpl(self, tmpl)
    }
    fn size_hint(&self) -> usize {
        (**self).size_hint()
    }
}

/// Mark a string as a raw. The string will not be rendered.
#[macro_export]
macro_rules! raw {
    ($e:expr) => { $crate::Raw::new($e) }
}

impl<S> RenderOnce for Raw<S> where S: AsRef<str> {
    fn render_tmpl(self, tmpl: &mut TemplateBuilder) {
        tmpl.write_raw(self.0.as_ref())
    }
    fn size_hint(&self) -> usize {
        self.0.as_ref().len()
    }
}

impl<S> RenderMut for Raw<S> where S: AsRef<str> {
    fn render_tmpl(&mut self, tmpl: &mut TemplateBuilder) {
        tmpl.write_raw(self.0.as_ref())
    }
}

impl<S> Render for Raw<S> where S: AsRef<str> {
    fn render_tmpl(&self, tmpl: &mut TemplateBuilder) {
        tmpl.write_raw(self.0.as_ref())
    }
}

impl<'a> RenderOnce for &'a str {
    #[inline]
    fn render_tmpl(self, tmpl: &mut TemplateBuilder) {
        tmpl.write_str(self)
    }
    fn size_hint(&self) -> usize {
        self.len()
    }
}

impl<'a> RenderMut for &'a str {
    #[inline]
    fn render_tmpl(&mut self, tmpl: &mut TemplateBuilder) {
        tmpl.write_str(self)
    }
}

impl<'a> Render for &'a str {
    #[inline]
    fn render_tmpl(&self, tmpl: &mut TemplateBuilder) {
        tmpl.write_str(self)
    }
}

impl RenderOnce for String {
    #[inline]
    fn render_tmpl(self, tmpl: &mut TemplateBuilder) {
        tmpl.write_str(&self)
    }
    fn size_hint(&self) -> usize {
        self.len()
    }
}

impl RenderMut for String {
    #[inline]
    fn render_tmpl(&mut self, tmpl: &mut TemplateBuilder) {
        tmpl.write_str(self)
    }
}

impl Render for String {
    #[inline]
    fn render_tmpl(&self, tmpl: &mut TemplateBuilder) {
        tmpl.write_str(self)
    }
}

impl<'a, 'b, T> std::ops::Shl<T> for &'a mut TemplateBuilder<'b> where T: RenderOnce {
    type Output = &'a mut TemplateBuilder<'b>;
    /// Render the component into the template.
    ///
    /// Note: If writing to the template fails, this method will neither panic nor return errors.
    /// Instead, no more data will be written to the template and horrorshow abort template
    /// rendering (return an error) when it re-gains control.
    fn shl(self, component: T) -> &'a mut TemplateBuilder<'b> {
        component.render_tmpl(self);
        self
    }
}

/// A template builder. This is the type that gets passed to closures inside templates.
///
/// Example:
///
/// ```
/// # #[macro_use] extern crate horrorshow;
/// # fn main() {
///     html! {
///         |tmpl /*: &mut TemplateBuilder */| tmpl << "Some String";
///     };
/// # }
/// ```
pub struct TemplateBuilder<'a>(TemplateWriter<'a>);

enum TemplateWriter<'a> {
    Io {
        writer: &'a mut io::Write,
        error: Option<io::Error>,
    },
    Fmt {
        writer: &'a mut fmt::Write,
        error: Option<fmt::Error>,
    },
    Str {
        writer: &'a mut String,
    }
}

/// Used by the `html! {}` macro
#[doc(hidden)]
pub fn __new_renderer<F: FnOnce(&mut TemplateBuilder)>(expected_size: usize, f: F) -> Renderer<F> {
    Renderer {
        renderer: f,
        expected_size: expected_size,
    }
}

impl<'a> TemplateBuilder<'a> {
    fn new_fmt(w: &mut fmt::Write) -> TemplateBuilder {
        TemplateBuilder(TemplateWriter::Fmt { writer: w, error: None })
    }
    fn new_io(w: &mut io::Write) -> TemplateBuilder {
        TemplateBuilder(TemplateWriter::Io { writer: w, error: None })
    }
    fn new_str(w: &mut String) -> TemplateBuilder {
        TemplateBuilder(TemplateWriter::Str { writer: w })
    }

    /// Write a raw string to the template output.
    #[inline]
    pub fn write_raw(&mut self, text: &str) {
        use TemplateWriter::*;
        use std::fmt::Write;
        match self.0 {
            Io { ref mut writer, ref mut error } => {
                if error.is_some() { return; }
                if let Err(e) = writer.write_all(text.as_bytes()) {
                    *error = Some(e);
                }
            },
            Fmt {ref mut writer, ref mut error } => {
                if error.is_some() { return; }
                if let Err(e) = writer.write_str(text) {
                    *error = Some(e);
                }
            },
            Str {ref mut writer } => {
                let _ = writer.write_str(text);
            },
        }
    }

    /// Escape and write the formatted arguments to the template output.
    ///
    /// Example:
    ///
    /// ```norun
    /// write!(tmpl, "{} + {}", 0, 1);
    /// ```
    #[inline]
    pub fn write_fmt(&mut self, args: fmt::Arguments) {
        use std::fmt::Write;
        let _ = self.0.write_fmt(args);
    }

    /// Escape and write a string to the template output.
    #[inline]
    pub fn write_str(&mut self, text: &str) {
        use std::fmt::Write;
        let _ = self.0.write_str(text);
    }
}

impl<'a> TemplateWriter<'a> {
    fn fmt_error(self) -> Option<fmt::Error> {
        match self {
            TemplateWriter::Fmt { error, .. } => error,
            _ => panic!(),
        }
    }
    fn io_error(self) -> Option<io::Error> {
        match self {
            TemplateWriter::Io { error, .. } => error,
            _ => panic!(),
        }
    }
}

impl<'a> fmt::Write for TemplateWriter<'a> {
    #[inline]
    fn write_str(&mut self, text: &str) -> fmt::Result {
        use TemplateWriter::*;
        match self {
            &mut Io { ref mut writer, ref mut error } => {
                if error.is_some() { return Ok(()); }
                for b in text.bytes() {
                    if let Err(e) = match b {
                        b'&' => writer.write_all("&amp;".as_bytes()),
                        b'"' => writer.write_all("&quot;".as_bytes()),
                        b'<' => writer.write_all("&lt;".as_bytes()),
                        b'>' => writer.write_all("&gt;".as_bytes()),
                        _ => writer.write_all(&[b] as &[u8]),
                    } {
                        *error = Some(e);
                        break;
                    }
                }
            },
            &mut Fmt { ref mut writer, ref mut error } => {
                if error.is_some() { return Ok(()); }
                for c in text.chars() {
                    if let Err(e) = match c {
                        '&' => writer.write_str("&amp;"),
                        '"' => writer.write_str("&quot;"),
                        '<' => writer.write_str("&lt;"),
                        '>' => writer.write_str("&gt;"),
                        _ => writer.write_char(c),
                    } {
                        *error = Some(e);
                        break;
                    }
                }
            },
            &mut Str { ref mut writer } => {
                // TODO: Consider using a forloop. LLVM isn't optimizing this quite as well as it
                // could 0.96x slowdown.
                for b in text.bytes() {
                    match b {
                        b'&' => writer.push_str("&amp;"),
                        b'"' => writer.push_str("&quot;"),
                        b'<' => writer.push_str("&lt;"),
                        b'>' => writer.push_str("&gt;"),
                        _ => unsafe { writer.as_mut_vec() }.push(b),
                    }
                }
            }
        }
        Ok(())
    }
}

// We shouldn't need this but without it I get the following error:
// error: unexpected token: `an interpolated tt`
#[macro_export]
#[doc(hidden)]
macro_rules! __horrorshow_block_identity {
    ($b:block) => { $b };
}
