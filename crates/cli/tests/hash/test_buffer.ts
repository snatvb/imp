import { utf8 } from "imp:encoding"
import { sha256 } from "imp:hash"

{
  const buf = utf8.encode("hello")
  const r = sha256(buf)
  assert(r === "2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824", `sha256(utf8("hello")) = ${r}`)
}

{
  const buf = utf8.encode("hello")
  const r = sha256(buf, "base64")
  assert(typeof r === "string", `sha256(buf, base64) is string`)
  assert(r.length > 0, `sha256(buf, base64) non-empty`)
}

{
  const buf = utf8.encode("hello")
  const r = sha256(buf, "bytes")
  assert(r instanceof ByteBuffer, `sha256(buf, bytes) is ByteBuffer`)
  assert(r.length === 32, `sha256(buf, bytes) length ${r.length}`)
}

console.log("ALL HASH BUFFER TESTS PASSED")
