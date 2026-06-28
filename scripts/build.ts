import clap from "imp:clap"

import { TARGETS, buildAll, packageAll } from "./lib.ts"

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

console.log("\nDone!")
