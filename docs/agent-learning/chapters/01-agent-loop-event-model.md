# Chapter 1: Agent Loop and Event Model

## Goal
Understand how Codex represents a conversation turn as a stream of events, and how the core agent loop dispatches work (model calls, tool calls, sandboxed exec) while emitting those events.

## Core concepts
- Submission/Event queue: UI or caller submits work; core emits events describing progress and outputs.
- Conversation lifecycle: configure session -> turns -> items -> completion.
- Event mapping: model/tool outputs map into protocol events consumed by UI or app-server.
- Reasoning vs assistant output: separate deltas for chain-of-thought vs final response.

## Key code map (start here)
- Agent core loop: codex-rs/core/src/codex.rs
- Conversation orchestration: codex-rs/core/src/conversation_manager.rs
- Conversation wrapper: codex-rs/core/src/codex_conversation.rs
- Event mapping helpers: codex-rs/core/src/event_mapping.rs
- Stream handling: codex-rs/core/src/stream_events_utils.rs
- Session state machine: codex-rs/core/src/state/
- Tasks: codex-rs/core/src/tasks/
- Tool routing (agent side): codex-rs/core/src/tools/
- Protocol event types: codex-rs/protocol/src/protocol.rs
- Turn items & history types: codex-rs/protocol/src/items.rs
- User input types: codex-rs/protocol/src/user_input.rs

## Reading order (suggested)
1) codex-rs/core/src/codex.rs
2) codex-rs/core/src/conversation_manager.rs
3) codex-rs/core/src/codex_conversation.rs
4) codex-rs/protocol/src/protocol.rs
5) codex-rs/core/src/stream_events_utils.rs
6) codex-rs/core/src/event_mapping.rs
7) codex-rs/core/src/state/ and codex-rs/core/src/tasks/

## Data flow sketch
- UI (tui/exec/app-server) creates a conversation.
- Core emits SessionConfigured event.
- For each user input, UI submits a Submission/Op; core generates model requests.
- Model responses are streamed into ResponseEvent -> mapped into protocol events.
- Tool calls trigger tool routing; tool outputs are reported back as events.
- Turn is finalized and core emits turn completion + token usage.

## Questions to answer after reading
- What are the first events emitted after a conversation is spawned?
- Where is the boundary between protocol types and core logic?
- How are tool calls represented in the event stream?
- Which component owns the event queue and how is backpressure handled?

## Exercises (verification)
- Trace a single turn from submission to final event:
  - Identify the function that receives the submission.
  - Identify the function that turns model stream events into protocol events.
  - Identify where tool output is formatted for the model.
- Draw a quick sequence diagram from codex.rs and protocol.rs.
- Run the minimal agent loop: `docs/agent-learning/impl/01-agent-loop` (see README).

## Notes
- Keep a running glossary of terms: Submission, Event, Turn, Item, ToolCall.
