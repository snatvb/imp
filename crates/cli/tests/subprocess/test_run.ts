import { exists } from "imp:fs"
import { run } from "imp:subprocess"

const isWindows = await exists("C:\\Windows")
const SH = isWindows ? "C:\\Windows\\System32\\cmd.exe" : "/bin/sh"
const SH_FLAG = isWindows ? "/c" : "-c"

function sh(
  cmd: string,
  options?: {
    input?: string
    timeout?: number | Duration
    maxOutput?: number
    env?: Record<string, string>
    cwd?: string
    signal?: AbortSignal
    encoding?: "utf8" | "binary"
  },
) {
  return run(SH, [SH_FLAG, cmd], options)
}

console.assert(typeof run === "function", "run is a function")

{
  const r = await run(SH, [SH_FLAG, "echo no_opts"])
  console.assert(r.code === 0, `no options: code=${r.code}`)
  console.assert(String(r.stdout).toLowerCase().includes("no_opts"), `no options: stdout=${r.stdout}`)
}

{
  const r = await sh("echo hello")
  console.assert(r.code === 0, `echo: code=${r.code}`)
  console.assert(String(r.stdout).toLowerCase().includes("hello"), `echo: stdout=${r.stdout}`)
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
  console.assert(String(r.stderr).toLowerCase().includes("to_stderr"), `stderr: ${r.stderr}`)
  console.assert(!String(r.stdout).toLowerCase().includes("to_stderr"), `stdout clean: ${r.stdout}`)
}

{
  const stdinCmd = isWindows ? "C:\\Windows\\System32\\more.com" : "cat"
  const r = await run(stdinCmd, [], { input: "piped data" })
  console.assert(String(r.stdout).toLowerCase().includes("piped"), `stdin: ${r.stdout}`)
}

{
  const start = Date.now()
  let threw = false
  try {
    const longCmd = isWindows ? "C:\\Windows\\System32\\ping.exe -n 5 127.0.0.1" : "sleep 5"
    await sh(longCmd, { timeout: 200 })
  } catch (e) {
    threw = true
  }
  const elapsed = Date.now() - start
  console.assert(threw, "timeout: threw")
  console.assert(elapsed < 2000, `timeout: elapsed=${elapsed}ms`)
}

{
  const start = Date.now()
  let threw = false
  try {
    const longCmd = isWindows ? "C:\\Windows\\System32\\ping.exe -n 5 127.0.0.1" : "sleep 5"
    await sh(longCmd, { timeout: Duration.millis(200) })
  } catch (e) {
    threw = true
  }
  const elapsed = Date.now() - start
  console.assert(threw, "timeout Duration: threw")
  console.assert(elapsed < 2000, `timeout Duration: elapsed=${elapsed}ms`)
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
  console.assert(String(r.stdout).includes("unique_value_42"), `env: ${r.stdout}`)
}

{
  const r = await sh(isWindows ? "cd" : "pwd", { cwd: process.cwd() })
  console.assert(String(r.stdout).length > 0, `cwd: ${r.stdout}`)
}

{
  const ctrl = new AbortController()
  ctrl.abort()
  let threw = false
  try {
    await sh("echo hi", { signal: ctrl.signal })
  } catch (e) {
    threw = true
  }
  console.assert(threw, "pre-aborted signal: threw")
}

{
  const sig = AbortSignal.timeout(100)
  const start = Date.now()
  let threw = false
  try {
    const longCmd = isWindows ? "C:\\Windows\\System32\\ping.exe -n 10 127.0.0.1" : "sleep 10"
    await sh(longCmd, { signal: sig })
  } catch (e) {
    threw = true
  }
  const elapsed = Date.now() - start
  console.assert(threw, "signal during run: threw")
  console.assert(elapsed < 2000, `signal during run: elapsed=${elapsed}ms`)
}

{
  const r = await sh("echo hi", { encoding: "binary" })
  const out = r.stdout as ByteBuffer
  const err = r.stderr as ByteBuffer
  console.assert(out !== null, "binary: stdout is object")
  console.assert(typeof out.toArray === "function", "binary: stdout has toArray method")
  const arr = out.toArray()
  console.assert(arr.length >= 2, `binary: stdout length=${arr.length}`)
  console.assert(err !== null, "binary: stderr is object")
}

{
  const r = await sh("echo hi")
  console.assert(typeof r.stdout === "string", "utf8 default: stdout is string")
  console.assert(String(r.stdout).toLowerCase().includes("hi"), `utf8 default: stdout contains hi: ${r.stdout}`)
}

console.log("ALL SUBPROCESS RUN TESTS PASSED")
