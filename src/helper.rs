//! Helpers templates.

macro_rules! c {
    ($name:ident $value:tt) => {
        #[allow(non_upper_case_globals)]
        pub const $name: $crate::render::Raw<&'static str> = $crate::render::Raw($value);
    };
}

/// Helpers for inserting `DOCTYPE`s.
pub mod doctype {
    /// The standard HTML5 doctype (this *is* the doctype you're looking for).
    c!(HTML "<!DOCTYPE html>");

    /// HTML 4.01 doctypes
    #[allow(non_snake_case)]
    pub mod HTML4 {
        /// The HTML 4.001 strict doctype.
        c!(Strict "<!DOCTYPE HTML PUBLIC \"-//W3C//DTD HTML 4.01//EN\" \"http://www.w3.org/TR/html4/strict.dtd\">");

        /// The HTML 4.001 transitional doctype.
        c!(Transitional "<!DOCTYPE HTML PUBLIC \"-//W3C//DTD HTML 4.01 Transitional//EN\" \"http://www.w3.org/TR/html4/loose.dtd\">");

        /// The HTML 4.001 frameset doctype.
        c!(Frameset "<!DOCTYPE HTML PUBLIC \"-//W3C//DTD HTML 4.01 Frameset//EN\" \"http://www.w3.org/TR/html4/frameset.dtd\">");
    }
}
