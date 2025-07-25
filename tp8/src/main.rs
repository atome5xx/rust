mod protocol;
mod server;
mod client;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} [server|client]", args[0]);
        return;
    }

    match args[1].as_str() {
        "server" => server::run_server().unwrap(),
        "client" => client::run_client().unwrap(),
        _ => eprintln!("Invalid mode. Use 'server' or 'client'."),
    }
}