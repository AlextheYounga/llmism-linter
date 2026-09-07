# LLMism Linter

`llmisms` flags stock phrases that can make prose sound preassembled. It scans a
single Markdown or text file, or recursively scans a directory for `.md` and
`.txt` files.



```shell
llmisms ./writings
```

```
./writings/draft-2023.v1.md:71:45: warning: abstract-noun fog (`landscape`)
./writings/draft-2023.v1.md:181:45: warning: abstract-noun fog (`landscape`)
./writings/draft-2024.v3.md:57:84: warning: manufactured bottom line (`Ultimately`)
./writings/draft-2024.v3.md:116:1: warning: significance announcement (variation of `This is a crucial point`)
./writings/draft-2024.v3.md:128:189: warning: abstract-noun fog (`framework`)
./writings/draft-2024.v3.md:152:1: warning: balanced duality (variation of `These aren't mutually exclusive`)
./writings/draft-2024.v3.md:160:172: warning: abstract-noun fog (`framework`)
./writings/draft-2024.v3.md:178:829: warning: unearned gold star (variation of `That's an excellent question`)
./writings/draft-2025.v4.md:55:68: warning: manufactured bottom line (`Ultimately`)
./writings/draft-2025.v4.md:86:822: warning: unearned gold star (variation of `That's an excellent question`)
./writings/draft-2025.v4.md:108:46: warning: abstract-noun fog (`landscape`)
./writings/draft-2025.v4.md:176:381: warning: unpacking ritual (variation of `Let's break this down`)
./writings/draft:latest.md:35:390: warning: compulsory caveat (variation of `It's important to note`)
./writings/draft:latest.md:171:154: warning: abstract-noun fog (`complexity`)
./writings/draft:latest.md:177:162: warning: question-fragment reveal (variation of `The problem? Complexity`)
./writings/final/part-1.md:112:75: warning: abstract-noun fog (`landscape`)
./writings/final/part-1.md:182:163: warning: abstract-noun fog (`complexity`)
./writings/final/part-1.md:188:155: warning: question-fragment reveal (variation of `The problem? Complexity`)
./writings/final/part-2.md:92:153: warning: abstract-noun fog (`dynamics`)
./writings/final/part-3.md:15:1: warning: significance announcement (`That distinction matters`)
./writings/final/part-3.md:189:255: warning: abstract-noun fog (`framework`)
./writings/final/part-3.md:191:119: warning: abstract-noun fog (`framework`)
```


## Installation

Install the CLI directly from GitHub:

```shell
cargo install --git https://github.com/AlextheYounga/llmism-linter.git
```

Cargo installs the `llmisms` binary into `~/.cargo/bin`; ensure that directory
is on your `PATH`, then run it from any local project:

```shell
llmisms ./docs
```

To install the checked-out repository instead, run:

```shell
cargo install --path .
```

## Usage

```shell
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
