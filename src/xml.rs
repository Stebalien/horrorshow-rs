/// Crate a new xml template
#[macro_export]
macro_rules! xml {
    ($($inner:tt)*) => {{
        // Stringify the template content to get a hint at how much we should allocate...
        $crate::__with_template_scope(stringify!($($inner)*).len(), || {
            append_xml!($($inner)*);
        })
    }}
}

#[macro_export]
macro_rules! stringify_compressed {
    ($($tok:tt)*) => {
        concat!($(stringify!($tok)),*)
    };
}

/// Append xml to the current template.
#[macro_export]
macro_rules! append_xml {
    (: {$($code:tt)*} $($next:tt)*) => {{
        append!({$($code)*})
        append_xml!($($next)*);
    }};
    (: $code:expr; $($next:tt)* ) => {{
        append!($code);
        append_xml!($($next)*);
    }};
    (: $code:expr ) => {{
        append!($code);
    }};
    (! {$($code:tt)*} $($next:tt)*) => {{
        append_raw!({$($code)*});
        append_xml!($($next)*);
    }};
    (! $code:expr; $($next:tt)* ) => {{
        append_raw!( $code);
        append_xml!($($next)*);
    }};
    (! $code:expr ) => {{
        append_raw!($code);
    }};
    (@ {$($code:tt)*} $($next:tt)*) => {{
        __horrorshow_block_identity!({$($code)*});
        append_xml!($($next)*);
    }};
    (@ $code:stmt; $($next:tt)* ) => {{
        $code;
        append_xml!($($next)*);
    }};
    (@ $code:stmt ) => {{
        $code;
    }};
    (#{$($tok:tt)+} $($next:tt)*) => {{
        append_fmt!($($tok)+);
        append_xml!($($next)*);
    }};
    ($($tag:ident):+($($($($attr:ident)-+):+ = $value:expr),+) { $($children:tt)* } $($next:tt)* ) => {{
        append_raw!(concat!("<", stringify_compressed!($($tag):+)));
        $(
            append_raw!(concat!(" ", stringify_compressed!($($($attr)-+):+), "=\""));
            append_fmt!("{}", $value);
            append_raw!("\"");
        )+
        append_raw!(">");
        append_xml!($($children)*);
        append_raw!(concat!("</", stringify_compressed!($($tag):+), ">"));
        append_xml!($($next)*);
    }};
    ($($tag:ident):+($($($($attr:ident)-+):+ = $value:expr),+); $($next:tt)*) => {{
        append_raw!(concat!("<", stringify_compressed!($($tag):+)));
        $(
            append_raw!(concat!(" ", stringify_compressed!($($($attr)-+):+), "=\""));
            append_fmt!("{}", $value);
            append_raw!("\"");
        )+
        append_raw!(" />");
        append_xml!($($next)*);
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
        append_xml!($($children)*);
        append_raw!(concat!("</", stringify_compressed!($($tag):+), ">"));
        append_xml!($($next)*);
    }};
    ($($tag:ident):+; $($next:tt)*) => {{
        append_raw!(concat!("<", stringify_compressed!($($tag):+), " />"));
        append_xml!($($next)*);
    }};
    ($($tag:ident):+) => {{
        append_raw!(concat!("<", stringify_compressed!($($tag):+), "/>"))
    }};
    () => {};
}
