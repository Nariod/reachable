use rcgen::generate_simple_self_signed;
use sha2::{Digest, Sha256};
use std::convert::Infallible;
use std::net::Ipv4Addr;
use warp::Filter;
use warp::{http::StatusCode, Reply};

use crate::arg_parser::Target;

async fn hash_body(body: bytes::Bytes) -> Result<impl Reply, Infallible> {
    let mut hasher = Sha256::new();
    hasher.update(&body);
    let result = hasher.finalize();

    let hex_string = result
        .iter()
        .map(|byte| format!("{:02x}", byte))
        .collect::<String>();

    Ok(hex_string)
}

async fn http_server(
    target_ip: Ipv4Addr,
    target_port: u16,
) -> Result<(), Box<dyn std::error::Error>> {
    println!(
        "[+] Starting HTTP server on {:#?}:{}",
        target_ip, target_port
    );
    let is_reachable = warp::post()
        .and(warp::path("hello"))
        .and(warp::path("reachable"))
        .and(warp::path::end())
        .and(warp::body::bytes())
        .and_then(|body: bytes::Bytes| async move {
            hash_body(body)
                .await
                .map(|hash| warp::reply::with_status(hash, StatusCode::OK))
                .map_err(|_| warp::reject())
        });

    //let hello = warp::path!("hello" / String);
    warp::serve(is_reachable)
        .run((target_ip, target_port))
        .await;

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

    //let hello = warp::path!("hello" / String);
    let is_reachable = warp::post()
        .and(warp::path("hello"))
        .and(warp::path("reachable"))
        .and(warp::path::end())
        .and(warp::body::bytes())
        .and_then(|body: bytes::Bytes| async move {
            hash_body(body)
                .await
                .map(|hash| warp::reply::with_status(hash, StatusCode::OK))
                .map_err(|_| warp::reject())
        });

    warp::serve(is_reachable)
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
        Target::Domain(_content) => Ipv4Addr::new(0, 0, 0, 0),
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
