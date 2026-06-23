import { sha1 } from "imp:hash"

{
  const r = sha1("")
  assert(r === "da39a3ee5e6b4b0d3255bfef95601890afd80709", `sha1("") = ${r}`)
}

{
  const r = sha1("hello")
  assert(r === "aaf4c61ddcc5e8a2dabede0f3b482cd9aea9434d", `sha1("hello") = ${r}`)
}

{
  const r = sha1("abc")
  assert(r === "a9993e364706816aba3e25717850c26c9cd0d89d", `sha1("abc") = ${r}`)
}

{
  const r = sha1("hello", "hex")
  assert(r === "aaf4c61ddcc5e8a2dabede0f3b482cd9aea9434d", `sha1 hex = ${r}`)
}

{
  const r = sha1("hello", "base64")
  assert(typeof r === "string", `sha1 base64 is string`)
  assert(r.length > 0, `sha1 base64 non-empty`)
}

{
  const r = sha1("hello", "bytes")
  assert(r instanceof ByteBuffer, `sha1 bytes is ByteBuffer`)
  assert(r.length === 20, `sha1 bytes length ${r.length} (expected 20)`)
}

console.log("ALL HASH SHA1 TESTS PASSED")
