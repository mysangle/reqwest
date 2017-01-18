extern crate reqwest;
extern crate env_logger;

use std::env;

fn main() {
    env_logger::init().unwrap();

    let url = match env::args().nth(1) {
        Some(url) => url,
        None => {
            println!("Usage: reqwest <url>");
            return;
        }
    };

    let res = reqwest::get(&url).unwrap();

    println!("Status: {}", res.status());
    println!("Headers:\n{}", res.headers());
}
