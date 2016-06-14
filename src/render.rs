use std::fmt;
use std::ops::Deref;

use template::{TemplateBuffer, Template};


/// Something that can be rendered once.
pub trait RenderOnce {
    /// Render this into a template buffer.
    fn render_once(self, tmpl: &mut TemplateBuffer) where Self: Sized;

    /// Returns a (very) rough estimate of how many bytes this Render will use.
    fn size_hint(&self) -> usize {
        0
    }
}

/// Something that can be rendered by mutable reference.
pub trait RenderMut: RenderOnce {
    /// Render this into a template buffer.
    fn render_mut<'a>(&mut self, tmpl: &mut TemplateBuffer<'a>);
}

/// Something that can be rendered by reference.
pub trait Render: RenderMut {
    /// Render this into a template buffer.
    fn render<'a>(&self, tmpl: &mut TemplateBuffer<'a>);
}

// RenderOnce is the trait we really care about.

impl<'a, T: ?Sized> RenderOnce for &'a mut T
    where T: RenderMut
{
    fn render_once(self, tmpl: &mut TemplateBuffer) {
        RenderMut::render_mut(self, tmpl)
    }
    fn size_hint(&self) -> usize {
        (**self).size_hint()
    }
}

impl<'a, T: ?Sized> RenderOnce for &'a T
    where T: Render
{
    fn render_once(self, tmpl: &mut TemplateBuffer) {
        Render::render(self, tmpl)
    }
    fn size_hint(&self) -> usize {
        (**self).size_hint()
    }
}

// Box Stuff

/// Something that can be rendered once out of a box.
///
/// This should only ever be used in the form `Box<RenderBox>` by casting `Box<RenderOnce>` to
/// `Box<RenderBox>`. This trait has methods but I've hidden them because you should never call
/// them directly.  Instead, you should call the `RenderOnce` methods implemented on
/// `Box<RenderBox>`.
pub trait RenderBox {
    /// Do not call. Called by RenderOnce impl on Box<RenderBox>
    #[doc(hidden)]
    fn render_box(self: Box<Self>, tmpl: &mut TemplateBuffer);

    /// Do not call. Called by RenderOnce impl on Box<RenderBox>
    #[doc(hidden)]
    fn size_hint_box(&self) -> usize;
}

impl<T> RenderBox for T
    where T: RenderOnce
{
    fn render_box(self: Box<T>, tmpl: &mut TemplateBuffer) {
        (*self).render_once(tmpl);
    }

    fn size_hint_box(&self) -> usize {
        RenderOnce::size_hint(self)
    }
}

// Box<RenderBox>

impl<'b> RenderOnce for Box<RenderBox + 'b> {
    #[inline]
    fn render_once(self, tmpl: &mut TemplateBuffer) {
        RenderBox::render_box(self, tmpl);
    }

    #[inline]
    fn size_hint(&self) -> usize {
        RenderBox::size_hint_box(self)
    }
}

impl<'b> RenderOnce for Box<RenderBox + 'b + Send> {
    #[inline]
    fn render_once(self, tmpl: &mut TemplateBuffer) {
        RenderBox::render_box(self, tmpl);
    }

    #[inline]
    fn size_hint(&self) -> usize {
        RenderBox::size_hint_box(self)
    }
}

// Box<RenderMut>

impl<'b> RenderOnce for Box<RenderMut + 'b> {
    #[inline]
    fn render_once(mut self, tmpl: &mut TemplateBuffer) {
        RenderMut::render_mut(&mut *self, tmpl);
    }

    #[inline]
    fn size_hint(&self) -> usize {
        RenderMut::size_hint(&**self)
    }
}

impl<'b> RenderMut for Box<RenderMut + 'b> {
    #[inline]
    fn render_mut<'a>(&mut self, tmpl: &mut TemplateBuffer<'a>) {
        RenderMut::render_mut(&mut **self, tmpl);
    }
}

impl<'b> RenderOnce for Box<RenderMut + 'b + Send> {
    #[inline]
    fn render_once(mut self, tmpl: &mut TemplateBuffer) {
        RenderMut::render_mut(&mut *self, tmpl);
    }

    #[inline]
    fn size_hint(&self) -> usize {
        RenderMut::size_hint(&**self)
    }
}

impl<'b> RenderMut for Box<RenderMut + 'b + Send> {
    #[inline]
    fn render_mut<'a>(&mut self, tmpl: &mut TemplateBuffer<'a>) {
        RenderMut::render_mut(&mut **self, tmpl);
    }
}

// Box<Render>

impl<'b> RenderOnce for Box<Render + 'b> {
    #[inline]
    fn render_once(self, tmpl: &mut TemplateBuffer) {
        Render::render(&*self, tmpl);
    }

    #[inline]
    fn size_hint(&self) -> usize {
        Render::size_hint(&**self)
    }
}

impl<'b> RenderMut for Box<Render + 'b> {
    #[inline]
    fn render_mut<'a>(&mut self, tmpl: &mut TemplateBuffer<'a>) {
        Render::render(&*self, tmpl);
    }
}

impl<'b> Render for Box<Render + 'b> {
    #[inline]
    fn render<'a>(&self, tmpl: &mut TemplateBuffer<'a>) {
        Render::render(&**self, tmpl);
    }
}

impl<'b> RenderOnce for Box<Render + 'b + Send> {
    #[inline]
    fn render_once(self, tmpl: &mut TemplateBuffer) {
        Render::render(&*self, tmpl);
    }

    #[inline]
    fn size_hint(&self) -> usize {
        Render::size_hint(&**self)
    }
}

impl<'b> RenderMut for Box<Render + 'b + Send> {
    #[inline]
    fn render_mut<'a>(&mut self, tmpl: &mut TemplateBuffer<'a>) {
        Render::render(&*self, tmpl);
    }
}

impl<'b> Render for Box<Render + 'b + Send> {
    #[inline]
    fn render<'a>(&self, tmpl: &mut TemplateBuffer<'a>) {
        Render::render(&**self, tmpl);
    }
}

/// A template renderer. The `html! {}` macro returns a `FnRenderer`.
pub struct FnRenderer<F> {
    renderer: F,
    expected_size: usize,
}

impl<F> FnRenderer<F>
    where F: FnOnce(&mut TemplateBuffer)
{
    pub fn new(f: F) -> Self {
        FnRenderer {
            renderer: f,
            expected_size: 0,
        }
    }

    pub fn with_capacity(expected_size: usize, f: F) -> Self {
        FnRenderer {
            renderer: f,
            expected_size: expected_size,
        }
    }
}

impl<F> RenderOnce for FnRenderer<F>
    where F: FnOnce(&mut TemplateBuffer)
{
    fn render_once(self, tmpl: &mut TemplateBuffer) {
        (self.renderer)(tmpl)
    }

    fn size_hint(&self) -> usize {
        self.expected_size
    }
}

impl<F> RenderMut for FnRenderer<F>
    where F: FnMut(&mut TemplateBuffer)
{
    fn render_mut(&mut self, tmpl: &mut TemplateBuffer) {
        (self.renderer)(tmpl)
    }
}

impl<F> Render for FnRenderer<F>
    where F: Fn(&mut TemplateBuffer)
{
    fn render(&self, tmpl: &mut TemplateBuffer) {
        (self.renderer)(tmpl)
    }
}

// I'd like to be able to say impl Display for T where T: Render but coherence.
impl<F> fmt::Display for FnRenderer<F>
    where FnRenderer<F>: Render
{
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
        self.write_to_fmt(&mut Adapter(f)).or(Err(fmt::Error))
    }
}

/// Raw content marker.
///
/// When rendered, raw content will not be escaped.
#[derive(Eq, PartialEq, Ord, PartialOrd, Clone, Copy)]
pub struct Raw<S: AsRef<str>>(S);

impl<S> Deref for Raw<S>
    where S: AsRef<str>
{
    type Target = S;
    fn deref(&self) -> &S {
        &self.0
    }
}

// No DerefMut for safety. Once you mark something as Raw, don't change it.

impl<S> Raw<S>
    where S: AsRef<str>
{
    /// Mark as raw.
    pub fn new(content: S) -> Raw<S> {
        Raw(content)
    }
}

impl<S> RenderOnce for Raw<S>
    where S: AsRef<str>
{
    fn render_once(self, tmpl: &mut TemplateBuffer) {
        tmpl.write_raw(self.0.as_ref())
    }
    fn size_hint(&self) -> usize {
        self.0.as_ref().len()
    }
}

impl<S> RenderMut for Raw<S>
    where S: AsRef<str>
{
    fn render_mut(&mut self, tmpl: &mut TemplateBuffer) {
        tmpl.write_raw(self.0.as_ref())
    }
}

impl<S> Render for Raw<S>
    where S: AsRef<str>
{
    fn render(&self, tmpl: &mut TemplateBuffer) {
        tmpl.write_raw(self.0.as_ref())
    }
}

impl<'a> RenderOnce for &'a str {
    #[inline]
    fn render_once(self, tmpl: &mut TemplateBuffer) {
        tmpl.write_str(self)
    }

    #[inline]
    fn size_hint(&self) -> usize {
        self.len()
    }
}

impl<'a> RenderMut for &'a str {
    #[inline]
    fn render_mut(&mut self, tmpl: &mut TemplateBuffer) {
        tmpl.write_str(self)
    }
}

impl<'a> Render for &'a str {
    #[inline]
    fn render(&self, tmpl: &mut TemplateBuffer) {
        tmpl.write_str(self)
    }
}

impl RenderOnce for String {
    #[inline]
    fn render_once(self, tmpl: &mut TemplateBuffer) {
        tmpl.write_str(&self)
    }
    #[inline]
    fn size_hint(&self) -> usize {
        self.len()
    }
}

impl RenderMut for String {
    #[inline]
    fn render_mut(&mut self, tmpl: &mut TemplateBuffer) {
        tmpl.write_str(self)
    }
}

impl Render for String {
    #[inline]
    fn render(&self, tmpl: &mut TemplateBuffer) {
        tmpl.write_str(self)
    }
}

impl<T> RenderOnce for Option<T>
    where T: RenderOnce
{
    #[inline]
    fn render_once(self, tmpl: &mut TemplateBuffer) {
        if let Some(v) = self {
            v.render_once(tmpl);
        }
    }
}

impl<T> RenderMut for Option<T>
    where T: RenderMut
{
    #[inline]
    fn render_mut(&mut self, tmpl: &mut TemplateBuffer) {
        if let Some(v) = self.as_mut() {
            v.render_mut(tmpl);
        }
    }
}

impl<T> Render for Option<T>
    where T: Render
{
    #[inline]
    fn render(&self, tmpl: &mut TemplateBuffer) {
        if let Some(v) = self.as_ref() {
            v.render(tmpl);
        }
    }
}

impl<T, E> RenderOnce for Result<T, E>
    where T: RenderOnce,
          E: Into<Box<::std::error::Error + Send + Sync>>
{
    #[inline]
    fn render_once(self, tmpl: &mut TemplateBuffer) {
        match self {
            Ok(v) => v.render_once(tmpl),
            Err(e) => tmpl.record_error(e),
        }
    }
}

impl<'a> RenderOnce for fmt::Arguments<'a> {
    #[inline]
    fn render_once(self, tmpl: &mut TemplateBuffer) {
        self.render(tmpl)
    }
}

impl<'a> RenderMut for fmt::Arguments<'a> {
    #[inline]
    fn render_mut(&mut self, tmpl: &mut TemplateBuffer) {
        self.render(tmpl);
    }
}

impl<'a> Render for fmt::Arguments<'a> {
    #[inline]
    fn render(&self, tmpl: &mut TemplateBuffer) {
        tmpl.write_fmt(*self);
    }
}

macro_rules! impl_fmt_render {
    ($($t:ty),+) => {
        $(
            impl Render for $t {
                #[inline]
                fn render(&self, tmpl: &mut TemplateBuffer) {
                    write!(tmpl, "{}", self)
                }
            }
            impl RenderMut for $t {
                #[inline]
                fn render_mut(&mut self, tmpl: &mut TemplateBuffer) {
                    self.render(tmpl)
                }
            }

            impl RenderOnce for $t {
                #[inline]
                fn render_once(self, tmpl: &mut TemplateBuffer) {
                    self.render(tmpl)
                }
            }
        )+
    }
}

impl_fmt_render!(i8, i16, i32, i64, isize,
                 u8, u16, u32, u64, usize,
                          f32, f64, char);
