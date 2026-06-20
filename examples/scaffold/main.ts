import { resolve } from "path"

import { mkdir, writeFile } from "imp:fs"
import inq from "imp:inq"

const name = await inq.prompt("Project name?")

const projectTypes = ["lib", "cli", "web", "api"]
const projectType = await inq.select("Project type?", projectTypes)

const langs = ["TypeScript", "JavaScript"]
const lang = await inq.select("Language?", langs)

const useGit = await inq.confirm("Initialize git?", true)

const useTests = await inq.confirm("Add test scaffold?", true)

const targetDir = resolve(process.cwd(), name)
await mkdir(targetDir, { recursive: true })

console.log("")
console.log(`Creating ${name} in ${targetDir}...`)

const ext = lang === "TypeScript" ? "ts" : "js"
const isCli = projectType === "cli"

const main = isCli
  ? `import clap from "imp:clap"

const parser = new clap.Parser()
  .name(${JSON.stringify(name)})
  .arg({ name: "input", action: "set", help: "input file" })
  .arg({ name: "output", short: "o", long: "output", action: "set", help: "output file" })

const result = parser.parse(clap.args)
if (String(result.type) === "result") {
  console.log("input:", String(result.input ?? ""))
  console.log("output:", String(result.output ?? ""))
}
console.log("hello from", ${JSON.stringify(name)})
`
  : `export function greet(name: string): string {
  return \`Hello, \${name}!\`
}

if (import.meta.main) {
  console.log(greet("world"))
}
`

await writeFile(resolve(targetDir, `main.${ext}`), main)

if (useTests) {
  const test = `import { assert } from "console"

{
  // ${name} smoke test
  assert(true, "smoke")
  console.log("PASS: smoke")
}
console.log("ALL ${name.toUpperCase()} TESTS PASSED")
`
  await writeFile(resolve(targetDir, `test.${ext}`), test)
}

if (projectType === "web" || projectType === "api") {
  await writeFile(
    resolve(targetDir, "index.html"),
    `<!DOCTYPE html>
<html><body><h1>${name}</h1></body></html>
`,
  )
}

if (useGit) {
  await writeFile(
    resolve(targetDir, ".gitignore"),
    `node_modules/
*.tmp
dist/
target/
.idea/
.vscode/
`,
  )
  await writeFile(
    resolve(targetDir, "README.md"),
    `# ${name}

${projectType} project created with imp scaffold.

## Run

\`\`\`bash
imp run main.${ext}
\`\`\`
`,
  )
}

console.log("")
console.log("Created:")
for (const f of [
  "main." + ext,
  useTests ? `test.${ext}` : null,
  useGit ? ".gitignore" : null,
  useGit ? "README.md" : null,
].filter(Boolean)) {
  console.log(`  ${f}`)
}
console.log("")
console.log(`Next: cd ${name} && imp run main.${ext}`)
