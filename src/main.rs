extern crate ctrlc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use std::env; 
use std::io::stdin;
use std::io::prelude::*;
use std::net::TcpStream;




fn main() {
    let args: Vec<String> = env::args().collect();
    let config = parse_config(&args);
    let mut message = String::new();
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();
    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
    }).expect("Error setting Ctrl-C handler");
    println!("Waiting for Ctrl-C...");
    
    println!("ip: {}, port: {}", config.ip, config.port);
    let mut client_stream = TcpStream::connect(format!("{}:{}", config.ip, config.port)).expect("Couldnt connect to target");
    while running.load(Ordering::SeqCst) {
        // stdin().read_line(&mut message).expect("how");
        // let _ = client_stream.write(message.as_bytes());
    }
    println!("Got it! Exiting...");
}

struct Config {
    ip: String,
    port: i32,
}

fn parse_config(args: &[String]) -> (Config) {

    if 3 > args.len() {
        panic!("usage: client <ip> <port>")
    }
    let ip = args[1].clone();
    let port = args[2].parse::<i32>().expect("port must be an unsigned int");

    Config {ip, port}
}