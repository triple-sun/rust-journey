# Rust Learning Checklist

Progress tracker for [LEARNING_PLAN.md](./LEARNING_PLAN.md).

---

## Block 1: Getting Started & Foundations
> Rust Book chapters: 1, 2, 3

### Exercises
- [v] Set up Rust environment (`rustc --version`, `cargo --version`)
- [v] Complete the guessing game (Ch.2)
- [v] Fahrenheit ↔ Celsius converter
- [v] Nth Fibonacci number
- [v] "Twelve Days of Christmas" with loops

### Continuous Project: Mini-Redis (Milestone 1)
- [v] Initialize a new project: `cargo new mini-redis`
- [v] Create a `loop { ... }` in `main.rs` that prompts the user for input.
- [v] Read standard input using `std::io::stdin().read_line(&mut buf)`.
- [v] Use `.trim()` to remove trailing newlines and `.to_uppercase()` to normalize commands.
- [v] Echo the input back to the console to confirm it works.
- [v] Add a quit condition: break the loop if the text is `"QUIT"` or `"EXIT"`.

### Checkpoint
- [v] Can create, build, and run projects with `cargo`
- [v] Understand `let` vs `let mut`
- [v] Can write functions with parameters and return values
- [v] Comfortable with `if/else`, `loop`, `while`, `for`

---

## Block 2: Ownership
> Rust Book chapter: 4

### Exercises
- [v] Function that takes and returns ownership of a `String`
- [v] Function that borrows `&String` and calculates length
- [v] Intentionally violate borrowing rules — read compiler errors
- [v] `first_word(s: &str) -> &str` function
- [v] Experiment with `String` vs `&str`

### Continuous Project: Mini-Redis (Milestone 2)
- [v] Write a parser function that takes a string slice (`&str`).
- [v] Use the `.split_whitespace()` iterator to extract parts of the string without allocating new memory.
- [v] Identify the first word as the command (e.g., `SET`, `GET`) and subsequent words as arguments (key, value).
- [v] Handle malformed input (like `SET key_without_value`) gracefully without panicking.
- [v] Print the sliced out components: e.g., `println!("CMD: {}, KEY: {}", cmd, key)`.

### Checkpoint
- [v] Can explain the three ownership rules
- [v] Understand move semantics (`String` vs `i32`)
- [v] Can write functions that borrow without taking ownership
- [v] Understand `String` vs `&str` difference

---

## Block 3: Structuring Data & Code
> Rust Book chapters: 5, 6, 7

### Exercises
- [v] `Rectangle` struct with `area()`, `perimeter()`, `can_hold()`
- [v] `IpAddr` enum with `V4` and `V6` variants
- [v] Function on `Option<i32>` that doubles value or returns `None`
- [v] Mini library with `pub` module structure
- [v] Practice `use` to bring items into scope

### Continuous Project: Mini-Redis (Milestone 3)
- [ ] Create a new file `src/command.rs`.
- [ ] Define an enum: `pub enum Command { Set(String, String), Get(String), Unknown }`.
- [ ] Move your parsing logic into a function like `pub fn parse(input: &str) -> Command`.
- [ ] In `main.rs`, use a `match` statement on the returned `Command` to decide whether to echo back, get data, or print an error message.

### Checkpoint
- [v] Can define structs and implement methods
- [v] Can create enums with data-carrying variants
- [v] Can use `match`, `if let`, `let else`
- [v] Understand `pub`, `mod`, `use`

---

## Block 4: Collections, Error Handling & Generics
> Rust Book chapters: 8, 9, 10

### Exercises
- [v] Mean, median, mode from a list of integers
- [ ] Pig Latin converter
- [ ] Employee department tracker CLI
- [ ] Generic `largest<T: PartialOrd>` function
- [ ] `Summary` trait with `NewsArticle` and `Tweet` implementations
- [ ] Lifetime annotations: return longer of two `&str`

### Continuous Project: Mini-Redis (Milestone 4)
- [ ] Instantiate `let mut store: HashMap<String, String> = HashMap::new();` before your REPL loop.
- [ ] Implement the execution of `Command::Set` (inserting into the `HashMap`) and `Command::Get` (retrieving).
- [ ] Mimic Redis behavior: if `GET` succeeds, print the value. If it fails, print `(nil)`.
- [ ] Create a custom error enum if needed (e.g., `enum DbError { KeyNotFound, InvalidCommand }`) and have your parser and executor return `Result<T, DbError>`.

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

### Continuous Project: Mini-Redis (Milestone 5)
- [ ] In your `Set` command execution, after updating the `HashMap`, append the command to a `db.log` file using `std::fs::OpenOptions` in `append` mode.
- [ ] Modify your startup sequence: before entering the REPL loop, read `db.log` line-by-line and populate your `HashMap` to recover previous state.
- [ ] Write unit tests (`#[test]`) for your command parser (e.g., making sure `"SET name alice"` turns into `Command::Set("name", "alice")`) and for the hashmap operations.

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

### Continuous Project: Mini-Redis (Milestone 6)
- [ ] Prepare your DB for concurrency by wrapping it in `Arc<RwLock<HashMap<String, String>>>`.
- [ ] Refactor your code so that you `write()` lock the data structure for `SET` and `read()` lock it for `GET`.
- [ ] Add a background process: use `std::thread::spawn` to launch a thread that wakes up every N seconds. It should read lock the database, write out all current key-value pairs to a new clean log file, and overwrite the old `db.log` (compaction).

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

### Continuous Project: Mini-Redis (Milestone 7)
- [ ] Define a `trait Storage { fn set(&mut self, k: &str, v: &str); fn get(&self, k: &str) -> Option<String>; }`.
- [ ] Implement the `Storage` trait for your in-memory + WAL database.
- [ ] Create a `--mode memory-only` CLI flag. When passed, instantiate a purely in-memory map without file logging. Use trait objects (`Box<dyn Storage>`) to swap between the two implementations seamlessly at runtime.
- [ ] Use advanced pattern matching with match guards to handle special commands like `SET foo bar EX 60` (expiration in 60 seconds).

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

### Continuous Project: Mini-Redis (Milestone 8)
- [ ] Add the `tokio` crate to your dependencies. Change your main function to `#[tokio::main] async fn main()`.
- [ ] Replace your standard input REPL loop with `tokio::net::TcpListener::bind("127.0.0.1:6379").await;`.
- [ ] Use `listener.accept().await` in a loop. For every connected client socket, use `tokio::spawn` to handle its commands concurrently.
- [ ] Replace your `std::sync::RwLock` with `tokio::sync::RwLock` so async tasks don't block OS threads while waiting for a lock.
- [ ] Test it by connecting via a tool like `netcat` (`nc 127.0.0.1 6379`) or a standard Redis client!

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
