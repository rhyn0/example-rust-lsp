mod log;
use log::info;
use std::{env::current_dir, net::TcpListener};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:5678").unwrap();
    info(current_dir().unwrap().as_os_str().try_into().unwrap());
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        info("opened stream");
        println!("Connection established!");
    }
}
