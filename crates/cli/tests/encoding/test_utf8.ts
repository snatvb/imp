import { utf8 } from "imp:encoding"

{
  const r = utf8.encode("hello")
  assert(r instanceof ByteBuffer, `encode returns ByteBuffer: ${typeof r}`)
  assert(r.length === 5, `encode "hello" length ${r.length} (expected 5)`)
  const arr = r.toArray()
  const expected = [104, 101, 108, 108, 111]
  assert(
    arr.length === expected.length && arr.every((v: number, i: number) => v === expected[i]),
    `encode "hello" bytes [${arr}] expected [${expected}]`,
  )
}

{
  const r = utf8.encode("привет")
  assert(r instanceof ByteBuffer, "encode cyrillic returns ByteBuffer")
  assert(r.length === 12, `encode "привет" length ${r.length} (expected 12)`)
}

{
  const r = utf8.encode("")
  assert(r.length === 0, `encode empty -> length ${r.length}`)
}

{
  const buf = new ByteBuffer(5)
  const arr = buf.toArray()
  arr[0] = 104
  arr[1] = 101
  arr[2] = 108
  arr[3] = 108
  arr[4] = 111
  const r = utf8.decode(buf)
  assert(typeof r === "string", `decode returns string: ${typeof r}`)
  assert(r === "hello", `decode bytes -> "${r}"`)
}

{
  const buf = new ByteBuffer(12)
  const arr = buf.toArray()
  const bytes = [0xd0, 0xbf, 0xd1, 0x80, 0xd0, 0xb8, 0xd0, 0xb2, 0xd0, 0xb5, 0xd1, 0x82]
  for (let i = 0; i < bytes.length; i++) arr[i] = bytes[i]
  const r = utf8.decode(buf)
  assert(r === "привет", `decode cyrillic bytes -> "${r}"`)
}

{
  const buf = new ByteBuffer(2)
  const arr = buf.toArray()
  arr[0] = 0xff
  arr[1] = 0xfe
  let threw = false
  try {
    utf8.decode(buf)
  } catch {
    threw = true
  }
  assert(threw, "decode invalid utf-8 throws")
}

{
  const original = "привет, world! 🌍"
  const encoded = utf8.encode(original)
  const decoded = utf8.decode(encoded)
  assert(decoded === original, `roundtrip unicode -> "${decoded}"`)
}

console.log("ALL ENCODING UTF8 TESTS PASSED")
