use std::fmt;
use std::error::Error;

#[derive(Debug)]
#[allow(dead_code)]
pub enum ClinerError {
    Io(std::io::Error),
    Serde(serde_json::Error),
    Parse(String),
    MissingField(String),
    InvalidFormat(String),
}

impl Error for ClinerError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            ClinerError::Io(err) => Some(err),
            ClinerError::Serde(err) => Some(err),
            _ => None,
        }
    }
}

impl fmt::Display for ClinerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ClinerError::Io(err) => write!(f, "I/O Error: {}", err),
            ClinerError::Serde(err) => write!(f, "JSON Serialization Error: {}", err),
            ClinerError::Parse(msg) => write!(f, "Parse Error: {}", msg),
            ClinerError::MissingField(field) => write!(f, "Missing Required Field: {}", field),
            ClinerError::InvalidFormat(msg) => write!(f, "Invalid Format: {}", msg),
        }
    }
}

impl From<std::io::Error> for ClinerError {
    fn from(err: std::io::Error) -> Self {
        ClinerError::Io(err)
    }
}

impl From<serde_json::Error> for ClinerError {
    fn from(err: serde_json::Error) -> Self {
        ClinerError::Serde(err)
    }
}

pub type Result<T> = std::result::Result<T, ClinerError>;

#[cfg(test)]
mod tests {
    use super::*;
    use std::io;
    
    #[test]
    fn test_from_io_error() {
        let io_error = io::Error::new(io::ErrorKind::NotFound, "file not found");
        let cliner_error = ClinerError::from(io_error);
        
        match cliner_error {
            ClinerError::Io(_) => assert!(true),
            _ => panic!("Expected ClinerError::Io variant"),
        }
    }
    
    #[test]
    fn test_from_serde_error() {
        let invalid_json = "{\"key\": invalid}";
        let parse_result = serde_json::from_str::<serde_json::Value>(invalid_json);
        assert!(parse_result.is_err());
        
        let serde_error = parse_result.unwrap_err();
        let cliner_error = ClinerError::from(serde_error);
        
        match cliner_error {
            ClinerError::Serde(_) => assert!(true),
            _ => panic!("Expected ClinerError::Serde variant"),
        }
    }
    
    #[test]
    fn test_display() {
        let parse_error = ClinerError::Parse("parse error".to_string());
        let missing_field = ClinerError::MissingField("name".to_string());
        let invalid_format = ClinerError::InvalidFormat("invalid".to_string());
        
        assert!(format!("{}", parse_error).contains("Parse Error: parse error"));
        assert!(format!("{}", missing_field).contains("Missing Required Field: name"));
        assert!(format!("{}", invalid_format).contains("Invalid Format: invalid"));
    }
}
