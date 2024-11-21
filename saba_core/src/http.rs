use alloc::string::String;
use alloc::vec::Vec;

#[derive(Debug)]
pub struct HttpResponse {
    pub version: String,
    pub status_code: u32,
    pub reason: String,
    pub headers: Vec<(String, String)>,
    pub body: String,
}

impl HttpResponse {
    pub fn new(
        version: String,
        status_code: u32,
        reason: String,
        headers: Vec<(String, String)>,
        body: String,
    ) -> Self {
        Self {
            version,
            status_code,
            reason,
            headers,
            body,
        }
    }
}

#[derive(Debug)]
pub struct Header {
    pub name: String,
    pub value: String,
}

impl Header {
    pub fn new(name: String, value: String) -> Self {
        Self { name, value }
    }
}
