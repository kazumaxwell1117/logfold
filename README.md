# logfold

[![CI](https://github.com/kazumaxwell1117/logfold/actions/workflows/ci.yml/badge.svg)](https://github.com/kazumaxwell1117/logfold/actions/workflows/ci.yml)
[![Crates.io](https://img.shields.io/crates/v/logfold.svg)](https://crates.io/crates/logfold)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)

**Collapse thousands of duplicate log lines into a ranked summary — in milliseconds.**

Tired of scrolling through 50,000 log lines to find what's actually broken?
`logfold` groups repeated messages and shows you what matters, sorted by frequency.

---

## Demo

```
$ wc -l app.log
   48291 app.log

$ logfold app.log --ignore-prefix "^\S+Z "
[9823x] 2026-03-13T08:00:01.001Z [WARN]  Cache miss for key: session_store
[7401x] 2026-03-13T08:00:02.196Z [ERROR] Failed to connect to Redis: connection refused
[4218x] 2026-03-13T08:00:03.366Z [INFO]  Processing background job: email_digest
 [834x] 2026-03-13T08:00:10.445Z [WARN]  Response time exceeded threshold: 2500ms
   [2x] 2026-03-13T08:00:01.001Z [INFO]  Server started on port 8080
   [1x] 2026-03-13T08:00:05.312Z [INFO]  Database connection established
   [1x] 2026-03-13T08:30:00.001Z [ERROR] Disk usage above 90% on /dev/sda1
```

48,291 lines → 7 unique messages. Instant.

---

## Installation

**From source (requires [Rust](https://rustup.rs)):**

```sh
git clone https://github.com/kazumaxwell1117/logfold.git
cd logfold
cargo build --release
# binary: ./target/release/logfold
```

**Via cargo install** *(after publishing to crates.io)*:

```sh
cargo install logfold
```

---

## Usage

### Basic — count every unique line

```sh
logfold app.log
```

### Ignore timestamps and variable prefixes

Strip a leading timestamp so lines like these are grouped together:

```
2026-03-13 10:00:01 connection timeout
2026-03-13 10:00:02 connection timeout   ← same message, different time
```

```sh
logfold app.log --ignore-prefix "^[0-9-: ]+"
```

Output:

```
[2x] 2026-03-13 10:00:01 connection timeout
[1x] 2026-03-13 10:00:03 retrying request
```

The original line (with prefix) is preserved in the output — only grouping is affected.

### Show only the top N results

```sh
logfold app.log --ignore-prefix "^[0-9-: ]+" --top 10
```

### Read from stdin

Works in any Unix pipeline:

```sh
cat app.log | logfold --ignore-prefix "^[0-9-: ]+"
kubectl logs my-pod | logfold --ignore-prefix "^[0-9-:TZ.]+ "
journalctl -u nginx | logfold --ignore-prefix "^[A-Za-z]+ [0-9: ]+ \S+ \S+: "
```

---

## Options

| Option | Description |
|---|---|
| `[FILE]` | Input log file. Reads from stdin if omitted. |
| `--ignore-prefix <REGEX>` | Strip a matching prefix before grouping (output is unchanged). |
| `--top <N>` | Show only the top N most frequent results. |
| `--help` | Print help. |
| `--version` | Print version. |

---

## Common Patterns

| Log format | `--ignore-prefix` value |
|---|---|
| `2026-03-13 10:00:01 …` (syslog-style) | `^[0-9-: ]+` |
| `2026-03-13T10:00:01.123Z …` (ISO 8601) | `^\S+Z ` |
| `192.168.1.1 - GET /…` (Nginx access) | `^[0-9.]+ - ` |
| `[ERROR] 2026/03/13 …` (Go log) | `^\[[A-Z]+\] [0-9/: ]+` |

---

## How it works

1. Read lines sequentially from a file or stdin.
2. Normalize each line by stripping `--ignore-prefix` (if set).
3. Group lines by the normalized key; store the first occurrence's original text.
4. Sort by count descending, then alphabetically by normalized key.
5. Print `[Nx] first_seen_line` for each group.

Memory usage is proportional to the number of **unique** messages, not total lines — so it handles large log files efficiently.

---

## License

MIT
