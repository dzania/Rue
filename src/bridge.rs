use reqwest;
use reqwest::{Client, Error};
use serde::Deserialize;
use serde_json::Value;
use std::collections::HashMap;
use tokio;
//use std::thread;
//use std::time::Duration;
//use futures::{stream, StreamExt};

#[derive(Deserialize, Debug)]
pub struct Bridge {
    id: String,
    internalipaddress: String,
}

pub async fn find_bridges() -> Result<Vec<Bridge>, Error> {
    let request: Vec<Bridge> = reqwest::get("https://discovery.meethue.com/")
        .await?
        .json()
        .await?;
    Ok(request)
}

#[derive(Deserialize, Debug)]
pub struct User {
    username: String,
}
/// Send parallel requests to all bridges found
pub async fn authorize_parallel(bridges: Vec<Bridge>) -> Result<(), Error> {
    let mut ips: Vec<String> = bridges
        .into_iter()
        .map(|bridge| bridge.internalipaddress)
        .collect();

    ips.retain(|ip| ip != "192.168.0.100");
    println!("{:?}", ips);

    let client = Client::new();

    for ip in ips {
        let result = authorize_user_request(&client, &ip).await?;
        println!("{:?}", result);
    }

    Ok(())
}

/// Send request to bridge to get User
// TODO: Fix deserialization on unauthorized user
pub async fn authorize_user_request(client: &Client, ip: &str) -> Result<User, Error> {
    let address = format!("http://{}/api/newdeveloper", ip);
    println!("{}", address);

    let mut params = HashMap::new();
    params.insert("devicetype", "rue_pc_app");
    println!("Sendingrequest");
    let resp = client.post(&address).json(&params).send().await?;
    let user: User = resp.json::<User>().await?;
    println!("Po req");

    Ok(user)
}
