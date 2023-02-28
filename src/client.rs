use crate::arg_parser::Target;

async fn http_request(target: Target, target_port: u16) -> Result<(), Box<dyn std::error::Error>> {
    let domain = match target {
        Target::Domain(content) => content,
        Target::Ipv4Addr(content) => content.to_string(),
    };

    let url = format!("http://{domain}:{target_port}/hello/reachable");

    println!("[+] Performing HTTP request on {}", &url);

    let resp = reqwest::get(url).await?.status();

    println!("HTTP response status: {:#?}", resp);

    Ok(())
}

async fn https_request(target: Target, target_port: u16) -> Result<(), Box<dyn std::error::Error>> {
    let domain = match target {
        Target::Domain(content) => content,
        Target::Ipv4Addr(content) => content.to_string(),
    };
    let url = format!("https://{domain}:{target_port}/hello/reachable");

    //todo: allow self signed certificate
    let resp = reqwest::get(url).await?.status();

    println!("HTTPS response status: {:#?}", resp);

    Ok(())
}

#[tokio::main]
pub async fn meta_client(target: Target) -> Result<(), Box<dyn std::error::Error>> {
    let mut handles = vec![];

    let handle = tokio::spawn({
        let target_ip = target.clone();
        let target_port = 80;
        async move {
            let _ = http_request(target_ip, target_port).await;
        }
    });
    handles.push(handle);

    let handle = tokio::spawn({
        let target_ip = target.clone();
        let target_port = 443;
        async move {
            let _ = https_request(target_ip, target_port).await;
        }
    });
    handles.push(handle);

    for handle in handles {
        handle.await?;
    }

    Ok(())
}
