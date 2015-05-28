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
//!                         li {
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
//! }.into_string();
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
//!       <li>1</li>\
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
//!    attributes. The attribute values will be evaluated as rust expressions at runtime.
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
//! (of type `&mut TemplateBuilder`).
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
//! IO errors (writing to the buffer) are handled in the background. If an io (or fmt) error
//! occurs, template rendering will continue but no more data will be written and the original
//! `write_to_fmt`/`write_to_io` call will return the error when rendering terminates.
//!
//! There is no way to abort template rendering other than panicing. Try to do everything that can
//! fail before rendering a template.
#[macro_use]
mod macros;

#[cfg(feature = "ops")]
mod ops;

mod template;
pub use template::{TemplateBuilder, Template};
mod render;
pub use render::{RenderOnce, RenderMut, Render, RenderBox, Renderer, Raw,
                 __new_renderer, __new_boxed_renderer};

/// Traits that should always be imported.
pub mod prelude;
