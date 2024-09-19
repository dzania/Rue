use crate::app::App;
use crate::{config::User, errors::BridgeError};
use futures::{pin_mut, stream, StreamExt};
use mdns::{Record, RecordKind};
use reqwest::{Client, StatusCode};
use serde::Deserialize;
use std::collections::HashMap;
use std::{
    net::IpAddr,
    sync::{Arc, Mutex},
    time::Duration,
};
use tokio::sync::mpsc;

const MDNS_BRIDGE_SERVICE_NAME: &str = "_hue._tcp.local";
const DISCOVERY_URL: &str = "https://discovery.meethue.com/";

#[derive(Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Bridge {
    #[serde(rename = "internalipaddress")]
    internal_ip_address: IpAddr,
}

#[derive(Debug, Deserialize)]
struct SuccessResponse {
    success: HashMap<String, String>,
}

#[derive(Debug, Deserialize)]
struct ErrorResponse {
    error: HashMap<String, String>,
}

impl Bridge {
    pub async fn discover_bridges() -> Result<Vec<Self>, BridgeError> {
        return Bridge::nupnp_discovery().await;
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
    // TODO: Remove this code and write our own dns
    // this is not reliable as it can hang forever?
    async fn mdns_discovery() -> Result<Vec<Self>, BridgeError> {
        debug!("entering mdns");
        let stream =
            mdns::discover::all(MDNS_BRIDGE_SERVICE_NAME, Duration::from_secs(5))?.listen();
        let mut bridges: Vec<Bridge> = vec![];
        pin_mut!(stream);
        for response in (stream.next().await).into_iter().flatten() {
            debug!("response: {response:?}");
            let addr = response.records().find_map(Self::to_ip_addr);
            if let Some(addr) = addr {
                bridges.push(Bridge {
                    internal_ip_address: addr,
                });
            } else {
                return Err(BridgeError::NoBridgesFound);
            }
        }
        debug!("returning mdns");
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
    pub async fn create_user(bridges: Vec<Self>) -> Result<User, BridgeError> {
        // Poll bridge for minute
        debug!("Creating user");
        let (tx, mut rx) = mpsc::channel(bridges.len());
        let requests = stream::iter(bridges.clone())
            .map(|bridge| {
                tokio::spawn(async move {
                    Bridge::authorize_user_request(bridge.internal_ip_address).await
                })
            })
            .buffer_unordered(bridges.len());

        // Use channel to because we cant break main loop from this scope
        requests
            .for_each(|result| async {
                match result {
                    Ok(resp) => match resp {
                        Ok(user) => {
                            debug!("User created: {:?}", user);
                            let _ = tx.send(Ok(user)).await;
                        }
                        Err(BridgeError::ButtonNotPressed) => {
                            debug!("Button not pressed");
                            let _ = tx.send(Err(BridgeError::ButtonNotPressed)).await;
                        }
                        Err(e) => {
                            let _ = tx
                                .send(Err(BridgeError::InternalError(e.to_string())))
                                .await;
                        }
                    },
                    Err(e) => {
                        let _ = tx
                            .send(Err(BridgeError::InternalError(e.to_string())))
                            .await;
                    }
                }
            })
            .await;
        if let Some(resp) = rx.recv().await {
            resp
        } else {
            Err(BridgeError::InternalError("Nothing returned".into()))
        }
    }

    /// Send request to bridge to get User
    pub async fn authorize_user_request(ip: IpAddr) -> Result<User, BridgeError> {
        let address = format!("http://{}/api", ip);
        let client = Client::new();
        let mut body = HashMap::new();
        body.insert("devicetype", "rue_tui_app");
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

        if let Ok(success_response) = serde_json::from_str::<SuccessResponse>(&data) {
            if let Some(username) = success_response.success.get("username") {
                debug!("Received reponse: {success_response:?}");
                let user = User {
                    username: username.clone(),
                    bridge_address: ip,
                };
                return Ok(user);
            }
        } else if let Ok(error_response) = serde_json::from_str::<ErrorResponse>(&data) {
            if let Some(description) = error_response.error.get("description") {
                if description == "Link button not pressed" {
                    debug!("Received reponse: {error_response:?}");
                    return Err(BridgeError::ButtonNotPressed);
                }
            }
        }
        Err(BridgeError::ResponseError("Unknown response format".into()))
    }
}
