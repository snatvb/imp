import { run } from "imp:subprocess"
import { readFile, writeFile, exists } from "imp:fs"
import { sha256 } from "imp:hash"
import { loadFile } from "imp:env"
import { join } from "path"
import clap from "imp:clap"

const PROJECT_ROOT = process.cwd()

const parser = new clap.Parser()
  .name("release")
  .about("Cross-platform release for imp")
  .arg({ name: "version", short: "v", long: "version", help: "Release version (e.g. v0.1.0)", action: "set", required: true })
  .arg({ name: "dry-run", short: "n", long: "dry-run", help: "Show what would be done", action: "flag" })

interface Target {
  triple: string | null
  archive: "zip" | "tar.gz"
  suffix: string
  label: string
  buildArgs: string[]
  binDir: string
}

const TARGETS: Target[] = [
  {
    triple: null,
    archive: "tar.gz",
    suffix: "",
    label: "mac-arm64",
    buildArgs: ["build", "--release", "-p", "cli"],
    binDir: join(PROJECT_ROOT, "target", "release"),
  },
  {
    triple: "x86_64-pc-windows-gnu",
    archive: "zip",
    suffix: ".exe",
    label: "windows-x64",
    buildArgs: ["build", "--release", "--target", "x86_64-pc-windows-gnu", "-p", "cli"],
    binDir: join(PROJECT_ROOT, "target", "x86_64-pc-windows-gnu", "release"),
  },
  {
    triple: "x86_64-unknown-linux-gnu",
    archive: "tar.gz",
    suffix: "",
    label: "linux-x64",
    buildArgs: ["build", "--release", "--target", "x86_64-unknown-linux-gnu", "-p", "cli"],
    binDir: join(PROJECT_ROOT, "target", "x86_64-unknown-linux-gnu", "release"),
  },
  {
    triple: "aarch64-unknown-linux-gnu",
    archive: "tar.gz",
    suffix: "",
    label: "linux-arm64",
    buildArgs: ["zigbuild", "--release", "--target", "aarch64-unknown-linux-gnu", "-p", "cli"],
    binDir: join(PROJECT_ROOT, "target", "aarch64-unknown-linux-gnu", "release"),
  },
]

async function sh(cmd: string, args: string[], opts?: { cwd?: string }) {
  const r = await run(cmd, args, opts)
  if (!r.success) {
    console.error(`Command failed: ${cmd} ${args.join(" ")}`)
    console.error(r.stderr)
    process.exit(1)
  }
  return r.stdout
}

async function fileHash(path: string): Promise<string> {
  const data = await readFile(path, "binary")
  return sha256(data, "hex")
}

async function loadConfig(): Promise<{ brewDir: string; scoopDir: string }> {
  const envPath = join(PROJECT_ROOT, ".env.release")
  if (!(await exists(envPath))) {
    console.error("Missing .env.release. Copy .env.example to .env.release and fill in paths.")
    process.exit(1)
  }
  const env = await loadFile(envPath)
  return {
    brewDir: String(env.BREW_DIR),
    scoopDir: String(env.SCOOP_DIR),
  }
}

async function buildAll(dryRun: boolean) {
  console.log("==> Building all targets...")
  if (dryRun) {
    console.log("  [dry-run] would build 4 targets")
    return
  }

  for (const t of TARGETS) {
    console.log(`  Building ${t.label}...`)
    await sh("cargo", t.buildArgs, { cwd: PROJECT_ROOT })
  }
}

async function packageAll(version: string, dryRun: boolean) {
  console.log("==> Packaging archives...")
  if (dryRun) {
    console.log("  [dry-run] would package 4 archives")
    return
  }

  for (const t of TARGETS) {
    const binName = t.suffix ? `imp${t.suffix}` : "imp"
    const binPath = join(t.binDir, binName)
    const archiveName = `imp-${version}-${t.label}`

    if (!(await exists(binPath))) {
      console.error(`Binary not found: ${binPath}`)
      process.exit(1)
    }

    if (t.archive === "zip") {
      const archivePath = join(PROJECT_ROOT, `${archiveName}.zip`)
      await sh("zip", ["-j", archivePath, binPath], { cwd: PROJECT_ROOT })
    } else {
      const archivePath = join(PROJECT_ROOT, `${archiveName}.tar.gz`)
      await sh("tar", ["czf", archivePath, "-C", t.binDir, binName], { cwd: PROJECT_ROOT })
    }
    console.log(`  ${archiveName}.${t.archive === "zip" ? "zip" : "tar.gz"}`)
  }
}

async function computeHashes(version: string): Promise<Record<string, string>> {
  console.log("==> Computing SHA256 hashes...")
  const hashes: Record<string, string> = {}

  for (const t of TARGETS) {
    const archiveName = `imp-${version}-${t.label}`
    const ext = t.archive === "zip" ? ".zip" : ".tar.gz"
    const archivePath = join(PROJECT_ROOT, `${archiveName}${ext}`)
    const hash = await fileHash(archivePath)
    hashes[t.label] = hash
    console.log(`  ${t.label}: ${hash}`)
  }

  return hashes
}

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
    const archivePath = join(PROJECT_ROOT, `${archiveName}${ext}`)
    assetArgs.push(`${archivePath}#${archiveName}${ext}`)
  }

  await sh("gh", [
    "release", "create", version,
    "--title", version,
    "--notes", `Release ${version}`,
    ...assetArgs,
  ], { cwd: PROJECT_ROOT })

  console.log(`  Release ${version} created`)
}

const result = parser.parse(clap.args)

if (result.type !== "result") {
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

await buildAll(dryRun)
await packageAll(version, dryRun)
const hashes = await computeHashes(version)
await updateFormula(version, hashes, config.brewDir)
await updateManifest(version, hashes, config.scoopDir)
await pushBuckets(config.brewDir, config.scoopDir, dryRun)
await createTag(version, dryRun)
await createRelease(version, dryRun)

console.log(`\nDone! Released imp ${version}`)
