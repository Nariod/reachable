// Module dedicated to parsing arguments

use std::env;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

#[derive(Debug)]
pub struct Inputs {
    pub mode: Mode,
    pub ip: IpAddr,
}

#[derive(Debug)]
pub enum Mode {
    Client,
    Server
}

fn getting_arguments() -> Vec<String> {
    let inputs: Vec<String> = env::args().collect();
    let mode = &inputs[1];
    let ip = &inputs[2];
    let result = vec![mode.clone(), ip.clone()];
    result
}

fn is_mode_ok(input: &String) -> Mode {
    let result;
    if input.clone() == String::from("client") {
        result = Mode::Client;
    } else if input.clone() == String::from("server") {
        result = Mode::Server;
    } else {
        panic!("Please select either client or server mode");
    }
    result
}


pub fn meta_arg_parser() {
    let args = getting_arguments();
    let mode = is_mode_ok(&args[0]);
    println!("Selected mode: {:?}", mode);
}