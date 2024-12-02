use crate::http::request::{RequestParseError::*, StartLineParseError::*};
use crate::http::version::HttpVersion;
use crate::http::HttpMethod;

#[derive(Debug, PartialEq)]
pub struct HttpRequest {
    method: HttpMethod,
    path: String,
    version: HttpVersion,
}

#[derive(Debug, PartialEq, Eq)]
pub enum StartLineParseError {
    InvalidHttpMethod,
    InvalidHttpVersion,
    MissingInformation(String),
}

#[derive(Debug, PartialEq, Eq)]
pub enum RequestParseError {
    InvalidStartLine(StartLineParseError),
    MissingStartLine,
}

impl HttpRequest {
    fn from(line: &str) -> Result<HttpRequest, StartLineParseError> {
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
        // stop parsing after the request ends with an empty line
        let mut lines = http_request_lines.take_while(|line| !line.is_empty());
        lines
            .next()
            .map(|start_line| HttpRequest::from(&start_line))
            .ok_or(MissingStartLine)?
            .map_err(|err| InvalidStartLine(err))
    }

    pub fn method(&self) -> HttpMethod {
        self.method
    }

    pub fn path(&self) -> &String {
        &self.path
    }

    pub fn version(&self) -> HttpVersion {
        self.version
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

    fn call_from_lines(start_line: &str) -> Result<HttpRequest, RequestParseError> {
        let lines = vec![start_line.to_string()].into_iter();
        HttpRequest::from_lines(lines)
    }

    #[test]
    fn test_from_lines() {
        let result = call_from_lines("POST /code HTTP/1.1").expect("Should parse correctly");
        assert_eq!(
            result,
            HttpRequest {
                method: HttpMethod::Post,
                path: "/code".to_string(),
                version: HttpVersion::Http1_1
            }
        );
    }

    #[test]
    fn test_from_lines_errors() {
        assert_eq!(
            call_from_lines("HELLO /path HTTP/2"),
            Err(InvalidStartLine(InvalidHttpMethod))
        );
        assert_eq!(
            call_from_lines("GET /path HTTP/4"),
            Err(InvalidStartLine(InvalidHttpVersion))
        );
        assert!(matches!(
            call_from_lines("GET /path"),
            Err(InvalidStartLine(MissingInformation(_)))
        ));

        let result = HttpRequest::from_lines(Vec::new().into_iter());
        assert_eq!(result, Err(MissingStartLine));
    }
}
