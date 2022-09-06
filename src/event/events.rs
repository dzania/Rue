use super::key::Key;
use crossterm::event;
use std::sync::{
    mpsc,
    mpsc::{Receiver, Sender},
};
use std::thread;
use std::time::Duration;

/// Main event handler to communicate between
/// input handler and rendering loop
pub struct Events {
    pub rx: Receiver<IoEvent>,
    pub tx: Sender<IoEvent>,
}

impl Events {
    pub fn new(tick_rate: Duration) -> Self {
        let (tx, rx) = mpsc::channel();
        let event_tx = tx.clone();
        thread::spawn(move || {
            loop {
                // poll for tick rate duration, if no event, sent tick event.
                if event::poll(tick_rate).unwrap() {
                    if let event::Event::Key(key) = event::read().unwrap() {
                        let key = Key::from(key);

                        event_tx.send(IoEvent::Input(key)).unwrap();
                    }
                }

                event_tx.send(IoEvent::Tick).unwrap();
            }
        });
        Events { rx, tx }
    }
    /// Block current thread and try to read event
    pub fn next(&self) -> Result<IoEvent, mpsc::RecvError> {
        self.rx.recv()
    }
}
impl Default for Events {
    fn default() -> Self {
        Self::new(Duration::from_millis(150))
    }
}

#[derive(Debug)]
pub enum IoEvent {
    Input(Key),
    Tick,
}
