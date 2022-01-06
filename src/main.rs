use tokio;
mod bridge;

#[tokio::main]
async fn main() {
    bridge::create_user().await;
}
