# Pulse — Real-Time Event Fanout & Social Feed Engine (Rust)

Pulse is a high-performance real-time backend written in **Rust** that simulates the core infrastructure behind social media feeds, notifications, and system alerts.

It ingests domain events over HTTP, routes them through an internal broker, and fans them out to thousands of WebSocket clients in real time with **backpressure control, batching, priority queues, graceful shutdown, and metrics**.

---

## Core Concepts

- **Events**: Domain messages such as posts, notifications, and system alerts.
- **Topics**: Logical routing keys (e.g. `feed:user-123`, `notifications:user-42`).
- **Broker**: Internal async queue that decouples ingestion from fanout.
- **WebSocket Hub**: Manages subscriptions and performs fanout with backpressure.
- **Simulator (pulse-sim)**: Generates realistic traffic and concurrent users.
- **Metrics**: Tracks received, delivered, dropped events, and WS connections.

---

## Features

### Real-Time WebSocket Fanout
- Thousands of concurrent WebSocket clients
- Topic-based subscriptions (`feed:<user_id>`)
- Batched delivery for efficiency

### Backpressure & Drop Policy
- Bounded per-subscriber channels
- Non-blocking delivery for normal priority events
- Guaranteed delivery for critical system alerts
- Slow consumers are detected and dropped

### Priority-Aware Routing
Events carry semantic priority:
- `PostCreated` → Normal priority
- `Notification` → Normal priority
- `SystemAlert` → Critical priority

Critical events bypass drop policies to ensure delivery.

### Event Batching
- Events are buffered and flushed every 50ms
- Reduces syscall overhead and improves throughput
- Mimics real-world feed batching

### Graceful Shutdown
- Broker workers listen for shutdown signals
- WebSocket connections close cleanly
- Prevents orphaned tasks and partial sends

### Metrics & Observability
- `/metrics` endpoint exposes:
  - events received
  - events delivered
  - events dropped
  - active WebSocket connections
- Structured logs using `tracing`

### Load Simulation (pulse-sim)
- Thousands of simulated users
- Concurrent WebSocket connections
- Realistic traffic mix:
  - 70% posts
  - 20% notifications
  - 10% system alerts
- Traffic shaping (normal load + spikes)
- Final metrics snapshot after simulation

---

## How to Run

```bash
cd pulse
cargo run
```
### Run the Simulator

```bash
cd pulse-sim
cargo run
```
