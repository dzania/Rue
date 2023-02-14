use crate::{config::User, errors::BridgeError, App};
use futures::{stream, StreamExt};
//use mdns::{Record, RecordKind};
use reqwest::Client;
use serde::Deserialize;
use serde_json::Value;
use simple_mdns::sync_discovery::OneShotMdnsResolver;
use std::collections::HashMap;
use std::{
    net::IpAddr,
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};
use tokio::sync::mpsc;

const MDNS_BRIDGE_SERVICE_NAME: &str = "_hue._tcp.local";
const DISCOVERY_URL: &str = "https://discovery.meethue.com/";

#[derive(Deserialize, Debug, Clone)]
pub struct Bridge {
    #[serde(rename = "internalipaddress")]
    internal_ip_address: String,
}
impl Bridge {
    // find bridges using discovery url
    pub async fn find_bridges() -> Result<Vec<Self>, BridgeError> {
        //println!("find request");
        let request: Vec<Bridge> = reqwest::get(DISCOVERY_URL).await?.json().await?;
        Ok(request)
    }
    /// Find bridges using mdns method
    /// https://developers.meethue.com/develop/application-design-guidance/hue-bridge-discovery/#mDNS
    // FIXME: remove print later and refactor to_ip_addr(record: &Record)
    pub async fn mdns_discovery() /*Result<Vec<Self>, mdns::Error>*/
    {
        //println!("Starting mdns search...");
        let resolver = OneShotMdnsResolver::new().expect("Failed to create resolver");
        let answer = resolver
            .query_service_address(MDNS_BRIDGE_SERVICE_NAME)
            .expect("Failed to query service address");

        println!("{:?}", answer);
    }
    /// Helper function to map Record for IpAddr
    //fn to_ip_addr(record: &Record) -> Option<IpAddr> {
    //match record.kind {
    //RecordKind::A(addr) => Some(addr.into()),
    //RecordKind::AAAA(addr) => Some(addr.into()),
    //_ => None,
    //}
    //}

    // Send parallel requests to all bridges found
    pub async fn create_user(loader_progress: Arc<Mutex<u64>>) -> Result<(), BridgeError> {
        // Search bridges using mdns method if no result
        // search using discovery endpoint
        Bridge::mdns_discovery().await;
        let bridges = Bridge::find_bridges().await?;

        // Poll bridge for minute
        for i in 1..25 {
            let (tx, mut rx) = mpsc::channel(4);
            let requests = stream::iter(bridges.clone())
                .map(|bridge| {
                    tokio::spawn(async move {
                        Bridge::authorize_user_request(&bridge.internal_ip_address).await
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
            //println!("{}", loader);
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
