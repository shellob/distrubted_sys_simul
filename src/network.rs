use std::collections::HashMap;
use std::sync::Arc;

use crate::process::Process;
use crate::message::Message;
use crate::channel::Channel;

pub struct Network {
    processes: HashMap<usize, Arc<Process>>,
    channels: Vec<Channel>,
}

impl Network {
    pub fn new() -> Self {
        Self {
            processes: HashMap::new(),
            channels: Vec::new(),
        }
    }

    pub fn add_process(&mut self, process: Arc<Process>) {
        self.processes.insert(process.id, process);
    }

    pub fn add_channel(&mut self, from: usize, to: usize, delay_ms: u64) {
        self.channels.push(Channel::new(from, to, delay_ms));
    }

    pub fn send(&self, from: usize, to: usize, message: Message) {
        if let Some(channel) = self.channels.iter().find(|c| c.from == from && c.to == to) {
            if let Some(receiver) = self.processes.get(&to) {
                channel.send(Arc::clone(receiver), message);
            } else {
                println!("Получатель процесса {} не найден", to);
            }
        } else {
            println!("Канал от {} к {} не найден", from, to);
        }
    }

    pub fn broadcast(&self, from: usize, message_builder: impl Fn(usize) -> Message) {
        for channel in self.channels.iter().filter(|c| c.from == from) {
            if let Some(receiver) = self.processes.get(&channel.to) {
                let msg = message_builder(channel.to);
                channel.send(Arc::clone(receiver), msg);
            }
        }
    }

    pub fn export_dot(&self) -> String {
        let mut output = String::new();
        output.push_str("digraph G {\n");

        for process in self.processes.values() {
            output.push_str(&format!("    {} [label=\"P{}\"];\n", process.id, process.id));
        }

        for channel in &self.channels {
            output.push_str(&format!(
                "    {} -> {} [label=\"{}ms\"];\n",
                channel.from, channel.to, channel.delay_ms
            ));
        }

        output.push_str("}\n");
        output
    }
}
