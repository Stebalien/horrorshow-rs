//! # Horrorshow
//!
//! An html templating library.
//!
//! ## Example:
//!
//! ```
//! # #[macro_use] extern crate horrorshow;
//! # fn main() {
//! use horrorshow::prelude::*;
//! let actual = html! {
//!     html {
//!         head {
//!             title { : "Hello world!" }
//!         }
//!         body {
//!             // attributes
//!             h1(id="heading") {
//!                 // Insert escaped text
//!                 : "Hello! This is <html />"
//!             }
//!             p {
//!                 // Insert raw text (unescaped)
//!                 : raw!("Let's <i>count</i> to 10!")
//!             }
//!             ol(id="count") {
//!                 // run some inline code...
//!                 |mut tmpl| for i in 0..10 {
//!                     // append to the current template.
//!                     // store output because rust bug #25753
//!                     tmpl = tmpl << html! {
//!                         li(first? = (i == 0)) {
//!                             // format some text
//!                             #{"{}", i+1 }
//!                         }
//!                     };
//!                 }
//!             }
//!             // You need semi-colons for tags without children.
//!             br; br;
//!             p {
//!                 : "Easy!"
//!             }
//!         }
//!     }
//! }.into_string().unwrap();
//!
//! let expected = "\
//! <html>\
//!   <head>\
//!     <title>Hello world!</title>\
//!   </head>\
//!   <body>\
//!     <h1 id=\"heading\">Hello! This is &lt;html /&gt;</h1>\
//!     <p>Let's <i>count</i> to 10!</p>\
//!     <ol id=\"count\">\
//!       <li first>1</li>\
//!       <li>2</li>\
//!       <li>3</li>\
//!       <li>4</li>\
//!       <li>5</li>\
//!       <li>6</li>\
//!       <li>7</li>\
//!       <li>8</li>\
//!       <li>9</li>\
//!       <li>10</li>\
//!     </ol>\
//!     <br /><br />\
//!     <p>Easy!</p>\
//!   </body>\
//! </html>";
//! assert_eq!(expected, actual);
//!
//! # }
//! ```
//!
//! ## Usage
//!
//! Inside an html template, the following expressions are valid:
//!
//! * `some_tag;` -- Insert a the tag `some_tag`.
//!
//! * `some_tag(attr=rust_expresion,...);` -- Insert a the tag `some_tag` with the specified
//!    attributes. The attribute values will be evaluated as rust expressions at runtime and they
//!    must implement RenderOnce (already implemented on &str, String, other templates, etc.).
//!
//! * `some_tag(attr,...);` -- You can also omit the value.
//!
//! * `some_tag(attr=#{"{}", 1},...);` -- You can also use format strings.
//!
//! * `some_tag(attr? = Some("test"),...);` -- You can optionally include an attribute.
//!
//! * `some_tag(attr? = some_boolean,...);` -- You can optionally include an attribute without a value.
//!
//! * `some_tag { ... }` -- Insert the tag `some_tag` and recursively evaluate the `...`.
//!
//! * `some_tag(...) { ... }` -- Same as above but with custom attributes.
//!
//! * `: rust_expression`, `: { rust_code }` -- Evaluate the expression or block and insert result
//! current position. To insert literal html, mark it as raw with the `raw!` macro.
//!
//! * `#{"format_str", rust_expressions... }` -- Format the arguments according to `format_str` and
//! insert the result at the current position.
//!
//! * `|tmpl| rust_expression`, `|tmpl| { rust_code }` -- Evaluate the expression or block. This is
//! actually a closure so the block/expression can append to the current template through `tmpl`
//! (of type `&mut TemplateBuffer`).
//!
//! ## Traits, traits oh-my!
//!
//! You will likely notice that there are four render traits:
//!
//! 1. `RenderOnce`
//! 2. `RenderMut`
//! 3. `Render`
//! 4. `RenderBox`
//!
//! These three traits map to the four `Fn` traits and reflect the fact that some templates need
//! exclusive access (`RenderMut`) in order to be rendered and others might even consume their
//! environment (`RenderOnce`).
//!
//! In general, just import `Template` into your environment (or import the prelude).
//!
//! ## Error Handling
//!
//! Both render and IO errors are handled in the background. If an io (or fmt) error occurs,
//! template rendering will continue but no more data will be written and the original `write_to_*`
//! call will return the error when rendering terminates. If you need to record a render error, use
//! `TemplateBuffer::record_error`. As with IO errors, custom errors DO NOT cause rendering to be
//! aborted. Instead, all recorded errors (if any) are returned when rendering completes.
//!
//! TL;DR: There is no way to abort rendering but you can report errors.
//!
//! ## Escaping
//!
//! This library does HTML escaping by default. However, it doesn't do any javascript/URL escaping.
//! Furthermore, it will do html escaping in any literal javascript you might include.
//!
//! For example, the following will display an alert:
//!
//! ```norun
//! html! {
//!   script {
//!     : "alert('hello');"
//!   }
//! }
//! ```
//!
//! The following will break due to html escaping (the `"` will be escaped to `&quot;`):
//!
//! ```norun
//! html! {
//!   script {
//!     : 'alert("hello");'
//!   }
//! }
//! ```
//!
//! And the following will display as-is (but won't run any javascript) due to the HTML escaping:
//!
//! ```norun
//! html! {
//!     : '<script>alert("hello");</script>'
//! }
//! ```
//!
//! Output:
//!
//! ```html
//! &lt;script&gt;alert(&quot;hello&quot;);&lt;/script&gt;
//! ```
//!
//! ## Returning Templates
//!
//! To return a template directly, you have to create it using the `box_html!` macro instead of the
//! `html!` macro. The template type will be one of `Box<RenderBox>` (can be rendered once),
//! `Box<RenderMut>`, or `Box<Render>` depending on how the template affects its environment.
//!
//! ```
//! #[macro_use]
//! extern crate horrorshow;
//!
//! use horrorshow::{RenderOnce, RenderBox, RenderMut, Render};
//!
//! // Consume the environment
//! fn identity<T: RenderOnce + 'static>(something: T) -> Box<RenderBox + 'static> {
//!     box_html! {
//!         : something
//!     }
//! }
//!
//! // Mutate the environment
//! fn counter() -> Box<RenderMut> {
//!     let mut counter = 0;
//!     box_html! {
//!         |t| {
//!             write!(t, "{}", counter);
//!             counter += 1;
//!         }
//!     }
//! }
//!
//! // Borrow the environment.
//! fn say_hello(name: String) -> Box<Render> {
//!     let mut counter = 0;
//!     box_html! {
//!         span {
//!             : "Hello ";
//!             : &name;
//!             : ".";
//!         }
//!     }
//! }
//!
//! # fn main() {}
//! ```
//!
//! *Note*: To avoid allocating, you can implement render manually instead of returning a boxed
//! template:
//!
//! ```
//! #[macro_use]
//! extern crate horrorshow;
//!
//! use horrorshow::{RenderOnce, TemplateBuffer, Template};
//!
//! struct Page<C> {
//!     title: String,
//!     content: C,
//! }
//!
//! impl Page<String> {
//!     fn from_string_content(title: String, content: String) -> Self {
//!         Page { title: title, content: content }
//!     }
//! }
//!
//! impl<C> RenderOnce for Page<C> where C: RenderOnce {
//!     fn render_once(self, tmpl: &mut TemplateBuffer) {
//!         let Page {title, content} = self;
//!         // The actual template:
//!         tmpl << html! {
//!             article {
//!                 header {
//!                     h1 : title
//!                 }
//!                 section : content
//!             }
//!         };
//!     }
//! }
//!
//! fn main() {
//!   let page = Page::from_string_content(String::from("My title"),
//!                                        String::from("Some content."));
//!   assert_eq!(page.into_string().unwrap(),
//!              "<article>\
//!                 <header><h1>My title</h1></header>\
//!                 <section>Some content.</section>\
//!               </article>");
//! }
//! ```
//!
//! # Examples
//!
//! See the test cases.
#[macro_use]
mod macros;

#[cfg(feature = "ops")]
mod ops;

mod error;
pub use error::Error;

mod template;
pub use template::{TemplateBuffer, Template};
mod render;
pub use render::{RenderOnce, RenderMut, Render, RenderBox, Renderer, Raw,
                 __new_renderer, __new_boxed_renderer};

/// Traits that should always be imported.
pub mod prelude;


/// Helper trait for dispatching `attr ?= `.
///
/// attr ?= Some("test") -> attr="test"
/// attr ?= true -> attr
#[doc(hidden)]
pub trait BoolOption: Sized {
    type Value;
    fn bool_option(self) -> (bool, Option<Self::Value>);
}

impl<T> BoolOption for Option<T> {
    type Value = T;
    #[inline(always)]
    fn bool_option(self) -> (bool, Option<T>) {
        (false, self)
    }
}

// Need &str because Value needs to implement RenderOnce (even though we never actually render
// it...)
impl BoolOption for bool {
    type Value = &'static str;
    #[inline(always)]
    fn bool_option(self) -> (bool, Option<&'static str>) {
        (true, if self { Some("") } else { None })
    }
}


