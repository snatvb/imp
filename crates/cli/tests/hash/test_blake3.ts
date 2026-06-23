import { blake3 } from "imp:hash"

{
  const r = blake3("hello")
  assert(typeof r === "string", `blake3 returns string`)
  assert(r.length === 64, `blake3 hex length ${r.length} (expected 64)`)
  assert(r.startsWith("ea8f163db3"), `blake3("hello") starts with ea8f163db3: ${r}`)
}

{
  const r = blake3("")
  assert(typeof r === "string", `blake3("") returns string`)
  assert(r.length === 64, `blake3("") hex length ${r.length}`)
}

{
  const r = blake3("hello", "hex")
  assert(r.length === 64, `blake3 hex length ${r.length}`)
}

{
  const r = blake3("hello", "base64")
  assert(typeof r === "string", `blake3 base64 is string`)
  assert(r.length > 0, `blake3 base64 non-empty`)
}

{
  const r = blake3("hello", "bytes")
  assert(r instanceof ByteBuffer, `blake3 bytes is ByteBuffer`)
  assert(r.length === 32, `blake3 bytes length ${r.length} (expected 32)`)
}

console.log("ALL HASH BLAKE3 TESTS PASSED")
