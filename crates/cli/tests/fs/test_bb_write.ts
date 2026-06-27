import { openWrite, readFile, remove } from "imp:fs"

const testPath = process.cwd() + "\\test_bb_write.tmp"

{
  using wh = await openWrite(testPath, "w", 8192)
  await wh.write("AAA")

  const bb = new ByteBuffer(3)
  const arr = bb.toArray()
  arr[0] = 66 // 'B'
  arr[1] = 66
  arr[2] = 66
  await wh.write(bb)
}

{
  const buf = await readFile(testPath, "buffer")
  const view = new Uint8Array(buf)
  console.log("file length:", view.length, "expected: 6")
  console.log("content:", String.fromCharCode(...view))
  assert(view.length === 6, "file should be 6 bytes, got " + view.length)
  assert(view[0] === 65, "first byte is A")
  assert(view[3] === 66, "fourth byte is B")
}

await remove(testPath)
console.log("ALL BB WRITE TESTS PASSED")
