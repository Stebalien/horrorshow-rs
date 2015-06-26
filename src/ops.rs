use std::ops::Shl;
use template::TemplateBuffer;
use render::RenderOnce;

impl<'a, 'b, T> Shl<T> for &'a mut TemplateBuffer<'b>
    where T: RenderOnce
{
    type Output = ();
    /// Render the component into the template.
    ///
    /// Note: If writing to the template fails, this method will neither panic nor return errors.
    /// Instead, no more data will be written to the template and horrorshow abort template
    /// rendering (return an error) when it re-gains control.
    fn shl(self, component: T) {
        component.render_once(self);
    }
}
