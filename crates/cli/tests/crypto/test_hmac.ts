import { hmac, randomBytes } from "imp:crypto"

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

{
  const r = hmac("sha256", "", "message")
  assert(typeof r === "string", "hmac with empty key works")
  assert(r.length === 64, "hmac empty key returns valid hex")
}

{
  const r = hmac("sha256", "key", "")
  assert(typeof r === "string", "hmac with empty data works")
  assert(r.length === 64, "hmac empty data returns valid hex")
}

{
  const r = hmac("sha256", "", "")
  assert(typeof r === "string", "hmac with both empty works")
  assert(r.length === 64, "hmac both empty returns valid hex")
}

{
  let threw = false
  try {
    hmac("md5" as any, "key", "message")
  } catch (e) {
    threw = true
  }
  assert(threw, "hmac with unsupported algo md5 throws")
}

{
  let threw = false
  try {
    hmac("sha1" as any, "key", "message")
  } catch (e) {
    threw = true
  }
  assert(threw, "hmac with unsupported algo sha1 throws")
}

{
  let threw = false
  try {
    hmac("invalid" as any, "key", "message")
  } catch (e) {
    threw = true
  }
  assert(threw, "hmac with invalid algo throws")
}

{
  const buf = randomBytes(32)
  const r = hmac("sha256", buf, "message")
  assert(typeof r === "string", "hmac with ByteBuffer key works")
  assert(r.length === 64, "hmac ByteBuffer key returns valid hex")
}

{
  const buf = randomBytes(10)
  const r = hmac("sha256", "key", buf)
  assert(typeof r === "string", "hmac with ByteBuffer data works")
  assert(r.length === 64, "hmac ByteBuffer data returns valid hex")
}

{
  const keyBuf = randomBytes(16)
  const dataBuf = randomBytes(32)
  const r = hmac("sha256", keyBuf, dataBuf, "hex")
  assert(typeof r === "string", "hmac with both ByteBuffer works")
  assert(r.length === 64, "hmac both ByteBuffer returns valid hex")
}

{
  const keyBuf = randomBytes(16)
  const dataBuf = randomBytes(32)
  const r = hmac("sha256", keyBuf, dataBuf, "base64")
  assert(typeof r === "string", "hmac ByteBuffer base64 is string")
}

{
  const keyBuf = randomBytes(16)
  const dataBuf = randomBytes(32)
  const r = hmac("sha256", keyBuf, dataBuf, "bytes")
  assert(r instanceof ByteBuffer, "hmac ByteBuffer bytes is ByteBuffer")
  assert(r.length === 32, "hmac ByteBuffer bytes length 32")
}

console.log("ALL CRYPTO HMAC TESTS PASSED")
