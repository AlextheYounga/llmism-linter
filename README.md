# LLMism Linter

`llmisms` flags stock phrases that can make prose sound preassembled. It scans a
single Markdown or text file, or recursively scans a directory for `.md` and
`.txt` files.

## Installation

Install the CLI directly from GitHub:

```console
cargo install --git https://github.com/<owner>/llmism-linter.git
```

Replace `<owner>` with the GitHub account or organization that hosts this
repository. Cargo installs the `llmisms` binary into `~/.cargo/bin`; ensure that
directory is on your `PATH`, then run it from any local project:

```console
llmisms ./docs
```

To install the checked-out repository instead, run:

```console
cargo install --path .
```

## Usage

```console
cargo run -- README.md
cargo run -- ./docs
```

Each finding includes its file, line, column, rule name, and matched phrase:

```text
draft.md:12:1: warning: unpacking ritual (`Let's unpack that`)
```

The process exits with status `0` when no phrases are found, `1` when it reports
findings, and `2` when it cannot scan the requested input.

The rule set matches the fixed stock phrases listed in `CONTEXT.md`,
case-insensitively. For examples containing at least two meaningful words, it
also detects stemmed variations when those words occur in the same sentence.
For example, `Those distinctions still mattered` is treated as a variation of
`That distinction matters`.

The linter does not attempt to infer broad structural habits such as excessive
recaps, suspicious symmetry, or compulsive outlining.
