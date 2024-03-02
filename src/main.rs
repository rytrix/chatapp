mod client;
mod server;

use server::Server;
use client::Client;

fn usage(args: &Vec<String>) {
    eprintln!("usage: {} <server, client> <ip>", args[0])
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 3 {
        usage(&args);
    }

    match args[1].as_ref() {
        "server" => {
            let s = Server::new(args[2].as_ref()).unwrap();
            s.run();
        }
        "client" => {
            let mut c = Client::new(args[2].as_ref()).unwrap();
            c.run().unwrap();
        }
        _ => {
            usage(&args);
        }
    }
}
