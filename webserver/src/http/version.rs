use std::fmt::{Display, Formatter};
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq)]
pub enum HttpVersion {
    Http1,
    Http1_1,
    Http2,
    Http3,
}

impl Display for HttpVersion {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                HttpVersion::Http1 => "HTTP/1",
                HttpVersion::Http1_1 => "HTTP/1.1",
                HttpVersion::Http2 => "HTTP/2",
                HttpVersion::Http3 => "HTTP/3",
            }
        )
    }
}

impl FromStr for HttpVersion {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "HTTP/1" => Ok(HttpVersion::Http1),
            "HTTP/1.1" => Ok(HttpVersion::Http1_1),
            "HTTP/2" => Ok(HttpVersion::Http2),
            "HTTP/3" => Ok(HttpVersion::Http3),
            _ => Err(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::http::version::HttpVersion;

    #[test]
    fn test_serialisation_round_trip() {
        let versions = vec!["HTTP/1", "HTTP/1.1", "HTTP/2", "HTTP/3"];
        for version in versions {
            assert_eq!(
                version,
                &version.parse::<HttpVersion>().unwrap().to_string()
            );
        }
    }
}
