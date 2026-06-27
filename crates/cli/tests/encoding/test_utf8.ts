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
  const buf = utf8.encode("hello")
  const r = utf8.decode(buf)
  assert(typeof r === "string", `decode returns string: ${typeof r}`)
  assert(r === "hello", `decode "hello" roundtrip -> "${r}"`)
}

{
  const buf = utf8.encode("привет")
  const r = utf8.decode(buf)
  assert(r === "привет", `decode cyrillic roundtrip -> "${r}"`)
}

{
  const buf = ByteBuffer.fromArray([0xff, 0xfe])
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
