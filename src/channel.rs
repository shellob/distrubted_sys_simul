use std::thread;
use std::time::Duration;
use std::sync::Arc;

use crate::process::Process;
use crate::message::Message;

#[derive(Debug, Clone)]
pub struct Channel {
    pub from: usize,
    pub to: usize,
    pub delay_ms: u64,
}

impl Channel {
    pub fn new(from: usize, to: usize, delay_ms: u64) -> Self {
        Self { from, to, delay_ms }
    }

    pub fn send(&self, to: Arc<Process>, message: Message) {
        let delay = self.delay_ms;
        thread::spawn(move || {
            thread::sleep(Duration::from_millis(delay));
            to.receive(message);
        });
    }
}
