use error::{self, Error};
use render::RenderOnce;
use std::fmt;
use std::io;

/// A template that can be rendered into something.
///
/// Don't let the single impl below fool you, these methods are available on all `Render*`'s
/// (through impls on references and boxes).
pub trait Template: RenderOnce + Sized {
    /// Render this into a new String.
    fn into_string(self) -> Result<String, Error> {
        let mut string = String::with_capacity(self.size_hint());
        self.write_to_string(&mut string)?;
        string.shrink_to_fit();
        Ok(string)
    }

    /// Render this into an existing String.
    ///
    /// Note: You could also use render_into_fmt but this is noticeably faster.
    fn write_to_string(self, string: &mut String) -> Result<(), Error> {
        let mut buffer = TemplateBuffer {
            writer: InnerTemplateWriter::Str(string),
            error: Default::default(),
        };
        self.render_once(&mut buffer);
        buffer.into_result()
    }

    /// Render this into something that implements fmt::Write.
    ///
    /// FnRenderer also implements Display but that's about twice as slow...
    fn write_to_fmt(self, writer: &mut fmt::Write) -> Result<(), Error> {
        let mut buffer = TemplateBuffer {
            writer: InnerTemplateWriter::Fmt(writer),
            error: Default::default(),
        };
        self.render_once(&mut buffer);
        buffer.into_result()
    }

    /// Render this into something that implements io::Write.
    ///
    /// Note: If you're writing directly to a file/socket etc., you should *seriously* consider
    /// wrapping your writer in a BufWriter. Otherwise, you'll end up making quite a few unnecessary
    /// system calls.
    fn write_to_io(self, writer: &mut io::Write) -> Result<(), Error> {
        let mut buffer = TemplateBuffer {
            writer: InnerTemplateWriter::Io(writer),
            error: Default::default(),
        };
        self.render_once(&mut buffer);
        buffer.into_result()
    }
}

impl<T: RenderOnce + Sized> Template for T {}

/// A template buffer. This is the type that gets passed to closures inside templates.
///
/// Example:
///
/// ```
/// # #[macro_use] extern crate horrorshow;
/// # fn main() {
///     html! {
///         |tmpl /*: &mut TemplateBuffer */| tmpl << "Some String";
///     };
/// # }
/// ```
pub struct TemplateBuffer<'a> {
    writer: InnerTemplateWriter<'a>,
    error: Error,
}

enum InnerTemplateWriter<'a> {
    Io(&'a mut io::Write),
    Fmt(&'a mut fmt::Write),
    Str(&'a mut String),
}

impl<'a> TemplateBuffer<'a> {
    #[cold]
    pub fn record_error<E: Into<Box<::std::error::Error + Send + Sync>>>(&mut self, e: E) {
        self.error.render.push(e.into());
    }

    /// Write a raw string to the template output.
    // NEVER REMOVE THIS INLINE!
    #[inline(always)]
    pub fn write_raw(&mut self, text: &str) {
        use std::fmt::Write;
        let _ = self.as_raw_writer().write_str(text);
    }

    /// Escape and write the formatted arguments to the template output.
    ///
    /// Example:
    ///
    /// ```
    /// # #[macro_use] extern crate horrorshow;
    /// # use horrorshow::prelude::*;
    /// # fn main() {
    /// #     let result = html! {
    /// #         |tmpl| {
    /// write!(tmpl, "{} + {}", 0, 1);
    /// #         }
    /// #     };
    /// #     assert_eq!(result.into_string().unwrap(), "0 + 1");
    /// # }
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

pub struct RawTemplateWriter<'a, 'b>(&'b mut TemplateBuffer<'a>)
where
    'a: 'b;

impl<'a, 'b> fmt::Write for RawTemplateWriter<'a, 'b> {
    // This is the fast-path.
    // Inlining this allows LLVM to optimize it significantly.
    #[inline(always)]
    fn write_str(&mut self, text: &str) -> fmt::Result {
        use self::InnerTemplateWriter::*;
        if !error::is_empty(&self.0.error) {
            return Ok(());
        }
        match self.0.writer {
            Io(ref mut writer) => {
                self.0.error.write = writer.write_all(text.as_bytes()).err();
            }
            Fmt(ref mut writer) => {
                if writer.write_str(text).is_err() {
                    self.0.error.write = Some(io::Error::new(io::ErrorKind::Other, "Format Error"));
                }
            }
            Str(ref mut writer) => {
                let _ = writer.write_str(text);
            }
        }
        Ok(())
    }
}

pub struct TemplateWriter<'a, 'b>(&'b mut TemplateBuffer<'a>)
where
    'a: 'b;

impl<'a, 'b> fmt::Write for TemplateWriter<'a, 'b> {
    fn write_str(&mut self, text: &str) -> fmt::Result {
        // NOTE to future self: don't try to be fancy here. This optimizes well
        // and all you're fancy algorithms are actually slower.
        //
        // Also, don't try short-circuiting entirely. That is, don't check if we
        // need to escape and use write_raw otherwise. It's slower.
        use self::InnerTemplateWriter::*;
        if !error::is_empty(&self.0.error) {
            return Ok(());
        }

        fn should_escape(b: u8) -> bool {
            (b | 0x4) == b'&' || (b | 0x2) == b'>'
        }

        match self.0.writer {
            Io(ref mut writer) => {
                for b in text.bytes() {
                    if let Err(e) = match (should_escape(b), b) {
                        (true, b'&') => writer.write_all(b"&amp;"),
                        (true, b'"') => writer.write_all(b"&quot;"),
                        (true, b'<') => writer.write_all(b"&lt;"),
                        (true, b'>') => writer.write_all(b"&gt;"),
                        _ => writer.write_all(&[b] as &[u8]),
                    } {
                        self.0.error.write = Some(e);
                        break;
                    }
                }
            }
            Fmt(ref mut writer) => {
                for c in text.chars() {
                    if (match (should_escape(c as u8), c as u8) {
                        (true, b'&') => writer.write_str("&amp;"),
                        (true, b'"') => writer.write_str("&quot;"),
                        (true, b'<') => writer.write_str("&lt;"),
                        (true, b'>') => writer.write_str("&gt;"),
                        _ => writer.write_char(c),
                    })
                    .is_err()
                    {
                        self.0.error.write =
                            Some(io::Error::new(io::ErrorKind::Other, "Format Error"));
                        break;
                    }
                }
            }
            Str(ref mut writer) => {
                for b in text.bytes() {
                    match (should_escape(b), b) {
                        (true, b'&') => writer.push_str("&amp;"),
                        (true, b'"') => writer.push_str("&quot;"),
                        (true, b'<') => writer.push_str("&lt;"),
                        (true, b'>') => writer.push_str("&gt;"),
                        // NOTE: Do not add an unreachable case. It makes this slower.
                        _ => unsafe { writer.as_mut_vec() }.push(b),
                    }
                }
            }
        }
        Ok(())
    }
}
