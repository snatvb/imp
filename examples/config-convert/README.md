# config-convert

Convert config files between JSON, YAML, and TOML using imp's `imp:parsers` module.

## What it shows

- `imp:parsers` — 7 data format parsers in one module
- `path` module — `extname`, `basename`, `resolve`
- `imp:fs` — `readFile`, `writeFile`
- `process.exit()` for error handling
- Top-level `await`

## Run

```bash
imp run main.ts
```

Default reads `fixtures/sample.json` and writes `sample.yaml` next to it.
Edit the `input` variable in `main.ts` to convert a different file.

## Supported conversions

| Input          | Output  | Roundtrip |
| -------------- | ------- | --------- |
| `.json`        | `.yaml` | yes       |
| `.yaml`/`.yml` | `.toml` | yes       |
| `.toml`        | `.json` | yes       |

## Expected output

```
Reading sample.json...
Converting json → yaml...
Wrote sample.yaml

name: imp
version: 0.1.0
description: Lightweight async JS runtime
...
```

## Try other formats

```ts
const input = import.meta.dirname + "/fixtures/sample.yaml" // → sample.toml
const input = import.meta.dirname + "/fixtures/sample.toml" // → sample.json
```

## Why this matters

Most languages need a separate library for each format (e.g. `serde_json` +
`serde_yaml` + `toml` crate in Rust, 3 npm packages in Node.js). imp bundles
all major formats into a single zero-config `imp:parsers` module.
