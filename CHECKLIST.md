# Rust Learning Checklist

Progress tracker for [LEARNING_PLAN.md](./LEARNING_PLAN.md).

---

## Block 1: Getting Started & Foundations
> Rust Book chapters: 1, 2, 3

### Exercises
- [v] Set up Rust environment (`rustc --version`, `cargo --version`)
- [v] Complete the guessing game (Ch.2)
- [ ] Fahrenheit ↔ Celsius converter
- [v] Nth Fibonacci number
- [ ] "Twelve Days of Christmas" with loops

### Checkpoint
- [v] Can create, build, and run projects with `cargo`
- [v] Understand `let` vs `let mut`
- [v] Can write functions with parameters and return values
- [ ] Comfortable with `if/else`, `loop`, `while`, `for`

---

## Block 2: Ownership
> Rust Book chapter: 4

### Exercises
- [ ] Function that takes and returns ownership of a `String`
- [ ] Function that borrows `&String` and calculates length
- [ ] Intentionally violate borrowing rules — read compiler errors
- [ ] `first_word(s: &str) -> &str` function
- [ ] Experiment with `String` vs `&str`

### Checkpoint
- [ ] Can explain the three ownership rules
- [ ] Understand move semantics (`String` vs `i32`)
- [ ] Can write functions that borrow without taking ownership
- [ ] Understand `String` vs `&str` difference

---

## Block 3: Structuring Data & Code
> Rust Book chapters: 5, 6, 7

### Exercises
- [ ] `Rectangle` struct with `area()`, `perimeter()`, `can_hold()`
- [ ] `IpAddr` enum with `V4` and `V6` variants
- [ ] Function on `Option<i32>` that doubles value or returns `None`
- [ ] Mini library with `pub` module structure
- [ ] Practice `use` to bring items into scope

### Checkpoint
- [ ] Can define structs and implement methods
- [ ] Can create enums with data-carrying variants
- [ ] Can use `match`, `if let`, `let else`
- [ ] Understand `pub`, `mod`, `use`

---

## Block 4: Collections, Error Handling & Generics
> Rust Book chapters: 8, 9, 10

### Exercises
- [ ] Mean, median, mode from a list of integers
- [ ] Pig Latin converter
- [ ] Employee department tracker CLI
- [ ] Generic `largest<T: PartialOrd>` function
- [ ] `Summary` trait with `NewsArticle` and `Tweet` implementations
- [ ] Lifetime annotations: return longer of two `&str`

### Checkpoint
- [ ] Fluent with `Vec`, `String`, `HashMap`
- [ ] Use `Result<T, E>` and `?` for error handling
- [ ] Can write generics with trait bounds
- [ ] Understand lifetime annotations

---

## Block 5: Functional Patterns, Testing & I/O
> Rust Book chapters: 11, 12, 13

### Exercises
- [ ] Unit and integration tests for `Rectangle`
- [ ] Build `minigrep` (Ch.12)
- [ ] Refactor `minigrep` to use iterators
- [ ] Closure that filters `Vec<String>` by prefix
- [ ] Iterator-based function returning only primes

### Checkpoint
- [ ] Can write and run tests with `cargo test`
- [ ] Completed `minigrep`
- [ ] Can write closures and use iterator methods
- [ ] Understand lazy evaluation and iterator chains

---

## Block 6: Smart Pointers, Concurrency & Unsafe
> Rust Book chapters: 14, 15, 16, 20

### Exercises
- [ ] Recursive cons list with `Box<T>`
- [ ] Shared list tail with `Rc<T>`
- [ ] 10 threads incrementing `Arc<Mutex<i32>>`
- [ ] Producer-consumer with `mpsc::channel`
- [ ] Unsafe raw pointer dereference — then make it safe
- [ ] Cargo workspace: library + binary crate

### Checkpoint
- [ ] Can choose between `Box`, `Rc`, `Arc`
- [ ] Can spawn threads with `Mutex`/`Arc` for shared state
- [ ] Can use channels for thread communication
- [ ] Understand `unsafe` scope and justifications

---

## Block 7: Advanced Rust
> Rust Book chapters: 17, 18, 19, 21

### Exercises
- [ ] `Draw` trait with `Vec<Box<dyn Draw>>` dynamic dispatch
- [ ] State pattern: `Draft → PendingReview → Published`
- [ ] Complex pattern matching with `@` bindings and guards
- [ ] `Add` trait for custom `Point` struct
- [ ] `macro_rules! my_vec` declarative macro

### Checkpoint
- [ ] Can use trait objects for dynamic dispatch
- [ ] Can apply complex pattern matching
- [ ] Understand associated types, operator overloading, newtype pattern
- [ ] Basic understanding of declarative macros

---

## Block 8: Async Rust & Ecosystem
> Rust Book chapter: 17 (Async), Tokio tutorial

### Exercises
- [ ] Async URL fetch with `reqwest`
- [ ] TCP echo server with `tokio::net::TcpListener`
- [ ] `tokio::select!` racing two async operations
- [ ] Serialize/deserialize with `serde_json`
- [ ] CLI tool with `clap` + async URL fetch

### Checkpoint
- [ ] Can write async functions with `tokio`
- [ ] Can build a basic TCP server/client
- [ ] Can use `serde` for JSON
- [ ] Familiar with key ecosystem crates

---

## Final Project 1: Chat Application

### Milestone 1: Server Core
- [ ] Set up cargo workspace (`server`, `tui-client`, `shared`)
- [ ] Define shared message types in `shared` crate
- [ ] Build axum server with WebSocket upgrade
- [ ] Manage connected clients with `Arc<Mutex<HashMap>>`
- [ ] Broadcast messages to all clients
- [ ] Handle user join/leave lifecycle
- [ ] Add `tracing` logging

### Milestone 2: Message Persistence
- [ ] Add SQLite with `sqlx`
- [ ] Create `users` and `messages` tables
- [ ] Store messages; load history on join
- [ ] Run migrations at startup

### Milestone 3: Terminal Client (TUI)
- [ ] WebSocket connection with `tokio-tungstenite`
- [ ] Split-pane UI with `ratatui`
- [ ] CLI args with `clap` (URL, username)
- [ ] Real-time message display
- [ ] Commands: `/users`, `/quit`
- [ ] Graceful Ctrl+C shutdown

### Milestone 4: Web Client
- [ ] Serve static HTML/CSS/JS from axum
- [ ] Browser WebSocket connection
- [ ] Functional UI: messages, input, user list
- [ ] Display history on connect
- [ ] Join/leave notifications

### Milestone 5: Polish
- [ ] Message timestamps
- [ ] Reconnection logic in both clients
- [ ] Integration tests (server + 2 clients)
- [ ] Unit tests for serialization and logic

### Verification
- [ ] Both clients can exchange messages
- [ ] Message history survives server restart
- [ ] 10+ simultaneous clients work
- [ ] `cargo test --workspace` passes

---

## Final Project 2: ESP32 Telegram Notification Display

### Milestone 1: Dev Environment
- [ ] Install `espup` and ESP32 Rust toolchain
- [ ] Create project with `cargo generate esp-rs/esp-idf-template`
- [ ] Flash "Hello" to serial output
- [ ] Blink LED with `esp-idf-hal` GPIO

### Milestone 2: WiFi Connection
- [ ] Connect to WiFi with `esp-idf-svc::wifi`
- [ ] Store credentials in NVS
- [ ] Retry logic with backoff
- [ ] LED status indicator (solid = connected, blink = reconnecting)

### Milestone 3: Display Driver
- [ ] Wire up display (SSD1306/ST7789)
- [ ] Initialize display driver
- [ ] Draw text with `embedded-graphics`
- [ ] Helper functions: `clear_screen()`, `draw_text()`, `draw_notification()`
- [ ] Show WiFi status on display

### Milestone 4: Telegram Bot Integration
- [ ] Create bot via BotFather, get API token
- [ ] HTTP polling of `getUpdates`
- [ ] Parse JSON into Rust structs with `serde`
- [ ] Track `last_update_id` for deduplication
- [ ] Store bot token in NVS

### Milestone 5: Integration
- [ ] Main loop: poll → parse → display
- [ ] Show sender name + message text
- [ ] Scroll/rotate multiple messages
- [ ] Graceful error handling (network, JSON, display)
- [ ] LED indicator for new notifications
- [ ] WiFi reconnect and resume polling

### Milestone 6: Polish
- [ ] Startup splash screen
- [ ] Timestamps on notifications
- [ ] Message length truncation/scrolling
- [ ] "No new messages" idle screen
- [ ] Optional: button to cycle/dismiss notifications

### Verification
- [ ] Telegram message → appears on ESP32 display
- [ ] WiFi disconnect → auto-reconnect and resume
- [ ] Different WiFi credentials via NVS work
- [ ] Stable for 1+ hours without crash
