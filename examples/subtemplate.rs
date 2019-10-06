use std::error::Error;
use std::io;

use horrorshow::owned_html;
use horrorshow::prelude::*;

// page_title and content can be anything that can be rendered. A string, a
// template, a number, etc.
fn layout(page_title: impl Render, content: impl Render) -> impl Render {
    // owned_html _moves_ the arguments into the template. Useful for returning
    // owned (movable) templates.
    owned_html! {
        head {
            title : &page_title;
        }
        body {
            :&content
        }
    }
}

fn home_content() -> impl Render {
    owned_html! {
        h1 { :"Home Page" }
    }
}

fn about_content() -> impl Render {
    owned_html! {
        h1 { :"About Us" }
    }
}

fn contact_content() -> impl Render {
    owned_html! {
        h1 { :"Contact Us" }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    layout("Home", home_content()).write_to_io(&mut io::stdout())?;
    layout("About", about_content()).write_to_io(&mut io::stdout())?;
    layout("Contact", contact_content()).write_to_io(&mut io::stdout())?;
    Ok(())
}
