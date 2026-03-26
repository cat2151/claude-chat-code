# claude-chat-code

Claude chat からzipダウンロードしたか監視して自動ビルドと起動をする Windows 用 TUI 。Rustで書かれています。

---

## インストール

Rustが必要です。

```powershell
cargo install --force --git https://github.com/cat2151/claude-chat-code
```

インストール後、任意のターミナルで起動できます。

```powershell
claude-chat-code
```

組み込みの help を表示するときは以下を実行します。

```powershell
claude-chat-code --help
```

インストール済みバイナリを update するときは以下を実行します。

```powershell
claude-chat-code update
```

---

## 課題と解決

### 従来の課題

Claude chat にコードを生成・修正させるとき、従来は以下の手順が毎回必要でした。

1. Claude chat から zip をダウンロードします
2. 作業ディレクトリのバックアップをとります
3. 古いファイルを削除します（旧ファイルの残留による衝突ビルドエラー等を防ぐため）
4. zip を展開します
5. `cargo run` を実行します
6. ビルドエラーがあれば Claude chat に貼り付けて修正を指示します
7. 1 に戻ります

このサイクルを何回も繰り返すとき、手作業は集中を妨げます。
「ファイルを動かすこと」に意識配分をとられ、「仕様を考えること」の質が下がります。

### このアプリの解決

Claude chatのダウンロードボタンを押すだけで、以下がすべて自動で走ります。

1. zip を検出します
2. project のバックアップを取ります
3. 既存の `project/src/` を削除します（旧ファイルの残留を防ぐため）
4. zip を展開します
5. `cargo run` を別ウィンドウで起動します

あとは Claude chat とビルド結果のやり取りに集中できます。

---

## 使い方

```
Claude chat に指示してコーディングさせ、結果をzipでダウンロード
        ↓
claude-chat-code がzipを自動で検出・ビルド・アプリ起動
        ↓
エラーならビルド結果を Claude chat に貼る、アプリ起動成功したらUXを検証して仕様追加や改善を指示する
        ↓
繰り返す
```

---

## 設定

初回起動時に設定ファイルが自動生成されます。

```
%LOCALAPPDATA%\claude-chat-code\config.toml
```

```toml
# watch_dir: 監視対象ディレクトリ。コメントアウト時は Windows Desktop を監視します。
# watch_dir = "C:\Users\<your name>\Desktop"

# watch_interval: 監視チェック間隔。コメントアウト時のデフォルトは 500ms です。
# watch_interval = "500ms"
```

---

## キーバインド

| キー | 動作 |
|------|------|
| `q` | 終了 |
| `c` | ログをクリップボードにコピー |
| `F5` | `cargo run` を再実行 |
| `y` / `N` | 起動時 ZIP 確認ダイアログへの応答 |

---

## 動作環境

- **OS**: Windows
- **Rust**: 最新の stable 推奨（`cargo install` で自動解決）

---

## 技術スタック

| 技術 | 用途 |
|------|------|
| [Rust](https://www.rust-lang.org/) | 言語 |
| [ratatui](https://ratatui.rs/) | TUI フレームワーク |
| [crossterm](https://github.com/crossterm-rs/crossterm) | ターミナル制御・キー入力 |
| [tokio](https://tokio.rs/) | 非同期ランタイム（監視・パイプライン分離） |
| [zip](https://crates.io/crates/zip) | ZIP 展開・単一トップディレクトリの自動剥ぎ取り |
| [reqwest](https://crates.io/crates/reqwest) | GitHub API による update check |
| [arboard](https://crates.io/crates/arboard) | クリップボード操作 |
| [serde](https://serde.rs/) + [toml](https://crates.io/crates/toml) | config.toml の読み書き |
| [chrono](https://crates.io/crates/chrono) | タイムスタンプ・経過時間表示 |
| [walkdir](https://crates.io/crates/walkdir) | src/ 配下の再帰探索 |
| [filetime](https://crates.io/crates/filetime) | ファイルの mtime 更新（touch） |

## 前提
- もっとClaudeを使いたいのでClaude codeをサブスクしたい！と思ったらサブスクしましょう！
- 自分用のアプリですので、他の人が使うことを想定していません。似たような機能がほしいときはcloneや自作をおすすめします。

## 想定している用途
- Claude chatとclaude-chat-codeを利用し、手軽に最小限の機能をUX検証しながら実装していくスタイル。
- 作るアプリは、小規模なRust TUI。小さく始める。listを表示し、qでquit。のち機能追加。慣れてきたら最初から骨子となる機能の仕様を指定。

## claude-chat-codeが目指すもの
- PoC。Claude無料chatだけでちょっと楽しいアプリが楽に素早く作れることを実証する（実証した）

## 目指さないもの（スコープ外）
- サポート。要望や提案に応える

## 免責事項
- AIセキュリティについて十分にご理解の上、自己責任でお願いします。
