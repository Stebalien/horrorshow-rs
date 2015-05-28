use std::ops::Shl;
use template::TemplateBuilder;
use render::RenderOnce;

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

/* I may someday want these but not now... (probably never).

impl<F> Shr<String> for Renderer<F> where Renderer<F>: Template {
    type Output = String;
    fn shr(self, mut r: String) -> String {
        self.write_to_string(&mut r);
        r
    }
}

impl<'a, F> Shr<String> for &'a Renderer<F> where &'a Renderer<F>: Template {
    type Output = String;
    fn shr(self, mut r: String) -> String {
        self.write_to_string(&mut r);
        r
    }
}

impl<'a, F> Shr<String> for &'a mut Renderer<F> where &'a mut Renderer<F>: Template {
    type Output = String;
    fn shr(self, mut r: String) -> String {
        self.write_to_string(&mut r);
        r
    }
}

impl<'s, F> Shr<&'s mut String> for Renderer<F> where Renderer<F>: Template {
    type Output = &'s mut String;
    fn shr(self, r: &'s mut String) -> Self::Output {
        self.write_to_string(r);
        r
    }
}

impl<'a, 's, F> Shr<&'s mut String> for &'a Renderer<F> where &'a Renderer<F>: Template {
    type Output = &'s mut String;
    fn shr(self, r: &'s mut String) -> Self::Output {
        self.write_to_string(r);
        r
    }
}

impl<'a, 's, F> Shr<&'s mut String> for &'a mut Renderer<F> where &'a mut Renderer<F>: Template {
    type Output = &'s mut String;
    fn shr(self, r: &'s mut String) -> Self::Output {
        self.write_to_string(r);
        r
    }
}

impl<'s, F> Shr<&'s mut io::Write> for Renderer<F> where Renderer<F>: Template {
    type Output = Result<&'s mut io::Write, io::Error>;
    fn shr(self, r: &'s mut io::Write) -> Self::Output {
        self.write_to_io(r).and(Ok(r))
    }
}

impl<'a, 's, F> Shr<&'s mut io::Write> for &'a Renderer<F> where &'a Renderer<F>: Template {
    type Output = Result<&'s mut io::Write, io::Error>;
    fn shr(self, r: &'s mut io::Write) -> Self::Output {
        self.write_to_io(r).and(Ok(r))
    }
}

impl<'a, 's, F> Shr<&'s mut io::Write> for &'a mut Renderer<F> where &'a mut Renderer<F>: Template {
    type Output = Result<&'s mut io::Write, io::Error>;
    fn shr(self, r: &'s mut io::Write) -> Self::Output {
        self.write_to_io(r).and(Ok(r))
    }
}

impl<'s, F> Shr<&'s mut fmt::Write> for Renderer<F> where Renderer<F>: Template {
    type Output = Result<&'s mut fmt::Write, fmt::Error>;
    fn shr(self, r: &'s mut fmt::Write) -> Self::Output {
        self.write_to_fmt(r).and(Ok(r))
    }
}

impl<'a, 's, F> Shr<&'s mut fmt::Write> for &'a Renderer<F> where &'a Renderer<F>: Template {
    type Output = Result<&'s mut fmt::Write, fmt::Error>;
    fn shr(self, r: &'s mut fmt::Write) -> Self::Output {
        self.write_to_fmt(r).and(Ok(r))
    }
}

impl<'a, 's, F> Shr<&'s mut fmt::Write> for &'a mut Renderer<F> where &'a mut Renderer<F>: Template {
    type Output = Result<&'s mut fmt::Write, fmt::Error>;
    fn shr(self, r: &'s mut fmt::Write) -> Self::Output {
        self.write_to_fmt(r).and(Ok(r))
    }
}
*/
