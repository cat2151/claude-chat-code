# claude-chat-code

A TUI for Windows that monitors for ZIP downloads from Claude chat, then automatically builds and launches the application. Written in Rust.

---

## Installation

Rust is required.

```powershell
cargo install --force --git https://github.com/cat2151/claude-chat-code
```

After installation, you can launch it from any terminal.

```powershell
claude-chat-code
```

---

## Challenges and Solutions

### Traditional Challenges

When generating or modifying code with Claude chat, the following steps were traditionally required every time:

1. Download the ZIP from Claude chat.
2. Back up your working directory.
3. Delete old files (to prevent build errors due to conflicts from leftover files).
4. Extract the ZIP.
5. Run `cargo run`.
6. If there are build errors, paste them into Claude chat and instruct it to fix them.
7. Go back to step 1.

Repeating this cycle multiple times with manual steps disrupts concentration.
Attention is diverted to "moving files," reducing the quality of "thinking about specifications."

### Solution Provided by This App

Simply pressing the download button in Claude chat automatically performs all of the following:

1. Detects the ZIP file.
2. Backs up the project.
3. Deletes the existing `project/src/` (to prevent leftover old files).
4. Extracts the ZIP file.
5. Launches `cargo run` in a separate window.

Afterward, you can focus on interacting with Claude chat based on the build results.

---

## Usage

```
Instruct Claude chat to code, then download the result as a ZIP.
        ↓
claude-chat-code automatically detects the ZIP, builds, and launches the application.
        ↓
If there's an error, paste the build result into Claude chat; if the app launches successfully, verify the UX and instruct for specification additions or improvements.
        ↓
Repeat.
```

---

## Configuration

A configuration file is automatically generated on first launch.

```
%LOCALAPPDATA%\claude-chat-code\config.toml
```

```toml
# watch_dir: Directory to monitor. If commented out, Windows Desktop will be monitored.
# watch_dir = "C:\Users\<your name>\Desktop"

# watch_interval: Monitoring check interval. Default is 500ms if commented out.
# watch_interval = "500ms"
```

---

## Keybindings

| Key | Action |
|-----|--------|
| `q` | Quit |
| `c` | Copy logs to clipboard |
| `y` / `N` | Respond to launch-time ZIP confirmation dialog |

---

## System Requirements

- **OS**: Windows
- **Rust**: Latest stable recommended (automatically resolved by `cargo install`)

---

## Technology Stack

| Technology | Purpose |
|------------|---------|
| [Rust](https://www.rust-lang.org/) | Language |
| [ratatui](https://ratatui.rs/) | TUI framework |
| [crossterm](https://github.com/crossterm-rs/crossterm) | Terminal control, key input |
| [tokio](https://tokio.rs/) | Asynchronous runtime (monitoring, pipeline isolation) |
| [zip](https://crates.io/crates/zip) | ZIP extraction, automatic stripping of single top-level directory |
| [reqwest](https://crates.io/crates/reqwest) | Update check via GitHub API |
| [arboard](https://crates.io/crates/arboard) | Clipboard operations |
| [serde](https://serde.rs/) + [toml](https://crates.io/crates/toml) | Reading and writing config.toml |
| [chrono](https://crates.io/crates/chrono) | Timestamp, elapsed time display |
| [walkdir](https://crates.io/crates/walkdir) | Recursive search under src/ |
| [filetime](https://crates.io/crates/filetime) | File mtime update (touch) |

## Prerequisites
- If you wish to use Claude more extensively and are considering subscribing to Claude code, please do so!
- This application is for personal use and is not intended for others. If you desire similar functionality, it is recommended to clone or build your own.

## Intended Use Cases
- A style of implementation that leverages Claude chat and claude-chat-code to easily build minimal features while performing UX validation.
- The intended applications are small Rust TUIs. Start small. Display a list, quit with 'q'. Add features later. Once comfortable, specify core feature requirements from the beginning.

## What claude-chat-code Aims For
- Proof of Concept. To demonstrate that fun little apps can be easily and quickly created using only Claude's free chat (demonstrated).

## What is Not Aimed For (Out of Scope)
- Support. Responding to requests or suggestions.

## Disclaimer
- Please ensure you fully understand AI security and proceed at your own risk.
