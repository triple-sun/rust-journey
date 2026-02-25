# Rust Learning Plan

A structured learning plan based on [The Rust Programming Language](https://doc.rust-lang.org/book/) book (2024 Edition).

**Goal:** Learn Rust from zero to building two real-world projects:
1. A **chat web application** (HTTP server + web client + terminal client)
2. **ESP32 firmware** that displays Telegram notifications

**How to use this plan:** Each block is self-contained. Work through them in order. Every block has:
- **Topics** — what you'll learn (mapped to Rust Book chapters)
- **Key concepts** — the core ideas to focus on
- **Exercises** — practice tasks to solidify understanding
- **Checkpoint** — what you should be able to do before moving on

---

## Block 1: Getting Started & Foundations

**Rust Book chapters:** 1, 2, 3

### Topics
- Installing Rust (`rustup`, `cargo`, `rustc`)
- Hello World and Hello Cargo
- The guessing game (guided project from Ch.2)
- Variables, mutability, shadowing
- Scalar types: integers, floats, booleans, characters
- Compound types: tuples, arrays
- Functions, statements, expressions
- Control flow: `if`, `loop`, `while`, `for`
- Comments

### Key Concepts
- Rust is compiled, not interpreted — understand the compile-check-run cycle
- `cargo` is both build tool and package manager
- Variables are immutable by default — mutability is opt-in with `mut`
- Expressions vs. statements (expressions return values, statements don't)
- Type inference exists but explicit annotations are sometimes required

### Exercises
1. Set up your Rust environment. Run `rustc --version` and `cargo --version`
2. Complete the guessing game from Chapter 2
3. Write a function that converts Fahrenheit ↔ Celsius
4. Generate the nth Fibonacci number
5. Print the lyrics to "The Twelve Days of Christmas" using loops

### Continuous Project: Mini-Redis (Milestone 1)
**Goal: A basic Read-Eval-Print Loop (REPL).**
- Initialize a new project: `cargo new mini-redis`
- Create a `loop { ... }` in `main.rs` that prompts the user for input.
- Read standard input using `std::io::stdin().read_line(&mut buf)`.
- Use `.trim()` to remove trailing newlines and `.to_uppercase()` to normalize commands.
- Echo the input back to the console to confirm it works.
- Add a quit condition: break the loop if the text is `"QUIT"` or `"EXIT"`.

### Checkpoint
- [x] You can create a new project with `cargo new`, build with `cargo build`, run with `cargo run`
- [x] You understand the difference between `let` and `let mut`
- [x] You can write functions with parameters and return values
- [x] You can use `if/else`, `loop`, `while`, `for` comfortably

---

## Block 2: Ownership — Rust's Core Idea

**Rust Book chapters:** 4

### Topics
- What is ownership?
- The stack and the heap
- Ownership rules
- Move semantics
- Clone and Copy
- References and borrowing
- Mutable vs. immutable references
- The slice type (`&str`, `&[T]`)
- Dangling references (and why Rust prevents them)

### Key Concepts
- Each value has exactly one owner; when the owner goes out of scope, the value is dropped
- Assignment of heap data **moves** ownership (not a shallow copy)
- References allow borrowing without taking ownership
- You can have either ONE mutable reference OR any number of immutable references (not both)
- Slices are references to a contiguous sequence of elements

### Exercises
1. Write a function that takes ownership of a `String` and returns it back
2. Write a function that takes a `&String` reference and calculates its length
3. Intentionally write code that violates borrowing rules — read and understand the compiler errors
4. Write a function `first_word(s: &str) -> &str` that returns the first word of a string
5. Experiment with `String` vs `&str` — when do you use which?

### Continuous Project: Mini-Redis (Milestone 2)
**Goal: Parse text safely using references without cloning data.**
- Write a parser function that takes a string slice (`&str`).
- Use the `.split_whitespace()` iterator to extract parts of the string without allocating new memory.
- Identify the first word as the command (e.g., `SET`, `GET`) and subsequent words as arguments (key, value).
- Handle malformed input (like `SET key_without_value`) gracefully without panicking.
- Print the sliced out components: e.g., `println!("CMD: {}, KEY: {}", cmd, key)`.

### Checkpoint
- [v] You can explain the three ownership rules from memory
- [v] You understand why `let s2 = s1;` invalidates `s1` for `String` but not for `i32`
- [v] You can write functions that borrow data without taking ownership
- [v] You understand the difference between `String` and `&str`

---

## Block 3: Structuring Data & Code

**Rust Book chapters:** 5, 6, 7

### Topics
- Defining and instantiating structs
- Methods and associated functions (`impl` blocks)
- Tuple structs, unit structs
- Enums and pattern matching (`match`)
- `Option<T>` — Rust's way of handling nullable values
- `if let` and `let else` for concise pattern matching
- Packages, crates, and modules
- Paths, `use`, and `pub`
- Separating modules into different files

### Key Concepts
- Structs group related data; `impl` blocks attach behavior to structs
- Enums can hold data in their variants — much more powerful than C-style enums
- `Option<T>` replaces null — forces you to handle the "no value" case explicitly
- `match` is exhaustive — the compiler ensures you handle every variant
- The module system controls visibility (`pub`) and organization

### Exercises
1. Create a `Rectangle` struct with `width` and `height`. Implement methods: `area()`, `perimeter()`, `can_hold(&self, other: &Rectangle) -> bool`
2. Create an `IpAddr` enum with `V4(u8, u8, u8, u8)` and `V6(String)` variants. Write a function that formats them as strings
3. Write a function that takes `Option<i32>` and doubles the value if present, returns `None` otherwise
4. Create a mini library with a `pub` module structure: `lib/network/server.rs`, `lib/network/client.rs`
5. Practice using `use` to bring module items into scope

### Continuous Project: Mini-Redis (Milestone 3)
**Goal: Model commands via enums and separate code into modules.**
- Create a new file `src/command.rs`.
- Define an enum: `pub enum Command { Set(String, String), Get(String), Unknown }`.
- Move your parsing logic into a function like `pub fn parse(input: &str) -> Command`.
- In `main.rs`, use a `match` statement on the returned `Command` to decide whether to echo back, get data, or print an error message.

### Checkpoint
- [v] You can define structs and implement methods on them
- [v] You can create enums with data-carrying variants
- [v] You can use `match`, `if let`, and `let else` for pattern matching
- [v] You understand `pub`, `mod`, and `use` for organizing code

---

## Block 4: Collections, Error Handling & Generics

**Rust Book chapters:** 8, 9, 10

### Topics
- `Vec<T>` — growable arrays
- `String` — the owned string type in depth
- `HashMap<K, V>` — key-value storage
- Unrecoverable errors with `panic!`
- Recoverable errors with `Result<T, E>`
- The `?` operator for error propagation
- When to panic vs. return `Result`
- Generic types in functions, structs, enums
- Traits: defining shared behavior
- Trait bounds and `impl Trait`
- Lifetimes: ensuring references are valid

### Key Concepts
- `Vec`, `String`, and `HashMap` are the three essential collections
- Rust has two error categories: `panic!` (unrecoverable) and `Result` (recoverable)
- The `?` operator is syntactic sugar for early return on `Err`
- Generics enable code reuse; traits define behavior; lifetimes connect reference validity to scopes
- `'a` lifetimes tell the compiler how long references must remain valid relative to each other

### Exercises
1. Given a list of integers: compute mean, median, and mode using `Vec` and `HashMap`
2. Convert a string to Pig Latin (first consonant goes to end + "ay", vowels get "hay")
3. Write a generic function `largest<T: PartialOrd>(list: &[T]) -> &T`
4. Define a `Summary` trait with a `summarize()` method. Implement it for `NewsArticle` and `Tweet` structs
5. Write a function with lifetime annotations that returns the longer of two string slices

### Continuous Project: Mini-Redis (Milestone 4)
**Goal: Actually store data in memory and handle errors.**
- Instantiate `let mut store: HashMap<String, String> = HashMap::new();` before your REPL loop.
- Implement the execution of `Command::Set` (inserting into the `HashMap`) and `Command::Get` (retrieving).
- Mimic Redis behavior: if `GET` succeeds, print the value. If it fails, print `(nil)`.
- Create a custom error enum if needed (e.g., `enum DbError { KeyNotFound, InvalidCommand }`) and have your parser and executor return `Result<T, DbError>`.

### Checkpoint
- [v] You can use `Vec`, `String`, `HashMap` fluently
- [v] You use `Result<T, E>` and `?` for error handling
- [v] You can write generic functions and structs with trait bounds
- [v] You understand lifetime annotations and when they're needed

---

## Block 5: Functional Patterns, Testing & I/O

**Rust Book chapters:** 11, 12, 13

### Topics
- Writing tests: `#[test]`, `assert!`, `assert_eq!`, `assert_ne!`
- Test organization: unit tests, integration tests
- Running tests: filtering, ignored tests, `cargo test`
- Building a CLI program: the `minigrep` project (Ch.12)
- Accepting command-line arguments
- Reading files
- Refactoring for modularity and error handling
- Using environment variables
- Writing to `stderr`
- Closures: anonymous functions that capture their environment
- Iterators and the `Iterator` trait
- Iterator adaptors: `map`, `filter`, `collect`, `fold`, etc.
- Performance: iterators vs. loops (zero-cost abstractions)

### Key Concepts
- Tests in Rust are first-class: `cargo test` discovers and runs them automatically
- The `minigrep` project teaches real-world code organization patterns
- Closures capture variables from their scope; they implement `Fn`, `FnMut`, or `FnOnce`
- Iterators are lazy — they don't do work until consumed
- Chaining iterator methods (`.iter().filter().map().collect()`) is idiomatic Rust

### Exercises
1. Write unit tests and integration tests for your `Rectangle` struct from Block 3
2. Build `minigrep` following Chapter 12 — implement case-insensitive search as shown
3. Refactor `minigrep` to use iterators instead of index-based loops
4. Write a closure that captures a `Vec<String>` and filters it by a prefix
5. Implement a function using iterators that takes a list of numbers and returns only primes

### Continuous Project: Mini-Redis (Milestone 5)
**Goal: Persistence via a Write-Ahead Log (WAL) and ensuring correctness with tests.**
- In your `Set` command execution, after updating the `HashMap`, append the command to a `db.log` file using `std::fs::OpenOptions` in `append` mode.
- Modify your startup sequence: before entering the REPL loop, read `db.log` line-by-line and populate your `HashMap` to recover previous state.
- Write unit tests (`#[test]`) for your command parser (e.g., making sure `"SET name alice"` turns into `Command::Set("name", "alice")`) and for the hashmap operations.

### Checkpoint
- [ ] You can write and run tests with `cargo test`
- [ ] You completed the `minigrep` project
- [ ] You can write closures and use them with iterator methods
- [ ] You understand lazy evaluation and iterator chains

---

## Block 6: Smart Pointers, Concurrency & Unsafe

**Rust Book chapters:** 14, 15, 16, 20

### Topics
- Cargo workspaces and publishing crates
- Documentation comments (`///`, `//!`)
- `cargo doc`
- `Box<T>` — heap allocation
- `Rc<T>` — reference counting
- `RefCell<T>` — interior mutability
- `Deref` and `Drop` traits
- Reference cycles and `Weak<T>`
- Threads: `std::thread::spawn`
- Message passing with channels (`mpsc`)
- Shared state with `Mutex<T>` and `Arc<T>`
- `Send` and `Sync` traits
- Unsafe Rust: raw pointers, unsafe functions, unsafe traits
- `extern` and FFI basics

### Key Concepts
- Smart pointers own the data they point to (`Box`, `Rc`, `Arc`)
- `Rc<T>` for single-threaded multiple ownership; `Arc<T>` for multi-threaded
- `Mutex<T>` provides interior mutability with locking; always use with `Arc<T>` for shared access across threads
- Channels (`mpsc`) are Rust's message-passing mechanism between threads
- `unsafe` doesn't turn off the borrow checker — it unlocks 5 specific superpowers (dereferencing raw pointers, calling unsafe functions, accessing mutable statics, implementing unsafe traits, accessing union fields)

### Exercises
1. Create a `Box<T>` recursive data structure: a cons list `enum List { Cons(i32, Box<List>), Nil }`
2. Use `Rc<T>` to create two lists that share a common tail
3. Spawn 10 threads that each increment a shared `Arc<Mutex<i32>>` counter
4. Build a simple producer-consumer using `mpsc::channel`: one thread produces numbers, another thread prints them
5. Write unsafe code that dereferences a raw pointer — then make it safe again
6. Set up a cargo workspace with two crates: a library crate and a binary crate that depends on it

### Continuous Project: Mini-Redis (Milestone 6)
**Goal: Prepare for a networked server by making the storage thread-safe.**
- Prepare your DB for concurrency by wrapping it in `Arc<RwLock<HashMap<String, String>>>`.
- Refactor your code so that you `write()` lock the data structure for `SET` and `read()` lock it for `GET`.
- Add a background process: use `std::thread::spawn` to launch a thread that wakes up every N seconds. It should read lock the database, write out all current key-value pairs to a new clean log file, and overwrite the old `db.log` (compaction).

### Checkpoint
- [ ] You can choose between `Box`, `Rc`, `Arc` based on use case
- [ ] You can spawn threads and use `Mutex`/`Arc` for shared state
- [ ] You can use channels for thread communication
- [ ] You understand what `unsafe` allows and when it's justified

---

## Block 7: Advanced Rust

**Rust Book chapters:** 17, 18, 19, 21

### Topics
- Trait objects and dynamic dispatch (`dyn Trait`)
- Object safety rules
- `impl Trait` in argument and return position
- The state pattern and OOP comparisons
- Pattern matching in depth: destructuring, guards, bindings
- `match`, `if let`, `while let`, `let` patterns
- Irrefutable vs. refutable patterns
- Advanced traits: associated types, default type parameters, operator overloading
- Fully qualified syntax for disambiguation
- Supertraits and the newtype pattern
- Advanced types: type aliases, `!` (never type), dynamically sized types
- Advanced functions: function pointers (`fn`), returning closures
- Macros: declarative macros with `macro_rules!` and procedural macros (overview)

### Key Concepts
- `dyn Trait` enables runtime polymorphism (dynamic dispatch) — used when you don't know the concrete type at compile time
- Patterns are everywhere in Rust: `match`, `let`, function parameters, `for` loops
- Associated types in traits (`type Output`) are preferred over generics when there should be only one implementation per type
- The newtype pattern (`struct Wrapper(Vec<String>)`) lets you implement external traits on external types
- Macros operate on syntax tokens — they're code that writes code

### Exercises
1. Implement a `Draw` trait with a `draw()` method. Create a `Screen` struct that holds `Vec<Box<dyn Draw>>` and calls `draw()` on each
2. Implement the state pattern: a `Post` that goes through `Draft → PendingReview → Published` states
3. Practice pattern matching: destructure nested structs, use `@` bindings and match guards
4. Implement `Add` trait to allow adding two custom `Point` structs with `+`
5. Write a simple declarative macro `macro_rules! my_vec` that mimics a simplified `vec![]`

### Continuous Project: Mini-Redis (Milestone 7)
**Goal: Advanced design patterns using traits.**
- Define a `trait Storage { fn set(&mut self, k: &str, v: &str); fn get(&self, k: &str) -> Option<String>; }`.
- Implement the `Storage` trait for your in-memory + WAL database.
- Create a `--mode memory-only` CLI flag. When passed, instantiate a purely in-memory map without file logging. Use trait objects (`Box<dyn Storage>`) to swap between the two implementations seamlessly at runtime.
- Use advanced pattern matching with match guards to handle special commands like `SET foo bar EX 60` (expiration in 60 seconds).

### Checkpoint
- [ ] You can use trait objects for dynamic dispatch
- [ ] You can apply complex pattern matching with guards and bindings
- [ ] You understand associated types, operator overloading, and the newtype pattern
- [ ] You have a basic understanding of declarative macros

---

## Block 8: Async Rust & Ecosystem Essentials

**Rust Book chapter:** 17 (Async section), plus ecosystem resources

> **Note:** The Rust Book's async chapter provides a foundation. For deeper async understanding,
> refer to the [Async Book](https://rust-lang.github.io/async-book/) and the
> [Tokio tutorial](https://tokio.rs/tokio/tutorial).

### Topics
- Async/await syntax
- `Future` trait
- `tokio` runtime
- Async I/O: reading/writing files and network sockets
- `async fn`, `.await`, `tokio::spawn`
- `tokio::select!` for concurrent operations
- Streams
- Key ecosystem crates overview:
  - `serde` / `serde_json` — serialization/deserialization
  - `reqwest` — HTTP client
  - `tokio` — async runtime
  - `axum` or `actix-web` — web frameworks
  - `tokio-tungstenite` — WebSocket support
  - `clap` — CLI argument parsing
  - `tracing` — structured logging

### Key Concepts
- `async fn` returns a `Future` — it doesn't execute until polled (`.await`)
- `tokio` is the de-facto async runtime; it drives futures to completion
- `tokio::spawn` creates a new async task (like a lightweight thread)
- `select!` races multiple futures and handles the first one to complete
- `serde` is the universal serialization framework — derive `Serialize`/`Deserialize`

### Exercises
1. Write an async function that fetches a URL with `reqwest` and prints the response body
2. Build a simple TCP echo server with `tokio::net::TcpListener`
3. Use `tokio::select!` to race two async operations (e.g., a timer vs. reading stdin)
4. Serialize and deserialize a struct to/from JSON using `serde` and `serde_json`
5. Build a CLI tool with `clap` that accepts a URL argument and fetches it asynchronously

### Continuous Project: Mini-Redis (Milestone 8)
**Goal: A fully functional asynchronous TCP server.**
- Add the `tokio` crate to your dependencies. Change your main function to `#[tokio::main] async fn main()`.
- Replace your standard input REPL loop with `tokio::net::TcpListener::bind("127.0.0.1:6379").await;`.
- Use `listener.accept().await` in a loop. For every connected client socket, use `tokio::spawn` to handle its commands concurrently.
- Replace your `std::sync::RwLock` with `tokio::sync::RwLock` so async tasks don't block OS threads while waiting for a lock.
- Test it by connecting via a tool like `netcat` (`nc 127.0.0.1 6379`) or a standard Redis client!

### Checkpoint
- [ ] You can write async functions and use `tokio` as the runtime
- [ ] You can build a basic TCP server/client
- [ ] You can use `serde` for JSON serialization
- [ ] You are familiar with the key ecosystem crates

---

## Final Project 1: Chat Application

**A full-stack chat app with three components: server, web client, terminal client.**

### Architecture Overview

```
┌──────────────┐     WebSocket     ┌──────────────┐
│  Web Client  │◄─────────────────►│              │
│  (HTML/JS)   │                   │              │
└──────────────┘                   │  Chat Server │
                                   │  (Rust/Axum) │
┌──────────────┐     WebSocket     │              │
│  TUI Client  │◄─────────────────►│              │
│  (Rust)      │                   │              │
└──────────────┘                   └──────┬───────┘
                                          │
                                   ┌──────┴───────┐
                                   │   Storage    │
                                   │  (SQLite)    │
                                   └──────────────┘
```

### Prerequisites (all covered in Blocks 1-8)
- Structs, enums, traits, generics, error handling
- Async/await with `tokio`
- Serialization with `serde`
- Web framework basics (axum)
- WebSocket protocol
- CLI argument parsing

### Suggested Crates
| Crate | Purpose |
|-------|---------|
| `axum` | HTTP/WebSocket server framework |
| `tokio` | Async runtime |
| `tokio-tungstenite` | WebSocket client |
| `serde` / `serde_json` | Message serialization |
| `sqlx` | Async SQLite database |
| `clap` | CLI argument parsing |
| `ratatui` | Terminal UI for the TUI client |
| `tracing` | Structured logging |
| `uuid` | Unique message/user IDs |
| `tower-http` | Middleware (CORS, static file serving) |

### Milestones

#### Milestone 1: Chat Server Core
- [ ] Set up a new cargo workspace with `server`, `tui-client`, and `shared` crates
- [ ] Define shared message types in the `shared` crate:
  ```rust
  enum ClientMessage {
      Join { username: String },
      SendMessage { content: String },
      ListUsers,
  }
  enum ServerMessage {
      Welcome { user_id: Uuid },
      UserJoined { username: String },
      UserLeft { username: String },
      NewMessage { from: String, content: String, timestamp: DateTime },
      UserList { users: Vec<String> },
      Error { message: String },
  }
  ```
- [ ] Build an axum HTTP server that upgrades connections to WebSocket
- [ ] Manage connected clients with `Arc<Mutex<HashMap<Uuid, Client>>>`
- [ ] Broadcast messages to all connected clients
- [ ] Handle user join/leave lifecycle
- [ ] Add `tracing` for structured logging

#### Milestone 2: Message Persistence
- [ ] Add SQLite storage with `sqlx`
- [ ] Create tables: `users`, `messages`
- [ ] Store messages on receipt; load message history on join
- [ ] Run database migrations at startup

#### Milestone 3: Terminal Client (TUI)
- [ ] Connect to the server via WebSocket using `tokio-tungstenite`
- [ ] Use `ratatui` for a split-pane UI (message list + input area)
- [ ] Accept server URL and username via `clap` arguments
- [ ] Handle incoming messages and display them in real time
- [ ] Support commands: `/users` (list), `/quit` (disconnect)
- [ ] Graceful shutdown on Ctrl+C

#### Milestone 4: Web Client
- [ ] Serve a static HTML/CSS/JS page from axum (using `tower-http::services::ServeDir`)
- [ ] Connect to the server via browser `WebSocket` API
- [ ] Minimal but functional UI: message list, input field, user list sidebar
- [ ] Display message history on connect
- [ ] Show join/leave notifications

#### Milestone 5: Polish
- [ ] Add typing indicators (optional)
- [ ] Add message timestamps with formatting
- [ ] Add reconnection logic in both clients
- [ ] Write integration tests: spin up a server, connect two clients, verify message delivery
- [ ] Write unit tests for message serialization and business logic

### Verification
- [ ] Start the server, connect via both web and terminal clients, exchange messages
- [ ] Restart the server — verify message history is preserved
- [ ] Stress test: connect 10+ clients simultaneously
- [ ] All tests pass with `cargo test --workspace`

---

## Final Project 2: ESP32 Telegram Notification Display

**ESP32 firmware in Rust that connects to WiFi, polls a Telegram bot, and displays notifications on a screen.**

### Architecture Overview

```
┌────────────────────────────────────────────────┐
│                   ESP32 Board                  │
│                                                │
│  ┌──────────┐    ┌───────────┐    ┌─────────┐ │
│  │  WiFi    │───►│  Telegram │───►│ Display │ │
│  │  Module  │    │  Poller   │    │ Driver  │ │
│  └──────────┘    └───────────┘    └─────────┘ │
│                                                │
│  ┌──────────┐    ┌───────────┐                 │
│  │  Config  │    │   LED     │                 │
│  │  (NVS)   │    │ Indicator │                 │
│  └──────────┘    └───────────┘                 │
└────────────────────────────────────────────────┘
         │
         │ HTTPS (polling)
         ▼
┌──────────────────┐
│  Telegram Bot    │
│  API             │
│  (getUpdates)    │
└──────────────────┘
```

### Prerequisites (all covered in Blocks 1-8, especially Block 6)
- Ownership, borrowing, lifetimes
- Error handling with `Result`
- Traits and generics
- Unsafe Rust and FFI basics
- Serialization with `serde`

### Additional Learning Required
> These topics are ESP32/embedded-specific and not covered in the Rust Book.
> Study them before starting this project.

| Topic | Resource |
|-------|----------|
| `esp-idf-hal` and `esp-idf-svc` | [esp-rs Book](https://docs.esp-rs.org/book/) |
| ESP32 development setup | [esp-rs Book: Setting up](https://docs.esp-rs.org/book/installation/index.html) |
| `no_std` vs `std` on ESP32 | [esp-rs Book: ecosystem overview](https://docs.esp-rs.org/book/overview/index.html) |
| SPI/I2C display drivers | [`embedded-graphics` docs](https://docs.rs/embedded-graphics/latest/embedded_graphics/) |
| NVS (Non-Volatile Storage) | [esp-idf-svc docs](https://docs.rs/esp-idf-svc/latest/esp_idf_svc/) |
| Telegram Bot API | [Telegram Bot API docs](https://core.telegram.org/bots/api) |

### Suggested Crates
| Crate | Purpose |
|-------|---------|
| `esp-idf-hal` | Hardware abstraction (GPIO, SPI, I2C) |
| `esp-idf-svc` | Services (WiFi, HTTP client, NVS) |
| `esp-idf-sys` | Low-level ESP-IDF bindings |
| `embedded-graphics` | Drawing to display |
| `ssd1306` or `st7789` | Display driver (depends on your screen) |
| `serde` / `serde_json` | Telegram API response parsing |
| `heapless` | Fixed-size collections for constrained environments |

### Hardware You'll Need
- ESP32 dev board (ESP32, ESP32-S3, or ESP32-C3)
- SSD1306 OLED (128x64, I2C) or ST7789 TFT display
- USB cable for flashing
- Breadboard + jumper wires

### Milestones

#### Milestone 1: Development Environment
- [ ] Install `espup` and set up the ESP32 Rust toolchain
- [ ] Create a new project with `cargo generate esp-rs/esp-idf-template`
- [ ] Flash a "Hello" to serial output — verify the toolchain works
- [ ] Blink an LED using `esp-idf-hal` GPIO

#### Milestone 2: WiFi Connection
- [ ] Connect to WiFi using `esp-idf-svc::wifi`
- [ ] Store WiFi credentials in NVS (non-volatile storage)
- [ ] Implement retry logic with backoff for WiFi connection
- [ ] Light LED solid when connected, blink when disconnecting/reconnecting

#### Milestone 3: Display Driver
- [ ] Wire up the display (SSD1306 via I2C or ST7789 via SPI)
- [ ] Initialize the display driver
- [ ] Draw text on screen using `embedded-graphics`
- [ ] Create helper functions: `clear_screen()`, `draw_text(line, text)`, `draw_notification(title, body)`
- [ ] Show WiFi connection status on display

#### Milestone 4: Telegram Bot Integration
- [ ] Create a Telegram bot via BotFather, obtain the API token
- [ ] Implement HTTP polling of `getUpdates` endpoint using `esp-idf-svc::http::client`
- [ ] Parse JSON responses with `serde_json` into Rust structs:
  ```rust
  struct TelegramUpdate {
      update_id: i64,
      message: Option<TelegramMessage>,
  }
  struct TelegramMessage {
      message_id: i64,
      from: Option<TelegramUser>,
      text: Option<String>,
      date: i64,
  }
  ```
- [ ] Track `last_update_id` to avoid processing duplicates
- [ ] Store the bot token in NVS

#### Milestone 5: Putting It All Together
- [ ] Main loop: poll Telegram → parse → display notifications on screen
- [ ] Show the sender name and message text
- [ ] Scroll or rotate through messages if there are multiple
- [ ] Handle errors gracefully: network timeouts, JSON parse failures, display errors
- [ ] Add a visual/LED indicator for new notifications
- [ ] Handle WiFi drops: reconnect and resume polling

#### Milestone 6: Polish
- [ ] Add a startup splash screen with project name / version
- [ ] Show timestamp for each notification
- [ ] Limit displayed message length (truncate or scroll)
- [ ] Add a "no new messages" idle screen
- [ ] Optional: add a button to mark notifications as read / cycle through them

### Verification
- [ ] Send a message to the Telegram bot → it appears on the ESP32 display
- [ ] Disconnect WiFi → device reconnects and resumes polling
- [ ] Flash with different WiFi credentials stored in NVS
- [ ] Device runs stable for 1+ hours without crashing

---

## Recommended Study Schedule

| Week | Block | Estimated Hours |
|------|-------|-----------------|
| 1 | Block 1: Getting Started & Foundations | 8-10 |
| 2 | Block 2: Ownership | 6-8 |
| 3 | Block 3: Structuring Data & Code | 8-10 |
| 4 | Block 4: Collections, Error Handling & Generics | 10-12 |
| 5 | Block 5: Functional Patterns, Testing & I/O | 10-12 |
| 6 | Block 6: Smart Pointers, Concurrency & Unsafe | 10-12 |
| 7 | Block 7: Advanced Rust | 8-10 |
| 8 | Block 8: Async Rust & Ecosystem | 8-10 |
| 9-10 | Final Project 1: Chat Application | 20-30 |
| 11-12 | Final Project 2: ESP32 Telegram Display | 20-30 |

**Total estimated time: ~120-150 hours**

---

## Tips for LLM-Assisted Learning

1. **One block at a time.** Give the LLM the current block as context. Don't dump the entire plan.
2. **Ask for explanations.** After each exercise, ask the LLM to review your code and explain what could be improved.
3. **Intentional errors.** Try writing code that breaks the borrow checker. Ask the LLM to explain the error message.
4. **Rubber duck.** Before asking for help, explain your understanding to the LLM and ask it to correct misconceptions.
5. **Code review.** Paste your code and ask: "What would a senior Rust developer change here?"
6. **Build incrementally.** For the final projects, complete one milestone fully before starting the next.
7. **Use the Rust Book directly.** This plan references chapters — read them. The LLM supplements the book, not replaces it.
