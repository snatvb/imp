import { extname, basename, resolve } from "path"

import { readFile, writeFile } from "imp:fs"
import { json, yaml, toml } from "imp:parsers"

const FORMATS: Record<string, { parse: (s: JsString) => unknown; stringify: (v: unknown) => any }> = {
  json: json,
  yaml: yaml,
  yml: yaml,
  toml: toml,
}

const EXT_OUT: Record<string, string> = {
  json: "yaml",
  yaml: "toml",
  yml: "toml",
  toml: "json",
}

const input = import.meta.dirname + "/fixtures/sample.json"
const rawExt = extname(input).toLowerCase().replace(/^\./, "")
const inFmt = FORMATS[rawExt]
if (!inFmt) {
  console.error(`unsupported input format: ${rawExt}`)
  process.exit(1)
}

const name = basename(input, "." + rawExt)
const outExt = EXT_OUT[rawExt] ?? "json"
const outFmt = FORMATS[outExt]
if (!outFmt) {
  console.error(`unsupported output format: ${outExt}`)
  process.exit(1)
}

console.log(`Reading ${basename(input)}...`)
const src = await readFile(input, "utf8")
const data = inFmt.parse(src)

console.log(`Converting ${rawExt} → ${outExt}...`)
const out = outFmt.stringify(data)

const outPath = resolve(import.meta.dirname, `${name}.${outExt}`)
await writeFile(outPath, out)
console.log(`Wrote ${basename(outPath)}`)
console.log("")
console.log(out)
