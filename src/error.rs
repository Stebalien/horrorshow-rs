use core::fmt;

#[cfg(feature = "alloc")]
use alloc::vec::Vec;

#[cfg(feature = "std")]
use std::io;

#[derive(Debug, Default)]
#[cfg(feature = "std")]
pub struct Error {
    pub write: Option<io::Error>,
    pub render: Vec<Box<dyn std::error::Error + Send + Sync>>,
}

#[cfg(not(feature = "std"))]
#[derive(Debug, Default)]
pub struct Error {
    pub write: Option<fmt::Error>,
    #[cfg(feature = "alloc")]
    pub render: Vec<alloc::string::String>,
    #[cfg(not(feature = "alloc"))]
    pub render: Option<&'static str>,
}

#[inline]
#[cfg(feature = "alloc")]
pub fn is_empty(e: &Error) -> bool {
    e.write.is_none() && e.render.is_empty()
}

#[inline]
#[cfg(not(feature = "alloc"))]
pub fn is_empty(e: &Error) -> bool {
    e.write.is_none() && e.render.is_none()
}

#[cfg(feature = "std")]
impl ::std::error::Error for Error {
    fn description(&self) -> &'static str {
        "Template rendering failed"
    }
}

#[cfg(feature = "std")]
impl From<io::Error> for Error {
    fn from(e: io::Error) -> Error {
        Error {
            write: Some(e),
            render: Vec::new(),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut displayed = false;
        if let Some(e) = self.write.as_ref() {
            displayed = true;
            write!(f, "write error: {}", e)?
        }
        #[cfg(feature = "alloc")]
        {
            if !self.render.is_empty() {
                displayed = true;
                write!(f, "render errors: ")?;
                for i in 0..(self.render.len() - 1) {
                    write!(f, "{}, ", self.render[i])?;
                }
                write!(f, "{}", self.render.last().unwrap())?
            }
        }
        #[cfg(not(feature = "alloc"))]
        {
            if let Some(e) = self.render.as_ref() {
                displayed = true;
                write!(f, "render error: {}", e)?;
            }
        }
        if !displayed {
            write!(f, "unspecified error")?;
        }
        Ok(())
    }
}
