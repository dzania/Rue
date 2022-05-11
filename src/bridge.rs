use futures::{stream, StreamExt};
use reqwest;
use reqwest::{Client, Error};
use serde::Deserialize;
use serde_json::Value;
use std::collections::HashMap;
use std::thread;
use std::time::Duration;
use tokio::sync::mpsc;

#[derive(Deserialize, Debug)]
pub struct Bridge {
    internalipaddress: String,
}

#[derive(Deserialize, Debug)]
pub struct User {
    username: String,
}

// TODO: create config methods
impl User {
    // TODO: store user credentials method
    pub async fn store_credentials(self) -> Result<(), ()> {
        Ok(())
    }
}

// TODO: Bridge api errors
pub enum BridgeError {
    ButtonNotPressed,
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
pub async fn create_user(bridges: Vec<Bridge>) -> Result<(), ()> {
    let mut ips: Vec<String> = bridges
        .into_iter()
        .map(|bridge| bridge.internalipaddress)
        .collect();

    // Remove router adress from bridges
    ips.retain(|ip| ip != "192.168.0.100");

    let mut counter = 0;
    while counter < 25 {
        let (tx, mut rx) = mpsc::channel(4);
        let requests = stream::iter(ips.clone())
            .map(|ip| {
                tokio::spawn(async move {
                    let resp = authorize_user_request(&ip).await;
                    resp
                })
            })
            .buffer_unordered(ips.len());

        requests
            .for_each(|b| async {
                match b {
                    Ok(Ok(b)) => {
                        tx.send(b).await; // TODO: Handle result
                    }
                    // FIXME: Shouldn't print to std
                    Ok(Err(e)) => eprintln!("Got a reqwest::Error: {}", e),
                    Err(e) => println!("Error: {}", e),
                }
            })
            .await;

        if let Some(message) = rx.recv().await {
            match handle_authorize_response(message).await {
                Ok(user) => {
                    user.store_credentials().await?;
                    break;
                }
                Err(_) => (),
            }
        }
        thread::sleep(Duration::from_secs(4));
        counter += 1;
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

// TODO: Reasonable error
pub async fn handle_authorize_response(message: serde_json::Value) -> Result<User, BridgeError> {
    match message[0].get("success") {
        Some(message) => {
            let user: User = serde_json::from_value(message.to_owned()).unwrap();
            println!("Bridge created user: {:?}", user);
            // TODO: create config file to store data
            Ok(user)
        }
        None => {
            println!("Bridge returned error response: {:?}", message[0]);
            Err(BridgeError::ButtonNotPressed)
        }
    }
}
