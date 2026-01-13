use std::fmt;

#[derive(Debug, Clone)]
pub struct Submission {
    pub id: u64,
    pub user_input: String,
}

#[derive(Debug, Clone)]
pub enum Event {
    SessionConfigured {
        session_id: String,
    },
    TurnStarted {
        submission_id: u64,
    },
    ReasoningDelta {
        submission_id: u64,
        text: String,
    },
    AssistantDelta {
        submission_id: u64,
        text: String,
    },
    ToolCall {
        submission_id: u64,
        name: String,
        args: String,
    },
    ToolResult {
        submission_id: u64,
        output: String,
    },
    TurnCompleted {
        submission_id: u64,
    },
}

impl fmt::Display for Event {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Event::SessionConfigured { session_id } => {
                write!(f, "SessionConfigured session_id={session_id}")
            }
            Event::TurnStarted { submission_id } => {
                write!(f, "TurnStarted submission_id={submission_id}")
            }
            Event::ReasoningDelta {
                submission_id,
                text,
            } => {
                write!(f, "ReasoningDelta submission_id={submission_id} text={text}")
            }
            Event::AssistantDelta {
                submission_id,
                text,
            } => {
                write!(f, "AssistantDelta submission_id={submission_id} text={text}")
            }
            Event::ToolCall {
                submission_id,
                name,
                args,
            } => {
                write!(
                    f,
                    "ToolCall submission_id={submission_id} name={name} args={args}"
                )
            }
            Event::ToolResult {
                submission_id,
                output,
            } => {
                write!(
                    f,
                    "ToolResult submission_id={submission_id} output={output}"
                )
            }
            Event::TurnCompleted { submission_id } => {
                write!(f, "TurnCompleted submission_id={submission_id}")
            }
        }
    }
}
