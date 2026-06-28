import { join } from "path"

import clap from "imp:clap"
import { readFile, writeFile } from "imp:fs"

import { TARGETS, TEMP_DIR, sh, loadConfig, buildAll, packageAll, computeHashes } from "./lib.ts"

const PROJECT_ROOT = process.cwd()

const parser = new clap.Parser()
  .name("release")
  .about("Cross-platform release for imp")
  .arg({ name: "version", help: "Release version (e.g. v0.1.0)", action: "set", required: true })
  .arg({ name: "dry-run", short: "n", long: "dry-run", help: "Show what would be done", action: "flag" })

async function updateFormula(version: string, hashes: Record<string, string>, brewDir: string) {
  console.log("==> Updating Homebrew formula...")
  const formulaPath = join(brewDir, "Formula/imp.rb")
  let content = (await readFile(formulaPath, "utf8")).toString()
  content = content.replaceAll('"0.1.0"', `"${version}"`)
  content = content.replace("PLACEHOLDER_MAC_ARM64_SHA256", hashes["mac-arm64"])
  content = content.replace("PLACEHOLDER_LINUX_ARM64_SHA256", hashes["linux-arm64"])
  content = content.replace("PLACEHOLDER_LINUX_X64_SHA256", hashes["linux-x64"])
  await writeFile(formulaPath, content)
  console.log(`  Updated ${formulaPath}`)
}

async function updateManifest(version: string, hashes: Record<string, string>, scoopDir: string) {
  console.log("==> Updating Scoop manifest...")
  const manifestPath = join(scoopDir, "imp.json")
  let content = (await readFile(manifestPath, "utf8")).toString()
  content = content.replaceAll('"0.1.0"', `"${version}"`)
  content = content.replace("PLACEHOLDER_WIN64_SHA256", hashes["windows-x64"])
  await writeFile(manifestPath, content)
  console.log(`  Updated ${manifestPath}`)
}

async function pushBuckets(brewDir: string, scoopDir: string, dryRun: boolean) {
  console.log("==> Pushing bucket updates...")
  if (dryRun) {
    console.log("  [dry-run] would push both buckets")
    return
  }

  await sh("git", ["add", "."], { cwd: brewDir })
  await sh("git", ["commit", "-m", "Update imp formula"], { cwd: brewDir })
  await sh("git", ["push"], { cwd: brewDir })
  console.log("  Pushed homebrew-tap")

  await sh("git", ["add", "."], { cwd: scoopDir })
  await sh("git", ["commit", "-m", "Update imp manifest"], { cwd: scoopDir })
  await sh("git", ["push"], { cwd: scoopDir })
  console.log("  Pushed scoop-bucket")
}

async function createTag(version: string, dryRun: boolean) {
  console.log("==> Creating git tag...")
  if (dryRun) {
    console.log(`  [dry-run] would create tag ${version}`)
    return
  }

  await sh("git", ["tag", "-a", version, "-m", version], { cwd: PROJECT_ROOT })
  await sh("git", ["push", "origin", version], { cwd: PROJECT_ROOT })
  console.log(`  Tag ${version} pushed`)
}

async function createRelease(version: string, dryRun: boolean) {
  console.log("==> Creating GitHub release...")
  if (dryRun) {
    console.log("  [dry-run] would create release with 4 assets")
    return
  }

  const assetArgs: string[] = []
  for (const t of TARGETS) {
    const archiveName = `imp-${version}-${t.label}`
    const ext = t.archive === "zip" ? ".zip" : ".tar.gz"
    const archivePath = join(TEMP_DIR, `${archiveName}${ext}`)
    assetArgs.push(`${archivePath}#${archiveName}${ext}`)
  }

  await sh("gh", ["release", "create", version, "--title", version, "--notes", `Release ${version}`, ...assetArgs], {
    cwd: PROJECT_ROOT,
  })

  console.log(`  Release ${version} created`)
}

const result = parser.parse(clap.args)

if (result.type !== "result") {
  if (result.type === "error") {
    console.error(result.message)
  }
  process.exit(1)
}

const version = result.version.toString()
const dryRun = Boolean(result["dry-run"])

if (!version.startsWith("v")) {
  console.error("Version must start with 'v' (e.g. v0.1.0)")
  process.exit(1)
}

const config = await loadConfig()

console.log(`Releasing imp ${version}${dryRun ? " (dry-run)" : ""}\n`)

await buildAll(TARGETS, dryRun)
await packageAll(TARGETS, version, dryRun)
const hashes = await computeHashes(TARGETS, version, dryRun)
await updateFormula(version, hashes, config.brewDir)
await updateManifest(version, hashes, config.scoopDir)
await pushBuckets(config.brewDir, config.scoopDir, dryRun)
await createTag(version, dryRun)
await createRelease(version, dryRun)

console.log(`\nDone! Released imp ${version}`)
