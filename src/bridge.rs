use crate::{config::User, errors::ConfigError};
use futures::{stream, StreamExt};
use reqwest::{Client, Error};
use serde::Deserialize;
use serde_json::Value;
use std::collections::HashMap;
use std::{thread, time::Duration};
use tokio::sync::mpsc;

#[derive(Deserialize, Debug)]
pub struct Bridge {
    internalipaddress: String,
}

pub enum BridgeErrors {
    ButtonNotPressed,
    #[allow(dead_code)]
    NoBridgesFound,
}

impl Bridge {
    // find bridges using discovery url
    pub async fn find_bridges() -> Result<Vec<Self>, Error> {
        let request: Vec<Bridge> = reqwest::get("https://discovery.meethue.com/")
            .await?
            .json()
            .await?;
        if request.is_empty() {
            panic!("No bridges found");
        }
        Ok(request)
    }

    // Send parallel requests to all bridges found
    pub async fn create_user() -> Result<(), ConfigError> {
        let ips: Vec<String> = Bridge::find_bridges()
            .await
            .expect("No bridges found")
            .into_iter()
            .map(|bridge| bridge.internalipaddress)
            .collect();

        // Poll bridge for minute
        for _ in 1..25 {
            let (tx, mut rx) = mpsc::channel(4);
            let requests = stream::iter(ips.clone())
                .map(|ip| {
                    tokio::spawn(async move {
                        let resp = Bridge::authorize_user_request(&ip).await;
                        resp
                    })
                })
                .buffer_unordered(ips.len());

            requests
                .for_each(|b| async {
                    match b {
                        Ok(Ok(b)) => {
                            let _ = tx.send(b).await;
                        }
                        // FIXME: Shouldn't print to std
                        Ok(Err(e)) => println!("Got a reqwest::Error: {}", e),
                        Err(e) => println!("Error: {}", e),
                    }
                })
                .await;

            if let Some(message) = rx.recv().await {
                if let Ok(user) = Bridge::handle_authorize_response(message).await {
                    user.save().await?;
                    break;
                }
            }
            thread::sleep(Duration::from_secs(4));
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
    pub async fn handle_authorize_response(
        message: serde_json::Value,
    ) -> Result<User, BridgeErrors> {
        match message[0].get("success") {
            Some(message) => {
                let user: User = serde_json::from_value(message.to_owned()).unwrap();
                Ok(user)
            }
            None => Err(BridgeErrors::ButtonNotPressed),
        }
    }
}
