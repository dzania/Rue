use crossterm::event::KeyCode;
use tokio::sync::mpsc;

/// Main event handler to communicate between
/// input handler and rendering loop
pub struct Events {
    pub rx: mpsc::UnboundedReceiver<InputEvent>,
    pub tx: mpsc::UnboundedSender<InputEvent>,
}

pub enum InputEvent {
    Input(KeyCode),
    Tick,
}
