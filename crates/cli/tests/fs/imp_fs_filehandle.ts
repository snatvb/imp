import { resolve } from "path"

import { open } from "imp:fs"

const fixture = (name: string) => resolve(import.meta.dirname, "fixtures", name)

// --- test: basic read ---

{
  const fh = await open(fixture("hello.txt"), 64)

  const chunk = await fh.read()

  assert(chunk !== undefined, "read returns chunk")

  assert(chunk.length === 11, `chunk size is 11, got ${chunk.length}`)

  const text = chunk.toString()

  assert(text === "hello world", `content: "${text}"`)

  const eof = await fh.read()

  assert(eof === undefined, "read returns undefined at EOF")

  await fh.close()

  console.log("PASS: basic read")
}

// --- test: multiple reads (buffer reuse) ---
{
  const fh = await open(fixture("hello.txt"), 5)

  const c1 = await fh.read()

  assert(c1 !== undefined, "first read")

  assert(c1.length === 5, `first chunk size 5, got ${c1.length}`)

  const c2 = await fh.read()

  assert(c2 !== undefined, "second read")

  assert(c2.length === 5, `second chunk size 5, got ${c2.length}`)

  const c3 = await fh.read()

  assert(c3 !== undefined, "third read")

  assert(c3.length === 1, `third chunk size 1, got ${c3.length}`)

  const eof = await fh.read()

  assert(eof === undefined, "EOF after all chunks")

  const full = c1.toString() + c2.toString() + c3.toString()

  assert(full === "hello world", `reassembled: "${full}"`)

  await fh.close()

  console.log("PASS: multiple reads")
}

// --- test: zero-copy independence ---

{
  const fh = await open(fixture("hello.txt"), 5)

  const c1 = await fh.read()

  const c2 = await fh.read()

  const t1 = c1.toString()
  const t2 = c2.toString()
  assert(t1 === "hello", `first: "${t1}"`)

  assert(t2 === " worl", `second: "${t2}"`)

  await fh.close()

  console.log("PASS: zero-copy independence")
}

// --- test: seek ---

{
  const fh = await open(fixture("hello.txt"), 64)

  const pos = await fh.seek(6, "start")

  assert(pos === 6, `seek start returns 6, got ${pos}`)

  const chunk = await fh.read()

  const text = chunk.toString()

  assert(text === "world", `after seek: "${text}"`)

  await fh.close()

  console.log("PASS: seek")
}

// --- test: seek current ---

{
  const fh = await open(fixture("hello.txt"), 64)

  await fh.seek(3, "start")

  const pos = await fh.seek(2, "current")

  assert(pos === 5, `seek current returns 5, got ${pos}`)

  const chunk = await fh.read()

  const text = chunk.toString()

  assert(text === " world", `after seek current: "${text}"`)

  await fh.close()

  console.log("PASS: seek current")
}

// --- test: close is idempotent ---

{
  const fh = await open(fixture("hello.txt"), 64)

  await fh.close()

  await fh.close()

  await fh.close()

  console.log("PASS: close idempotent")
}

// --- test: read after close errors ---

{
  const fh = await open(fixture("hello.txt"), 64)

  await fh.close()

  let threw = false

  try {
    await fh.read()
  } catch (e) {
    threw = true
  }

  assert(threw === true, "read after close throws")

  console.log("PASS: read after close")
}

// --- test: seek after close errors ---

{
  const fh = await open(fixture("hello.txt"), 64)

  await fh.close()

  let threw = false

  try {
    await fh.seek(0, "start")
  } catch (e) {
    threw = true
  }

  assert(threw === true, "seek after close throws")

  console.log("PASS: seek after close")
}

// --- test: open non-existent file ---

{
  let threw = false

  try {
    await open(fixture("DOES_NOT_EXIST.txt"), 64)
  } catch (e) {
    threw = true
  }

  assert(threw === true, "open non-existent throws")

  console.log("PASS: open non-existent")
}

// --- test: seek end ---

{
  const fh = await open(fixture("hello.txt"), 64)

  const pos = await fh.seek(-5, "end")

  assert(pos === 6, `seek end returns 6, got ${pos}`)

  const chunk = await fh.read()

  const text = chunk.toString()

  assert(text === "world", `after seek end: "${text}"`)

  await fh.close()

  console.log("PASS: seek end")
}

// --- test: larger file with bigger chunk ---

{
  const fh = await open(fixture("readfile.bin"), 32)

  let all = ""

  let chunk

  while ((chunk = await fh.read()) !== undefined) {
    all += chunk.toString()
  }

  assert(all === "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz\n", `larger file: "${all}"`)

  await fh.close()

  console.log("PASS: larger file")
}

// --- test: invalid whence ---

{
  const fh = await open(fixture("hello.txt"), 64)

  let threw = false

  try {
    await fh.seek(0, "invalid")
  } catch (e) {
    threw = true
  }

  assert(threw === true, "invalid whence throws")

  await fh.close()

  console.log("PASS: invalid whence")
}

// --- test: ByteBuffer.toStr() returns RsString ---

{
  const fh = await open(fixture("hello.txt"), 64)

  const chunk = await fh.read()

  const rs = chunk.toStr()

  assert(rs.length === 11, `RsString length 11, got ${rs.length}`)

  assert(rs.toString() === "hello world", `RsString content: "${rs}"`)

  await fh.close()

  console.log("PASS: ByteBuffer.toStr()")
}

// --- test: ByteBuffer.toArrayBuffer() ---

{
  const fh = await open(fixture("hello.txt"), 64)

  const chunk = await fh.read()

  const ab = chunk.toArrayBuffer()

  assert(ab instanceof ArrayBuffer, "toArrayBuffer returns ArrayBuffer")

  assert(ab.byteLength === 11, `ArrayBuffer byteLength 11, got ${ab.byteLength}`)

  await fh.close()

  console.log("PASS: ByteBuffer.toArrayBuffer()")
}

// --- test: ByteBuffer.slice() ---

{
  const fh = await open(fixture("hello.txt"), 64)

  const chunk = await fh.read()

  const sliced = chunk.slice(0, 5)

  assert(sliced.length === 5, `sliced length 5, got ${sliced.length}`)

  assert(sliced.toString() === "hello", `sliced content: "${sliced.toString()}"`)

  await fh.close()

  console.log("PASS: ByteBuffer.slice()")
}

console.log("ALL IMP:FS FILEHANDLE TESTS PASSED")
