use crate::{
    config::User,
    errors::{BridgeError, ConfigError},
};
use futures::{pin_mut, stream, StreamExt};
use mdns::{Record, RecordKind};
use reqwest::{Client, Error};
use serde::Deserialize;
use serde_json::Value;
use std::collections::HashMap;
use std::{net::IpAddr, thread, time::Duration};
use tokio::sync::mpsc;

const MDNS_SERVICE_NAME: &str = "_hue._tcp.local";
const DISCOVERY_URL: &str = "https://discovery.meethue.com/";

#[derive(Deserialize, Debug, Clone)]
pub struct Bridge {
    internalipaddress: String,
}
impl Bridge {
    // find bridges using discovery url
    pub async fn find_bridges() -> Result<Vec<Self>, Error> {
        let request: Vec<Bridge> = reqwest::get(DISCOVERY_URL).await?.json().await?;
        if request.is_empty() {
            panic!("No bridges found");
        }
        Ok(request)
    }
    /// Find bridges using mdns method
    /// https://developers.meethue.com/develop/application-design-guidance/hue-bridge-discovery/#mDNS
    // FIXME: remove print later and refactor to_ip_addr(record: &Record)
    pub async fn mdns_discovery() -> Result<Self, mdns::Error> {
        println!("Starting mdns search...");
        let stream = mdns::discover::all(MDNS_SERVICE_NAME, Duration::from_millis(10))?.listen();
        pin_mut!(stream);
        let mut bridge = Bridge {
            internalipaddress: "".into(),
        };
        while let Some(Ok(response)) = stream.next().await {
            let addr = response.records().filter_map(Bridge::to_ip_addr).next();

            if let Some(addr) = addr {
                println!("Found bridge at {}", addr);
                bridge.internalipaddress = addr.to_string();
                break;
            } else {
                println!("Bridge does not advertise address");
            }
        }
        Ok(bridge)
    }
    fn to_ip_addr(record: &Record) -> Option<IpAddr> {
        match record.kind {
            RecordKind::A(addr) => Some(addr.into()),
            RecordKind::AAAA(addr) => Some(addr.into()),
            _ => None,
        }
    }

    // Send parallel requests to all bridges found
    pub async fn create_user() -> Result<(), ConfigError> {
        let bridges: Vec<Bridge> = Bridge::find_bridges()
            .await
            .expect("No bridges found")
            .into_iter()
            .collect();

        // Poll bridge for minute
        for _ in 1..25 {
            let (tx, mut rx) = mpsc::channel(4);
            let requests = stream::iter(bridges.clone())
                .map(|bridge| {
                    tokio::spawn(async move {
                        Bridge::authorize_user_request(&bridge.internalipaddress).await
                    })
                })
                .buffer_unordered(bridges.len());

            requests
                .for_each(|b| async {
                    match b {
                        Ok(Ok(b)) => {
                            let _ = tx.send(b).await;
                        }
                        // FIXME: Shouldn't print to std
                        Ok(Err(e)) => println!("Got a reqwest::Error: {:?}", e),
                        Err(e) => println!("Error: {}", e),
                    }
                })
                .await;

            if let Some(user) = rx.recv().await {
                user.save().await?;
                break;
            };
            thread::sleep(Duration::from_secs(5));
        }
        Ok(())
    }

    /// Send request to bridge to get User
    pub async fn authorize_user_request(ip: &str) -> Result<User, BridgeError> {
        let address = format!("http://{}/api", ip);
        let client = Client::new();
        let mut body = HashMap::new();
        body.insert("devicetype", "rue_pc_app");
        let resp = client
            .post(&address)
            .json(&body)
            .send()
            .await
            .map_err(|e| BridgeError::RequestError(e.to_string()))?;
        let data = resp
            .text()
            .await
            .map_err(|e| BridgeError::ResponseError(e.to_string()))?;
        let value: Value = serde_json::from_str(&data).unwrap();

        match value[0].get("success") {
            Some(message) => {
                let username: String = serde_json::from_value(message.to_owned()).unwrap();
                let user = User {
                    username,
                    bridge_address: ip.into(),
                };
                Ok(user)
            }
            None => Err(BridgeError::ButtonNotPressed),
        }
    }
}
