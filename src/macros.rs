/// Crate a new html template
#[macro_export]
macro_rules! html {
    ($($inner:tt)*) => {{
        // Define this up here to prevent rust from saying:
        // Hey look, it's an FnOnce (this could be Fn/FnMut).
        let f = |tmpl: &mut $crate::TemplateBuffer| -> () {
            append_html!(tmpl, $($inner)*);
        };
        // Stringify the template content to get a hint at how much we should allocate...
        $crate::FnRenderer::with_capacity(stringify!($($inner)*).len(), f)
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
///             |t| for i in 0..10 {
///                 // Waiting for non-lexical borrows!!!!
///                 let tmp = format!("Spam post {}", i);
///                 let post = post(&tmp);
///                 &mut *t << post;
///             };
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
        let f = move |tmpl: &mut $crate::TemplateBuffer| -> () {
            append_html!(tmpl, $($inner)*);
        };
        // Stringify the template content to get a hint at how much we should allocate...
        Box::new($crate::FnRenderer::with_capacity(stringify!($($inner)*).len(), f))
    }}
}

/// Mark a string as a raw. The string will not be rendered.
#[macro_export]
macro_rules! raw {
    ($e:expr) => { $crate::Raw::new($e) }
}

/// Append html to the current template.
/// Don't call this manually.
#[doc(hidden)]
#[macro_export]
macro_rules! append_html {

    (@stringify_compressed $($tok:tt)*) => {
        concat!($(stringify!($tok)),*)
    };

    (@block_identity $b:block) => { $b };
    (@cont $tmpl:ident, ($s:stmt), $($next:tt)*) => {
        $s;
        append_html!($tmpl, $($next)*);
    };
    (@expr_and_block $tmpl:ident, $goto:ident, ($($prefix:tt)*), {$($inner:tt)*} $($next:tt)*) => {
        append_html!(@$goto $tmpl, ($($prefix)* {append_html!($tmpl, $($inner)*);}), $($next)*);
    };
    (@expr_and_block $tmpl:ident, $goto:ident, ($($prefix:tt)*), $first:tt $($next:tt)*) => {
        append_html!(@expr_and_block $tmpl, $goto, ($($prefix)* $first), $($next)*);
    };
    (@append_attrs $tmpl:ident, $($($attr:ident)-+):+ ?= $value:expr, $($rest:tt)+) => {
        append_html!(@append_attrs $tmpl, $($($attr)-+):+ ?= $value);
        append_html!(@append_attrs $tmpl, $($rest)+);
    };
    (@append_attrs $tmpl:ident, $($($attr:ident)-+):+ ?= $value:expr) => {
        match $crate::BoolOption::bool_option($value) {
            (_, None) => {},
            (true, Some(_)) => { append_html!(@append_attrs $tmpl, $($($attr)-+):+); }
            (false, Some(v)) => { append_html!(@append_attrs $tmpl, $($($attr)-+):+ = v); }
        };
    };
    (@append_attrs $tmpl:ident, $($($attr:ident)-+):+ = $value:expr, $($rest:tt)+) => {
        append_html!(@append_attrs $tmpl, $($($attr)-+):+ = $value);
        append_html!(@append_attrs $tmpl, $($rest)+);
    };
    (@append_attrs $tmpl:ident, $($($attr:ident)-+):+, $($rest:tt)+) => {
        append_html!(@append_attrs $tmpl, $($($attr)-+):+);
        append_html!(@append_attrs $tmpl, $($rest)+);
    };
    (@append_attrs $tmpl:ident, $($($attr:ident)-+):+ = $value:expr) => {
        $tmpl.write_raw(concat!(" ", append_html!(@stringify_compressed $($($attr)-+):+), "=\""));
        $crate::RenderOnce::render_once($value, $tmpl);
        $tmpl.write_raw("\"");
    };
    (@append_attrs $tmpl:ident, $($($attr:ident)-+):+) => {
        $tmpl.write_raw(concat!(" ", append_html!(@stringify_compressed $($($attr)-+):+)));
    };
    //////// IF CHAINS
    //// Begin
    (@parse_if $tmpl:ident, ($($prefix:tt)*), if let $p:pat = $e:tt $($next:tt)+) => {
        append_html!(@expr_and_block $tmpl, parse_if_block, ($($prefix)* if let $p = $e), $($next)+);
    };
    (@parse_if $tmpl:ident, ($($prefix:tt)*), if $e:tt $($next:tt)+) => {
        append_html!(@expr_and_block $tmpl, parse_if_block, ($($prefix)* if $e), $($next)+);
    };
    //// End
    // Else if
    (@parse_if_block $tmpl:ident, ($($prefix:tt)*), else if $($next:tt)*) => {
        append_html!(@parse_if $tmpl, ($($prefix)* else), if $($next)*);
    };
    // Else
    (@parse_if_block $tmpl:ident, ($($prefix:tt)*), else {$($inner:tt)*} $($next:tt)*) => {
        append_html!(@cont $tmpl, ($($prefix)* else {append_html!($tmpl, $($inner)*);}), $($next)*);
    };
    // No else.
    (@parse_if_block $tmpl:ident, ($($prefix:tt)*), $($next:tt)*) => {
        append_html!(@cont $tmpl, ($($prefix)*), $($next)*);
    };
    //// Condition
    ($tmpl:ident, @ if $($next:tt)+) => {
        append_html!(@parse_if $tmpl, (), if $($next)*);
    };
    ($tmpl:ident, @ for $p:pat in $e:tt $($next:tt)*) => {
        append_html!(@expr_and_block $tmpl, cont, (for $p in $e), $($next)*);
    };
    ($tmpl:ident, @ while let $p:pat = $e:tt $($next:tt)*) => {
        append_html!(@expr_and_block $tmpl, cont, (while let $p = $e), $($next)*);
    };
    ($tmpl:ident, @ while $e:tt $($next:tt)*) => {
        append_html!(@expr_and_block $tmpl, cont, (while $e), $($next)*);
    };
    ($tmpl:ident, : {$($code:tt)*} $($next:tt)*) => {
        $crate::RenderOnce::render_once({$($code)*}, $tmpl);
        append_html!($tmpl, $($next)*);
    };
    ($tmpl:ident, : $code:expr; $($next:tt)* ) => {
        $crate::RenderOnce::render_once($code, $tmpl);
        append_html!($tmpl, $($next)*);
    };
    ($tmpl:ident, : $code:expr ) => {
        $crate::RenderOnce::render_once($code, $tmpl);
    };
    ($tmpl:ident, |$var:ident| {$($code:tt)*} $($next:tt)*) => {
        {
            let $var: &mut $crate::TemplateBuffer = &mut *$tmpl;
            append_html!(@block_identity {$($code)*})
        }
        append_html!($tmpl, $($next)*);
    };
    ($tmpl:ident, |mut $var:ident| {$($code:tt)*} $($next:tt)*) => {
        {
            let mut $var: &mut $crate::TemplateBuffer = &mut *$tmpl;
            append_html!(@block_identity {$($code)*})
        }
        append_html!($tmpl, $($next)*);
    };
    ($tmpl:ident, |$var:ident| $code:stmt; $($next:tt)* ) => {
        {
            let $var: &mut $crate::TemplateBuffer = &mut *$tmpl;
            $code;
        }
        append_html!($tmpl, $($next)*);
    };
    ($tmpl:ident, |mut $var:ident| $code:stmt; $($next:tt)* ) => {
        {
            let mut $var: &mut $crate::TemplateBuffer = &mut *$tmpl;
            $code;
        }
        append_html!($tmpl, $($next)*);
    };
    ($tmpl:ident, |$var:ident| $code:stmt ) => {{
        let $var: &mut $crate::TemplateBuffer = &mut *$tmpl;
        $code;
    }};
    ($tmpl:ident, |mut $var:ident| $code:stmt ) => {{
        let mut $var: &mut $crate::TemplateBuffer = &mut *$tmpl;
        $code;
    }};
    ($tmpl:ident, $tag:ident($($attrs:tt)+) { $($children:tt)* } $($next:tt)* ) => {
        $tmpl.write_raw(concat!("<", stringify!($tag)));
        append_html!(@append_attrs $tmpl, $($attrs)+);
        $tmpl.write_raw(">");
        append_html!($tmpl, $($children)*);
        $tmpl.write_raw(concat!("</", stringify!($tag), ">"));
        append_html!($tmpl, $($next)*);
    };
    ($tmpl:ident, $tag:ident($($attr:tt)+) : $e:expr; $($next:tt)* ) => {
        append_html!($tmpl, $tag($($attr)+) { : $e; } $($next)* );
    };
    ($tmpl:ident, $tag:ident($($attr:tt)+) : $e:expr) => {
        append_html!($tmpl, $tag($($attr)+) { : $e });
    };
    ($tmpl:ident, $tag:ident($($attr:tt)+) : {$($code:tt)*} $($next)* ) => {
        append_html!($tmpl, $tag($($attr)+) { : {$($code)*} } $($next)* );
    };
    ($tmpl:ident, $tag:ident($($attrs:tt)+); $($next:tt)*) => {
        $tmpl.write_raw(concat!("<", stringify!($tag)));
        append_html!(@append_attrs $tmpl, $($attrs)+);
        $tmpl.write_raw(" />");
        append_html!($tmpl, $($next)*);
    };
    ($tmpl:ident, $tag:ident($($attrs:tt)+)) => {
        $tmpl.write_raw(concat!("<", stringify!($tag)));
        append_html!(@append_attrs $tmpl, $($attrs)+);
        $tmpl.write_raw(" />");
    };
    ($tmpl:ident, $tag:ident { $($children:tt)* } $($next:tt)* ) => {
        $tmpl.write_raw(concat!("<", stringify!($tag), ">"));
        append_html!($tmpl, $($children)*);
        $tmpl.write_raw(concat!("</", stringify!($tag), ">"));
        append_html!($tmpl, $($next)*);
    };
    ($tmpl:ident, $tag:ident : $e:expr; $($next:tt)* ) => {
        append_html!($tmpl, $tag { : $e; } $($next)* );
    };
    ($tmpl:ident, $tag:ident : {$($code:tt)*} $($next:tt)* ) => {
        append_html!($tmpl, $tag { : {$($code)*} } $($next)* );
    };
    ($tmpl:ident, $tag:ident; $($next:tt)*) => {
        $tmpl.write_raw(concat!("<", stringify!($tag), " />"));
        append_html!($tmpl, $($next)*);
    };
    ($tmpl:ident, $tag:ident : $e:expr) => {
        append_html!($tmpl, $tag { : $e; });
    };
    ($tmpl:ident, $tag:ident) => {
        $tmpl.write_raw(concat!("<", stringify!($tag), "/>"));
    };
    ($tmpl:ident,) => {};
}

/// Create a new template.
///
/// This allows you to declare a template as follows:
///
/// ```
/// # #[macro_use]
/// # extern crate horrorshow;
/// template! {
///     MyTemplate(name: &str, age: &u32) {
///         p {
///            : "Hello, my name is ";
///            : name;
///            : " and I am ";
///            : age;
///            : " years old.";
///         }
///     }
/// }
/// # fn main() { }
/// ```
///
/// You can instantiate these templates by calling `new` on them:
///
/// ```
/// # #[macro_use]
/// # extern crate horrorshow;
/// # template! {
/// #     MyTemplate(name: &str, age: &u32) {
/// #         p {
/// #            : "Hello, my name is ";
/// #            : name;
/// #            : " and I am ";
/// #            : age;
/// #            : " years old.";
/// #         }
/// #     }
/// # }
///
/// # fn main() {
/// let age = 42;
/// let tmpl = MyTemplate::new("Not Me", &age);
/// # }
/// ```
///
/// These templates never own their content, they just borrow it. This is one of the reasons I call
/// this feature "experimental".
#[macro_export]
macro_rules! template {
    ($name:ident ($($field:ident : &$typ:ty),*) { $($tmpl:tt)* } $($rest:tt)*) => {
        struct $name<'a> { $( $field: &'a $typ),* }
        impl<'a> $name<'a> {
            pub fn new($($field: &'a $typ),*) -> Self {
                $name { $( $field: $field),* }
            }
        }
        impl<'a> $crate::RenderOnce for $name<'a> {
            fn render_once(self, tmpl: &mut $crate::TemplateBuffer) {
                $crate::Render::render(&self, tmpl);
            }
        }
        impl<'a> $crate::RenderMut for $name<'a> {
            fn render_mut(&mut self, tmpl: &mut $crate::TemplateBuffer) {
                $crate::Render::render(self, tmpl);
            }
        }
        impl<'a> $crate::Render for $name<'a> {
            fn render(&self, tmpl: &mut $crate::TemplateBuffer) {
                let &$name { $($field),* } = self;
                tmpl << html! { $($tmpl)* };
            }
        }
        template!($($rest)*);
    };
    (pub $name:ident ($($field:ident : &$typ:ty),*) { $($tmpl:tt)* } $($rest:tt)*) => {
        struct $name<'a> { $( $field: &'a $typ),* }
        impl<'a> $name<'a> {
            pub fn new($($field: &'a $typ),*) -> Self {
                $name { $( $field: $field),* }
            }
        }
        impl<'a> $crate::RenderOnce for $name<'a> {
            fn render_once(self, tmpl: &mut $crate::TemplateBuffer) {
                $crate::Render::render(&self, tmpl);
            }
        }
        impl<'a> $crate::RenderMut for $name<'a> {
            fn render_mut(&mut self, tmpl: &mut $crate::TemplateBuffer) {
                $crate::Render::render(self, tmpl);
            }
        }
        impl<'a> $crate::Render for $name<'a> {
            fn render(&self, tmpl: &mut $crate::TemplateBuffer) {
                let &$name { $($field),* } = self;
                tmpl << html! { $($tmpl)* };
            }
        }
        template!($($rest)*);
    };
    () => {}
}

/// Utility macro for generating a space-delimited string from a set of labels;
/// some of which may be conditionally included into the final string.
/// Labels are anything that implements the `RenderOnce` trait (e.g. `String` or `&str`).
///
/// This macro is an alias of: `labels_sep_by!(" "; maybe_label,...)`
///
/// Usage: `labels!(maybe_label,...)`
///
/// * `maybe_label` -- Either `label_expression`, or `label_expression => cond_test`.
///
/// * `label_expression` -- An expression that returns a label that implements
///   the `RenderOnce` trait (e.g. `String` or `&str`).
///
/// * `label_expression => cond_test` -- Conditionally include `label_expression` whenever `cond_test` is `true`.
///   `cond_test` is an expression that returns either `true` or `false`.
///
/// This useful in generating class attribute as follows:
///
/// ```
/// # #[macro_use]
/// # extern crate horrorshow;
/// # fn main() {
/// html! {
///     div(class = labels!("active" => true, "button-style")) {
///         : "This is a button"
///     }
/// }
/// # ;
/// # }
/// ```
///
#[macro_export]
macro_rules! labels {

    ($($tail:tt)+) => (
        labels_sep_by!(" "; $($tail)*)
    );

}

/// Utility macro for generating a delimited string from a set of labels;
/// some of which may be conditionally included into the final string.
/// The delimiter/seperator and labels are anything that implements
/// the `RenderOnce` trait (e.g. `String` or `&str`).
///
///
/// Usage: `labels_sep_by!(seperator; maybe_label,...)`
///
/// * `seperator` -- Delimiter/seperator that implements the `RenderOnce` trait (e.g. `String` or `&str`).
///
/// * `maybe_label` -- Either `label_expression`, or `label_expression => cond_test`.
///
/// * `label_expression` -- An expression that returns a label that implements
///   the `RenderOnce` trait (e.g. `String` or `&str`).
///
/// * `label_expression => cond_test` -- Conditionally include `label_expression` whenever `cond_test` is `true`.
///   `cond_test` is an expression that returns either `true` or `false`.
///
/// This useful in generating style attribute as follows:
///
/// ```
/// # #[macro_use]
/// # extern crate horrorshow;
/// # fn main() {
/// html! {
///     div(style = labels_sep_by!(";"; "color: #000" => true, "font-weight: bold")) {
///         : "This is a button"
///     }
/// }
/// # ;
/// # }
/// ```
///
#[macro_export]
macro_rules! labels_sep_by {

    (@inner_expand $has_before:expr; $sep:expr; $tmpl:ident $item:expr) => {
        if $has_before {
            $crate::RenderOnce::render_once($sep, $tmpl);
        }
        $crate::RenderOnce::render_once($item, $tmpl);
    };

    (@inner_expand $has_before:expr; $sep:expr; $tmpl:ident $item:expr => $should_include:expr) => {
        if $should_include {
            if $has_before {
                $crate::RenderOnce::render_once($sep, $tmpl);
            }
            $crate::RenderOnce::render_once($item, $tmpl);
        }
    };

    (@inner_expand $has_before:expr; $sep:expr; $tmpl:ident $item:expr, $($tail:tt)+) => {
        if $has_before {
            $crate::RenderOnce::render_once($sep, $tmpl);
        }
        $crate::RenderOnce::render_once($item, $tmpl);
        labels_sep_by!(@inner_expand true; $sep; $tmpl $($tail)*);
    };

    (@inner_expand $has_before:expr; $sep:expr; $tmpl:ident $item:expr => $should_include:expr, $($tail:tt)+) => {
        if $should_include {
            if $has_before {
                $crate::RenderOnce::render_once($sep, $tmpl);
            }
            $crate::RenderOnce::render_once($item, $tmpl);
        }
        labels_sep_by!(@inner_expand $has_before || $should_include; $sep; $tmpl $($tail)*);
    };

    // entries

    ($sep:expr; $item:expr) => {

        $crate::FnRenderer::new(|tmpl| {
            $crate::RenderOnce::render_once($item, tmpl);
        })

    };

    ($sep:expr; $item:expr => $should_include:expr) => {

        $crate::FnRenderer::new(|tmpl| {
            if $should_include {
                $crate::RenderOnce::render_once($item, tmpl);
            }
        })

    };

    ($sep:expr; $item:expr, $($tail:tt)+) => {

        $crate::FnRenderer::new(|tmpl| {
            $crate::RenderOnce::render_once($item, tmpl);
            labels_sep_by!(@inner_expand true; $sep; tmpl $($tail)*);
        })

    };

    ($sep:expr; $item:expr => $should_include:expr, $($tail:tt)+) => {

        $crate::FnRenderer::new(|tmpl| {
            if $should_include {
                $crate::RenderOnce::render_once($item, tmpl);
            }
            labels_sep_by!(@inner_expand $should_include; $sep; tmpl $($tail)*);
        })

    };

}
