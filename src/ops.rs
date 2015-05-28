use std::ops::Shl;
use template::TemplateBuilder;
use render::RenderOnce;

impl<'a, 'b, T> Shl<T> for &'a mut TemplateBuilder<'b>
    where T: RenderOnce
{
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
