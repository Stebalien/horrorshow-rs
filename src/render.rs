use std::fmt;
use std::io;

use template_builder::{self, TemplateBuilder};

/// Something that can be rendered once.
pub trait RenderOnce {
    /// Render this into a new String.
    fn render(self) -> String where Self: Sized {
        let mut string = String::with_capacity(self.size_hint());
        self.render_string(&mut string);
        string
    }

    /// Render this into an existing String.
    ///
    /// Note: You could also use render_into_fmt but this is noticeably faster.
    fn render_string(self, string: &mut String) where Self: Sized {
        template_builder::render_string(self, string)
    }

    /// Render this into something that implements fmt::Write.
    /// 
    /// Renderer also implements Display but that's about twice as slow...
    fn render_fmt(self, writer: &mut fmt::Write) -> Result<(), fmt::Error> where Self: Sized {
        template_builder::render_fmt(self, writer)
    }

    /// Render this into something that implements io::Write.
    ///
    /// Note: If you're writing directly to a file/socket etc., you should *seriously* consider
    /// wrapping your writer in a BufWriter. Otherwise, you'll end up making quite a few unnecessary
    /// system calls.
    fn render_io(self, writer: &mut io::Write) -> Result<(), io::Error> where Self: Sized {
        template_builder::render_io(self, writer)
    }

    /// Render this into a template builder.
    fn render_tmpl<'a>(self, tmpl: &mut TemplateBuilder<'a>) where Self: Sized;

    /// Yields a hint at how many bytes this component use.
    fn size_hint<'a>(&self) -> usize { 0 }
}

/// Something that can be rendered once out of a box.
pub trait RenderBox {
    /// Do not call. Called by RenderOnce impl on Box<RenderBox>
    #[doc(hidden)]
    fn boxed_render_tmpl<'a>(self: Box<Self>, tmpl: &mut TemplateBuilder<'a>);

    /// Do not call. Called by RenderOnce impl on Box<RenderBox>
    #[doc(hidden)]
    fn boxed_size_hint(&self) -> usize;
}

impl<T> RenderBox for T where T: RenderOnce {
    fn boxed_render_tmpl<'a>(self: Box<T>, tmpl: &mut TemplateBuilder<'a>) {
        (*self).render_tmpl(tmpl);
    }

    #[doc(hidden)]
    fn boxed_size_hint(&self) -> usize {
        RenderOnce::size_hint(self)
    }
}

impl<'b> RenderOnce for Box<RenderBox + 'b> {
    fn render_tmpl<'a>(self, tmpl: &mut TemplateBuilder<'a>) {
        RenderBox::boxed_render_tmpl(self, tmpl);
    }

    fn size_hint(&self) -> usize {
        RenderBox::boxed_size_hint(self)
    }
}

impl<'b> RenderOnce for Box<Render + 'b> {
    fn render_tmpl<'a>(self, tmpl: &mut TemplateBuilder<'a>) {
        Render::render_tmpl(&*self, tmpl);
    }

    fn size_hint(&self) -> usize { 
        Render::size_hint(&**self)
    }
}

impl<'b> RenderOnce for Box<RenderMut + 'b> {
    fn render_tmpl<'a>(mut self, tmpl: &mut TemplateBuilder<'a>) {
        RenderMut::render_tmpl(&mut *self, tmpl);
    }

    fn size_hint(&self) -> usize { 
        RenderMut::size_hint(&**self)
    }
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
        template_builder::render_string(self, string)
    }

    /// Render this into something that implements fmt::Write.
    /// 
    /// Renderer also implements Display but that's about twice as slow...
    fn render_fmt(&mut self, writer: &mut fmt::Write) -> Result<(), fmt::Error> {
        template_builder::render_fmt(self, writer)
    }

    /// Render this into something that implements io::Write.
    ///
    /// Note: If you're writing directly to a file/socket etc., you should *seriously* consider
    /// wrapping your writer in a BufWriter. Otherwise, you'll end up making quite a few unnecessary
    /// system calls.
    fn render_io(&mut self, writer: &mut io::Write) -> Result<(), io::Error> {
        template_builder::render_io(self, writer)
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
        template_builder::render_string(self, string)
    }

    /// Render this into something that implements fmt::Write.
    /// 
    /// Renderer also implements Display but that's about twice as slow...
    fn render_fmt(&self, writer: &mut fmt::Write) -> Result<(), fmt::Error> {
        template_builder::render_fmt(self, writer)
    }

    /// Render this into something that implements io::Write.
    ///
    /// Note: If you're writing directly to a file/socket etc., you should *seriously* consider
    /// wrapping your writer in a BufWriter. Otherwise, you'll end up making quite a few unnecessary
    /// system calls.
    fn render_io(&self, writer: &mut io::Write) -> Result<(), io::Error> {
        template_builder::render_io(self, writer)
    }

    /// Render this into a template builder.
    fn render_tmpl<'a>(&self, tmpl: &mut TemplateBuilder<'a>);
}

// {{{ Renderer

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

/// Used by the `html! {}` macro
#[doc(hidden)]
pub fn __new_renderer<F: FnOnce(&mut TemplateBuilder)>(expected_size: usize, f: F) -> Renderer<F> {
    Renderer {
        renderer: f,
        expected_size: expected_size,
    }
}

/// Used by the `html! {}` macro
#[doc(hidden)]
pub fn __new_boxed_renderer<F: FnOnce(&mut TemplateBuilder)>(expected_size: usize, f: F) -> Box<Renderer<F>> {
    Box::new(Renderer {
        renderer: f,
        expected_size: expected_size,
    })
}

// }}}

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

impl<'a, T: ?Sized> RenderOnce for &'a mut T where T: RenderMut {
    fn render_tmpl(self, tmpl: &mut TemplateBuilder) {
        RenderMut::render_tmpl(self, tmpl)
    }
    fn size_hint(&self) -> usize {
        (**self).size_hint()
    }
}

impl<'a, T: ?Sized> RenderOnce for &'a T where T: Render {
    fn render_tmpl(self, tmpl: &mut TemplateBuilder) {
        Render::render_tmpl(self, tmpl)
    }
    fn size_hint(&self) -> usize {
        (**self).size_hint()
    }
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

