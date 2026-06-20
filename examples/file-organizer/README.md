# file-organizer

Sort a chaotic directory by file extension into category folders. Walks the
source tree, classifies each file, and moves them **concurrently** with a
bounded worker pool.

## What it shows

- `walk()` — `AsyncIterable` recursive directory traversal
- `imp:fs` — `mkdir` (recursive), `readFile`, `writeFile`, `remove`, `exists`
- **Bounded concurrency** — `Promise.race` worker pool pattern
- `path.extname`, `path.dirname`, `path.basename`
- Top-level `await`

## Run

```bash
imp run main.ts
```

Organizes 28 fixture files in `fixtures/mixed/` into subfolders under
`organized/`:

| Category   | Extensions                                      |
| ---------- | ----------------------------------------------- |
| `docs/`    | `.txt`, `.md`, `.log`                           |
| `config/`  | `.json`, `.yaml`, `.toml`, `.env`, `.gitignore` |
| `code/`    | `.ts`, `.js`, `.mjs`, `.cjs`                    |
| `web/`     | `.html`, `.css`, `.scss`, `.svg`                |
| `scripts/` | `.sh`, `.bat`, `.ps1`                           |
| `images/`  | `.png`, `.jpg`, `.gif`, `.webp`                 |
| `other/`   | anything else                                   |

## Expected output

```
Organizing fixtures/mixed → organized/...

Found 28 files. Categories:
  config         7 files
  docs           7 files
  code           4 files
  web            4 files
  scripts        2 files
  other          2 files
  no-extension   2 files

Moved 28 files concurrently (limit=8) in <50ms

Final structure:
  code/
    helper.js
    helper.ts
    script.js
    script.ts
  config/
    .env
    .gitignore
    Cargo.toml
    config.yaml
    package.json
    tsconfig.app.json
    tsconfig.json
  ...
```

## The concurrency pattern

```ts
async function runWithLimit<T>(items: T[], limit: number, fn: (item: T) => Promise<void>) {
  const executing: Promise<void>[] = []
  for (const item of items) {
    const p = fn(item).then(() => {
      executing.splice(executing.indexOf(p), 1)
    })
    executing.push(p)
    if (executing.length >= limit) {
      await Promise.race(executing)
    }
  }
  await Promise.all(executing)
}
```

This is the standard "worker pool" pattern. Bounded at 8 concurrent ops
prevents disk thrashing while still parallelizing 28 file moves ~3-4× faster
than sequential.

For 1M files, increase the limit and watch throughput scale linearly with
disk I/O bandwidth.

## Compile to a real tool

`imp compile` embeds the runtime + your script into a single native binary
that runs without Node.js installed.

```bash
# Windows
imp compile main.ts organize.exe
.\organize.exe

# macOS / Linux
imp compile main.ts organize
./organize
```

Now you have a CLI tool that cleans up `Downloads/` chaos. Ship the binary
(`.exe` on Windows, no extension on macOS / Linux).
