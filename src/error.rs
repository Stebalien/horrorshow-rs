use core::fmt;

#[cfg(feature = "alloc")]
use alloc::vec::Vec;

#[cfg(feature = "std")]
use std::io;

/// Error type returned when formatting templates.
#[derive(Debug, Default)]
#[cfg(feature = "std")]
pub struct Error {
    /// The error returned by the underlying writer when formatting a template.
    ///
    /// FEATURE: When built without "std", this is a `core::fmt::Error`.
    pub write: Option<io::Error>,
    /// The error(s) emitted by the template itself.
    ///
    /// FEATURE:
    ///
    /// * When built without "std" but with "alloc", this is a `Vec<String>`.
    /// * When built without "std" and without "alloc", this is an
    ///   `Option<&'static str>`. If multiple errors are emitted, only the first
    ///   is stored.
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
pub(crate) fn is_empty(e: &Error) -> bool {
    e.write.is_none() && e.render.is_empty()
}

#[inline]
#[cfg(not(feature = "alloc"))]
pub(crate) fn is_empty(e: &Error) -> bool {
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
            let mut errs = self.render.iter().fuse();
            if let Some(err) = errs.next() {
                if displayed {
                    write!(f, "; ")?;
                } else {
                    displayed = true;
                }
                write!(f, "render errors: {}", err)?;
                for err in errs {
                    write!(f, ", {}", err)?;
                }
                write!(f, "; ")?;
            }
        }
        #[cfg(not(feature = "alloc"))]
        {
            if let Some(e) = self.render.as_ref() {
                if displayed {
                    write!(f, "; ")?;
                } else {
                    displayed = true;
                }
                write!(f, "render error: {}", e)?;
            }
        }
        if !displayed {
            write!(f, "unspecified error")?;
        }
        Ok(())
    }
}
