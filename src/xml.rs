#[macro_export]
macro_rules! xml {
    ($($inner:tt)*) => {{
        $crate::__with_template_scope(|| {
            append_xml!($($inner)*);
        })
    }}
}


#[macro_export]
macro_rules! append_xml {
    (: {$($code:expr);+} $($next:tt)*) => {{
        append!("{}", {$($code);+});
        append_xml!($($next)*);
    }};
    (: $code:expr; $($next:tt)* ) => {{
        append!("{}", $code);
        append_xml!($($next)*);
    }};
    (: $code:expr ) => {{
        append!("{}", $code);
    }};
    (@ {$($code:expr);+} $($next:tt)*) => {{
        $($code);+
        append_xml!($($next)*);
    }};
    (@ $code:expr; $($next:tt)* ) => {{
        $code;
        append_xml!($($next)*);
    }};
    (@ $code:expr ) => {{
        append_xml!(@ {$code});
    }};
    (#{$($tok:tt)+} $($next:tt)*) => {{
        append!($($tok)+);
        append_xml!($($next)*);
    }};
    ($tag:ident($($attr:ident=$value:expr),+) { $($children:tt)* } $($next:tt)* ) => {{
        append!(concat!("<", stringify!($tag), $(concat!(" ", stringify!($attr), "=\"{}\"")),+, ">"), $($value),+);
        append_xml!($($children)*);
        append!(concat!("</", stringify!($tag), ">"));
        append_xml!($($next)*);
    }};
    ($tag:ident($($attr:ident=$value:expr),+); $($next:tt)*) => {{
        append!(concat!("<", stringify!($tag), $(concat!(" ", stringify!($attr), "=\"{}\"")),+, " />"), $($value),+);
        append_xml!($($next)*);
    }};
    ($tag:ident($($attr:ident=$value:expr),+)) => {{
        append!(concat!("<", stringify!($tag), $(concat!(" ", stringify!($attr), "=\"{}\"")),+, "/>"), $($value),+);
    }};
    ($tag:ident { $($children:tt)* } $($next:tt)* ) => {{
        append!(concat!("<", stringify!($tag), ">"));
        append_xml!($($children)*);
        append!(concat!("</", stringify!($tag), ">"));
        append_xml!($($next)*);
    }};
    ($tag:ident; $($next:tt)*) => {{
        append!(concat!("<", stringify!($tag), " />"));
        append_xml!($($next)*);
    }};
    ($tag:ident) => {{
        append!(concat!("<", stringify!($tag), "/>"))
    }};
    () => {""};
}
