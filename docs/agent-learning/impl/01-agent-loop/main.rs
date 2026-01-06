mod agent;
mod protocol;

use std::env;
use std::sync::mpsc::channel;
use std::thread;

use agent::Agent;
use protocol::Submission;

fn main() {
    let input = env::args().skip(1).collect::<Vec<_>>().join(" ");
    let user_input = if input.is_empty() {
        "hello from codex learning".to_string()
    } else {
        input
    };

    let (tx_sub, rx_sub) = channel();
    let (tx_event, rx_event) = channel();

    let agent = Agent::new("sess-001".to_string());
    let handle = thread::spawn(move || {
        agent.run(rx_sub, tx_event);
    });

    let submission = Submission { id: 1, user_input };
    let _ = tx_sub.send(submission);
    drop(tx_sub);

    while let Ok(event) = rx_event.recv() {
        println!("{event}");
    }

    let _ = handle.join();
}
