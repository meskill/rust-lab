/// Representation of the HTTP method (GET, POST etc.)
///
/// # Caveats
///
/// Currently supports only GET and HEAD
///
/// # Panics
///
/// In case of unknown method besides supported
#[derive(Debug, PartialEq)]
pub enum HttpMethod {
    GET,
    HEAD,
}

impl HttpMethod {
    pub fn parse(method: &str) -> Self {
        match method {
            "GET" => HttpMethod::GET,
            "HEAD" => HttpMethod::HEAD,
            _ => panic!("Unknown method"),
        }
    }
}
