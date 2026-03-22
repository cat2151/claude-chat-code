# claude-chat-code

A Windows TUI written in Rust that monitors for zip downloads from Claude chat, then automatically builds and launches the project.

---

## Status
- The following are planned for modification:
- Pressing F5 will execute `cargo run`. This is for when the application has been closed but needs to be run again.

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

1. Download the zip file from Claude chat.
2. Back up the working directory.
3. Delete old files (to prevent build errors due to conflicts from leftover files).
4. Unzip the file.
5. Execute `cargo run`.
6. If there are build errors, paste them into Claude chat and instruct it to fix them.
7. Return to step 1.

When repeating this cycle many times, manual steps disrupt concentration. Mental effort spent on "moving files" detracts from the quality of "thinking about specifications".

### This Application's Solution

Simply pressing the download button in Claude chat will automatically execute all of the following:

1. Detects the zip file.
2. Backs up the project.
3. Deletes the existing `project/src/` (to prevent leftover old files).
4. Unzips the file.
5. Launches `cargo run` in a separate window.

After this, you can focus on interacting with Claude chat and the build results.

---

## Usage

```
Instruct Claude chat to code, download the results as a zip.
        ↓
claude-chat-code automatically detects zip, builds, and launches the application.
        ↓
If there's an error, paste the build results into Claude chat; if the app launches successfully, verify the UX and instruct for feature additions or improvements.
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
# watch_dir: Directory to watch. If commented out, Windows Desktop is monitored.
# watch_dir = "C:\Users\<your name>\Desktop"

# watch_interval: Watch check interval. Default is 500ms if commented out.
# watch_interval = "500ms"
```

---

## Keybindings

| Key | Action |
|------|------|
| `q` | Quit |
| `c` | Copy logs to clipboard |
| `y` / `N` | Respond to startup ZIP confirmation dialog |

---

## Operating Environment

- **OS**: Windows
- **Rust**: Latest stable recommended (automatically resolved by `cargo install`)

---

## Tech Stack

| Technology | Purpose |
|------|------|
| [Rust](https://www.rust-lang.org/) | Language |
| [ratatui](https://ratatui.rs/) | TUI framework |
| [crossterm](https://github.com/crossterm-rs/crossterm) | Terminal control / Key input |
| [tokio](https://tokio.rs/) | Async runtime (monitoring, pipeline separation) |
| [zip](https://crates.io/crates/zip) | ZIP decompression / Automatic removal of single top-level directory |
| [reqwest](https://crates.io/crates/reqwest) | Update check via GitHub API |
| [arboard](https://crates.io/crates/arboard) | Clipboard operations |
| [serde](https://serde.rs/) + [toml](https://crates.io/crates/toml) | Reading/writing config.toml |
| [chrono](https://crates.io/crates/chrono) | Timestamp / Elapsed time display |
| [walkdir](https://crates.io/crates/walkdir) | Recursive traversal under src/ |
| [filetime](https://crates.io/crates/filetime) | Updating file mtime (touch) |

## Assumptions
- If you want to use Claude more and are thinking of subscribing to Claude Code, go for it!
- This is an app for personal use, not intended for others. If you want similar functionality, I recommend building it yourself.

## Intended Use Cases
- A style of development that uses Claude chat and claude-chat-code to easily implement minimal features while performing UX validation.
- The apps to be created are small Rust TUIs. Start small. Display a list, 'q' to quit. Add features later. Once familiar, specify core feature requirements from the start.

## What claude-chat-code Aims For
- Proof of Concept (PoC). To demonstrate that slightly fun apps can be easily and quickly built using only the free Claude chat (demonstrated).

## What is Not Aimed For (Out of Scope)
- Support. Responding to requests or suggestions.

## Disclaimer
- Please ensure you fully understand AI security and proceed at your own risk.