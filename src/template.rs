use render::RenderOnce;
use std::fmt;
use std::io;
use error::{self, Error};

/// A template that can be rendered into something.
///
/// Don't let the single impl below fool you, these methods are available on all `Render*`'s
/// (through impls on references and boxes).
pub trait Template: RenderOnce + Sized {
    /// Render this into a new String.
    fn into_string(self) -> Result<String, Error> {
        let mut string = String::with_capacity(self.size_hint());
        self.write_to_string(&mut string).and(Ok(string))
    }

    /// Render this into an existing String.
    ///
    /// Note: You could also use render_into_fmt but this is noticeably faster.
    fn write_to_string(self, string: &mut String) -> Result<(), Error> {
        let mut builder = TemplateBuilder {
            writer: InnerTemplateWriter::Str(string),
            error: Error::new(),
        };
        self.render_once(&mut builder);
        builder.into_result()
    }

    /// Render this into something that implements fmt::Write.
    ///
    /// Renderer also implements Display but that's about twice as slow...
    fn write_to_fmt(self, writer: &mut fmt::Write) -> Result<(), Error> {
        let mut builder = TemplateBuilder {
            writer: InnerTemplateWriter::Fmt(writer),
            error: Error::new(),
        };
        self.render_once(&mut builder);
        builder.into_result()
    }

    /// Render this into something that implements io::Write.
    ///
    /// Note: If you're writing directly to a file/socket etc., you should *seriously* consider
    /// wrapping your writer in a BufWriter. Otherwise, you'll end up making quite a few unnecessary
    /// system calls.
    fn write_to_io(self, writer: &mut io::Write) -> Result<(), Error> {
        let mut builder = TemplateBuilder {
            writer: InnerTemplateWriter::Io(writer),
            error: Error::new(),
        };
        self.render_once(&mut builder);
        builder.into_result()
    }
}

impl<T: RenderOnce + Sized> Template for T { }


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
pub struct TemplateBuilder<'a> {
    writer: InnerTemplateWriter<'a>,
    error: Error,
}

enum InnerTemplateWriter<'a> {
    Io(&'a mut io::Write),
    Fmt(&'a mut fmt::Write),
    Str(&'a mut String),
}

impl<'a> TemplateBuilder<'a> {
    #[cold]
    pub fn record_error<E: Into<Box<::std::error::Error + Send + Sync>>>(&mut self, e: E) {
        self.error.render.push(e.into());
    }

    /// Write a raw string to the template output.
    #[inline(always)]
    pub fn write_raw(&mut self, text: &str) {
        use std::fmt::Write;
        let _ = self.as_raw_writer().write_str(text);
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
        let _ = self.as_writer().write_fmt(args);
    }

    /// Escape and write a string to the template output.
    #[inline]
    pub fn write_str(&mut self, text: &str) {
        use std::fmt::Write;
        let _ = self.as_writer().write_str(text);
    }

    /// Returns an escaping Write adapter.
    #[inline]
    pub fn as_writer<'b>(&'b mut self) -> TemplateWriter<'a, 'b> {
        TemplateWriter(self)
    }

    /// Returns a non-escaping Write adapter.
    #[inline]
    pub fn as_raw_writer<'b>(&'b mut self) -> RawTemplateWriter<'a, 'b> {
        RawTemplateWriter(self)
    }

    fn into_result(self) -> Result<(), Error> {
        if error::is_empty(&self.error) {
            Ok(())
        } else {
            Err(self.error)
        }
    }
}

pub struct RawTemplateWriter<'a, 'b>(&'b mut TemplateBuilder<'a>) where 'a: 'b;

impl<'a, 'b> fmt::Write for RawTemplateWriter<'a, 'b> {
    // This is the fast-path.
    // Inlining this allows LLVM to optimize it significantly.
    #[inline(always)]
    fn write_str(&mut self, text: &str) -> fmt::Result {
        use self::InnerTemplateWriter::*;
        use std::fmt::Write;
        if !error::is_empty(&self.0.error) {
            return Ok(())
        }
        match self.0.writer {
            Io(ref mut writer) => {
                self.0.error.write = writer.write_all(text.as_bytes()).err();
            },
            Fmt(ref mut writer) => {
                if writer.write_str(text).is_err() {
                    self.0.error.write = Some(io::Error::new(io::ErrorKind::Other, "Format Error"));
                }
            },
            Str(ref mut writer) => {
                let _ = writer.write_str(text);
            },
        }
        Ok(())
    }
}

pub struct TemplateWriter<'a, 'b>(&'b mut TemplateBuilder<'a>) where 'a: 'b;


impl<'a, 'b> fmt::Write for TemplateWriter<'a, 'b> {
    fn write_str(&mut self, text: &str) -> fmt::Result {
        use self::InnerTemplateWriter::*;
        if !error::is_empty(&self.0.error) {
            return Ok(())
        }
        match self.0.writer {
            Io(ref mut writer) => {
                for b in text.bytes() {
                    if let Err(e) = match b {
                        b'&' => writer.write_all("&amp;".as_bytes()),
                        b'"' => writer.write_all("&quot;".as_bytes()),
                        b'<' => writer.write_all("&lt;".as_bytes()),
                        b'>' => writer.write_all("&gt;".as_bytes()),
                        _ => writer.write_all(&[b] as &[u8]),
                    } {
                        self.0.error.write = Some(e);
                        break;
                    }
                }
            },
            Fmt(ref mut writer) => {
                let mut s = String::with_capacity(4);
                for c in text.chars() {
                    if (match c {
                        '&' => writer.write_str("&amp;"),
                        '"' => writer.write_str("&quot;"),
                        '<' => writer.write_str("&lt;"),
                        '>' => writer.write_str("&gt;"),
                        _ => {
                            // TODO: Use fmt::Write::write_char once beta stabalizes. This is very
                            // slow!
                            s.push(c);
                            let r = writer.write_str(&s);
                            s.clear();
                            r
                        }
                    }).is_err() {
                        self.0.error.write = Some(io::Error::new(io::ErrorKind::Other, "Format Error"));
                        break;
                    }
                }
            },
            Str(ref mut writer) => {
                // TODO: Consider using a indexing. LLVM isn't optimizing this quite as well as it
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
