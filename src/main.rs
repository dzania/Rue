use rue::bridge::Bridge;
use std::path::Path;

#[tokio::main]
async fn main() {
    match Path::new(&dirs::home_dir().unwrap().join(".config/rue/rue.conf")).exists() {
        true => {
            println!("da");
        }
        false => {
            Bridge::create_user().await.unwrap();
        }
    }
}
