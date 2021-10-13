//! Helpers templates.

macro_rules! c {
    ($doc:expr => $name:ident $value:tt) => {
        #[doc = $doc]
        #[allow(non_upper_case_globals)]
        pub const $name: $crate::render::Raw<&'static str> = $crate::render::Raw($value);
    };
}

/// Helpers for inserting `DOCTYPE`s.
pub mod doctype {
    c!("The standard HTML5 doctype (this *is* the doctype you're looking for)." => HTML "<!DOCTYPE html>");

    /// HTML 4.01 doctypes
    #[allow(non_snake_case)]
    pub mod HTML4 {
        c!("The HTML 4.001 strict doctype." =>
           Strict "<!DOCTYPE HTML PUBLIC \"-//W3C//DTD HTML 4.01//EN\" \"http://www.w3.org/TR/html4/strict.dtd\">");

        c!("The HTML 4.001 transitional doctype." =>
           Transitional "<!DOCTYPE HTML PUBLIC \"-//W3C//DTD HTML 4.01 Transitional//EN\" \"http://www.w3.org/TR/html4/loose.dtd\">");

        c!("The HTML 4.001 frameset doctype." =>
           Frameset "<!DOCTYPE HTML PUBLIC \"-//W3C//DTD HTML 4.01 Frameset//EN\" \"http://www.w3.org/TR/html4/frameset.dtd\">");
    }
}
