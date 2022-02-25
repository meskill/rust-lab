use std::collections::HashMap;
use super::method::HttpMethod;

pub struct HttpRequest {
    pub method: HttpMethod,
    pub path: String,
    pub headers: HashMap<String, String>,
}

impl HttpRequest {
    pub fn parse(request: &str) -> Self {
        let mut lines = request.lines();
        let first_line = lines.next();

        let first_line_entries: Vec<_> = first_line
            .expect("Expected non-empty string")
            .split_whitespace()
            .collect();

        let (method, path) = match &first_line_entries[..] {
            &[method, path, _] => (HttpMethod::parse(method), path.to_string()),
            _ => {
                panic!("Unknown line")
            }
        };

        let headers: HashMap<_, _> = lines
            .take_while(|s| !s.is_empty())
            .map(|s| {
                let line: Vec<_> = s.splitn(2, ": ").collect();

                (line[0].to_string(), line[1].to_string())
            })
            .collect();

        HttpRequest {
            method,
            path,
            headers,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::HttpMethod;
    use super::*;

    #[test]
    fn should_parse_get_method() {
        let request =
            HttpRequest::parse("GET / HTTP/1.1\r\nHOST: localhost:3000\r\nConnection: keep-alive");

        assert_eq!(request.method, HttpMethod::GET);
        assert_eq!(request.path, "/");
        assert_eq!(request.headers.len(), 2);
        assert_eq!(request.headers.get("HOST").unwrap(), "localhost:3000");
        assert_eq!(request.headers.get("Connection").unwrap(), "keep-alive");
    }

    #[test]
    fn should_parse_head_method() {
        let request =
            HttpRequest::parse("HEAD /test HTTP/1.1\r\nHOST: localhost:3000\r\nCookie: test:123");

        assert_eq!(request.method, HttpMethod::HEAD);
        assert_eq!(request.path, "/test");
        assert_eq!(request.headers.len(), 2);
        assert_eq!(request.headers.get("HOST").unwrap(), "localhost:3000");
        assert_eq!(request.headers.get("Cookie").unwrap(), "test:123");
    }

    #[test]
    #[should_panic(expected = "Expected non-empty string")]
    fn should_panic_on_empty() {
        HttpRequest::parse("");
    }

    #[test]
    #[should_panic]
    fn should_panic_on_wrong_headers() {
        HttpRequest::parse("HEAD /test HTTP/1.1\r\nHOST: localhost:3000\r\nCookiifjawiofjw");
    }
}
