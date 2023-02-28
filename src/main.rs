use crate::arg_parser::meta_arg_parser;

mod arg_parser;

fn main() {
    println!("Hello from main!");
    let args = meta_arg_parser();
    println!("Selected mode: {:?}", args.0);
    println!("Selected target: {:?}", args.1);
}
