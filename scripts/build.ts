import { TARGETS, sh, buildAll, packageAll, computeHashes } from "./lib.ts"
import clap from "imp:clap"

const parser = new clap.Parser()
  .name("build")
  .about("Cross-compile imp for all platforms")
  .arg({ name: "dry-run", short: "n", long: "dry-run", help: "Show what would be done", action: "flag" })

const result = parser.parse(clap.args)

if (result.type !== "result") {
  if (result.type === "error") {
    console.error(result.message)
  }
  process.exit(1)
}

const dryRun = Boolean(result["dry-run"])

await buildAll(TARGETS, dryRun)
await packageAll(TARGETS, "local", dryRun)
const hashes = await computeHashes(TARGETS, "local", dryRun)

console.log("\n==> Summary:")
for (const [label, hash] of Object.entries(hashes)) {
  console.log(`  ${label}: ${hash}`)
}

console.log("\nDone!")
