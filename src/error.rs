use std::io;
use std::fmt;

#[derive(Debug, Default)]
pub struct Error {
    pub write: Option<io::Error>,
    pub render: Vec<Box<::std::error::Error + Send + Sync>>,
}

#[inline]
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
        let mut displayed = false;
        if let Some(e) = self.write.as_ref() {
            displayed = true;
            write!(f, "write error: {}", e)?
        }
        if !self.render.is_empty() {
            displayed = true;
            write!(f, "render errors: ")?;
            for i in 0..(self.render.len() - 1) {
                write!(f, "{}, ", self.render[i])?;
            }
            write!(f, "{}", self.render.last().unwrap())?
        }
        if !displayed {
            write!(f, "unspecified error")?;
        }
        Ok(())
    }
}
