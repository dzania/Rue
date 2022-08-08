use crossterm::event::KeyCode;
use tokio::sync::mpsc;

/// Main event handler to communicate between
/// input handler and rendering loop
pub struct Events {
    pub rx: mpsc::UnboundedReceiver<IoEvent>,
    pub tx: mpsc::UnboundedSender<IoEvent>,
}

impl Events {
    pub fn new(tick_rate: Duration) -> Self {
        let (tx, rx) = mpsc::channel();
        let event_tx = tx.clone();
        thread::spawn(move || {
            loop {
                // poll for tick rate duration, if no event, sent tick event.
                if crossterm::event::poll(tick_rate).unwrap() {
                    if let event::Event::Key(key) = event::read().unwrap() {
                        let key = Key::from(key);
                        event_tx.send(InputEvent::Input(key)).unwrap();
                    }
                }
                event_tx.send(InputEvent::Tick).unwrap();
            }
        });

        Events { rx, _tx: tx }
    }
}

pub enum IoEvent {
    Input(KeyCode),
    Tick,
}
