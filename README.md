# 🦀 wtasks – Rust async task manager

`wtasks` is a minimal command-line task tracker written in Rust.  
It supports adding, listing, marking, and removing tasks, with optional persistent storage via `tasks.json`.

---

## ⚙️ Features

- ✅ Add tasks via CLI
- 📋 List all tasks
- ✅ Mark specific tasks as done
- ❌ Remove tasks by index
- 💾 Auto-saves tasks in `tasks.json`
- 🧪 Fully tested with `cargo test`

---

## 🚀 Installation

```bash
git clone https://github.com/vogonpoet/wtasks.git
cd wtasks
cargo build --release
```

You can optionally install it globally:

```bash
cargo install --path .
```

---

## 🧪 Run tests

```bash
cargo test
```

---

## 🧑‍💻 Usage

```bash
cargo run -- [OPTIONS]
```

### Add a new task

```bash
cargo run -- --name "Fix the prod bug"
```

### List all tasks

```bash
cargo run -- --list
```

Example output:

```
[ ] 1: Fix the prod bug
[ ] 2: Write README
[x] 3: Buy coffee
```

### Mark a task as done

```bash
cargo run -- --done 1
```

### Remove a task

```bash
cargo run -- --remove 3
```

---

## 🧾 Command-line options

| Option      | Alias | Description                            |
| ----------- | ----- | -------------------------------------- |
| `--name`    | `-n`  | Add a new task                         |
| `--list`    | `-l`  | List all tasks                         |
| `--done`    |       | Mark a task as done by index (1-based) |
| `--remove`  | `-r`  | Remove a task by index (1-based)       |
| `--help`    | `-h`  | Show CLI help                          |
| `--version` | `-V`  | Show version                           |

---

## 🧠 Built With

- [Rust](https://www.rust-lang.org/)
- [Clap](https://crates.io/crates/clap)
- [Serde](https://serde.rs/)
- [Tempfile](https://crates.io/crates/tempfile) for testing
- [Tokio](https://tokio.rs/) (optional)

---

## 📂 Persistence

Tasks are stored in a local `tasks.json` file in the current directory.  
Each action (`--name`, `--done`, `--remove`) automatically updates the file.

---

## 📎 License

MIT License.  
Written with ❤️ by wann.
