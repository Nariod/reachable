use core::panic;
use rcgen::generate_simple_self_signed;
use std::net::Ipv4Addr;

use crate::arg_parser::Target;

async fn http_server(
    target_ip: Ipv4Addr,
    target_port: u16,
) -> Result<(), Box<dyn std::error::Error>> {
    println!(
        "[+] Starting HTTP server on {:#?}:{}",
        target_ip, target_port
    );
    let hello = warp::path!("hello" / String);
    warp::serve(hello).run((target_ip, target_port)).await;

    Ok(())
}

async fn https_server(
    target_ip: Ipv4Addr,
    target_port: u16,
) -> Result<(), Box<dyn std::error::Error>> {
    println!(
        "[+] Starting HTTPS server on {:#?}:{}",
        target_ip, target_port
    );
    let subject_alt_names = vec!["localhost".to_string()];
    let cert = generate_simple_self_signed(subject_alt_names).unwrap();

    let hello = warp::path!("hello" / String);
    warp::serve(hello)
        .tls()
        .cert(cert.serialize_pem().unwrap())
        .key(cert.serialize_private_key_pem())
        .run((target_ip, target_port))
        .await;

    Ok(())
}

#[tokio::main]
pub async fn meta_server(target: Target) -> Result<(), Box<dyn std::error::Error>> {
    let target_ip: Ipv4Addr = match target {
        Target::Ipv4Addr(content) => content,
        Target::Domain(_content) => {
            panic!("Can't use a domain for server mode, use a valid IPv4 instead")
        }
    };
    let mut handles = vec![];

    let handle = tokio::spawn({
        let target_ip = target_ip;
        let target_port = 80;
        async move {
            let _ = http_server(target_ip, target_port).await;
        }
    });
    handles.push(handle);

    let handle = tokio::spawn({
        let target_ip = target_ip;
        let target_port = 443;
        async move {
            let _ = https_server(target_ip, target_port).await;
        }
    });
    handles.push(handle);

    for handle in handles {
        handle.await?;
    }

    Ok(())
}
