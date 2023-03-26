use random_string::generate;
use sha2::{Digest, Sha256};

use crate::arg_parser::Target;

fn create_challenge() -> String {
    let charset = "1234567890";
    generate(15, charset)
}

fn check_challenge(challenge: &str, response: &str) -> bool {
    let mut hasher = Sha256::new();
    hasher.update(challenge);
    let result = hasher.finalize();
    let res_hex = hex::encode(result);

    response == res_hex
}

async fn http_request(
    target: Target,
    target_port: u16,
    challenge: String,
) -> Result<(), Box<dyn std::error::Error>> {
    let chall1 = challenge.clone();
    let domain = match target {
        Target::Domain(content) => content,
        Target::Ipv4Addr(content) => content.to_string(),
    };

    let url = format!("http://{domain}:{target_port}/hello/reachable");

    println!("Trying HTTP request to {}", &url);

    let client = reqwest::Client::new();
    let resp = client
        .post(url)
        .body(challenge)
        .send()
        .await?
        .text()
        .await?;
    //let resp = reqwest::get(url).await?.status();
    //println!("HTTP response status: {:#?}", resp);

    let is_chall_ok = check_challenge(&chall1, &resp);

    if is_chall_ok {
        println!("[+] Challenge successful over HTTP !")
    } else {
        println!("[-] Challenge failed over HTTP")
    }

    Ok(())
}

async fn https_request(
    target: Target,
    target_port: u16,
    challenge: String,
) -> Result<(), Box<dyn std::error::Error>> {
    let chall1 = challenge.clone();
    let domain = match target {
        Target::Domain(content) => content,
        Target::Ipv4Addr(content) => content.to_string(),
    };
    let url = format!("https://{domain}:{target_port}/hello/reachable");
    println!("Trying HTTPS request to {}", &url);

    let resp = reqwest::Client::builder()
        .danger_accept_invalid_certs(true)
        .build()?
        .post(&url)
        .body(challenge)
        .send()
        .await?
        .text()
        .await?;

    let is_chall_ok = check_challenge(&chall1, &resp);

    if is_chall_ok {
        println!("[+] Challenge successful over HTTPS !")
    } else {
        println!("[-] Challenge failed over HTTPS")
    }
    Ok(())
}

#[tokio::main]
pub async fn meta_client(target: Target) -> Result<(), Box<dyn std::error::Error>> {
    let challenge = create_challenge();
    let chall1 = challenge.clone();

    let mut handles = vec![];

    let handle = tokio::spawn({
        let target_ip = target.clone();
        let target_port = 80;
        async move {
            let _ = http_request(target_ip, target_port, challenge.clone()).await;
        }
    });
    handles.push(handle);

    let handle = tokio::spawn({
        let target_ip = target.clone();
        let target_port = 443;
        async move {
            let _ = https_request(target_ip, target_port, chall1.clone()).await;
        }
    });
    handles.push(handle);

    for handle in handles {
        handle.await?;
    }

    Ok(())
}
