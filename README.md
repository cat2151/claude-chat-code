# claude-chat-code

A Windows TUI written in Rust that monitors for downloaded zip files from Claude chat and automatically builds and launches them.

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

To update an installed binary, run the following:

```powershell
claude-chat-code update
```

---

## Challenges and Solutions

### Traditional Challenges

When generating or modifying code with Claude chat, the following steps were traditionally required every time:

1.  Download the zip file from Claude chat.
2.  Back up the working directory.
3.  Delete old files (to prevent build errors due to leftover files).
4.  Extract the zip file.
5.  Run `cargo run`.
6.  If there are build errors, paste them into Claude chat and instruct it to fix them.
7.  Go back to step 1.

When repeating this cycle many times, manual work disrupts concentration. Focus is drawn to "moving files," reducing the quality of "thinking about specifications."

### This App's Solution

By simply clicking the download button in Claude chat, all of the following automatically occur:

1.  Detects the zip file.
2.  Backs up the project.
3.  Deletes the existing `project/src/` (to prevent leftover old files).
4.  Extracts the zip file.
5.  Launches `cargo run` in a separate window.

You can then concentrate on interacting with Claude chat and the build results.

---

## Usage

```
Instruct Claude chat to code, then download the result as a zip file.
        ↓
claude-chat-code automatically detects, builds, and launches the app from the zip.
        ↓
If there's an error, paste the build result into Claude chat; if the app launches successfully,
verify the UX and instruct for feature additions or improvements.
        ↓
Repeat.
```

---

## Settings

A configuration file is automatically generated on first launch.

```
%LOCALAPPDATA%\claude-chat-code\config.toml
```

```toml
# watch_dir: Directory to watch. If commented out, Windows Desktop will be watched.
# watch_dir = "C:\Users\<your name>\Desktop"

# watch_interval: Watch check interval. Default is 500ms if commented out.
# watch_interval = "500ms"
```

---

## Keybindings

| Key | Action |
|------|--------|
| `q` | Quit |
| `c` | Copy log to clipboard |
| `F5` | Rerun `cargo run` |
| `y` / `N` | Respond to launch ZIP confirmation dialog |

---

## Operating Environment

-   **OS**: Windows
-   **Rust**: Latest stable recommended (automatically resolved by `cargo install`)

---

## Technical Stack

| Technology | Purpose |
|------------|---------|
| [Rust](https://www.rust-lang.org/) | Language |
| [ratatui](https://ratatui.rs/) | TUI Framework |
| [crossterm](https://github.com/crossterm-rs/crossterm) | Terminal control, key input |
| [tokio](https://tokio.rs/) | Asynchronous runtime (monitoring, pipeline separation) |
| [zip](https://crates.io/crates/zip) | ZIP extraction, automatic stripping of single top-level directory |
| [reqwest](https://crates.io/crates/reqwest) | GitHub API for update check |
| [arboard](https://crates.io/crates/arboard) | Clipboard operations |
| [serde](https://serde.rs/) + [toml](https://crates.io/crates/toml) | Reading/writing `config.toml` |
| [chrono](https://crates.io/crates/chrono) | Timestamp, elapsed time display |
| [walkdir](https://crates.io/crates/walkdir) | Recursive search under `src/` |
| [filetime](https://crates.io/crates/filetime) | Updating file mtime (touch) |

## Prerequisites
- If you want to use Claude more and are thinking of subscribing to Claude code, then subscribe!
- This is an app for personal use, not intended for others. If you want similar functionality, I recommend cloning it or building your own.

## Intended Use Cases
- A style of easily implementing minimal features while verifying UX, using Claude chat and claude-chat-code.
- The app to be built is a small Rust TUI. Start small: display a list, quit with `q`, then add features. Once comfortable, specify core feature requirements from the start.

## What claude-chat-code Aims For
- PoC. To demonstrate that a slightly fun app can be easily and quickly made using only the free Claude chat (demonstrated).

## What claude-chat-code Does Not Aim For (Out of Scope)
- Support. Responding to requests or proposals.

## Disclaimer
- Please ensure you fully understand AI security and proceed at your own risk.