use crate::{config::User, errors::BridgeError};
use futures::{pin_mut, stream, StreamExt};
//use mdns_sd::{ServiceDaemon, ServiceEvent};
use mdns::{Record, RecordKind};
use reqwest::{Client, StatusCode};
use serde::Deserialize;
use serde_json::Value;
use std::collections::HashMap;
use std::{
    net::IpAddr,
    sync::{Arc, Mutex},
    time::Duration,
};
use tokio::sync::mpsc;

const MDNS_BRIDGE_SERVICE_NAME: &str = "_hue._tcp.local";
const DISCOVERY_URL: &str = "https://discovery.meethue.com/";

#[derive(Deserialize, Debug, Clone)]
pub struct Bridge {
    #[serde(rename = "internalipaddress")]
    internal_ip_address: IpAddr,
}
impl Bridge {
    pub async fn discover_bridges() -> Result<Vec<Self>, BridgeError> {
        match Bridge::mdns_discovery().await {
            Ok(bridges) => Ok(bridges),
            Err(_) => Bridge::nupnp_discovery().await,
        }
    }
    // find bridges using discovery url
    async fn nupnp_discovery() -> Result<Vec<Self>, BridgeError> {
        let response = reqwest::get(DISCOVERY_URL).await?;
        match response.status() {
            StatusCode::OK => {
                let bridges: Vec<Bridge> = response.json().await?;
                Ok(bridges)
            }
            _ => Err(BridgeError::ResponseError(response.text().await?)),
        }
    }
    /// Find bridges using mdns method
    /// https://developers.meethue.com/develop/application-design-guidance/hue-bridge-discovery/#mDNS
    async fn mdns_discovery() -> Result<Vec<Self>, BridgeError> {
        let stream =
            mdns::discover::all(MDNS_BRIDGE_SERVICE_NAME, Duration::from_secs(5))?.listen();
        pin_mut!(stream);
        let mut bridges: Vec<Bridge> = vec![];
        for response in stream.next().await {
            if let Ok(response) = response {
                let addr = response.records().find_map(Self::to_ip_addr);
                if let Some(addr) = addr {
                    bridges.push(Bridge {
                        internal_ip_address: addr,
                    });
                } else {
                    return Err(BridgeError::NoBridgesFound);
                }
            }
        }
        Ok(bridges)
    }
    fn to_ip_addr(record: &Record) -> Option<IpAddr> {
        match record.kind {
            RecordKind::A(addr) => Some(addr.into()),
            RecordKind::AAAA(addr) => Some(addr.into()),
            _ => None,
        }
    }

    // Send parallel requests to all bridges found
    pub async fn create_user(
        bridges: Vec<Self>,
        loader_progress: Arc<Mutex<u64>>,
    ) -> Result<(), BridgeError> {
        // Poll bridge for minute
        for i in 1..25 {
            let (tx, mut rx) = mpsc::channel(4);
            let requests = stream::iter(bridges.clone())
                .map(|bridge| {
                    tokio::spawn(async move {
                        Bridge::authorize_user_request(bridge.internal_ip_address).await
                    })
                })
                .buffer_unordered(bridges.len());

            // Use channel to because we cant break main loop from this scope
            // FIXME: Maybe there is better way to do this for now leave it as is
            requests
                .for_each(|b| async {
                    match b {
                        Ok(resp) => {
                            let _ = tx.send(resp).await;
                        }
                        Err(e) => {
                            let _ = tx
                                .send(Err(BridgeError::InternalError(e.to_string())))
                                .await;
                        }
                    }
                })
                .await;

            if let Some(resp) = rx.recv().await {
                println!("{:?}", resp);
                match resp {
                    Ok(user) => {
                        user.save()
                            .await
                            .map_err(|e| BridgeError::SaveUser(e.to_string()))?;
                        break;
                    }
                    Err(BridgeError::ButtonNotPressed) => (),
                    Err(_) => (),
                }
            };
            let mut loader = loader_progress.lock().unwrap();
            *loader += i;
            // FIXME: try to change to tokio::time::sleep
            std::thread::sleep(Duration::from_secs(5));
        }
        Ok(())
    }

    /// Send request to bridge to get User
    pub async fn authorize_user_request(ip: IpAddr) -> Result<User, BridgeError> {
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
        let value: Value =
            serde_json::from_str(&data).map_err(|e| BridgeError::InternalError(e.to_string()))?;

        // FIXME: Add better response parsing
        //
        println!("{:?}", value);
        match value[0].get("success") {
            Some(message) => {
                println!("WIADOMOŚĆ");
                println!("{message}");
                let username: String = serde_json::from_value(message.to_owned())
                    .map_err(|e| BridgeError::InternalError(e.to_string()))?;
                let user = User {
                    username,
                    bridge_address: ip,
                };
                Ok(user)
            }
            None => Err(BridgeError::ButtonNotPressed),
        }
    }
}
