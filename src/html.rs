/// Crate a new html template
#[macro_export]
macro_rules! html {
    ($($inner:tt)*) => {{
        #[allow(unused_imports)]
        use $crate::TemplateComponent;
        // Define this up here to prevent rust from saying:
        // Hey look, it's an FnOnce (this could be Fn/FnMut).
        let f = |tmpl: &mut $crate::TemplateBuilder| -> () {
            __append_html!(tmpl, $($inner)*);
        };
        // Stringify the template content to get a hint at how much we should allocate...
        $crate::__new_renderer(stringify!($($inner)*).len(), f)
    }}
}

#[doc(hidden)]
#[macro_export]
macro_rules! stringify_compressed {
    ($($tok:tt)*) => {
        concat!($(stringify!($tok)),*)
    };
}

/// Append html to the current template.
/// Don't call this manually.
#[doc(hidden)]
#[macro_export]
macro_rules! __append_html {
    ($tmpl:ident, : {$($code:tt)*} $($next:tt)*) => {{
        ({$($code)*}).render_into($tmpl);
        __append_html!($tmpl, $($next)*);
    }};
    ($tmpl:ident, : $code:expr; $($next:tt)* ) => {{
        (($code)).render_into($tmpl);
        __append_html!($tmpl, $($next)*);
    }};
    ($tmpl:ident, : $code:expr ) => {{
        (($code)).render_into($tmpl);
    }};
    ($tmpl:ident, |$var:ident| {$($code:tt)*} $($next:tt)*) => {{
        (|$var: &mut $crate::TemplateBuilder| {
            __horrorshow_block_identity!({$($code)*})
        })($tmpl);
        __append_html!($tmpl, $($next)*);
    }};
    ($tmpl:ident, |mut $var:ident| {$($code:tt)*} $($next:tt)*) => {{
        (|mut $var: &mut $crate::TemplateBuilder| {
            __horrorshow_block_identity!({$($code)*})
        })($tmpl);
        __append_html!($tmpl, $($next)*);
    }};
    ($tmpl:ident, |$var:ident| $code:stmt; $($next:tt)* ) => {{
        (|$var: &mut $crate::TemplateBuilder| {
            $code;
        })($tmpl);
        __append_html!($tmpl, $($next)*);
    }};
    ($tmpl:ident, |mut $var:ident| $code:stmt; $($next:tt)* ) => {{
        (|mut $var: &mut $crate::TemplateBuilder| {
            $code;
        })($tmpl);
        __append_html!($tmpl, $($next)*);
    }};
    ($tmpl:ident, |$var:ident| $code:stmt ) => {{
        (|$var: &mut $crate::TemplateBuilder| {
            $code;
        })($tmpl);
    }};
    ($tmpl:ident, |mut $var:ident| $code:stmt ) => {{
        (|mut $var: &mut $crate::TemplateBuilder| {
            $code;
        })($tmpl);
    }};
    ($tmpl:ident, #{$($tok:tt)+} $($next:tt)*) => {{
        write!($tmpl, $($tok)+);
        __append_html!($tmpl, $($next)*);
    }};
    ($tmpl:ident, $tag:ident($($($($attr:ident)-+):+ = $value:expr),+) { $($children:tt)* } $($next:tt)* ) => {{
        $tmpl.write_raw(concat!("<", stringify!($tag)));
        $(
            $tmpl.write_raw(concat!(" ", stringify_compressed!($($($attr)-+):+), "=\""));
            write!($tmpl, "{}", $value);
            $tmpl.write_raw("\"");
        )+
        $tmpl.write_raw(">");
        __append_html!($tmpl, $($children)*);
        $tmpl.write_raw(concat!("</", stringify!($tag), ">"));
        __append_html!($tmpl, $($next)*);
    }};
    ($tmpl:ident, $tag:ident($($attr:tt)+) : $e:expr; $($next:tt)* ) => {{
        __append_html!($tmpl, $tag($($attr)+) { : $e; } $($next)* );
    }};
    ($tmpl:ident, $tag:ident($($attr:tt)+) : {$($code:tt)*} $($next)* ) => {{
        __append_html!($tmpl, $tag($($attr)+) { : {$($code)*} } $($next)* );
    }};
    ($tmpl:ident, $tag:ident($($($($attr:ident)-+):+ = $value:expr),+); $($next:tt)*) => {{
        $tmpl.write_raw(concat!("<", stringify!($tag)));
        $(
            $tmpl.write_raw(concat!(" ", stringify_compressed!($($($attr)-+):+), "=\""));
            write!($tmpl, "{}", $value);
            $tmpl.write_raw("\"");
        )+
        $tmpl.write_raw(" />");
        __append_html!($tmpl, $($next)*);
    }};
    ($tmpl:ident, $tag:ident($($($($attr:ident)-+):+ = $value:expr),+)) => {{
        $tmpl.write_raw(concat!("<", stringify!($tag)));
        $(
            $tmpl.write_raw(concat!(" ", stringify_compressed!($($($attr)-+):+), "=\""));
            write!($tmpl, "{}", $value);
            $tmpl.write_raw("\"");
        )+
        $tmpl.write_raw(" />");
    }};
    ($tmpl:ident, $tag:ident { $($children:tt)* } $($next:tt)* ) => {{
        $tmpl.write_raw(concat!("<", stringify!($tag), ">"));
        __append_html!($tmpl, $($children)*);
        $tmpl.write_raw(concat!("</", stringify!($tag), ">"));
        __append_html!($tmpl, $($next)*);
    }};
    ($tmpl:ident, $tag:ident : $e:expr; $($next:tt)* ) => {{
        __append_html!($tmpl, $tag { : $e; } $($next)* );
    }};
    ($tmpl:ident, $tag:ident : {$($code:tt)*} $($next:tt)* ) => {{
        __append_html!($tmpl, $tag { : {$($code)*} } $($next)* );
    }};
    ($tmpl:ident, $tag:ident; $($next:tt)*) => {{
        $tmpl.write_raw(concat!("<", stringify!($tag), " />"));
        __append_html!($tmpl, $($next)*);
    }};
    ($tmpl:ident, $tag:ident : $e:expr) => {{
        __append_html!($tmpl, $tag { : $e; });
    }};
    ($tmpl:ident, $tag:ident) => {{
        $tmpl.write_raw(concat!("<", stringify!($tag), "/>"));
    }};
    ($tmpl:ident,) => {};
}
