/// Crate a new html template
#[macro_export]
macro_rules! html {
    ($($inner:tt)*) => {{
        // Stringify the template content to get a hint at how much we should allocate...
        $crate::__with_template_scope(stringify!($($inner)*).len(), || {
            append_html!($($inner)*);
        })
    }}
}

#[macro_export]
macro_rules! stringify_compressed {
    ($($tok:tt)*) => {
        concat!($(stringify!($tok)),*)
    };
}

/// Append html to the current template.
#[macro_export]
macro_rules! append_html {
    (: {$($code:tt)*} $($next:tt)*) => {{
        append!({$($code)*})
        append_html!($($next)*);
    }};
    (: $code:expr; $($next:tt)* ) => {{
        append!($code);
        append_html!($($next)*);
    }};
    (: $code:expr ) => {{
        append!($code);
    }};
    (! {$($code:tt)*} $($next:tt)*) => {{
        append_raw!({$($code)*});
        append_html!($($next)*);
    }};
    (! $code:expr; $($next:tt)* ) => {{
        append_raw!( $code);
        append_html!($($next)*);
    }};
    (! $code:expr ) => {{
        append_raw!($code);
    }};
    (@ {$($code:tt)*} $($next:tt)*) => {{
        __horrorshow_block_identity!({$($code)*});
        append_html!($($next)*);
    }};
    (@ $code:stmt; $($next:tt)* ) => {{
        $code;
        append_html!($($next)*);
    }};
    (@ $code:stmt ) => {{
        $code;
    }};
    (#{$($tok:tt)+} $($next:tt)*) => {{
        append_fmt!($($tok)+);
        append_html!($($next)*);
    }};
    ($($tag:ident):+($($($($attr:ident)-+):+ = $value:expr),+) { $($children:tt)* } $($next:tt)* ) => {{
        append_raw!(concat!("<", stringify_compressed!($($tag):+)));
        $(
            append_raw!(concat!(" ", stringify_compressed!($($($attr)-+):+), "=\""));
            append_fmt!("{}", $value);
            append_raw!("\"");
        )+
        append_raw!(">");
        append_html!($($children)*);
        append_raw!(concat!("</", stringify_compressed!($($tag):+), ">"));
        append_html!($($next)*);
    }};
    ($($tag:ident):+($($($($attr:ident)-+):+ = $value:expr),+); $($next:tt)*) => {{
        append_raw!(concat!("<", stringify_compressed!($($tag):+)));
        $(
            append_raw!(concat!(" ", stringify_compressed!($($($attr)-+):+), "=\""));
            append_fmt!("{}", $value);
            append_raw!("\"");
        )+
        append_raw!(" />");
        append_html!($($next)*);
    }};
    ($($tag:ident):+($($($($attr:ident)-+):+ = $value:expr),+)) => {{
        append_raw!(concat!("<", stringify_compressed!($($tag):+)));
        $(
            append_raw!(concat!(" ", stringify_compressed!($($($attr)-+):+), "=\""));
            append_fmt!("{}", $value);
            append_raw!("\"");
        )+
        append_raw!(" />");
    }};
    ($($tag:ident):+ { $($children:tt)* } $($next:tt)* ) => {{
        append_raw!(concat!("<", stringify_compressed!($($tag):+), ">"));
        append_html!($($children)*);
        append_raw!(concat!("</", stringify_compressed!($($tag):+), ">"));
        append_html!($($next)*);
    }};
    ($($tag:ident):+; $($next:tt)*) => {{
        append_raw!(concat!("<", stringify_compressed!($($tag):+), " />"));
        append_html!($($next)*);
    }};
    ($($tag:ident):+) => {{
        append_raw!(concat!("<", stringify_compressed!($($tag):+), "/>"))
    }};
    () => {};
}
