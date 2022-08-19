use crossterm::event::KeyCode;
use std::thread;
use std::time::Duration;
use tokio::sync::{
    mpsc,
    mpsc::{UnboundedReceiver, UnboundedSender},
};

/// Main event handler to communicate between
/// input handler and rendering loop
pub struct Events {
    pub rx: UnboundedReceiver<IoEvent>,
    pub tx: UnboundedSender<IoEvent>,
}

impl Events {
    pub fn new(tick_rate: Duration) -> Self {
        let (tx, rx) = mpsc::channel(4);
        let event_tx = tx.clone();
        thread::spawn(move || {
            loop {
                // poll for tick rate duration, if no event, sent tick event.
                if crossterm::event::poll(tick_rate).unwrap() {
                    if let IoEvent::Input(key) = events::read().unwrap() {
                        let key = Key::from(key);
                        event_tx.send(IoEvent::Input(key)).unwrap();
                    }
                }
                event_tx.send(IoEvent::Tick).unwrap();
            }
        });

        Events { rx, tx }
    }
}

pub enum IoEvent {
    Input(KeyCode),
    Tick,
}
