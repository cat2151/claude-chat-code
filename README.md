# claude-chat-code

A TUI for Windows that monitors for zip file downloads from Claude chat, then automatically builds and launches the project. Written in Rust.

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

Traditionally, when generating or modifying code with Claude chat, the following steps were required every time:

1.  Download the zip file from Claude chat.
2.  Back up the working directory.
3.  Delete old files (to prevent build errors caused by leftover files, etc.).
4.  Unzip the file.
5.  Run `cargo run`.
6.  If there are build errors, paste them into Claude chat and instruct it to fix them.
7.  Go back to step 1.

Repeating this cycle multiple times, manual operations disrupt concentration. Attention is diverted to "moving files," which degrades the quality of "thinking about specifications."

### This App's Solution

Simply pressing the download button in Claude chat will automatically execute all of the following:

1.  Detects the zip file.
2.  Backs up the project.
3.  Deletes the existing `project/src/` (to prevent leftover old files).
4.  Unzips the file.
5.  Launches `cargo run` in a separate window.

Afterward, you can focus on interacting with Claude chat and the build results.

---

## Usage

```
Instruct Claude chat to code, then download the result as a zip.
        ↓
claude-chat-code automatically detects the zip, builds, and launches the application.
        ↓
If there's an error, paste the build result into Claude chat; if the app launches successfully, verify the UX and instruct for specification additions or improvements.
        ↓
Repeat.
```

---

## Configuration

A configuration file is automatically generated on the first launch.

```
%LOCALAPPDATA%\claude-chat-code\config.toml
```

```toml
# watch_dir: The directory to monitor. If commented out, Windows Desktop is monitored.
# watch_dir = "C:\Users\<your name>\Desktop"

# watch_interval: Monitoring check interval. Default is 500ms if commented out.
# watch_interval = "500ms"
```

---

## Keybindings

| Key | Action |
|------|------|
| `q` | Quit |
| `c` | Copy logs to clipboard |
| `F5` | Rerun `cargo run` |
| `y` / `N` | Respond to launch-time ZIP confirmation dialog |

---

## Environment

-   **OS**: Windows
-   **Rust**: Latest stable recommended (automatically resolved by `cargo install`)

---

## Technology Stack

| Technology | Purpose |
|------|------|
| [Rust](https://www.rust-lang.org/) | Language |
| [ratatui](https://ratatui.rs/) | TUI Framework |
| [crossterm](https://github.com/crossterm-rs/crossterm) | Terminal Control & Key Input |
| [tokio](https://tokio.rs/) | Async Runtime (Monitoring, Pipeline Separation) |
| [zip](https://crates.io/crates/zip) | ZIP Extraction, Automatic stripping of single top-level directory |
| [reqwest](https://crates.io/crates/reqwest) | Update check via GitHub API |
| [arboard](https://crates.io/crates/arboard) | Clipboard Operations |
| [serde](https://serde.rs/) + [toml](https://crates.io/crates/toml) | Reading/writing config.toml |
| [chrono](https://crates.io/crates/chrono) | Timestamp, Elapsed Time Display |
| [walkdir](https://crates.io/crates/walkdir) | Recursive search under src/ |
| [filetime](https://crates.io/crates/filetime) | Updating file mtime (touch) |

## Prerequisites
- If you want to use Claude more and are thinking of subscribing to Claude code, go ahead and subscribe!
- This app is for personal use and not intended for others. If you need similar functionality, consider cloning or building your own.

## Intended Use Cases
- A development style that uses Claude chat and claude-chat-code to easily implement minimal features while verifying UX.
- The target app is a small Rust TUI. Start small: display a list, 'q' to quit, then add features. Once familiar, specify core feature requirements from the beginning.

## What claude-chat-code Aims For
- PoC. To demonstrate that a somewhat fun app can be easily and quickly built using only the free Claude chat (demonstrated).

## What It Does Not Aim For (Out of Scope)
- Support. Responding to requests or suggestions.

## Disclaimer
- Please ensure you fully understand AI security and proceed at your own risk.