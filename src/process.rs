use std::sync::{Arc, Mutex};
use std::time::Duration;
use std::thread;
use std::collections::VecDeque;

use crate::context::Context;
use crate::message::{Message, MessageType};
use crate::state::NETWORK;

pub struct Process {
    pub id: usize,
    pub context: Arc<Mutex<Context>>,
    pub inbox: Arc<Mutex<VecDeque<Message>>>,
    pub expected_sync_replies: usize,
}

impl Process {
    pub fn new(id: usize, expected_sync_replies: usize) -> Self {
        Self {
            id,
            context: Arc::new(Mutex::new(Context::new())),
            inbox: Arc::new(Mutex::new(VecDeque::new())),
            expected_sync_replies,
        }
    }

    pub fn receive(&self, message: Message) {
        let mut queue = self.inbox.lock().unwrap();
        queue.push_back(message);
    }

    pub fn run(self: Arc<Self>) {
        let inbox = Arc::clone(&self.inbox);
        let context = Arc::clone(&self.context);
        let id = self.id;
        let expected_replies = self.expected_sync_replies;

        thread::spawn(move || loop {
            // 🔄 Тикаем локальное время
            {
                let mut ctx = context.lock().unwrap();
                ctx.tick();
            }

            let mut queue = inbox.lock().unwrap();
            if let Some(msg) = queue.pop_front() {
                match msg.payload {
                    MessageType::Ping => {
                        println!("[Process {}] Ping от процесса {}", id, msg.from);
                    }
                    MessageType::Text(text) => {
                        println!("[Process {}] Текст от {}: {}", id, msg.from, text);
                    }
                    MessageType::SyncRequest => {
                        println!("[Process {}] Получен SyncRequest от {}", id, msg.from);
                        let current_time = context.lock().unwrap().local_time;

                        if let Some(net) = NETWORK.lock().unwrap().as_ref() {
                            let reply = Message {
                                from: id,
                                to: msg.from,
                                payload: MessageType::SyncReply { time: current_time },
                            };
                            net.send(id, msg.from, reply);
                        }
                    }
                    MessageType::SyncReply { time } => {
                        println!("[Process {}] Получен SyncReply от {}: {}", id, msg.from, time);

                        if id == 1 {
                            let mut ctx = context.lock().unwrap();
                            ctx.sync_replies.push(time);

                            if ctx.sync_replies.len() == expected_replies {
                                let sum: u64 = ctx.sync_replies.iter().sum();
                                let avg = sum / ctx.sync_replies.len() as u64;
                                let correction = avg as i64 - ctx.local_time as i64;

                                println!(
                                    "[Process {}] Коррекция времени: {} → +{}",
                                    id,
                                    ctx.local_time,
                                    correction
                                );

                                ctx.adjust_time(correction);

                                println!(
                                    "[Process {}] Новое локальное время: {} (offset = {})",
                                    id,
                                    ctx.local_time,
                                    ctx.offset
                                );

                                ctx.sync_replies.clear();
                            }
                        }
                    }
                }
            }

            drop(queue);
            thread::sleep(Duration::from_millis(500));
        });
    }

    /// Опционально: вывод состояния процесса
    pub fn print_status(&self) {
        let ctx = self.context.lock().unwrap();
        let full_time = ctx.local_time as i64 + ctx.offset;
        println!(
            "[Process {}] local_time = {}, offset = {}, full = {}",
            self.id, ctx.local_time, ctx.offset, full_time
        );
    }


}
