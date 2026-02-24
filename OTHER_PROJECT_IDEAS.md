# Other Rust Project Ideas

Here are some other fun project ideas that could be built incrementally while learning Rust.

## Option 1: A Terminal RPG / Text-Adventure Engine
A text-based adventure game where the player navigates rooms, collects items, and battles monsters. It naturally scales from a simple `while` loop to a networked multiplayer game.

*   **Block 1 (Foundations):** Build the REPL (Read-Eval-Print Loop). Accept user input to move (North, South) and print basic room descriptions. Use variables for player health/stats.
*   **Block 2 (Ownership):** Implement systems that pass room descriptions, items, and player names around using references to avoid unnecessary `String` cloning.
*   **Block 3 (Structs/Enums):** Define `Player`, `Room`, and `Monster` structs. Use a robust `Action` enum (`Move(Direction)`, `Take(String)`, `Attack`) and `match` statements to handle logic. Organize code into combat and exploration modules.
*   **Block 4 (Collections & Errors):** Use a `HashMap` for the world map and a `Vec` for the player's inventory. Use `Result` to handle invalid moves or missing items gracefully instead of panicking.
*   **Block 5 (I/O & Testing):** Move room descriptions out of the code and load them from a text or JSON file. Write unit tests for the combat math and inventory logic. 
*   **Block 6 (Concurrency):** Add a background thread for "wandering monsters" or weather changes that occur independently of the player's input loop. Use `Arc<Mutex<GameState>>`.
*   **Block 7 (Advanced):** Implement an `Interactable` trait. Use trait objects (`Box<dyn Interactable>`) so chests, NPCs, and doors can all share a room's entity list but behave differently.
*   **Block 8 (Async):** Turn it into a MUD (Multi-User Dungeon)! Add an async TCP server with `tokio` so multiple players can connect via terminal to explore the same world instance.

## Option 2: Static Site Generator (Markdown to HTML)
A developer tool that reads a directory of custom Markdown-like files and compiles them into a styled HTML website. 

*   **Block 1 (Foundations):** Accept a hardcoded string, detect basic formatting symbols (like `#` for headers), and print HTML to the console.
*   **Block 2 (Ownership):** Build a basic parser that iterates through a text string, using slices to keep track of the start and end of words/sentences.
*   **Block 3 (Structs/Enums):** Define a `Document` struct with metadata (title, date). Build an Abstract Syntax Tree (AST) using enums (e.g., `Node::Header`, `Node::Paragraph`).
*   **Block 4 (Collections & Errors):** Store parsed posts in a `Vec<Document>`. Use `Result` to surface syntax errors (e.g., unclosed tags) to the user with line numbers.
*   **Block 5 (I/O & Testing):** Actually read `.md` files from a `content/` directory and write `.html` files to a `public/` directory. Heavily utilize iterators (`.filter()`, `.map()`) and write parser tests.
*   **Block 6 (Concurrency):** Generating 1,000 pages takes time. Use `std::thread` (or the `rayon` crate) to process multiple Markdown files in parallel. 
*   **Block 7 (Advanced):** Implement a templating engine layout using advanced traits and dynamic dispatch, allowing themes to be swapped out easily.
*   **Block 8 (Async):** Add an async, live-reloading local HTTP server using `axum`. When a file in the `content/` folder changes, send a trigger via WebSockets to automatically refresh the browser.
