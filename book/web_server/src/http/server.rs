use super::request::HttpRequest;
use super::response::HttpResponse;
use crate::thread_pool::ThreadPool;
use std::io::{Read, Result, Write};
use std::net::{SocketAddr, TcpListener, ToSocketAddrs};
use std::sync::Arc;

pub struct HttpServer {
    tp: ThreadPool,
    listener: Option<TcpListener>,
}

/// HTTP server implementation
///
/// # Example
///
/// ```no_run
/// use std::net::{SocketAddrV4, Ipv4Addr};
/// use web_server::http::HttpServer;
///
/// let mut server = HttpServer::new(2);
///
/// server
///     .bind(SocketAddrV4::new(Ipv4Addr::UNSPECIFIED, 3000))
///     .expect("Cannot start server");
/// println!("Server started on {}", server.get_addr().expect("Cannot get local addr"));
/// server
///     .listen(move |request, response| {
///         println!("request: {}", request.path);
///
///         response.set_status(200);
///         response.set_body("Test Page".to_string());
///     })
///     .expect("Cannot listen for requests");
/// ```
impl HttpServer {
    /// Creates new server
    ///
    /// - thread_number - number of threads of the [ThreadPool](../thread_pool/struct.ThreadPool.html)
    pub fn new(thread_number: usize) -> Self {
        let tp = ThreadPool::new(thread_number);
        let listener = None;

        HttpServer { tp, listener }
    }

    /// Stars listen on the given addr
    ///
    /// - addr - address to listen
    pub fn bind<A: ToSocketAddrs>(&mut self, addr: A) -> Result<()> {
        let listener = TcpListener::bind(addr)?;
        let _ = self.listener.insert(listener);

        Ok(())
    }

    /// Get current listening address
    ///
    /// # Panics
    ///
    /// - in case .bind was not called before
    pub fn get_addr(&self) -> Result<SocketAddr> {
        self.listener
            .as_ref()
            .expect("Have you called HttpServer::bind first?")
            .local_addr()
    }

    // Start accepting and handling actual http requests
    pub fn listen<F>(&mut self, handler: F) -> Result<()>
    where
        F: for<'a> Fn(&'a HttpRequest, &'a mut HttpResponse),
        F: 'static + Send + Sync,
    {
        let listener = self
            .listener
            .as_ref()
            .expect("Have you called HttpServer::bind first?");

        let handler = Arc::new(handler);

        for stream in listener.incoming() {
            let handler = Arc::clone(&handler);

            self.tp.execute(move || {
                let mut buffer = [0u8; 1024];
                let mut stream = stream.as_ref().expect("Cannot get stream");

                stream.read(&mut buffer).expect("Cannot read from stream");

                let request = HttpRequest::parse(&String::from_utf8_lossy(&buffer));
                let mut response = HttpResponse::new();

                handler(&request, &mut response);

                stream
                    .write(response.to_string().as_bytes())
                    .expect("Cannot send response");

                stream.flush().expect("Cannot send response")
            })
        }

        Ok(())
    }
}
