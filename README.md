# claude-chat-code

A Windows TUI that monitors for zip file downloads from Claude chat, then automatically builds and launches the code. Written in Rust.

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

To display the built-in help, run:

```powershell
claude-chat-code --help
```

To update an installed binary, run:

```powershell
claude-chat-code update
```

---

## Challenges and Solutions

### Traditional Challenges

When generating or modifying code with Claude chat, the following steps were traditionally required every time:

1.  Download the zip from Claude chat.
2.  Back up the working directory.
3.  Delete old files (to prevent build errors caused by leftover files).
4.  Unzip the archive.
5.  Run `cargo run`.
6.  If there are build errors, paste them into Claude chat and instruct it to fix them.
7.  Go back to step 1.

Repeating this cycle many times, manual operations disrupt concentration. Diverting mental resources to "managing files" reduces the quality of "designing specifications."

### This Application's Solution

Simply by pressing the download button in Claude chat, all of the following happen automatically:

1.  Detects the zip file.
2.  Backs up the project.
3.  Deletes the existing `project/src/` (to prevent leftover old files).
4.  Unzips the archive.
5.  Launches `cargo run` in a separate window.

After that, you can focus on interacting with Claude chat and the build results.

---

## Usage

```
Instruct Claude chat to code, then download the result as a zip
        ↓
claude-chat-code automatically detects the zip, builds, and launches the app
        ↓
If there's an error, paste build results into Claude chat; if the app launches successfully, test the UX and instruct for feature additions or improvements
        ↓
Repeat
```

---

## Configuration

A configuration file is automatically generated on first launch.

```
%LOCALAPPDATA%\claude-chat-code\config.toml
```

```toml
# watch_dir: Directory to monitor. If commented out, Windows Desktop is monitored.
# watch_dir = "C:\Users\<your name>\Desktop"

# watch_interval: Monitoring check interval. Default is 500ms if commented out.
# watch_interval = "500ms"
```

---

## Keybindings

| Key | Action                                   |
|-----|------------------------------------------|
| `q` | Exit                                     |
| `c` | Copy log to clipboard                    |
| `F5` | Rerun `cargo run`                        |
| `y` / `N` | Respond to startup ZIP confirmation dialog |

---

## Environment

-   **OS**: Windows
-   **Rust**: Latest stable recommended (automatically resolved by `cargo install`)

---

## Tech Stack

| Technology                                               | Purpose                                             |
|----------------------------------------------------------|-----------------------------------------------------|
| [Rust](https://www.rust-lang.org/)                       | Language                                            |
| [ratatui](https://ratatui.rs/)                           | TUI framework                                       |
| [crossterm](https://github.com/crossterm-rs/crossterm)   | Terminal control & keyboard input                   |
| [tokio](https://tokio.rs/)                               | Asynchronous runtime (monitoring, pipeline separation) |
| [zip](https://crates.io/crates/zip)                      | ZIP extraction, automatic stripping of single top-level directory |
| [reqwest](https://crates.io/crates/reqwest)             | Update check via GitHub API                         |
| [arboard](https://crates.io/crates/arboard)              | Clipboard operations                                |
| [serde](https://serde.rs/) + [toml](https://crates.io/crates/toml) | Reading/writing config.toml                         |
| [chrono](https://crates.io/crates/chrono)                | Timestamp / Elapsed time display                    |
| [walkdir](https://crates.io/crates/walkdir)              | Recursive traversal under src/                      |
| [filetime](https://crates.io/crates/filetime)            | Updating file mtime (touch)                         |

## Assumptions
- If you want to use Claude more and are considering subscribing to Claude code, you should!
- This is a personal application, not intended for use by others. If you want similar functionality, it's recommended to clone it or create your own.

## Intended Use Cases
- A style of implementation using Claude chat and claude-chat-code to easily build minimal features while validating UX.
- The applications to be built are small-scale Rust TUIs. Start small: display a list, 'q' to quit, then add features. Once comfortable, specify core feature specifications from the start.

## What claude-chat-code Aims For
- Proof of Concept. To demonstrate (and has demonstrated) that a fun little application can be built easily and quickly using only the free Claude chat.

## What it Does Not Aim For (Out of Scope)
- Support. Responding to requests or suggestions.

## Disclaimer
- Please ensure you have a thorough understanding of AI security and proceed at your own risk.