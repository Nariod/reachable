
use crate::arg_parser::meta_arg_parser;

mod arg_parser;

fn main() {
    println!("Hello, world!");
    let _args = meta_arg_parser();
}
