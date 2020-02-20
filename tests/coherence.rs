#![cfg(feature = "alloc")]

#[macro_use]
extern crate horrorshow;

use horrorshow::prelude::*;

struct Test;

impl RenderOnce for Test {
    fn render_once(self, t: &mut TemplateBuffer<'_>) {
        self.render(t);
    }
}

impl RenderMut for Test {
    fn render_mut(&mut self, t: &mut TemplateBuffer<'_>) {
        self.render(t);
    }
}

impl Render for Test {
    fn render(&self, t: &mut TemplateBuffer<'_>) {
        t.write_str("Test");
    }
}

#[test]
fn test_coherence() {
    assert_eq!(
        (html! {
            |t| t << Test;
            |t| t << &mut Test;
            |t| t << &Test;
        })
        .into_string()
        .unwrap(),
        "TestTestTest"
    );
}
