# ImpJS

Everything that you need to create CLI easy! TypeScript or JavaScript → single native binary. No Node.js, no `node_modules`, no build step.

## Installation

### Quick install (Linux/macOS)

```bash
curl -fsSL https://raw.githubusercontent.com/snatvb/imp/main/scripts/install.sh | bash
```

This will:
- Auto-detect your OS and architecture
- Download the latest release to `~/.local/bin/imp`
- Add `~/.local/bin` to your PATH if needed

### Install specific version

```bash
curl -fsSL https://raw.githubusercontent.com/snatvb/imp/main/scripts/install.sh | bash -s -- v0.1.0
```

### Homebrew (macOS/Linux)

```bash
brew tap snatvb/brew git@github.com:snatvb/brew.git
brew install snatvb/brew/imp
```

### Scoop (Windows)

```bash
scoop bucket add imp https://github.com/snatvb/scoop-imp
scoop install imp
```

### Cargo (from source)

```bash
cargo install imp-cli
```

### Manual download

Download the latest binary from [GitHub Releases](https://github.com/snatvb/imp/releases).

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
- `imp:fs` — `readFile`, `writeFile`, `walk`, `glob`, `FileHandle`, `WriteHandle`
- `imp:parsers` — JSON, YAML, TOML, XML, RON, CSV, MessagePack in one module
- `imp:crypto` — `randomBytes`, `randomHex`, `randomUUID`, `randomInt`, `hmac`, `aesEncrypt`, `aesDecrypt`, `timingSafeEqual`
- `imp:encoding` — `base64`, `hex`, `utf8`, `uri` encode/decode
- `imp:hash` — `md5`, `sha1`, `sha256`, `sha512`, `blake3`
- `imp:env` — `parseIni`, `parseDotenv`, `expand`, `merge`, `loadFile`
- `imp:inq` — `confirm`, `select`, `prompt`, `multiSelect`, `password`, `editor`
- `imp:subprocess` — `run(cmd, args, options)` with `cwd`/`env`/`input`/`timeout`/`signal`/`encoding`
- `imp:signal` — OS signal handlers (SIGINT, SIGTERM, SIGHUP, SIGQUIT, SIGBREAK)
- `imp:time` — `Duration`, `ImpDate`, `ImpTime`, `ImpDateTime`, `ImpLocalDateTime`
- `imp:clap` — CLI argument parsing with `Parser`
- `imp:sys` — `inputSimulate`, `stdin` for system-level I/O
- `Duration`, `ByteBuffer`, `RsString` — Rust-backed primitives

## Quick start

```bash
# scaffold TypeScript types (optional — for IDE support)
imp init

# run a script
imp run hello.ts
imp run path/to/script.ts arg1 arg2
```

```typescript
// hello.ts
import { readFile, writeFile } from "imp:fs"
import { json, yaml } from "imp:parsers"

const raw = await readFile("config.json", "utf8")
const data = json.parse(raw)

await writeFile("config.yaml", yaml.stringify(data))
console.log("Converted JSON → YAML")
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

## API Examples

### File system

```typescript
import { readFile, writeFile, walk, glob, open, openWrite } from "imp:fs"

const text = await readFile("data.txt", "utf8")
await writeFile("out.txt", "result")

for await (const entry of walk("./src")) {
  console.log(entry.path)
}

const files = await glob("./src", "**/*.ts")
for (const file of files) {
  console.log(file)
}

// Streaming with FileHandle — using auto-closes via Symbol.dispose
using fh = await open("large.bin", 4096)
const chunk = await fh.read()

// WriteHandle
using wh = await openWrite("output.txt", "w")
await wh.write("hello")
```

### Parsers

```typescript
import { json, yaml, toml, csv, xml } from "imp:parsers"

const obj = json.parse('{"a": 1}')
const str = yaml.stringify({ key: "value" })

const cfg = toml.parse("[server]\nport = 8080")
const rows = csv.parse("name,age\nAlice,30")
```

### Crypto

```typescript
import { randomBytes, randomUUID, hmac, aesEncrypt, aesDecrypt } from "imp:crypto"

const bytes = randomBytes(32)
const id = randomUUID()
const sig = hmac("sha256", "secret", "message")

const key = randomBytes(32)
const iv = randomBytes(12)
const encrypted = aesEncrypt("aes-256-gcm", key, plaintext)
const decrypted = aesDecrypt("aes-256-gcm", key, encrypted)
```

### Encoding

```typescript
import { base64, hex, utf8, uri } from "imp:encoding"

const encoded = base64.encode("hello")
const decoded = base64.decode(encoded)

const hexStr = hex.encode(bytes)
const buf = utf8.encode("text")
const safe = uri.encode("path/to/file")
```

### Hashing

```typescript
import { sha256, blake3 } from "imp:hash"

const hash = sha256("hello world")
const h3 = blake3("data", "hex")
```

### Environment

```typescript
import { parseIni, parseDotenv, expand, loadFile } from "imp:env"

const ini = parseIni("[db]\nhost = localhost\nport = 5432")
const env = parseDotenv("FOO=bar\nBAZ=qux")
const expanded = expand("HOME=$HOME/user")
const config = await loadFile(".env")
```

### Interactive prompts

```typescript
import { prompt, select, confirm, multiSelect } from "imp:inq"

const name = await prompt("What is your name?")
const choice = await select("Pick one", ["a", "b", "c"])
const ok = await confirm("Continue?", true)
const picks = await multiSelect("Pick many", ["x", "y", "z"])
```

### Subprocess

```typescript
import { run } from "imp:subprocess"

const result = await run("git", ["status"])
console.log(result.stdout, result.code)

const r2 = await run("echo", ["hello"], { timeout: 5000 })
```

### Signals

```typescript
import { signal } from "imp:signal"

const dispose = signal.on("SIGINT", () => {
  console.log("Interrupted!")
  process.exit(0)
})

signal.once("SIGTERM", () => cleanup())
```

### Date and time

```typescript
import { Duration, ImpDate, ImpDateTime } from "imp:time"

const d = Duration.seconds(30)
const today = ImpDate.today()
const now = ImpDateTime.now()
const fmt = now.format("%Y-%m-%d %H:%M:%S")
```

## Built-in globals

No imports needed — available everywhere:

| Global                                      | Description                                                                  |
| ------------------------------------------- | ---------------------------------------------------------------------------- |
| `console`                                   | `log`, `error`, `warn`, `info`, `assert`, `trace`                            |
| `process`                                   | `cwd()`, `exit()`, `env`, `argv`, `pid`, `platform`                          |
| `path`                                      | `resolve`, `join`, `basename`, `dirname`, `extname`, `relative`, `normalize` |
| `fetch`                                     | HTTP client (Web standard `fetch`, `Request`, `Response`, `Headers`)         |
| `URL`, `URLSearchParams`                    | URL parsing and manipulation                                                 |
| `AbortController`, `AbortSignal`            | Request cancellation                                                         |
| `setTimeout`, `setInterval`, `setImmediate` | Async timers                                                                 |
| `TextEncoder`, `TextDecoder`                | UTF-8 encode/decode                                                          |
| `Buffer`                                    | Byte buffer (Node.js compatible subset)                                      |

## CLI args

```typescript
import clap from "imp:clap"

const parser = new clap.Parser()
  .name("hello")
  .arg({ name: "name", long: "name", action: "set", help: "who to greet" })
  .arg({ name: "count", short: "c", long: "count", action: "set", help: "how many times" })

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

## Building from source

Cross-compile Windows and Linux binaries from macOS:

```bash
# Install cross-compilation toolchains
brew install mingw-w64                              # Windows x64 linker
brew install messense/macos-cross-toolchains        # Linux x86_64 linker
cargo install cargo-zigbuild && brew install zig    # Linux ARM64

# Build all targets
cargo build --release -p cli                                            # macOS arm64
cargo build --release --target x86_64-pc-windows-gnu -p cli            # Windows x64
cargo build --release --target x86_64-unknown-linux-gnu -p cli         # Linux x86_64
cargo zigbuild --release --target aarch64-unknown-linux-gnu -p cli     # Linux ARM64
```

Output binaries:

| Target       | Path                                           | Size    |
| ------------ | ---------------------------------------------- | ------- |
| macOS arm64  | `target/release/imp`                           | ~11 MB  |
| Windows x64  | `target/x86_64-pc-windows-gnu/release/imp.exe` | ~9.6 MB |
| Linux x86_64 | `target/x86_64-unknown-linux-gnu/release/imp`  | ~10 MB  |
| Linux ARM64  | `target/aarch64-unknown-linux-gnu/release/imp` | ~8.7 MB |

All binaries are self-contained — no system OpenSSL/libssl required. Only `glibc` and `ca-certificates` needed on target Linux machines.

## License

MIT — <https://github.com/snatvb/imp>
