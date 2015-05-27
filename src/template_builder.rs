use render::RenderOnce;
use std::fmt;
use std::io;
use std::ops::Shl;

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

impl<'a, 'b, T> Shl<T> for &'a mut TemplateBuilder<'b> where T: RenderOnce {
    type Output = &'a mut TemplateBuilder<'b>;
    /// Render the component into the template.
    ///
    /// Note: If writing to the template fails, this method will neither panic nor return errors.
    /// Instead, no more data will be written to the template and horrorshow abort template
    /// rendering (return an error) when it re-gains control.
    fn shl(self, component: T) -> &'a mut TemplateBuilder<'b> {
        component.render_once(self);
        self
    }
}

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

/// Crate private
pub fn render_fmt<R: RenderOnce>(render: R, w: &mut fmt::Write) -> fmt::Result {
    let mut builder = TemplateBuilder(TemplateWriter::Fmt { writer: w, error: None });
    render.render_once(&mut builder);
    match builder.0 {
        TemplateWriter::Fmt { error: Some(e), .. } => Err(e),
        TemplateWriter::Fmt { error: None, .. } => Ok(()),
        _ => panic!(),
    }
}

/// Crate private
pub fn render_io<R: RenderOnce>(render: R, w: &mut io::Write) -> io::Result<()> {
    let mut builder = TemplateBuilder(TemplateWriter::Io { writer: w, error: None });
    render.render_once(&mut builder);
    match builder.0 {
        TemplateWriter::Io { error: Some(e), .. } => Err(e),
        TemplateWriter::Io { error: None, .. } => Ok(()),
        _ => panic!(),
    }
}

/// Crate private
pub fn render_string<R: RenderOnce>(render: R, w: &mut String) {
    let mut builder = TemplateBuilder(TemplateWriter::Str { writer: w });
    render.render_once(&mut builder);
}

impl<'a> TemplateBuilder<'a> {

    /// Write a raw string to the template output.
    #[inline]
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
    #[inline]
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

