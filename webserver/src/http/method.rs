use std::str::FromStr;

/// Represents the HTTP Methods
///
/// # Examples
///
/// Can parse this enum from strings
/// ```
/// use webserver::http::HttpMethod;
/// assert_eq!("get".parse(), Ok(HttpMethod::Get));
/// assert_eq!("post".parse(), Ok(HttpMethod::Post));
/// ```
#[derive(Debug, PartialEq, Eq)]
pub enum HttpMethod {
    Get,
    Head,
    Post,
    Put,
    Patch,
    Delete,
    Connect,
    Options,
    Trace,
}

impl FromStr for HttpMethod {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "get" => Ok(HttpMethod::Get),
            "head" => Ok(HttpMethod::Head),
            "post" => Ok(HttpMethod::Post),
            "put" => Ok(HttpMethod::Put),
            "patch" => Ok(HttpMethod::Patch),
            "delete" => Ok(HttpMethod::Delete),
            "connect" => Ok(HttpMethod::Connect),
            "options" => Ok(HttpMethod::Options),
            "trace" => Ok(HttpMethod::Trace),
            _ => Err(()),
        }
    }
}
