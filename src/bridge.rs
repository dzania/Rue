use futures::{stream, StreamExt}; // 0.3.8
use reqwest;
use reqwest::{Client, Error};
use serde::Deserialize;
use serde_json::Value;
use std::collections::HashMap;
use std::thread;
use std::time::Duration;
use tokio;

#[derive(Deserialize, Debug)]
pub struct Bridge {
    id: String,
    internalipaddress: String,
}
pub struct User {
    username: String,
}

/// find bridges using discovery url
pub async fn find_bridges() -> Result<Vec<Bridge>, Error> {
    let request: Vec<Bridge> = reqwest::get("https://discovery.meethue.com/")
        .await?
        .json()
        .await?;
    Ok(request)
}

/// Send parallel requests to all bridges found
pub async fn create_user(bridges: Vec<Bridge>) -> Result<(), Error> {
    let mut ips: Vec<String> = bridges
        .into_iter()
        .map(|bridge| bridge.internalipaddress)
        .collect();

    // Remove router adress from bridges
    ips.retain(|ip| ip != "192.168.0.100");

    let mut counter = 0;
    while counter < 15 {
        let requests = stream::iter(ips.clone())
            .map(|ip| {
                tokio::spawn(async move {
                    let resp = authorize_user_request(&ip).await;
                    resp
                })
            })
            .buffer_unordered(ips.len());

        let resp = requests
            .for_each(|b| async {
                match b {
                    Ok(Ok(b)) => Ok(b),
                    Ok(Err(e)) => Err(b),
                }
            })
            .await;
        println!("result: {:?}", resp);
        thread::sleep(Duration::from_secs(4));
        counter += 1
    }

    Ok(())
}

/// Send request to bridge to get User
pub async fn authorize_user_request(ip: &str) -> Result<serde_json::Value, Error> {
    let address = format!("http://{}/api", ip);
    let client = Client::new();
    let mut body = HashMap::new();
    body.insert("devicetype", "rue_pc_app");
    let resp = client.post(&address).json(&body).send().await?;
    let data = resp.text().await?;
    let value: Value = serde_json::from_str(&data).unwrap();
    Ok(value)
}
