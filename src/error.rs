use std::io;
use std::fmt;

#[derive(Debug)]
pub struct Error {
    pub write: Option<io::Error>,
    pub render: Vec<Box<::std::error::Error + Send + Sync>>
}

impl Error {
    pub fn new() -> Error {
        Error {
            write: None,
            render: Vec::new(),
        }
    }
}

#[inline(always)]
pub fn is_empty(e: &Error) -> bool {
    e.write.is_none() && e.render.is_empty()
}

impl ::std::error::Error for Error {
    fn description(&self) -> &'static str {
        "Template rendering failed"
    }
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Error {
        Error {
            write: Some(e),
            render: Vec::new(),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // TODO: Display both write/render errors at the same time.
        if let Some(e) = self.write.as_ref() {
            write!(f, "Write Error: {}", e)
        } else if !self.render.is_empty() {
            try!(write!(f, "Render Errors: "));
            for i in 0..(self.render.len()-1) {
                try!(write!(f, "{}, ", self.render[i]));
            }
            write!(f, "{}", self.render.last().unwrap())
        } else {
            // Panic?
            write!(f, "Empty Error")
        }
    }
}
