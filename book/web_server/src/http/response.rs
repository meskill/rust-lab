use std::{collections::HashMap, fmt::Display};

pub struct HttpResponse {
    body: Option<String>,
    status: u16,
    headers: HashMap<String, String>,
}

impl HttpResponse {
    pub fn new() -> Self {
        let headers = HashMap::new();

        HttpResponse {
            body: None,
            status: 200,
            headers,
        }
    }

    pub fn set_status(&mut self, status: u16) {
        assert!(
            status >= 100 && status < 600,
            "Status should be between 100 and 599"
        );

        self.status = status;
    }

    pub fn set_body(&mut self, body: String) {
        let _ = self.body.insert(body);
    }

    pub fn set_header(&mut self, name: String, value: String) {
        self.headers.insert(name, value);
    }

    fn status_to_name(status: u16) -> String {
        match status {
            100..=199 => "STATUS",
            200..=299 => "OK",
            300..=399 => "REDIRECT",
            400..=499 => "NOT FOUND",
            500..=599 => "SERVER ERROR",
            _ => "UNKNOWN",
        }
        .to_string()
    }
}

impl Display for HttpResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "HTTP/1.1 {} {}\r\n",
            self.status,
            HttpResponse::status_to_name(self.status)
        )?;

        for (k, v) in &self.headers {
            write!(f, "{}: {}\r\n", k, v)?;
        }

        if let Some(body) = &self.body {
            write!(f, "Content-Length: {}\r\n\r\n{}", body.len(), body)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_convert_to_string() {
        let mut response = HttpResponse::new();

        response.set_status(202);
        response.set_body("Response Test".to_string());
        response.set_header("X-Header".to_string(), "test".to_string());

        assert_eq!(
            response.to_string(),
            "HTTP/1.1 202 OK\r\nX-Header: test\r\nContent-Length: 13\r\n\r\nResponse Test"
        );
    }
}
