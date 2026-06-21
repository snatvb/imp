import {
  resolve,
  join,
  basename,
  dirname,
  extname,
  normalize,
  isAbsolute,
  parse,
  relative,
  toNamespacedPath,
  win32,
  posix,
} from "path"

import { readFile, mkdir, exists, remove, metadata, open } from "imp:fs"

const S = (v: string) => RsString.fromString(v)

const fixturesDir = import.meta.dirname + "/../fixtures"

{
  const r = join(S("/foo"), S("bar"))
  assert(r.includes("foo"), "join RsString")
}

{
  const r = resolve(S("/foo"), S("bar"))
  assert(isAbsolute(S(r)), "resolve RsString")
}

{
  assert(basename(S("/foo/bar.txt")) === "bar.txt", "basename RsString")
  assert(basename(S("/foo/bar.txt"), S(".txt")) === "bar", "basename RsString with suffix")
}

{
  assert(dirname(S("/foo/bar/baz.txt")).includes("bar"), "dirname RsString")
}

{
  assert(extname(S("file.txt")) === "txt", "extname RsString")
}

{
  assert(!normalize(S("/foo//bar//baz")).includes("//"), "normalize RsString")
}

{
  assert(isAbsolute(S("/foo")) || isAbsolute(S("C:\\foo")), "isAbsolute RsString")
  assert(!isAbsolute(S("foo/bar")), "isAbsolute relative RsString")
}

{
  const p = parse(S("C:\\path\\dir\\file.txt"))
  assert(p.base === "file.txt", "parse RsString base")
  assert(p.ext === ".txt", "parse RsString ext")
}

{
  const r = relative(S("/data/orandea/test/aaa"), S("/data/orandea/impl/bbb"))
  assert(r.includes(".."), "relative RsString")
}

{
  const r = toNamespacedPath(S("C:\\foo"))
  assert(typeof r === "string", "toNamespacedPath RsString")
}

{
  assert(win32.join(S("foo"), S("bar"), S("baz")).includes("bar"), "win32.join RsString")
  assert(win32.isAbsolute(S("C:\\foo")), "win32.isAbsolute RsString")
  assert(posix.isAbsolute(S("/foo")), "posix.isAbsolute RsString")
  assert(posix.basename(S("/foo/bar.txt")) === "bar.txt", "posix.basename RsString")
}

const fixturePath = S(fixturesDir + "/hello.txt")
{
  const content = await readFile(fixturePath, "utf8")
  assert(content.toString() === "hello world", "readFile RsString path utf8")
}

{
  const buf = await readFile(fixturePath)
  assert(buf instanceof ArrayBuffer, "readFile RsString path buffer")
  assert(buf.byteLength === 11, "readFile RsString buffer length")
}

{
  const dir = import.meta.dirname + "/rs_string_test_dir"
  const rsDir = S(dir)
  await mkdir(rsDir)
  assert((await exists(rsDir)) === true, "mkdir/exists RsString")

  const meta = await metadata(rsDir)
  assert(meta.isDirectory === true, "metadata RsString")

  await remove(rsDir)
  assert((await exists(rsDir)) === false, "remove RsString")
}

{
  const fh = await open(fixturePath, 8192)
  const chunk = await fh.read()
  assert(chunk !== undefined, "FileHandle open with RsString path")
  assert(chunk!.toString() === "hello world", "FileHandle read content")
  await fh.close()
}

console.log("ALL RSSTRING INTEROP TESTS PASSED")
