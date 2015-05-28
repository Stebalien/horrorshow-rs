use render::RenderOnce;
use std::fmt;
use std::io;

/// A template that can be rendered into something.
///
/// Don't let the single impl below fool you, these methods are available on all `Render*`'s
/// (through impls on references and boxes).
pub trait Template: RenderOnce + Sized {
    /// Render this into a new String.
    fn into_string(self) -> String {
        let mut string = String::with_capacity(self.size_hint());
        self.write_to_string(&mut string);
        string
    }

    /// Render this into an existing String.
    ///
    /// Note: You could also use render_into_fmt but this is noticeably faster.
    fn write_to_string(self, string: &mut String) {
        let mut builder = TemplateBuilder(TemplateWriter::Str { writer: string });
        self.render_once(&mut builder);
    }

    /// Render this into something that implements fmt::Write.
    ///
    /// Renderer also implements Display but that's about twice as slow...
    fn write_to_fmt(self, writer: &mut fmt::Write) -> Result<(), fmt::Error> {
        let mut builder = TemplateBuilder(TemplateWriter::Fmt { writer: writer, error: None });
        self.render_once(&mut builder);
        match builder.0 {
            TemplateWriter::Fmt { error: Some(e), .. } => Err(e),
            TemplateWriter::Fmt { error: None, .. } => Ok(()),
            _ => panic!(),
        }
    }

    /// Render this into something that implements io::Write.
    ///
    /// Note: If you're writing directly to a file/socket etc., you should *seriously* consider
    /// wrapping your writer in a BufWriter. Otherwise, you'll end up making quite a few unnecessary
    /// system calls.
    fn write_to_io(self, writer: &mut io::Write) -> Result<(), io::Error> {
        let mut builder = TemplateBuilder(TemplateWriter::Io { writer: writer, error: None });
        self.render_once(&mut builder);
        match builder.0 {
            TemplateWriter::Io { error: Some(e), .. } => Err(e),
            TemplateWriter::Io { error: None, .. } => Ok(()),
            _ => panic!(),
        }
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

impl<'a> TemplateBuilder<'a> {

    /// Write a raw string to the template output.
    #[inline(always)]
    pub fn write_raw(&mut self, text: &str) {
        use self::TemplateWriter::*;
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

impl<'a> fmt::Write for TemplateWriter<'a> {
    fn write_str(&mut self, text: &str) -> fmt::Result {
        use self::TemplateWriter::*;
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
                let mut s = String::with_capacity(4);
                for c in text.chars() {
                    if let Err(e) = match c {
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
