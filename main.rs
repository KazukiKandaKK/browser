#![no_std]
#![no_main]

extern create alloc;

use create::alloc::string::ToString;
use net_wasabi::http::HttpClient;
use noli::prelude::*;

fn main() -> u64 {
    let client = HttpClient::new();
    match client.get("example.com".to_string(), 80, "/".to_string()){
    Ok(res) => {
        print!("response:\n{:#?}", res);
    }
    Err(e) => {
        print!("error:\n{:#?}", e);
    }
    0
}

entory_point!(main);

