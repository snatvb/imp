# md-to-html

Convert Markdown to HTML with a ~70-line handwritten parser. No external
dependencies, just regex and string ops.

## What it shows

- `imp:fs` — `readFile`/`writeFile`
- `path` module — `basename`, `resolve`
- String/regex processing
- Stateful line-by-line parsing
- Top-level `await`

## Supported Markdown

- `#`, `##`, ..., `######` headings
- `**bold**`, `*italic*`, `` `code` ``
- `[text](url)` links
- `- item` unordered lists
- `1. item` ordered lists
- ` ``` code blocks ``` `
- `---` horizontal rules
- Paragraphs

## Run

```bash
imp run main.ts
```

Reads `fixtures/sample.md`, writes `sample.html` next to it.

## Expected output

```
Converted sample.md → sample.html (1234 bytes)

--- first 20 lines ---
<!DOCTYPE html>
<html>
<head><meta charset="utf-8"><title>Document</title></head>
<body>
<h1>ImpJS — Lightweight Async JS Runtime</h1>
<p>ImpJS is an embeddable JavaScript runtime built on <strong>QuickJS</strong> and <strong>tokio</strong>. It
strips TypeScript at parse time, runs async-first, and compiles to a single
standalone executable.</p>
<h2>Features</h2>
...
```

## Why handwritten?

- No npm dependency, no `marked` or `remark` package
- ~70 lines, fully auditable
- Easy to extend for custom syntax

For production use, swap in `imp:parsers` with a YAML frontmatter extractor, or
shell out to `pandoc`. But for ~95% of use cases, this is enough.
