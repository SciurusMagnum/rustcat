extern crate ctrlc;
extern crate futures;
// extern crate futures_cpupool;
extern crate tokio;
extern crate tokio_io;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

use std::env; 
use std::thread;
use std::io::{self, BufRead};
use std::io::prelude::*;
use std::sync::mpsc;
use std::net;

use tokio::net::TcpStream;


fn stdin() -> std::sync::mpsc::Receiver<String>{
    let (tx, rx) = mpsc::channel();
    thread::spawn( move || {
        loop{
            let mut message = String::new();
            io::stdin().read_line(&mut message).expect("how");
            tx.send(message).unwrap();
        }
    });
    rx
}



fn main() {
    let args: Vec<String> = env::args().collect();
    let config = parse_config(&args);
    let mut message = String::new();
    // let sleep_time = time::Duration::from_millis(100);
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();
    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
    }).expect("Error setting Ctrl-C handler");
    println!("Waiting for Ctrl-C...");
    
    println!("ip: {}, port: {}", config.ip, config.port);
    let addr = format!("{}:{}", config.ip, config.port).parse().unwrap();
    let mut client_future = TcpStream::connect(& addr);
    
    let mut client_stream = TcpStream::connect(& addr);
    let stdin_reciever =  stdin();
    while running.load(Ordering::SeqCst) {
        // thread::sleep(sleep_time);
        message::from = match stdin_reciever.try_recv(){
            Ok(result) => result,
            Err(result) => { match result {
               mpsc::TryRecvError::Empty => continue,
               mpsc::TryRecvError::Disconnected => panic!("disconnected {}", result)
                }
            }

        };

        let _ = client_stream.write(message.as_bytes());

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