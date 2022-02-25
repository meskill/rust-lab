use std::{
    collections::HashMap,
    fs,
    net::{Ipv4Addr, SocketAddrV4},
    path::Path,
    sync::Mutex,
};
use web_server::http::HttpServer;

struct HtmlFileResponse {
    cache: HashMap<String, String>,
}

impl HtmlFileResponse {
    fn new() -> Self {
        let cache = HashMap::new();

        HtmlFileResponse { cache }
    }

    fn get_content(&mut self, pathname: &str) -> Option<&str> {
        let pathname = pathname.strip_prefix("/").unwrap_or(pathname);

        if !self.cache.contains_key(pathname) {
            let html_path = format!("{}/{}{}", "public", pathname, ".html");
            let html_path = Path::new(&html_path).to_path_buf();
            let index_html_path = Path::new("public").join(pathname).join("index.html");

            let path = if html_path.exists() {
                Some(html_path)
            } else if index_html_path.exists() {
                Some(index_html_path)
            } else {
                None
            };

            if let Some(path) = path {
                if let Ok(content) = fs::read_to_string(path) {
                    self.cache.insert(pathname.to_string(), content);
                }
            }
        }

        match self.cache.get(pathname) {
            Some(content) => Some(content),
            None => None,
        }
    }

    fn get_not_found_content(&mut self) -> Option<&str> {
        self.get_content("404")
    }
}

fn main() {
    let mut server = HttpServer::new(2);

    server
        .bind(SocketAddrV4::new(Ipv4Addr::UNSPECIFIED, 3000))
        .expect("Cannot start server");

    println!(
        "Server started on {}",
        server.get_addr().expect("Cannot get local addr")
    );

    let html_file_response = Mutex::new(HtmlFileResponse::new());

    server
        .listen(move |request, response| {
            println!("request: {}", request.path);

            let mut html_file_response = html_file_response
                .lock()
                .expect("Cannot lock HtmlFileResponse instance");

            if let Some(content) = html_file_response.get_content(&request.path) {
                response.set_body(content.to_string());
            } else {
                response.set_status(404);
                if let Some(not_found_content) = html_file_response.get_not_found_content() {
                    response.set_body(not_found_content.to_string())
                }
            }
        })
        .expect("Cannot listen for requests");
}
