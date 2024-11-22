extern crate alloc;
use noli::net::lookup_host;
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
                return Err(Error::Network(format!(
                    "Failed to find IP Addresses: {:#?} ",
                    e
                )))
            }
        };
        if ips.len() < 1 {
            return Err(Error::Network("No IP Addresses found for".to_string()));
        }
    }
}
