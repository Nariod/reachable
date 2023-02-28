use crate::arg_parser::{meta_arg_parser, Mode};

mod arg_parser;
mod client;
mod server;

fn main() {
    //println!("Hello from main!");
    let args = meta_arg_parser();
    println!("Selected mode: {:?}", args.0);
    println!("Selected target: {:?}", args.1);
    if args.0 == Mode::Server {
        let _ = server::meta_server(args.1);
    } else if args.0 == Mode::Client {
        let _ = client::meta_client(args.1);
    }
}
