import { join } from "path"

import { loadFile } from "imp:env"
import { readFile, writeFile, exists, mkdir, remove } from "imp:fs"
import { sha256 } from "imp:hash"
import { run } from "imp:subprocess"

const PROJECT_ROOT = process.cwd()
export const TEMP_DIR = join(PROJECT_ROOT, "scripts", ".tmp")

export interface Target {
  triple: string | null
  archive: "zip" | "tar.gz"
  suffix: string
  label: string
  buildArgs: string[]
  binDir: string
}

export const TARGETS: Target[] = [
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

export async function sh(cmd: string, args: string[], opts?: { cwd?: string }) {
  const r = await run(cmd, args, opts)
  if (!r.success) {
    console.error(`Command failed: ${cmd} ${args.join(" ")}`)
    console.error(r.stderr)
    process.exit(1)
  }
  return r.stdout
}

export async function fileHash(path: string): Promise<string> {
  const data = await readFile(path, "binary")
  return sha256(data, "hex")
}

export async function loadConfig(): Promise<{ brewDir: string; scoopDir: string }> {
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

export async function buildAll(targets: Target[], dryRun: boolean) {
  console.log("==> Building all targets...")
  if (dryRun) {
    console.log(`  [dry-run] would build ${targets.length} targets`)
    return
  }

  for (const t of targets) {
    console.log(`  Building ${t.label}...`)
    await sh("cargo", t.buildArgs, { cwd: PROJECT_ROOT })
  }
}

export async function packageAll(targets: Target[], version: string, dryRun: boolean) {
  console.log("==> Packaging archives...")
  if (dryRun) {
    console.log(`  [dry-run] would package ${targets.length} archives`)
    return
  }

  await remove(TEMP_DIR, { recursive: true })
  await mkdir(TEMP_DIR, { recursive: true })

  for (const t of targets) {
    const binName = t.suffix ? `imp${t.suffix}` : "imp"
    const binPath = join(t.binDir, binName)
    const archiveName = `imp-${version}-${t.label}`

    if (!(await exists(binPath))) {
      console.error(`Binary not found: ${binPath}`)
      process.exit(1)
    }

    if (t.archive === "zip") {
      const archivePath = join(TEMP_DIR, `${archiveName}.zip`)
      await sh("zip", ["-j", archivePath, binPath], { cwd: PROJECT_ROOT })
    } else {
      const archivePath = join(TEMP_DIR, `${archiveName}.tar.gz`)
      await sh("tar", ["czf", archivePath, "-C", t.binDir, binName], { cwd: PROJECT_ROOT })
    }
    console.log(`  ${archiveName}.${t.archive === "zip" ? "zip" : "tar.gz"}`)
  }
}

export async function computeHashes(
  targets: Target[],
  version: string,
  dryRun: boolean,
): Promise<Record<string, string>> {
  console.log("==> Computing SHA256 hashes...")
  const hashes: Record<string, string> = {}

  for (const t of targets) {
    const archiveName = `imp-${version}-${t.label}`
    const ext = t.archive === "zip" ? ".zip" : ".tar.gz"
    const archivePath = join(TEMP_DIR, `${archiveName}${ext}`)
    if (dryRun) {
      hashes[t.label] = "dry-run"
      console.log(`  ${t.label}: dry-run`)
      continue
    }
    const hash = await fileHash(archivePath)
    hashes[t.label] = hash
    console.log(`  ${t.label}: ${hash}`)
  }

  return hashes
}
