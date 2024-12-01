use crate::http::request::{HeaderParseError::*, RequestParseError::*};
use crate::http::version::HttpVersion;
use crate::http::HttpMethod;

#[derive(Debug, PartialEq)]
pub struct HttpRequest {
    method: HttpMethod,
    path: String,
    version: HttpVersion,
}

#[derive(Debug, PartialEq)]
pub enum HeaderParseError {
    InvalidHttpMethod,
    InvalidHttpVersion,
    MissingInformation(String),
}

#[derive(Debug)]
pub enum RequestParseError {
    MissingHeader,
}

impl HttpRequest {
    fn from(line: &str) -> Result<HttpRequest, HeaderParseError> {
        let parts: Vec<&str> = line.split(' ').take(4).collect();
        match parts.as_slice() {
            [method, path, version] => {
                let method = method
                    .parse::<HttpMethod>()
                    .map_err(|_| InvalidHttpMethod)?;
                let version = version
                    .parse::<HttpVersion>()
                    .map_err(|_| InvalidHttpVersion)?;
                Ok(HttpRequest {
                    method,
                    path: path.to_string(),
                    version,
                })
            }
            _ => Err(MissingInformation(format!(
                "Wrong number of parts, expected 3 found: {}",
                parts.len()
            ))),
        }
    }

    pub fn from_lines(
        http_request_lines: impl Iterator<Item = String>,
    ) -> Result<HttpRequest, RequestParseError> {
        http_request_lines
            // stop parsing after the request ends with an empty line
            .take_while(|line| !line.is_empty())
            .find_map(|line| Self::from(&line).ok())
            .ok_or(MissingHeader)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from() {
        assert_eq!(
            HttpRequest::from("GET /path HTTP/2"),
            Ok(HttpRequest {
                method: HttpMethod::Get,
                path: "/path".to_string(),
                version: HttpVersion::Http2
            })
        );
        assert_eq!(
            HttpRequest::from("POST /code HTTP/1.1"),
            Ok(HttpRequest {
                method: HttpMethod::Post,
                path: "/code".to_string(),
                version: HttpVersion::Http1_1
            })
        );
    }

    // test errors
    #[test]
    fn test_from_errors() {
        assert_eq!(
            HttpRequest::from("HELLO /path HTTP/2"),
            Err(InvalidHttpMethod)
        );
        assert_eq!(
            HttpRequest::from("GET /path HTTP/4"),
            Err(InvalidHttpVersion)
        );
        assert!(matches!(
            HttpRequest::from("GET /path"),
            Err(MissingInformation(_))
        ));
    }
}
