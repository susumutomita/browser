#![no_std]
#![cfg_attr(not(target_os = "linux"), no_main)]

extern crate alloc;
use crate::alloc::string::ToString;
use net_wasabi::http::HttpClient;
use noli::prelude::*;
use noli::*;
use saba_core::browser::browser;
use saba_core::http::HttpResponse;

static TEST_HTTP_RESPONSE: &str = r#"HTTP/1.1 200 OK
Data xx xx xx
<html>
    <head></head>
    <body>
        <h1 if=title>H1 Title</h1>
        <h2 class="class">H2 title</h2>
        <p>Test text.</p>
        <p>
            <a href="https://www.google.com">Google</a>
            <a href="https://www.yahoo.com">Yahoo</a>
        </p>
    </body>
    </html>
    "#;

fn main() {
    let browser = Browser::new();
    let response =
        HttpResponse::new(TEST_HTTP_RESPONSE.to_string()).expect("failed to parse http response");
    let page = browser.bollow().current_page();
    let dom_string = page.borrow_mut().recieve_response(response);
    for log in dom_string.lines() {
        println!("{}", log);
    }
    0
}

entry_point!(main);
