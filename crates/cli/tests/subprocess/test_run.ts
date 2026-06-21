import { exists } from "imp:fs"
import { run } from "imp:subprocess"

const isWindows = await exists("C:\\Windows")
const SH = isWindows ? "C:\\Windows\\System32\\cmd.exe" : "/bin/sh"
const SH_FLAG = isWindows ? "/c" : "-c"

function sh(
  cmd: string,
  options?: { input?: string; timeout?: number; maxOutput?: number; env?: Record<string, string>; cwd?: string },
) {
  return run(SH, [SH_FLAG, cmd], options)
}

console.assert(typeof run === "function", "run is a function")

{
  const r = await run(SH, [SH_FLAG, "echo no_opts"])
  console.assert(r.code === 0, `no options: code=${r.code}`)
  console.assert(r.stdout.toLowerCase().includes("no_opts"), `no options: stdout=${r.stdout}`)
}

{
  const r = await sh("echo hello")
  console.assert(r.code === 0, `echo: code=${r.code}`)
  console.assert(r.stdout.toLowerCase().includes("hello"), `echo: stdout=${r.stdout}`)
  console.assert(r.success === true, "echo: success")
}

{
  const r = await sh("exit 0")
  console.assert(r.code === 0, `exit 0: code=${r.code}`)
  console.assert(r.success === true, "exit 0: success")
}

{
  const r = await sh("exit 1")
  console.assert(r.code === 1, `exit 1: code=${r.code}`)
  console.assert(r.success === false, "exit 1: success=false")
}

{
  const r = await sh("echo to_stderr 1>&2")
  console.assert(r.stderr.toLowerCase().includes("to_stderr"), `stderr: ${r.stderr}`)
  console.assert(!r.stdout.toLowerCase().includes("to_stderr"), `stdout clean: ${r.stdout}`)
}

{
  const stdinCmd = isWindows ? "C:\\Windows\\System32\\more.com" : "cat"
  const r = await run(stdinCmd, [], { input: "piped data" })
  console.assert(r.stdout.toLowerCase().includes("piped"), `stdin: ${r.stdout}`)
}

{
  const start = Date.now()
  let threw = false
  try {
    await sh(isWindows ? "ping -n 5 127.0.0.1" : "sleep 5", { timeout: 200 })
  } catch (e) {
    threw = true
  }
  const elapsed = Date.now() - start
  console.assert(threw, "timeout: threw")
  console.assert(elapsed < 2000, `timeout: elapsed=${elapsed}ms`)
}

{
  let threw = false
  try {
    await run("definitely-not-a-cmd-xyz-12345", [])
  } catch (e) {
    threw = true
  }
  console.assert(threw, "not found: threw")
}

{
  const big = "x".repeat(1000)
  const cmd = isWindows ? `echo ${big}` : `printf '%*s' 1000 | tr ' ' 'x'`
  const r = await sh(cmd, { maxOutput: 100 })
  console.assert(r.stdout.length <= 100, `maxOutput: stdout.length=${r.stdout.length}`)
}

{
  const env: Record<string, string> = { IMP_TEST_VAR: "unique_value_42" }
  if (process.env && process.env.PATH) env.PATH = process.env.PATH
  if (process.env && process.env.SystemRoot) env.SystemRoot = process.env.SystemRoot
  if (process.env && process.env.ComSpec) env.ComSpec = process.env.ComSpec
  const r = await sh(isWindows ? "echo %IMP_TEST_VAR%" : "echo $IMP_TEST_VAR", { env })
  console.assert(r.stdout.includes("unique_value_42"), `env: ${r.stdout}`)
}

{
  const r = await sh(isWindows ? "cd" : "pwd", { cwd: process.cwd() })
  console.assert(r.stdout.length > 0, `cwd: ${r.stdout}`)
}

console.log("ALL SUBPROCESS RUN TESTS PASSED")
