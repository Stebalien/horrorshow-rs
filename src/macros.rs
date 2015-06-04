/// Crate a new html template
#[macro_export]
macro_rules! html {
    ($($inner:tt)*) => {{
        // Define this up here to prevent rust from saying:
        // Hey look, it's an FnOnce (this could be Fn/FnMut).
        let f = |tmpl: &mut $crate::TemplateBuilder| -> () {
            __append_html!(tmpl, $($inner)*);
        };
        // Stringify the template content to get a hint at how much we should allocate...
        $crate::__new_renderer(stringify!($($inner)*).len(), f)
    }}
}

/// Crate a new owned html template.
///
/// This template will be boxed and will own it's environment. If you need to return a template
/// from a function, use this.
///
/// Example:
///
/// ```rust
/// # #[macro_use]
/// # extern crate horrorshow;
///
/// # fn main() {
/// fn post<'a>(title: &'a str) -> Box<horrorshow::RenderBox + 'a> {
///     box_html! {
///         article {
///             title { h1 : title }
///             p : "This is one paragraph.";
///             p : "This is a second.";
///         }
///     }
/// }
/// println!("{}", html! {
///     html {
///         body {
///             : post("First Post");
///             |t| (0..10).fold(t, |t, i| {
///                 // Waiting for non-lexical borrows!!!!
///                 let tmp = format!("Spam post {}", i);
///                 let post = post(&tmp);
///                 t << post
///             })
///         }
///     }
/// });
/// # }
/// ```
#[macro_export]
macro_rules! box_html {
    ($($inner:tt)*) => {{
        // Define this up here to prevent rust from saying:
        // Hey look, it's an FnOnce (this could be Fn/FnMut).
        let f = move |tmpl: &mut $crate::TemplateBuilder| -> () {
            __append_html!(tmpl, $($inner)*);
        };
        // Stringify the template content to get a hint at how much we should allocate...
        $crate::__new_boxed_renderer(stringify!($($inner)*).len(), f)
    }}
}

#[doc(hidden)]
#[macro_export]
macro_rules! stringify_compressed {
    ($($tok:tt)*) => {
        concat!($(stringify!($tok)),*)
    };
}

/// Mark a string as a raw. The string will not be rendered.
#[macro_export]
macro_rules! raw {
    ($e:expr) => { $crate::Raw::new($e) }
}

// We shouldn't need this but without it I get the following error:
// error: unexpected token: `an interpolated tt`
#[macro_export]
#[doc(hidden)]
macro_rules! __horrorshow_block_identity {
    ($b:block) => { $b };
}

/// Render attributes.
/// Don't call this manually.
#[macro_export]
#[doc(hidden)]
macro_rules! __append_attrs {
    ($tmpl:ident, $($($attr:ident)-+):+ = #{$($fmt_value:tt)+}, $($rest:tt)+) => {
        __append_attrs!($tmpl, $($($attr)-+):+ = #{$($fmt_value)+});
        __append_attrs!($tmpl, $($rest)+);
    };
    ($tmpl:ident, $($($attr:ident)-+):+ = #{$($fmt:tt)+}) => {
        $tmpl.write_raw(concat!(" ", stringify_compressed!($($($attr)-+):+), "=\""));
        write!($tmpl, $($fmt)*);
        $tmpl.write_raw("\"");
    };
    ($tmpl:ident, $($($attr:ident)-+):+ ?= $value:expr, $($rest:tt)+) => {
        __append_attrs!($tmpl, $($($attr)-+):+ ?= $value);
        __append_attrs!($tmpl, $($rest)+);
    };
    ($tmpl:ident, $($($attr:ident)-+):+ ?= $value:expr) => {
        match $crate::BoolOption::bool_option($value) {
            (_, None) => {},
            (true, Some(_)) => { __append_attrs!($tmpl, $($($attr)-+):+); }
            (false, Some(v)) => { __append_attrs!($tmpl, $($($attr)-+):+ = v); }
        };
    };
    ($tmpl:ident, $($($attr:ident)-+):+ = $value:expr, $($rest:tt)+) => {
        __append_attrs!($tmpl, $($($attr)-+):+ = $value);
        __append_attrs!($tmpl, $($rest)+);
    };
    ($tmpl:ident, $($($attr:ident)-+):+, $($rest:tt)+) => {
        __append_attrs!($tmpl, $($($attr)-+):+);
        __append_attrs!($tmpl, $($rest)+);
    };
    ($tmpl:ident, $($($attr:ident)-+):+ = $value:expr) => {
        $tmpl.write_raw(concat!(" ", stringify_compressed!($($($attr)-+):+), "=\""));
        $crate::RenderOnce::render_once($value, $tmpl);
        $tmpl.write_raw("\"");
    };
    ($tmpl:ident, $($($attr:ident)-+):+) => {
        $tmpl.write_raw(concat!(" ", stringify_compressed!($($($attr)-+):+)));
    };

}

/// Append html to the current template.
/// Don't call this manually.
#[doc(hidden)]
#[macro_export]
macro_rules! __append_html {
    ($tmpl:ident, : {$($code:tt)*} $($next:tt)*) => {
        $crate::RenderOnce::render_once({$($code)*}, $tmpl);
        __append_html!($tmpl, $($next)*);
    };
    ($tmpl:ident, : $code:expr; $($next:tt)* ) => {
        $crate::RenderOnce::render_once($code, $tmpl);
        __append_html!($tmpl, $($next)*);
    };
    ($tmpl:ident, : $code:expr ) => {
        $crate::RenderOnce::render_once($code, $tmpl);
    };
    ($tmpl:ident, |$var:ident| {$($code:tt)*} $($next:tt)*) => {
        (|$var: &mut $crate::TemplateBuilder| {
            __horrorshow_block_identity!({$($code)*})
        })($tmpl);
        __append_html!($tmpl, $($next)*);
    };
    ($tmpl:ident, |mut $var:ident| {$($code:tt)*} $($next:tt)*) => {
        (|mut $var: &mut $crate::TemplateBuilder| {
            __horrorshow_block_identity!({$($code)*})
        })($tmpl);
        __append_html!($tmpl, $($next)*);
    };
    ($tmpl:ident, |$var:ident| $code:stmt; $($next:tt)* ) => {
        (|$var: &mut $crate::TemplateBuilder| {
            $code;
        })($tmpl);
        __append_html!($tmpl, $($next)*);
    };
    ($tmpl:ident, |mut $var:ident| $code:stmt; $($next:tt)* ) => {
        (|mut $var: &mut $crate::TemplateBuilder| {
            $code;
        })($tmpl);
        __append_html!($tmpl, $($next)*);
    };
    ($tmpl:ident, |$var:ident| $code:stmt ) => {
        (|$var: &mut $crate::TemplateBuilder| {
            $code;
        })($tmpl);
    };
    ($tmpl:ident, |mut $var:ident| $code:stmt ) => {
        (|mut $var: &mut $crate::TemplateBuilder| {
            $code;
        })($tmpl);
    };
    ($tmpl:ident, #{$($tok:tt)+} $($next:tt)*) => {
        write!($tmpl, $($tok)+);
        __append_html!($tmpl, $($next)*);
    };
    ($tmpl:ident, $tag:ident($($attrs:tt)+) { $($children:tt)* } $($next:tt)* ) => {
        $tmpl.write_raw(concat!("<", stringify!($tag)));
        __append_attrs!($tmpl, $($attrs)+);
        $tmpl.write_raw(">");
        __append_html!($tmpl, $($children)*);
        $tmpl.write_raw(concat!("</", stringify!($tag), ">"));
        __append_html!($tmpl, $($next)*);
    };
    ($tmpl:ident, $tag:ident($($attr:tt)+) : $e:expr; $($next:tt)* ) => {
        __append_html!($tmpl, $tag($($attr)+) { : $e; } $($next)* );
    };
    ($tmpl:ident, $tag:ident($($attr:tt)+) : {$($code:tt)*} $($next)* ) => {
        __append_html!($tmpl, $tag($($attr)+) { : {$($code)*} } $($next)* );
    };
    ($tmpl:ident, $tag:ident($($attrs:tt)+); $($next:tt)*) => {
        $tmpl.write_raw(concat!("<", stringify!($tag)));
        __append_attrs!($tmpl, $($attrs)+);
        $tmpl.write_raw(" />");
        __append_html!($tmpl, $($next)*);
    };
    ($tmpl:ident, $tag:ident($($attrs:tt)+)) => {
        $tmpl.write_raw(concat!("<", stringify!($tag)));
        __append_attrs!($tmpl, $($attrs)+);
        $tmpl.write_raw(" />");
    };
    ($tmpl:ident, $tag:ident { $($children:tt)* } $($next:tt)* ) => {
        $tmpl.write_raw(concat!("<", stringify!($tag), ">"));
        __append_html!($tmpl, $($children)*);
        $tmpl.write_raw(concat!("</", stringify!($tag), ">"));
        __append_html!($tmpl, $($next)*);
    };
    ($tmpl:ident, $tag:ident : $e:expr; $($next:tt)* ) => {
        __append_html!($tmpl, $tag { : $e; } $($next)* );
    };
    ($tmpl:ident, $tag:ident : {$($code:tt)*} $($next:tt)* ) => {
        __append_html!($tmpl, $tag { : {$($code)*} } $($next)* );
    };
    ($tmpl:ident, $tag:ident; $($next:tt)*) => {
        $tmpl.write_raw(concat!("<", stringify!($tag), " />"));
        __append_html!($tmpl, $($next)*);
    };
    ($tmpl:ident, $tag:ident : $e:expr) => {
        __append_html!($tmpl, $tag { : $e; });
    };
    /* Wait for 1.2
    ($tmpl:ident, $tag:ident($($attrs:tt)+) $tag2:ident $($next:tt)* ) => {
        __append_html!($tmpl, $tag($($attrs)*) { $tag2 $($next)* });
    };
    ($tmpl:ident, $tag:ident $tag2:ident $($next:tt)* ) => {
        __append_html!($tmpl, $tag { $tag2 $($next)* });
    };
    */
    ($tmpl:ident, $tag:ident) => {
        $tmpl.write_raw(concat!("<", stringify!($tag), "/>"));
    };
    ($tmpl:ident,) => {};
}
