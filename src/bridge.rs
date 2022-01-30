use reqwest;
use reqwest::{Client, Error};
use serde::Deserialize;
use serde_json::Value;
use std::collections::HashMap;
use std::thread;
use std::time;

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
    for ip in ips {
        while counter < 25 {
            let result = authorize_user_request(&ip).await?;
            println!("{:?}", result);
            if let Some(_success) = result[0].get("success") {
                println!("User created");
                let username = result[0]["success"]["username"].to_string();
                let u = User { username };
            } else {
                println!("Error: {:?}", result[0]["error"]);
            }
            counter += 1;
            thread::sleep(time::Duration::from_secs(3));
        }
    }

    Ok(())
}

/// Send request to bridge to get User
pub async fn authorize_user_request(ip: &str) -> Result<serde_json::Value, Error> {
    let address = format!("http://{}/api", ip);
    println!("{}", address);

    let client = Client::new();
    let mut body = HashMap::new();
    body.insert("devicetype", "rue_pc_app");
    let resp = client.post(&address).json(&body).send().await?;
    let data = resp.text().await?;
    let value: Value = serde_json::from_str(&data).unwrap();
    Ok(value)
}
