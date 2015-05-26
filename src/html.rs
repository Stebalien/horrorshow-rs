/// Crate a new html template
#[macro_export]
macro_rules! html {
    ($($inner:tt)*) => {{
        use $crate::Result;
        #[allow(unused_imports)]
        use $crate::TemplateComponent;
        // Stringify the template content to get a hint at how much we should allocate...
        $crate::__new_renderer(stringify!($($inner)*).len(), |tmpl| -> Result<()> {
            __append_html!(tmpl, $($inner)*);
            Ok(())
        })
    }}
}

#[doc(hidden)]
#[macro_export]
macro_rules! stringify_compressed {
    ($($tok:tt)*) => {
        concat!($(stringify!($tok)),*)
    };
}

#[macro_export]
macro_rules! __horrorshow_try {
    ($e:expr) => {
        if let Err(e) = $e {
            return Err(e);
        }
    }
}

/// Append html to the current template.
/// Don't call this manually.
#[doc(hidden)]
#[macro_export]
macro_rules! __append_html {
    ($tmpl:ident, : {$($code:tt)*} $($next:tt)*) => {{
        __horrorshow_try!(({$($code)*}).render_into($tmpl));
        __append_html!($tmpl, $($next)*);
    }};
    ($tmpl:ident, : $code:expr; $($next:tt)* ) => {{
        __horrorshow_try!((($code)).render_into($tmpl));
        __append_html!($tmpl, $($next)*);
    }};
    ($tmpl:ident, : $code:expr ) => {{
        __horrorshow_try!((($code)).render_into($tmpl));
    }};
    ($tmpl:ident, |$var:ident| {$($code:tt)*} $($next:tt)*) => {{
        __horrorshow_try!((|$var: &mut $crate::TemplateBuilder| -> Result<()> {
            __horrorshow_block_identity!({$($code)*})
        })($tmpl));
        __append_html!($tmpl, $($next)*);
    }};
    ($tmpl:ident, |mut $var:ident| {$($code:tt)*} $($next:tt)*) => {{
        __horrorshow_try!((|mut $var: &mut $crate::TemplateBuilder| -> Result<()> {
            __horrorshow_block_identity!({$($code)*})
        })($tmpl));
        __append_html!($tmpl, $($next)*);
    }};
    ($tmpl:ident, |$var:ident| $code:stmt; $($next:tt)* ) => {{
        __horrorshow_try!((|$var: &mut $crate::TemplateBuilder| -> Result<()> {
            $code
        })($tmpl));
        __append_html!($tmpl, $($next)*);
    }};
    ($tmpl:ident, |mut $var:ident| $code:stmt; $($next:tt)* ) => {{
        __horrorshow_try!((|mut $var: &mut $crate::TemplateBuilder| -> Result<()> {
            $code
        })($tmpl));
        __append_html!($tmpl, $($next)*);
    }};
    ($tmpl:ident, |$var:ident| $code:stmt ) => {{
        __horrorshow_try!((|$var: &mut $crate::TemplateBuilder| -> Result<()> {
            $code
        })($tmpl));
    }};
    ($tmpl:ident, |mut $var:ident| $code:stmt ) => {{
        __horrorshow_try!((|mut $var: &mut $crate::TemplateBuilder| -> Result<()> {
            $code
        })($tmpl));
    }};
    ($tmpl:ident, #{$($tok:tt)+} $($next:tt)*) => {{
        __horrorshow_try!(write!($tmpl, $($tok)+));
        __append_html!($tmpl, $($next)*);
    }};
    ($tmpl:ident, $tag:ident($($($($attr:ident)-+):+ = $value:expr),+) { $($children:tt)* } $($next:tt)* ) => {{
        __horrorshow_try!($tmpl.write_raw(concat!("<", stringify!($tag))));
        $(
            __horrorshow_try!($tmpl.write_raw(concat!(" ", stringify_compressed!($($($attr)-+):+), "=\"")));
            __horrorshow_try!(write!($tmpl, "{}", $value));
            __horrorshow_try!($tmpl.write_raw("\""));
        )+
        __horrorshow_try!($tmpl.write_raw(">"));
        __append_html!($tmpl, $($children)*);
        __horrorshow_try!($tmpl.write_raw(concat!("</", stringify!($tag), ">")));
        __append_html!($tmpl, $($next)*);
    }};
    ($tmpl:ident, $tag:ident($($attr:tt)+) : $e:expr; $($next:tt)* ) => {{
        __append_html!($tmpl, $tag($($attr)+) { : $e; } $($next)* );
    }};
    ($tmpl:ident, $tag:ident($($attr:tt)+) : {$($code:tt)*} $($next)* ) => {{
        __append_html!($tmpl, $tag($($attr)+) { : {$($code)*} } $($next)* );
    }};
    ($tmpl:ident, $tag:ident($($($($attr:ident)-+):+ = $value:expr),+); $($next:tt)*) => {{
        __horrorshow_try!($tmpl.write_raw(concat!("<", stringify!($tag))));
        $(
            __horrorshow_try!($tmpl.write_raw(concat!(" ", stringify_compressed!($($($attr)-+):+), "=\"")));
            __horrorshow_try!(write!($tmpl, "{}", $value));
            __horrorshow_try!($tmpl.write_raw("\""));
        )+
        __horrorshow_try!($tmpl.write_raw(" />"));
        __append_html!($tmpl, $($next)*);
    }};
    ($tmpl:ident, $tag:ident($($($($attr:ident)-+):+ = $value:expr),+)) => {{
        __horrorshow_try!($tmpl.write_raw(concat!("<", stringify!($tag))));
        $(
            __horrorshow_try!($tmpl.write_raw(concat!(" ", stringify_compressed!($($($attr)-+):+), "=\"")));
            __horrorshow_try!(write!($tmpl, "{}", $value));
            __horrorshow_try!($tmpl.write_raw("\""));
        )+
        $tmpl.write_raw(" />");
    }};
    ($tmpl:ident, $tag:ident { $($children:tt)* } $($next:tt)* ) => {{
        __horrorshow_try!($tmpl.write_raw(concat!("<", stringify!($tag), ">")));
        __append_html!($tmpl, $($children)*);
        __horrorshow_try!($tmpl.write_raw(concat!("</", stringify!($tag), ">")));
        __append_html!($tmpl, $($next)*);
    }};
    ($tmpl:ident, $tag:ident : $e:expr; $($next:tt)* ) => {{
        __append_html!($tmpl, $tag { : $e; } $($next)* );
    }};
    ($tmpl:ident, $tag:ident : {$($code:tt)*} $($next:tt)* ) => {{
        __append_html!($tmpl, $tag { : {$($code)*} } $($next)* );
    }};
    ($tmpl:ident, $tag:ident; $($next:tt)*) => {{
        __horrorshow_try!($tmpl.write_raw(concat!("<", stringify!($tag), " />")));
        __append_html!($tmpl, $($next)*);
    }};
    ($tmpl:ident, $tag:ident : $e:expr) => {{
        __append_html!($tmpl, $tag { : $e; });
    }};
    ($tmpl:ident, $tag:ident) => {{
        __horrorshow_try!($tmpl.write_raw(concat!("<", stringify!($tag), "/>")));
    }};
    ($tmpl:ident,) => {};
}
