#![allow(dead_code)]
#![allow(unused_imports)]

use std::net::TcpListener;
use std::thread;
use std::io::{self, Write};

// mod server; // Assuming your server logic is in server.rs
use ivan_game::server;

fn main() {
    let address = "127.0.0.1:8080";
    println!("Starting server at {}", address);
    server::start_server(address);
}
