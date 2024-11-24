extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;
use core::fmt::Write;

use noli::net::lookup_host;
use noli::net::SocketAddr;
use noli::net::TcpStream;
use noli::println;
use saba_core::error::Error;
use saba_core::http::HttpResponse;

#[derive(Default)]
pub struct HttpClient {}

impl HttpClient {
    pub fn new() -> Self {
        Self {}
    }

    pub fn get(&self, host: String, port: u16, path: String) -> Result<HttpResponse, Error> {
        let ips = match lookup_host(&host) {
            Ok(ips) => ips,
            Err(e) => {
                // `format!`マクロの代わりに、`String`と`write!`マクロを使用します
                let mut s = String::from("Failed to find IP Addresses: ");
                write!(&mut s, "{:#?}", e)
                    .map_err(|_| Error::Other(String::from("Formatting Error")))?;
                return Err(Error::Network(s));
            }
        };

        if ips.is_empty() {
            return Err(Error::Network(String::from("No IP Addresses found for")));
        }

        let socket_addr: SocketAddr = (ips[0], port).into();
        println!("socket_addr: {:?}", socket_addr);

        let mut stream = match TcpStream::connect(socket_addr) {
            Ok(stream) => stream,
            Err(_e) => {
                return Err(Error::Network(String::from(
                    "Failed to connect to TCP stream",
                )))
            }
        };

        let mut request = String::from("GET /");
        request.push_str(&path);
        request.push_str(" HTTP/1.1\n");
        request.push_str("Host: ");
        request.push_str(&host);
        request.push_str("\r\n");
        request.push_str("Accept: text/html\r\n");
        request.push_str("Connection: close\r\n");
        request.push_str("\r\n");

        let _bytes_written = match stream.write(request.as_bytes()) {
            Ok(bytes) => bytes,
            Err(_e) => {
                return Err(Error::Network(String::from("Failed to write to stream")));
            }
        };

        // TODO: レスポンスの読み取りと解析を実装
        // 一時的な実装として、基本的なレスポンスを返す
        Ok(HttpResponse::new(
            String::from("HTTP/1.1"),
            200,
            String::from("OK"),
            Vec::new(),
            String::new(),
        ))
    }
}
