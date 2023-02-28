// Module dedicated to parsing and checking arguments

use addr::parse_domain_name;
use std::env;
use std::net::{IpAddr, Ipv4Addr};

#[derive(Debug)]
pub struct Inputs {
    pub mode: Mode,
    pub ip: IpAddr,
}

#[derive(Clone, Debug)]
pub enum Target {
    Ipv4Addr(Ipv4Addr),
    Domain(String),
}

#[derive(PartialEq, Debug)]
pub enum Mode {
    Client,
    Server,
}

fn getting_arguments() -> Vec<String> {
    let inputs: Vec<String> = env::args().collect();
    if inputs.len() != 3 {
        panic!("This program requires two arguments. Eg: `./reachable client my.domain.com`");
    }
    let mode = &inputs[1];
    let ip = &inputs[2];
    let result = vec![mode.clone(), ip.clone()];
    result
}

fn is_mode_ok(input: &str) -> Mode {
    let result;
    if *input == *"client" {
        result = Mode::Client;
    } else if *input == *"server" {
        result = Mode::Server;
    } else {
        panic!("Please select either client or server mode");
    }
    result
}

fn is_target_ok(input: &str) -> Target {
    let result;
    if is_ip_ok(input) {
        result = Target::Ipv4Addr(String::from(input).parse().unwrap());
    } else if is_domain_ok(input) {
        result = Target::Domain(String::from(input));
    } else {
        panic!("Please enter either a valid IPv4 or domain");
    }
    result
}

fn is_ip_ok(input: &str) -> bool {
    input.parse::<Ipv4Addr>().is_ok()
}

fn is_domain_ok(input: &str) -> bool {
    parse_domain_name(input).is_ok()
}

pub fn meta_arg_parser() -> (Mode, Target) {
    let args = getting_arguments();
    let mode = is_mode_ok(&args[0]);
    let target = is_target_ok(&args[1]);
    (mode, target)
}
