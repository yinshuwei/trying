pub mod rpc_server;
pub mod rpc_client;

use std::env;

pub fn run()  {
    let mut args = env::args();
    args.next();
    let is_cli = match args.next() {
        Some(x) if x == "cli" => true,
        _ => false,
    };
    
    if is_cli {
        match rpc_client::run() {
            Ok(_) => println!("OK"),
            Err(e) => println!("Client Err: {}", e),
        }
    } else {
        match rpc_server::run() {
            Ok(_) => println!("OK"),
            Err(e) => println!("Server Err: {}", e),
        }
    }
}
