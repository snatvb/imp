# ImpJS

TypeScript or JavaScript → single native binary. No Node.js, no `node_modules`, no build step.

```bash
cargo install --git https://github.com/snatvb/imp crates/cli
imp run hello.ts
```

## Why

You write a CLI tool in TypeScript. Today you ship a 200MB `node_modules`
folder and hope the user has the right Node version. With imp you ship
**one ~11MB binary** that just runs.

|              | Node.js           | ImpJS                         |
| ------------ | ----------------- | ----------------------------- |
| Distribution | needs Node + deps | one binary (`.exe` or no ext) |
| Cold start   | ~80ms             | <5ms                          |
| TypeScript   | needs build       | native (oxc at parse)         |
| Bin size     | ~100MB            | ~11MB                         |

## What you get

- `.ts` runs directly, no transpiler
- `fetch`, `URL`, `Headers`, `AbortController` — standard Web APIs
- `imp:fs` — `readFile`, `writeFile`, `walk`, `glob`, `FileHandle`, `WriteHandle`
- `imp:parsers` — JSON, YAML, TOML, XML, RON, CSV, MessagePack in one module
- `imp:inq` — `confirm`, `select`, `prompt`, `multiSelect`, `password`, `editor`
- `imp:clap`, `imp:time` — CLI args and date/time types
- `Duration`, `ByteBuffer`, `RsString` — Rust-backed primitives

## Quick start

```bash
imp run hello.ts
imp run path/to/script.ts arg1 arg2
```

```typescript
// hello.ts
import clap from "imp:clap"

const parser = new clap.Parser()
  .name("hello")
  .arg({ name: "name", long: "name", action: "set", help: "who to greet" })
  .arg({
    name: "count",
    short: "c",
    long: "count",
    action: "set",
    help: "how many times",
  })

const result = parser.parse(clap.args)
if (String(result.type) === "result") {
  const name = String(result.name ?? "world")
  const count = Number(result.count ?? 1)
  for (let i = 0; i < count; i++) {
    console.log(`Hello, ${name}!`)
  }
}
```

```bash
$ imp run hello.ts --name Alice --count 3
Hello, Alice!
Hello, Alice!
Hello, Alice!

$ imp run hello.ts --help
Usage: hello [OPTIONS] [NAME]
```

## Compile to a binary

The whole point. One file, no runtime, no installer on the target.

```bash
# Windows
imp compile hello.ts hello.exe
.\hello.exe

# macOS / Linux
imp compile hello.ts hello
./hello
```

On Windows you can omit `.exe` and it's added automatically. On macOS / Linux
the output keeps the exact name you pass.

The result is **a single ~11MB binary** containing the QuickJS runtime and
your bundled script. No Node, no `node_modules`, no shared libs. Copy it
to another machine, run it.

## CLI

```bash
imp <file>                  # run a script
imp run <file> [args...]    # same, explicit
imp compile <file> <output> # bundle to native binary
imp init [path]             # scaffold imp.d.ts + tsconfig.json
```

## Examples

See [`examples/`](./examples/) — 7 real-world scripts: HTTP client,
config format converter, parallel CSV stats, markdown renderer, interactive
scaffolder, `tail -f` clone, and a concurrent file sorter.

## Install

```bash
cargo install --git https://github.com/snatvb/imp crates/cli
```

This builds the `imp` binary and puts it in `~/.cargo/bin/`. Add that to
your `PATH` if it isn't already.

## License

MIT — <https://github.com/snatvb/imp>
