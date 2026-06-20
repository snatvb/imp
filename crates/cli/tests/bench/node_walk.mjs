import { readdir, glob } from "fs/promises"

import { bench, printResult, repoRoot } from "./harness.mjs"

async function main() {
  console.log("=== Walk/Glob Benchmark (Node.js) ===")
  console.log(`repoRoot: ${repoRoot}`)
  console.log("")

  const total = 4

  {
    const iters = 10
    const r = await bench("readdir(repoRoot, recursive) — full traverse", iters, async () => {
      const entries = await readdir(repoRoot, { recursive: true })
    })
    printResult(1, total, "readdir(repoRoot, recursive) — full traverse", iters, Math.floor(iters / 10), r)
  }

  {
    const iters = 10
    const r = await bench("readdir(repoRoot, recursive, withFileTypes)", iters, async () => {
      const entries = await readdir(repoRoot, { recursive: true, withFileTypes: true })
      const files = entries.filter((e) => e.isFile())
    })
    printResult(2, total, "readdir(repoRoot, recursive, withFileTypes)", iters, Math.floor(iters / 10), r)
  }

  {
    const iters = 10
    const r = await bench("readdir + JS filter (ignore target, node_modules)", iters, async () => {
      const entries = await readdir(repoRoot, { recursive: true, withFileTypes: true })
      const filtered = entries.filter((e) => {
        const path = e.parentPath ? e.parentPath + "/" + e.name : e.name
        return !path.includes("target") && !path.includes("node_modules")
      })
    })
    printResult(3, total, "readdir + JS filter (ignore target, node_modules)", iters, Math.floor(iters / 10), r)
  }

  {
    const iters = 20
    const r = await bench("fs.glob('**/*.rs', { cwd })", iters, async () => {
      const results = []
      for await (const entry of glob("**/*.rs", { cwd: repoRoot })) {
        results.push(entry)
      }
    })
    printResult(4, total, "fs.glob('**/*.rs', { cwd })", iters, Math.floor(iters / 10), r)
  }

  console.log("=== Done ===")
}

main()
