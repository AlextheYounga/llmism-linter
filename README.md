# LLMism Linter

`llmisms` flags stock phrases that can make prose sound preassembled. It scans a
single Markdown or text file, or recursively scans a directory for `.md` and
`.txt` files.

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
