# Chapter 1 Impl: Minimal Agent Loop (Rust)

Goal: implement a tiny agent loop that accepts a Submission and emits a stream of Events.
This mirrors the Codex core pattern (Submission -> Event queue) without external dependencies.

## What this exercise covers
- A queue-based agent core (Submission -> Event)
- A turn lifecycle: SessionConfigured -> TurnStarted -> Item* -> TurnCompleted
- A simple "model" that streams tokens
- A tool call that executes a local command (simulated)

## Files
- main.rs: minimal CLI runner
- agent.rs: agent loop + event model
- protocol.rs: minimal protocol types

## Run
```
cd docs/agent-learning/impl/01-agent-loop
rustc main.rs -o agent_demo
./agent_demo "hello"
```

## Expected behavior
- Prints events in order (session configured, turn started, reasoning deltas, assistant deltas, tool, completed).

## Next
- Add a real shell exec and output truncation in Chapter 2.
