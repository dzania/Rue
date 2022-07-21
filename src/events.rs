use tokio::sync::mpsc;

pub struct EventLoop {
    pub rx: mpsc::UnboundedReceiver<T>,
    pub tx: mpsc::UnboundedSender<T>,
}
