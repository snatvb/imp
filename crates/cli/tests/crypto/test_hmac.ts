import { hmac } from "imp:crypto"

{
  const r = hmac("sha256", "key", "message")
  assert(typeof r === "string", "hmac sha256 default is string")
  assert(r.length === 64, `hmac sha256 hex length = ${r.length} (expected 64)`)
}

{
  const r = hmac("sha256", "key", "message", "hex")
  assert(typeof r === "string", "hmac sha256 hex is string")
  assert(r.length === 64, `hmac sha256 hex length = ${r.length}`)
}

{
  const r = hmac("sha256", "key", "message", "base64")
  assert(typeof r === "string", "hmac sha256 base64 is string")
  assert(r.length > 0, "hmac sha256 base64 non-empty")
}

{
  const r = hmac("sha256", "key", "message", "bytes")
  assert(r instanceof ByteBuffer, "hmac sha256 bytes is ByteBuffer")
  assert(r.length === 32, `hmac sha256 bytes length = ${r.length} (expected 32)`)
}

{
  const r = hmac("sha512", "key", "message")
  assert(typeof r === "string", "hmac sha512 default is string")
  assert(r.length === 128, `hmac sha512 hex length = ${r.length} (expected 128)`)
}

{
  const r1 = hmac("sha256", "key1", "data")
  const r2 = hmac("sha256", "key2", "data")
  assert(r1 !== r2, "different keys produce different HMAC")
}

{
  const r1 = hmac("sha256", "key", "data1")
  const r2 = hmac("sha256", "key", "data2")
  assert(r1 !== r2, "different data produce different HMAC")
}

console.log("ALL CRYPTO HMAC TESTS PASSED")
