mod dns_message;
mod dns_client;
mod dns_server;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!("Usage: tp7_dns [client|server]");
        return;
    }

    match args[1].as_str() {
        "client" => client::run_client(),
        "server" => server::run_server(),
        _ => println!("Argument invalide. Utilisez 'client' ou 'server'"),
    }
}
