import { resolve } from "path"

import { open, exists } from "imp:fs"

const logPath = import.meta.dirname + "/fixtures/sample.log"

if (!(await exists(logPath))) {
  console.error(`file not found: ${logPath}`)
  process.exit(1)
}

console.log(`Tailing ${logPath} (Ctrl+C to stop)...`)
console.log("")

const POLL_MS = 200
let position = 0
let buffer = ""

function colorize(line: string): string {
  if (line.includes(" ERROR ")) return `[ERROR] ${line}`
  if (line.includes(" WARN ")) return `[WARN]  ${line}`
  if (line.includes(" INFO ")) return `[INFO]  ${line}`
  return `        ${line}`
}

function emit(pending: string) {
  const lines = pending.split("\n")
  for (const line of lines) {
    if (line.length > 0) {
      const ts = new Date().toISOString().slice(11, 19)
      console.log(`${ts}  ${colorize(line)}`)
    }
  }
}

const fh = await open(logPath, 4096)

const initial = await fh.read()
if (initial) {
  buffer = initial.toString()
  position = initial.length
  const lines = buffer.split("\n")
  buffer = lines.pop() ?? ""
  emit(lines.join("\n") + "\n")
}

const interval = setInterval(async () => {
  const chunk = await fh.read()
  if (!chunk) return
  buffer += chunk.toString()
  position += chunk.length

  const lines = buffer.split("\n")
  buffer = lines.pop() ?? ""
  emit(lines.join("\n") + "\n")
}, POLL_MS)

setTimeout(async () => {
  clearInterval(interval)
  await fh.close()
  console.log("")
  console.log("(reached end of fixture file, stopping)")
  process.exit(0)
}, 3000)
