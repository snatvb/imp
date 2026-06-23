import { md5 } from "imp:hash"

{
  const r = md5("")
  assert(r === "d41d8cd98f00b204e9800998ecf8427e", `md5("") = ${r}`)
}

{
  const r = md5("hello")
  assert(r === "5d41402abc4b2a76b9719d911017c592", `md5("hello") = ${r}`)
}

{
  const r = md5("abc")
  assert(r === "900150983cd24fb0d6963f7d28e17f72", `md5("abc") = ${r}`)
}

{
  const r = md5("hello", "hex")
  assert(r === "5d41402abc4b2a76b9719d911017c592", `md5 hex = ${r}`)
}

{
  const r = md5("hello", "base64")
  assert(typeof r === "string", `md5 base64 is string`)
  assert(r.length > 0, `md5 base64 non-empty`)
}

{
  const r = md5("hello", "bytes")
  assert(r instanceof ByteBuffer, `md5 bytes is ByteBuffer`)
  assert(r.length === 16, `md5 bytes length ${r.length} (expected 16)`)
}

console.log("ALL HASH MD5 TESTS PASSED")
