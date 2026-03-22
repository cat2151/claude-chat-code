# claude-chat-code

A Windows TUI written in Rust that monitors for zip downloads from Claude chat, then automatically builds and launches the project.

---

## Status
- Press `F5` to run `cargo run` again after the launched app has been closed.

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

## Problem and Solution

### Traditional Workflow Problems

When generating or modifying code with Claude chat, the traditional workflow required the following steps every time:

1.  Download the zip from Claude chat.
2.  Back up your working directory.
3.  Delete old files (to prevent build errors due to remnants of previous files).
4.  Unzip the archive.
5.  Run `cargo run`.
6.  If there are build errors, paste them into Claude chat and instruct it to fix them.
7.  Go back to step 1.

Repeating this cycle multiple times, manual operations disrupt concentration. Focusing on "moving files" detracts from the quality of "thinking about specifications".

### This Application's Solution

By simply clicking the download button in Claude chat, all of the following will run automatically:

1.  Detect the zip.
2.  Back up the project.
3.  Delete the existing `project/src/` (to prevent remnants of old files).
4.  Unzip the archive.
5.  Launch `cargo run` in a separate window.

After this, you can focus on interacting with Claude chat and the build results.

---

## Usage

```
Instruct Claude chat to code, then download the result as a zip.
        ↓
claude-chat-code automatically detects the zip, builds, and launches the application.
        ↓
If there's an error, paste the build result into Claude chat. If the app launches successfully, test the UX and instruct for feature additions or improvements.
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
# watch_dir: The directory to monitor. If commented out, Windows Desktop will be monitored.
# watch_dir = "C:\Users\<your name>\Desktop"

# watch_interval: The monitoring check interval. Default is 500ms if commented out.
# watch_interval = "500ms"
```

---

## Keybindings

| Key | Action |
|-----|--------|
| `q` | Quit |
| `c` | Copy logs to clipboard |
| `F5` | Run `cargo run` again |
| `y` / `N` | Respond to launch ZIP confirmation dialog |

---

## Environment

-   **OS**: Windows
-   **Rust**: Latest stable recommended (automatically resolved by `cargo install`)

---

## Tech Stack

| Technology | Purpose |
|------------|---------|
| [Rust](https://www.rust-lang.org/) | Language |
| [ratatui](https://ratatui.rs/) | TUI Framework |
| [crossterm](https://github.com/crossterm-rs/crossterm) | Terminal control and key input |
| [tokio](https://tokio.rs/) | Async runtime (monitoring, pipeline separation) |
| [zip](https://crates.io/crates/zip) | ZIP extraction, automatic stripping of single top-level directory |
| [reqwest](https://crates.io/crates/reqwest) | GitHub API for update check |
| [arboard](https://crates.io/crates/arboard) | Clipboard operations |
| [serde](https://serde.rs/) + [toml](https://crates.io/crates/toml) | Reading and writing config.toml |
| [chrono](https://crates.io/crates/chrono) | Timestamps, elapsed time display |
| [walkdir](https://crates.io/crates/walkdir) | Recursive traversal under src/ |
| [filetime](https://crates.io/crates/filetime) | Updating file mtime (touch) |

## Context
- If you want to use Claude more and are thinking of subscribing to Claude code, go ahead and subscribe!
- This application is for personal use and is not intended for others. If you desire similar functionality, it is recommended to clone or build your own version.

## Intended Use Cases
- A style of implementation where Claude chat and claude-chat-code are used to easily implement minimal features while performing UX validation.
- The apps to be created are small Rust TUIs. Start small: display a list, 'q' to quit, then add features. Once familiar, specify core feature specifications from the beginning.

## claude-chat-code's Goal
- PoC: Demonstrate that fun little applications can be easily and quickly created using only Claude's free chat (demonstrated).

## Not a Goal (Out of Scope)
- Support: Responding to requests or suggestions.

## Disclaimer
- Please ensure you fully understand AI security and proceed at your own risk.
