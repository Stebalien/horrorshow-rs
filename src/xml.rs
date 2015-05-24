/// Crate a new xml template
#[macro_export]
macro_rules! xml {
    ($($inner:tt)*) => {{
        // Stringify the template content to get a hint at how much we should allocate...
        $crate::__new_renderer(stringify!($($inner)*).len(), |tmpl| {
            use ::std::fmt::Write;
            __append_xml!(tmpl $($inner)*);
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
macro_rules! __append_xml {
    ($tmpl:ident : {$($code:tt)*} $($next:tt)*) => {{
        $tmpl.write_str(&{$($code)*}).unwrap();
        __append_xml!($tmpl $($next)*);
    }};
    ($tmpl:ident : $code:expr; $($next:tt)* ) => {{
        $tmpl.write_str(&($code)).unwrap();
        __append_xml!($tmpl $($next)*);
    }};
    ($tmpl:ident : $code:expr ) => {{
        $tmpl.write_str(&($code)).unwrap();
    }};
    ($tmpl:ident ! {$($code:tt)*} $($next:tt)*) => {{
        $tmpl.write_raw(&{$($code)*});
        __append_xml!($tmpl $($next)*);
    }};
    ($tmpl:ident ! $code:expr; $($next:tt)* ) => {{
        $tmpl.write_raw(&($code));
        __append_xml!($tmpl $($next)*);
    }};
    ($tmpl:ident ! $code:expr ) => {{
        $tmpl.write_raw(&($code));
    }};
    ($tmpl:ident |$var:ident| {$($code:tt)*} $($next:tt)*) => {{
        (|$var: &mut $crate::Template| {
            __horrorshow_block_identity!({$($code)*});
        })($tmpl);
        __append_xml!($tmpl $($next)*);
    }};
    ($tmpl:ident |mut $var:ident| {$($code:tt)*} $($next:tt)*) => {{
        (|mut $var: &mut $crate::Template| {
            __horrorshow_block_identity!({$($code)*});
        })($tmpl);
        __append_xml!($tmpl $($next)*);
    }};
    ($tmpl:ident |$var:ident| $code:stmt; $($next:tt)* ) => {{
        (|$var: &mut $crate::Template| { $code; })($tmpl);
        __append_xml!($tmpl $($next)*);
    }};
    ($tmpl:ident |mut $var:ident| $code:stmt; $($next:tt)* ) => {{
        (|mut $var: &mut $crate::Template| { $code; })($tmpl);
        __append_xml!($tmpl $($next)*);
    }};
    ($tmpl:ident |$var:ident| $code:stmt ) => {{
        (|$var: &mut $crate::Template| {$code;})($tmpl);
    }};
    ($tmpl:ident |mut $var:ident| $code:stmt ) => {{
        (|mut $var: &mut $crate::Template| {$code;})($tmpl);
    }};
    ($tmpl:ident #{$($tok:tt)+} $($next:tt)*) => {{
        write!($tmpl, $($tok)+).unwrap();
        __append_xml!($tmpl $($next)*);
    }};
    ($tmpl:ident $($tag:ident):+($($($($attr:ident)-+):+ = $value:expr),+) { $($children:tt)* } $($next:tt)* ) => {{
        $tmpl.write_raw(concat!("<", stringify_compressed!($($tag):+)));
        $(
            $tmpl.write_raw(concat!(" ", stringify_compressed!($($($attr)-+):+), "=\""));
            write!($tmpl, "{}", $value).unwrap();
            $tmpl.write_raw("\"");
        )+
        $tmpl.write_raw(">");
        __append_xml!($tmpl $($children)*);
        $tmpl.write_raw(concat!("</", stringify_compressed!($($tag):+), ">"));
        __append_xml!($tmpl $($next)*);
    }};
    ($tmpl:ident $($tag:ident):+($($($($attr:ident)-+):+ = $value:expr),+); $($next:tt)*) => {{
        $tmpl.write_raw(concat!("<", stringify_compressed!($($tag):+)));
        $(
            $tmpl.write_raw(concat!(" ", stringify_compressed!($($($attr)-+):+), "=\""));
            write!($tmpl, "{}", $value).unwrap();
            $tmpl.write_raw("\"");
        )+
        $tmpl.write_raw(" />");
        __append_xml!($tmpl $($next)*);
    }};
    ($tmpl:ident $($tag:ident):+($($($($attr:ident)-+):+ = $value:expr),+)) => {{
        $tmpl.write_raw(concat!("<", stringify_compressed!($($tag):+)));
        $(
            $tmpl.write_raw(concat!(" ", stringify_compressed!($($($attr)-+):+), "=\""));
            write!($tmpl, "{}", $value).unwrap();
            $tmpl.write_raw("\"");
        )+
        $tmpl.write_raw(" />");
    }};
    ($tmpl:ident $($tag:ident):+ { $($children:tt)* } $($next:tt)* ) => {{
        $tmpl.write_raw(concat!("<", stringify_compressed!($($tag):+), ">"));
        __append_xml!($tmpl $($children)*);
        $tmpl.write_raw(concat!("</", stringify_compressed!($($tag):+), ">"));
        __append_xml!($tmpl $($next)*);
    }};
    ($tmpl:ident $($tag:ident):+; $($next:tt)*) => {{
        $tmpl.write_raw(concat!("<", stringify_compressed!($($tag):+), " />"));
        __append_xml!($tmpl $($next)*);
    }};
    ($tmpl:ident $($tag:ident):+) => {{
        $tmpl.write_raw(concat!("<", stringify_compressed!($($tag):+), "/>"))
    }};
    ($tmpl:ident) => {};
}
