import { walk, glob, globStream } from "imp:fs"

import { bench, printResult, repoRoot } from "./harness"

async function main() {
  console.log("=== Walk/Glob Benchmark (imp) ===")
  console.log(`repoRoot: ${repoRoot}`)
  console.log("")

  const total = 5

  {
    const iters = 10
    const r = await bench("walk(repoRoot) — full traverse", iters, async () => {
      let count = 0
      for await (const _ of walk(repoRoot)) {
        count++
      }
    })
    printResult(1, total, "walk(repoRoot) — full traverse", iters, Math.floor(iters / 10), r)
  }

  {
    const iters = 10
    const r = await bench("walk(repoRoot, { filter: 'files' })", iters, async () => {
      let count = 0
      for await (const _ of walk(repoRoot, { filter: "files" })) {
        count++
      }
    })
    printResult(2, total, "walk(repoRoot, { filter: 'files' })", iters, Math.floor(iters / 10), r)
  }

  {
    const iters = 10
    const r = await bench("walk(repoRoot, { ignore: target, node_modules })", iters, async () => {
      let count = 0
      for await (const _ of walk(repoRoot, { ignore: ["**/target/**", "**/node_modules/**"] })) {
        count++
      }
    })
    printResult(3, total, "walk(repoRoot, { ignore: target, node_modules })", iters, Math.floor(iters / 10), r)
  }

  {
    const iters = 20
    const r = await bench("glob(repoRoot, '**/*.rs')", iters, async () => {
      const results = await glob(repoRoot, "**/*.rs")
    })
    printResult(4, total, "glob(repoRoot, '**/*.rs')", iters, Math.floor(iters / 10), r)
  }

  {
    const iters = 10
    const r = await bench("globStream(repoRoot, '**/*.ts')", iters, async () => {
      let count = 0
      for await (const _ of globStream(repoRoot, "**/*.ts")) {
        count++
      }
    })
    printResult(5, total, "globStream(repoRoot, '**/*.ts')", iters, Math.floor(iters / 10), r)
  }

  console.log("=== Done ===")
}

main()
