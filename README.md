# tiny-agent

Tiny Rust agent with persistent memory. Super simple, just to understand how it all works.

⚠️ This is solely for learning, not intended for any production use.

This project is a WIP.

### Quick start

```bash
cp .env.example .env
# Edit .env and add your Anthropic API key
cargo run
```

### V0: Bare-minimum CLI agent

#### What it does

A REPL (Read-Eval-Print Loop) that reads a prompt from stdin, sends it to the Anthropic Messages API, 
and prints the response. Single-turn only: no conversation history, no system prompt, no memory.

#### Architecture

```
┌─────────────────────────────────────────────────────┐
│                    REPL Loop                        │
│                                                     │
│  ┌───────────┐    ┌───────────┐    ┌─────────────┐  │
│  │  Read     │───▶│  Build    │───▶│  Call       │  │
│  │  user     │    │  API      │    │  Anthropic  │  │
│  │  input    │    │  request  │    │  API (HTTP) │  │
│  └───────────┘    └───────────┘    └──────┬──────┘  │
│                                           │         │
│  ┌───────────┐    ┌───────────┐           │         │
│  │  Print    │◀── │ Extract   │◀──────────┘         │
│  │  to       │    │ reply     │                     │
│  │  stdout   │    │ from JSON │                     │
│  └───────────┘    └───────────┘                     │
│       │                                             │
│       └──────── loop back to "Read user input" ─────│
└─────────────────────────────────────────────────────┘
```
