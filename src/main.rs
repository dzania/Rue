use rue::{bridge::Bridge, start_ui};
use std::path::Path;

#[tokio::main]
async fn main() {
    match Path::new(&dirs::home_dir().unwrap().join(".config/rue/rue.conf")).exists() {
        true => {
            start_ui();
        }
        false => {
            Bridge::create_user().await.unwrap();
        }
    }
}
