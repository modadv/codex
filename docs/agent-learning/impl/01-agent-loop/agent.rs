use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;
use std::thread;
use std::time::Duration;

use crate::protocol::Event;
use crate::protocol::Submission;

pub struct Agent {
    session_id: String,
}

impl Agent {
    pub fn new(session_id: String) -> Self {
        Self { session_id }
    }

    pub fn run(self, rx_sub: Receiver<Submission>, tx_event: Sender<Event>) {
        let session_id = self.session_id.clone();
        let _ = tx_event.send(Event::SessionConfigured { session_id });

        while let Ok(submission) = rx_sub.recv() {
            let submit_id = submission.id;
            let _ = tx_event.send(Event::TurnStarted {
                submission_id: submit_id,
            });

            // Simulated model streaming.
            for token in fake_model_stream(&submission.user_input) {
                let _ = tx_event.send(Event::ModelDelta {
                    submission_id: submit_id,
                    text: token,
                });
                thread::sleep(Duration::from_millis(50));
            }

            // Simulated tool call.
            let user_input = &submission.user_input;
            let tool_args = format!("cmd=echo input:{user_input}");
            let _ = tx_event.send(Event::ToolCall {
                submission_id: submit_id,
                name: "shell".to_string(),
                args: tool_args,
            });

            let output = format!("ok: echoed '{user_input}'");
            let _ = tx_event.send(Event::ToolResult {
                submission_id: submit_id,
                output,
            });

            let _ = tx_event.send(Event::TurnCompleted {
                submission_id: submit_id,
            });
        }
    }
}

fn fake_model_stream(input: &str) -> Vec<String> {
    let mut out = Vec::new();
    out.push("thinking:".to_string());
    for word in input.split_whitespace() {
        out.push(format!(" {word}"));
    }
    out.push(" -> done".to_string());
    out
}
