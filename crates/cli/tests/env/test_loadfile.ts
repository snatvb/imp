import { resolve } from "path"
import { writeFile, remove } from "imp:fs"
import { loadFile } from "imp:env"

const dir = import.meta.dirname
const isWindows = dir.includes("\\") || /^[A-Z]:/i.test(dir)
const sep = isWindows ? "\\" : "/"

function uniquePath(suffix: string): string {
  const stamp = Date.now().toString(36) + Math.floor(Math.random() * 1e6).toString(36)
  return resolve(dir, `test_env_${stamp}${suffix}`)
}

{
  const p = uniquePath(".ini")
  await writeFile(p, "[db]\nhost=localhost\nport=5432\n")
  const out = await loadFile(p) as any
  console.assert(out.db !== undefined, "loadFile .ini section")
  console.assert(RsString.equals(out.db.host, "localhost"), "loadFile host")
  console.assert(out.db.port === 5432, "loadFile port")
  await remove(p)
}

{
  const p = uniquePath(".cfg")
  await writeFile(p, "[server]\nport=8080\n")
  const out = await loadFile(p) as any
  console.assert(out.server !== undefined, "loadFile .cfg section")
  console.assert(out.server.port === 8080, "loadFile .cfg port")
  await remove(p)
}

{
  const p = uniquePath(".env")
  await writeFile(p, "FOO=bar\nBAZ=qux\n")
  const out = await loadFile(p) as any
  console.assert(RsString.equals(out.FOO, "bar"), "loadFile .env FOO")
  console.assert(RsString.equals(out.BAZ, "qux"), "loadFile .env BAZ")
  await remove(p)
}

{
  const p = uniquePath(".env.local")
  await writeFile(p, "LOCAL=1\n")
  const out = await loadFile(p) as any
  console.assert(RsString.equals(out.LOCAL, "1"), "loadFile .env.local")
  await remove(p)
}

{
  const p = uniquePath(".env.production")
  await writeFile(p, "PROD=1\n")
  const out = await loadFile(p) as any
  console.assert(RsString.equals(out.PROD, "1"), "loadFile .env.production")
  await remove(p)
}

{
  const p = uniquePath(".env.development")
  await writeFile(p, "DEV=1\n")
  const out = await loadFile(p) as any
  console.assert(RsString.equals(out.DEV, "1"), "loadFile .env.development")
  await remove(p)
}

{
  const p = uniquePath(".unknown")
  await writeFile(p, "DEFAULTED=1\n")
  const out = await loadFile(p) as any
  console.assert(RsString.equals(out.DEFAULTED, "1"), "loadFile .unknown falls back to dotenv")
  await remove(p)
}

{
  const p = uniquePath(".env")
  await writeFile(p, "EMPTY=\nSET=value\n")
  const out = await loadFile(p) as any
  console.assert("EMPTY" in out, "loadFile preserves EMPTY key")
  console.assert(RsString.equals(out.EMPTY, ""), "loadFile EMPTY is empty")
  console.assert(RsString.equals(out.SET, "value"), "loadFile SET")
  await remove(p)
}

{
  const p = uniquePath(".env")
  await writeFile(p, "URL=postgres://u:p@h:5432/d\n")
  const out = await loadFile(p) as any
  console.assert(
    RsString.equals(out.URL, "postgres://u:p@h:5432/d"),
    "loadFile URL preserved",
  )
  await remove(p)
}

{
  const p = uniquePath(".ini")
  await writeFile(p, "[main]\ndebug = true\nport = 3000\nname = \"myapp\"\n")
  const out = await loadFile(p) as any
  console.assert(out.main !== undefined, "loadFile .ini with various types")
  console.assert(out.main.debug === true, "loadFile .ini bool")
  console.assert(out.main.port === 3000, "loadFile .ini int")
  console.assert(RsString.equals(out.main.name, "myapp"), "loadFile .ini quoted")
  await remove(p)
}

{
  const p = uniquePath(".env")
  await writeFile(p, "GREETING=\"Hello $USER\"\n", "w")
  await writeFile(p, "PLAIN=plainvalue\n", "a")
  const out = await loadFile(p) as any
  console.assert(RsString.equals(out.PLAIN, "plainvalue"), "loadFile appended content")
  await remove(p)
}

{
  const p = uniquePath(".ini")
  await writeFile(p, "no section\nkey = val\n")
  const out = await loadFile(p) as any
  console.assert(RsString.equals(out.key, "val"), "loadFile .ini without section")
  await remove(p)
}

{
  const p = uniquePath(".env")
  await writeFile(p, "A=1\nB=$A\n")
  const out = await loadFile(p) as any
  console.assert(RsString.equals(out.A, "1"), "loadFile A=1")
  console.assert(RsString.equals(out.B, "1"), "loadFile B expands to A's value")
  await remove(p)
}

{
  const p = uniquePath(".ini")
  await writeFile(p, "[nested]\ndeep.path = 42\n")
  const out = await loadFile(p) as any
  console.assert(out.nested !== undefined, "loadFile .ini nested section")
  console.assert(out.nested.deep !== undefined, "loadFile .ini nested.deep")
  console.assert(out.nested.deep.path === 42, "loadFile .ini nested.deep.path")
  await remove(p)
}

{
  const p = uniquePath(".env")
  await writeFile(p, "export FOO=bar\n")
  const out = await loadFile(p) as any
  console.assert(RsString.equals(out.FOO, "bar"), "loadFile export prefix stripped")
  await remove(p)
}

{
  const p = uniquePath(".ini")
  await writeFile(p, "; comment line\n# another\n[real]\nkey = val\n")
  const out = await loadFile(p) as any
  console.assert(out.real !== undefined, "loadFile ignores comments")
  console.assert(RsString.equals(out.real.key, "val"), "loadFile real.key")
  await remove(p)
}

{
  let threw = false
  try {
    await loadFile(resolve(dir, "definitely_does_not_exist_xyz_12345.ini"))
  } catch (e) {
    threw = true
    console.assert(String(e).includes("ENOENT"), "ENOENT for missing file")
  }
  console.assert(threw, "loadFile throws on missing file")
}

console.log("ALL ENV LOADFILE TESTS PASSED")
