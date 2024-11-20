extern crate alloc;
// use alloc::string::String;
// use alloc::vec::Vec;
// use saba_core::error::Error;
// use saba_core::http::HttpResponse;

#[derive(Default)]
pub struct HttpClient {}

impl HttpClient {
    pub fn new() -> Self {
        Self {}
    }

    // pub fn get(&self, host: String, port: u16, path: String) -> Result<HttpResponse, Error> {
    //     Ok(HttpResponse::new(
    //         String::from("HTTP/1.1"),
    //         200,
    //         String::from("OK"),
    //         Vec::new(),
    //         String::from(""),
    //     ))
    // }
}
