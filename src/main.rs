mod ui;
use rue::App;
use std::sync::{Arc, Mutex};

#[tokio::main]
async fn main() {
    let app = Arc::new(Mutex::new(App::new()));
    ui::start_ui(&app).await.unwrap();
}
