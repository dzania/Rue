use rue::bridge::Bridge;
use rue::config::User;
use std::path::Path;

#[tokio::main]
async fn main() {
    match User::exists() {
        true => {
            println!("authorized");
        }
        false => {
            Bridge::create_user().await.unwrap();
        }
    }
}
