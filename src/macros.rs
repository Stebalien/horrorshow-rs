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


    // NOTE: You may notice a lot of $e:tt, and then $e:expr. This is because rust seems to parse
    // `ident {` as a struct declaration when using `$e:expr`.

    (@parse_if_chain $tmpl:ident, ($($prefix:tt)*), if let $p:pat = $e:tt { $($inner:tt)* } else $($next:tt)*) => {
        append_html!(@parse_if_chain $tmpl, ($($prefix)*), if let $p = append_html!(@block_identity {$e}) { $($inner)* } else $($next)*);
    };
    (@parse_if_chain $tmpl:ident, ($($prefix:tt)*), if let $p:pat = $e:expr { $($inner:tt)* } else $($next:tt)*) => {
        append_html!(@parse_if_chain $tmpl, ($($prefix)* if let $p = $e {
            append_html!($tmpl, $($inner)*);
        } else ), $($next)*);
    };
    (@parse_if_chain $tmpl:ident, ($($prefix:tt)*), if let $p:pat = $e:tt { $($inner:tt)* } $($next:tt)*) => {
        append_html!(@parse_if_chain $tmpl, ($($prefix)*), if let $p = append_html!(@block_identity {$e}) { $($inner)* } $($next)*);
    };
    (@parse_if_chain $tmpl:ident, ($($prefix:tt)*), if let $p:pat = $e:expr { $($inner:tt)* } $($next:tt)*) => {
        append_html!(@parse_if_chain $tmpl, ($($prefix)* if let $p = $e {
            append_html!($tmpl, $($inner)*);
        }));
        append_html!($tmpl, $($next)*);
    };
    (@parse_if_chain $tmpl:ident, ($($prefix:tt)*), if $e:tt { $($inner:tt)* } else $($next:tt)*) => {
        append_html!(@parse_if_chain $tmpl, ($($prefix)*), if append_html!(@block_identity {$e}) { $($inner)* } else $($next)*);
    };
    (@parse_if_chain $tmpl:ident, ($($prefix:tt)*), if $e:expr { $($inner:tt)* } else $($next:tt)*) => {
        append_html!(@parse_if_chain $tmpl, ($($prefix)* if $e {
            append_html!($tmpl, $($inner)*);
        } else ), $($next)*);
    };
    (@parse_if_chain $tmpl:ident, ($($prefix:tt)*), if $e:tt { $($inner:tt)* } $($next:tt)*) => {
        append_html!(@parse_if_chain $tmpl, ($($prefix)*), if append_html!(@block_identity {$e}) { $($inner)* } $($next)*);
    };
    (@parse_if_chain $tmpl:ident, ($($prefix:tt)*), if $e:expr { $($inner:tt)* } $($next:tt)*) => {
        append_html!(@parse_if_chain $tmpl, ($($prefix)* if $e {
            append_html!($tmpl, $($inner)*);
        }));
        append_html!($tmpl, $($next)*);
    };
    (@parse_if_chain $tmpl:ident, ($($prefix:tt)*), { $($inner:tt)* } $($next:tt)*) => {
        append_html!(@parse_if_chain $tmpl, ($($prefix)* {
            append_html!($tmpl, $($inner)*);
        }));
        append_html!($tmpl, $($next)*);
    };
    (@parse_if_chain $tmpl:ident, ($chain:stmt)) => {
        $chain
    };
    ($tmpl:ident, @ if $($next:tt)+) => {
        append_html!(@parse_if_chain $tmpl, (), if $($next)*);
    };
    /*
    ($tmpl:ident, @ for $p:pat in $e:expr { $($inner:tt)* } $($next:tt)*) => {
        for $p in $e {
            append_html!($tmpl, $($inner)*);
        }
    };
    */
    // In 1.2, replace $p:ident with $p:pat. Currently, this doesn't allow all forloop constructs.
    // See above ^^
    ($tmpl:ident, @ for $p:ident in $e:tt { $($inner:tt)* } $($next:tt)*) => {
        append_html!($tmpl, @ for $p in append_html!(@block_identity {$e}) { $($inner)* } $($next)*);
    };
    ($tmpl:ident, @ for $p:ident in $e:expr { $($inner:tt)* } $($next:tt)*) => {
        for $p in $e {
            append_html!($tmpl, $($inner)*);
        }
        append_html!($tmpl, $($next)*);
    };
    ($tmpl:ident, @ while let $p:pat = $e:tt { $($inner:tt)* } $($next:tt)*) => {
        while let $p = $e {
            append_html!($tmpl, $($inner)*);
        }
        append_html!($tmpl, $($next)*);
    };
    ($tmpl:ident, @ while let $p:pat = $e:expr { $($inner:tt)* } $($next:tt)*) => {
        while let $p = $e {
            append_html!($tmpl, $($inner)*);
        }
        append_html!($tmpl, $($next)*);
    };
    ($tmpl:ident, @ while $e:tt { $($inner:tt)* } $($next:tt)*) => {
        while $e {
            append_html!($tmpl, $($inner)*);
        }
        append_html!($tmpl, $($next)*);
    };
    ($tmpl:ident, @ while $e:expr { $($inner:tt)* } $($next:tt)*) => {
        while $e {
            append_html!($tmpl, $($inner)*);
        }
        append_html!($tmpl, $($next)*);
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
        (|$var: &mut $crate::TemplateBuffer| {
            append_html!(@block_identity {$($code)*})
        })($tmpl);
        append_html!($tmpl, $($next)*);
    };
    ($tmpl:ident, |mut $var:ident| {$($code:tt)*} $($next:tt)*) => {
        (|mut $var: &mut $crate::TemplateBuffer| {
            append_html!(@block_identity {$($code)*})
        })($tmpl);
        append_html!($tmpl, $($next)*);
    };
    ($tmpl:ident, |$var:ident| $code:stmt; $($next:tt)* ) => {
        (|$var: &mut $crate::TemplateBuffer| {
            $code;
        })($tmpl);
        append_html!($tmpl, $($next)*);
    };
    ($tmpl:ident, |mut $var:ident| $code:stmt; $($next:tt)* ) => {
        (|mut $var: &mut $crate::TemplateBuffer| {
            $code;
        })($tmpl);
        append_html!($tmpl, $($next)*);
    };
    ($tmpl:ident, |$var:ident| $code:stmt ) => {
        (|$var: &mut $crate::TemplateBuffer| {
            $code;
        })($tmpl);
    };
    ($tmpl:ident, |mut $var:ident| $code:stmt ) => {
        (|mut $var: &mut $crate::TemplateBuffer| {
            $code;
        })($tmpl);
    };
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
    /* Wait for 1.2
    ($tmpl:ident, $tag:ident($($attrs:tt)+) $tag2:ident $($next:tt)* ) => {
        append_html!($tmpl, $tag($($attrs)*) { $tag2 $($next)* });
    };
    ($tmpl:ident, $tag:ident $tag2:ident $($next:tt)* ) => {
        append_html!($tmpl, $tag { $tag2 $($next)* });
    };
    */
    ($tmpl:ident, $tag:ident) => {
        $tmpl.write_raw(concat!("<", stringify!($tag), "/>"));
    };
    ($tmpl:ident,) => {};
}

/// Create a new template.
/// 
/// This allows you to declare a template as follows:
///
/// ```norun
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
/// ```
///
/// You can instantiate these templates by calling `new` on them:
///
/// ```norun
/// let tmpl = MyTemplate::new("Not Me", &42);
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
