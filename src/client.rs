use ureq::{Agent, AgentBuilder};
use std::{time::Duration, net::Ipv4Addr};

use crate::arg_parser::Target;


async fn http_request(target: Target, target_port: u16) -> Result<(), Box<dyn std::error::Error>> {
    let agent: Agent = ureq::AgentBuilder::new()
        .timeout_read(Duration::from_secs(5))
        .timeout_write(Duration::from_secs(5))
        .build();

    let domain = match target {
        Target::Domain(content) => content,
        Target::Ipv4Addr(content) => content.parse::<Ipv4Addr>(),
    };

    let url = format!("http://{domain}:{target_port}/hello");

    let body: String = agent.get(&url)
        .call()?
        .into_string()?;
    
    Ok(())
}

async fn https_request(target: Target, target_port: u16) -> Result<(), Box<dyn std::error::Error>> {
    let agent: Agent = ureq::AgentBuilder::new()
        .timeout_read(Duration::from_secs(5))
        .timeout_write(Duration::from_secs(5))
        .build();

let body: String = agent.get("https://{}/page")
        .call()?
        .into_string()?;

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
    }});
    handles.push(handle);

    let handle = tokio::spawn({
        let target_ip = target.clone();
        let target_port = 443;
        async move {
        let _ = https_request(target_ip, target_port).await;    
    }});
    handles.push(handle);

    for handle in handles {
        handle.await?;
    }

    Ok(())
}