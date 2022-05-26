mod bridge;
mod config;

#[tokio::main]
async fn main() {
    let bridges = bridge::find_bridges().await.unwrap();
    println!("{:?}", bridges);
    bridge::create_user(bridges).await;
}
