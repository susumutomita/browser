extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;
use core::fmt::Write;

use noli::net::lookup_host;
use saba_core::error::Error;
use saba_core::http::HttpResponse;

#[derive(Default)]
pub struct HttpClient {}

impl HttpClient {
    pub fn new() -> Self {
        Self {}
    }

    pub fn get(&self, host: String, _port: u16, _path: String) -> Result<HttpResponse, Error> {
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

        // 仮のHTTPレスポンスを返します
        Ok(HttpResponse::new(
            String::from("HTTP/1.1"),
            200,
            String::from("OK"),
            Vec::new(),
            String::from(""),
        ))
    }
}
