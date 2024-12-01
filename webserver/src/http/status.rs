use std::fmt::{Display, Formatter};

/// Serializable representation of an HTTP Status
///
/// # Examples
///
/// ```
/// use webserver::http::HttpStatus;
/// assert_eq!(HttpStatus::Ok200.to_string(), "200 OK".to_string());
/// assert_eq!(HttpStatus::BadRequest400.to_string(), "400 Bad Request".to_string());
/// ```
#[derive(Debug, PartialEq, Eq)]
pub enum HttpStatus {
    Ok200,
    BadRequest400,
}

impl HttpStatus {
    pub fn status_code(&self) -> u16 {
        match self {
            HttpStatus::Ok200 => 200,
            HttpStatus::BadRequest400 => 400,
        }
    }

    pub fn status_phrase(&self) -> String {
        match self {
            HttpStatus::Ok200 => "OK".to_string(),
            HttpStatus::BadRequest400 => "Bad Request".to_string(),
        }
    }
}

impl Display for HttpStatus {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.status_code(), self.status_phrase())
    }
}
