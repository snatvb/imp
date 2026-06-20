# Examples

Real scripts showing how to use imp. Every example is a self-contained
folder you can run straight away.

## Run

```bash
cd fetch-demo
imp run main.ts
```

No `npm install`, no build step.

## What's here

| Example                             | What it shows                                        | Lines |
| ----------------------------------- | ---------------------------------------------------- | ----- |
| [fetch-demo](./fetch-demo/)         | HTTP request, JSON, `Response` API                   | ~25   |
| [config-convert](./config-convert/) | `imp:parsers` (JSON / YAML / TOML round-trip)        | ~40   |
| [csv-stats](./csv-stats/)           | Parallel CSV with `Promise.all` (120 rows / 4 files) | ~80   |
| [md-to-html](./md-to-html/)         | Handwritten markdown parser, zero deps               | ~110  |
| [scaffold](./scaffold/)             | Interactive CLI with `imp:inq`                       | ~75   |
| [log-tail](./log-tail/)             | `tail -f` via `FileHandle` + `setInterval`           | ~55   |
| [file-organizer](./file-organizer/) | Concurrent file sort, bounded worker pool            | ~85   |

## TypeScript

All examples are TypeScript. The `tsconfig.json` and `imp.d.ts` in this
folder give editor support. `imp` strips types at parse time — no build
step.

```bash
# from the project root, with deps installed
npx tsc --noEmit -p examples
```

## Compile any example to a binary

```bash
# Windows
imp compile fetch-demo\main.ts fetch.exe
.\fetch.exe

# macOS / Linux
imp compile fetch-demo/main.ts fetch
./fetch
```

One ~11MB binary, no Node.js needed on the target.

## More

For install, features, and the full pitch, see the
[project README](../README.md).
