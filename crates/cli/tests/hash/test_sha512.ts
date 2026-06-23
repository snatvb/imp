import { sha512 } from "imp:hash"

{
  const r = sha512("hello")
  assert(typeof r === "string", `sha512 returns string`)
  assert(r.length === 128, `sha512 hex length ${r.length} (expected 128)`)
  assert(r.startsWith("9b71d224"), `sha512("hello") starts with 9b71d224: ${r}`)
}

{
  const r = sha512("")
  assert(typeof r === "string", `sha512("") returns string`)
  assert(r.length === 128, `sha512("") hex length ${r.length} (expected 128)`)
}

{
  const r = sha512("hello", "hex")
  assert(r.length === 128, `sha512 hex length ${r.length}`)
}

{
  const r = sha512("hello", "base64")
  assert(typeof r === "string", `sha512 base64 is string`)
  assert(r.length > 0, `sha512 base64 non-empty`)
}

{
  const r = sha512("hello", "bytes")
  assert(r instanceof ByteBuffer, `sha512 bytes is ByteBuffer`)
  assert(r.length === 64, `sha512 bytes length ${r.length} (expected 64)`)
}

console.log("ALL HASH SHA512 TESTS PASSED")
