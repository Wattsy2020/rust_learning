use crate::http::{HttpStatus, HttpVersion};
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub struct HttpResponse {
    pub status: HttpStatus,
    pub content: String,
    pub version: HttpVersion,
}

impl Display for HttpResponse {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {}\nContent-Length: {}\n\n{}",
            self.version,
            self.status,
            self.content.len(),
            self.content
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::http::{HttpResponse, HttpStatus, HttpVersion};

    #[test]
    fn test_serialisation() {
        let response1 = HttpResponse {
            status: HttpStatus::Ok200,
            content: "Content".to_string(),
            version: HttpVersion::Http2,
        };
        assert_eq!(
            response1.to_string(),
            "HTTP/2 200 OK\nContent-Length: 7\n\nContent"
        );
    }
}
