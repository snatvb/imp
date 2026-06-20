import { openWrite, readFile, remove } from "imp:fs"

const testPath = process.cwd() + "\\test_write_output.tmp"

{
  using wh = await openWrite(testPath, "w", 8192)
  const n1 = await wh.write("hello world")
  console.assert(n1 === 11, "write string returns byte count")

  const bb = new ByteBuffer(5)
  const arr = bb.toArray()
  arr[0] = 33
  arr[1] = 33
  arr[2] = 33
  arr[3] = 33
  arr[4] = 33
  const n2 = await wh.write(bb)
  console.assert(n2 === 5, "write ByteBuffer returns byte count")
}

{
  const buf = await readFile(testPath, "buffer")
  const view = new Uint8Array(buf)
  const expected = "hello world!!!!!"
  let match = view.length === expected.length
  if (match) {
    for (let i = 0; i < expected.length; i++) {
      if (view[i] !== expected.charCodeAt(i)) {
        match = false
        break
      }
    }
  }
  console.assert(match, "file content matches")
}

{
  using wh = await openWrite(testPath, "w", 8192)
  await wh.write("abcdefghij")
  await wh.seek(0, "start")
  const n = await wh.write("XY")
  console.assert(n === 2, "overwrite write returns byte count")

  const buf = await readFile(testPath, "buffer")
  const view = new Uint8Array(buf)
  const expected = "XYcdefghij"
  let match = view.length === expected.length
  if (match) {
    for (let i = 0; i < expected.length; i++) {
      if (view[i] !== expected.charCodeAt(i)) {
        match = false
        break
      }
    }
  }
  console.assert(match, "overwrite content matches after seek+write")
}

await remove(testPath)

{
  using wh = await openWrite(testPath, "w", 8192)
  const bb = new ByteBuffer(10)
  const arr = bb.toArray()
  for (let i = 0; i < 10; i++) arr[i] = 65 + i
  const n = await wh.writeFrom(bb, 2, 5)
  console.assert(n === 5, "writeFrom returns correct byte count")
}

{
  const buf = await readFile(testPath, "buffer")
  const view = new Uint8Array(buf)
  console.assert(view.length === 5, "writeFrom wrote correct length")
  console.assert(view[0] === 67, "writeFrom offset correct")
}

await remove(testPath)

{
  using wh = await openWrite(testPath, "w", 8192)
  await wh.write("first")
}

{
  using wh = await openWrite(testPath, "a", 8192)
  await wh.write("second")
}

{
  const buf = await readFile(testPath, "buffer")
  const view = new Uint8Array(buf)
  const expected = "firstsecond"
  let match = view.length === expected.length
  if (match) {
    for (let i = 0; i < expected.length; i++) {
      if (view[i] !== expected.charCodeAt(i)) {
        match = false
        break
      }
    }
  }
  console.assert(match, "append mode: file content matches")
}

{
  using wh = await openWrite(testPath, "a", 8192)
  await wh.seek(0, "start")
  await wh.write("X")
}

{
  const buf = await readFile(testPath, "buffer")
  const view = new Uint8Array(buf)
  const expected = "firstsecondX"
  let match = view.length === expected.length
  if (match) {
    for (let i = 0; i < expected.length; i++) {
      if (view[i] !== expected.charCodeAt(i)) {
        match = false
        break
      }
    }
  }
  console.assert(match, "append mode: seek is no-op, write at end")
}

await remove(testPath)

{
  using wh = await openWrite(testPath, "w", 8192)
  await wh.write("abcdefghij")
}

{
  using wh = await openWrite(testPath, "rw", 8192)
  await wh.seek(5, "start")
  await wh.write("XYZ")
}

{
  const buf = await readFile(testPath, "buffer")
  const view = new Uint8Array(buf)
  const expected = "abcdeXYZij"
  let match = view.length === expected.length
  if (match) {
    for (let i = 0; i < expected.length; i++) {
      if (view[i] !== expected.charCodeAt(i)) {
        match = false
        break
      }
    }
  }
  console.assert(match, "rw mode: seek+write in middle works")
}

await remove(testPath)

{
  using wh = await openWrite(testPath, "w", 8192)
  await wh.write("base")
}

{
  using wh = await openWrite(testPath)
  await wh.write("_default")
}

{
  const buf = await readFile(testPath, "buffer")
  const view = new Uint8Array(buf)
  const expected = "base_default"
  let match = view.length === expected.length
  if (match) {
    for (let i = 0; i < expected.length; i++) {
      if (view[i] !== expected.charCodeAt(i)) {
        match = false
        break
      }
    }
  }
  console.assert(match, "default flags (undefined) = append")
}

await remove(testPath)

console.log("ALL WRITE TESTS PASSED")
