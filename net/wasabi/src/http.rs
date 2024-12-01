#![no_std]

pub mod http;

pub struct HttpClient {}

impl HttpClient {
    pub fn new() -> Self {
        Self {}
    }
}

extern create alloc;
use alloc::string::String;
use saba_core::error::Error;
use saba_core::http::HttpResponse;
use alloc::format;
use create::alloc::string::ToString;
use noli::net::lookup_host;

impl HttpClient {
    pub fn get(&self, host: String, port: u16, path: String) -> Result<HttpResponse, Error>{
        let ips = match lookup_host(&host) {
            Ok(ips) => ips,
            Err(e) => {
                return Err(Error::Network(format!(
                    "Faild to find IP addresses: {:#?}",
                    e
                )))
            }
        };

        if ips.len() < 1 {
            return Err(Error::Network("Failed to find IP addresses".to_string()));
        }

        let socket_addr: SocketAddr = (ips[0], port).into();

        let mut stream = match TcpStream::connect(socket_addr) {
            Ok(stream) => stream,
            Err(_) => {
                return Err(Error::Network(
                    "Failed to connect to TCP stream".to_string(),
                ))
            }
        };

        let mut request = String::from("Get /");
        request.push_str(&path);
        request.push_str(" HTTP/1.1\n");

        // ヘッダの追加
        request.push_str("HOST: ");
        request.push_str(&host);
        request.push('\n');
        request.push_str("Accept: text/html\n");
        request.push_str("Connection: close\n");
        request.push('\n');

        let _bytes_written = match stream.write(requesr.as_bytes()) {
            Ok(bytes) => bytes,
            Err(_) => {
                return Err(Error::Network(
                    "Failed to send a request to TCP stream".to_string(),
                ))
            }
        };
    }
}

