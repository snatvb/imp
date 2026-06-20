import { basename, dirname, extname, resolve } from "path"

import { walk, mkdir, readFile, writeFile, remove, exists } from "imp:fs"

const sourceDir = import.meta.dirname + "/fixtures/mixed"
const targetBase = import.meta.dirname + "/organized"

if (await exists(targetBase)) {
  await remove(targetBase, { recursive: true })
}
await mkdir(targetBase, { recursive: true })

console.log(`Organizing ${sourceDir} → ${targetBase}/...`)
console.log("")

type Plan = { src: string; dest: string; ext: string; size: number }

const extCategory = (ext: string): string => {
  const e = ext.startsWith(".") ? ext.slice(1) : ext
  if (!e) return "no-extension"
  if (["txt", "md", "log"].includes(e)) return "docs"
  if (["json", "yaml", "yml", "toml", "env", "gitignore"].includes(e)) return "config"
  if (["ts", "js", "mjs", "cjs"].includes(e)) return "code"
  if (["html", "css", "scss", "svg"].includes(e)) return "web"
  if (["sh", "bat", "ps1"].includes(e)) return "scripts"
  if (["png", "jpg", "jpeg", "gif", "webp"].includes(e)) return "images"
  return "other"
}

const plans: Plan[] = []
for await (const entry of walk(sourceDir, { absolute: true, filter: "files" })) {
  const src = entry.toString()
  const ext = extname(src).toLowerCase()
  const cat = extCategory(ext)
  const dest = resolve(targetBase, cat, basename(src))
  const buf = await readFile(src, "buffer")
  plans.push({ src, dest, ext: ext || "(none)", size: buf.byteLength })
}

console.log(`Found ${plans.length} files. Categories:`)
const byCat = new Map<string, number>()
for (const p of plans) {
  const cat = extCategory(p.ext === "(none)" ? "" : p.ext)
  byCat.set(cat, (byCat.get(cat) ?? 0) + 1)
}
for (const [cat, n] of [...byCat.entries()].sort((a, b) => b[1] - a[1])) {
  console.log(`  ${cat.padEnd(15)} ${n} files`)
}
console.log("")

const CONCURRENCY = 8

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

async function moveFile(plan: Plan) {
  await mkdir(dirname(plan.dest), { recursive: true })
  const buf = await readFile(plan.src, "buffer")
  await writeFile(plan.dest, buf)
}

const t0 = performance.now()
await runWithLimit(plans, CONCURRENCY, moveFile)
const elapsed = (performance.now() - t0).toFixed(1)

console.log(`Moved ${plans.length} files concurrently (limit=${CONCURRENCY}) in ${elapsed}ms`)
console.log("")

console.log("Final structure:")
for (const [cat, _] of [...byCat.entries()].sort()) {
  const files: string[] = []
  for await (const entry of walk(resolve(targetBase, cat), { filter: "files" })) {
    files.push(basename(entry.toString()))
  }
  console.log(`  ${cat}/`)
  for (const f of files.sort()) {
    console.log(`    ${f}`)
  }
}
