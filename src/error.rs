use std::fmt;
#[derive(Debug)]
pub struct UnauthorizedHeaderError;

// Implement std::fmt::Display for MissingHeaderError
impl fmt::Display for UnauthorizedHeaderError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Reader's attribute `has_header` is False") 
    }
}
