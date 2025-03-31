use std::sync::Arc;
use std::thread;
use std::time::Duration;

use distributed_system_simul::process::Process;
use distributed_system_simul::message::{Message, MessageType};
use distributed_system_simul::network::Network;
use distributed_system_simul::state::NETWORK;

fn main() {
    let total_processes = 5;
    let initiator_id = 1;
    let expected_replies = total_processes - 1;


    let mut processes = Vec::new();
    for id in 1..=total_processes {
        let expected = if id == initiator_id { expected_replies } else { 0 };
        processes.push(Arc::new(Process::new(id, expected)));
    }

    for process in &processes {
        process.clone().run();
    }

    let mut net = Network::new();
    for process in &processes {
        net.add_process(Arc::clone(process));
    }

    for from in 1..=total_processes {
        for to in 1..=total_processes {
            if from != to {
                net.add_channel(from, to, 1000);
            }
        }
    }

    *NETWORK.lock().unwrap() = Some(net);

    if let Some(net) = NETWORK.lock().unwrap().as_ref() {
        net.broadcast(initiator_id, |to| Message {
            from: initiator_id,
            to,
            payload: MessageType::SyncRequest,
        });
    }


    thread::sleep(Duration::from_secs(10));
}
