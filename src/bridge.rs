use reqwest;
use reqwest::Error;
use serde::Deserialize;
use std::thread;

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
struct User {
    username: String,
}

pub async fn create_user() -> Result<(), Error> {
    let bridges = find_bridges().await?;
    let client = reqwest::Client::new();

    //let res = client.post("")

    for bridge in bridges.iter() {
        client.post(format!("{}", &bridge.internalipaddress));
        println!("{:?}", &bridge.internalipaddress);
    }
    //println!("{:#?}", bridges[0..].await);

    Ok(())
}
