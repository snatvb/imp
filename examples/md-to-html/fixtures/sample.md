# ImpJS — Lightweight Async JS Runtime

ImpJS is an embeddable JavaScript runtime built on **QuickJS** and **tokio**. It
strips TypeScript at parse time, runs async-first, and compiles to a single
standalone executable.

## Features

- **Single binary** — no Node.js needed on the target machine
- **TypeScript** — no build step, just write `.ts`
- **Async-first** — tokio underneath, true concurrent I/O
- **7 data formats** — JSON, YAML, TOML, XML, RON, CSV, MessagePack
- **Interactive prompts** — `inq` module for CLI input
- **Fast startup** — QuickJS boots in microseconds

## Quick Start

```bash
imp run main.ts
```

That's it. No `npm install`, no `tsconfig.json` required.

## Installation

```bash
cargo install imp
```

## Why ImpJS?

| Concern      | Node.js           | ImpJS                           |
| ------------ | ----------------- | ------------------------------- |
| Runtime size | ~100MB            | ~11MB single binary             |
| Startup      | ~80ms             | <5ms                            |
| Distribution | needs Node + deps | single binary (`.exe` / no ext) |
| TypeScript   | needs build       | native                          |

## Examples

- `fetch-demo/` — HTTP request
- `config-convert/` — JSON ↔ YAML ↔ TOML
- `csv-stats/` — parallel CSV processing
- `md-to-html/` — markdown converter
- `scaffold/` — interactive project generator
- `log-tail/` — tail -f clone
- `file-organizer/` — concurrent file sorter

## License

MIT
