import { aesEncrypt, aesDecrypt, randomBytes } from "imp:crypto"
import { utf8 } from "imp:encoding"

{
  const key = randomBytes(32)
  const pt = utf8.encode("hello world")
  const ct = aesEncrypt("aes-256-gcm", key, pt)
  assert(ct instanceof ByteBuffer, "aesEncrypt returns ByteBuffer")
  assert(ct.length > pt.length, `ciphertext length ${ct.length} > plaintext ${pt.length}`)
}

{
  const key = randomBytes(16)
  const pt = utf8.encode("hello world")
  const ct = aesEncrypt("aes-128-gcm", key, pt)
  assert(ct instanceof ByteBuffer, "aes-128-gcm returns ByteBuffer")
  assert(ct.length > pt.length, `ciphertext length ${ct.length} > plaintext ${pt.length}`)
}

{
  const key = randomBytes(32)
  const pt = utf8.encode("hello world")
  const ct = aesEncrypt("aes-256-gcm", key, pt)
  const dec = aesDecrypt("aes-256-gcm", key, ct)
  assert(dec instanceof ByteBuffer, "aesDecrypt returns ByteBuffer")
  assert(dec.toString() === "hello world", `aes-256 roundtrip: "${dec.toString()}"`)
}

{
  const key = randomBytes(16)
  const pt = utf8.encode("secret message")
  const ct = aesEncrypt("aes-128-gcm", key, pt)
  const dec = aesDecrypt("aes-128-gcm", key, ct)
  assert(dec.toString() === "secret message", `aes-128 roundtrip: "${dec.toString()}"`)
}

{
  const key = randomBytes(32)
  const pt = utf8.encode("test")
  const ct = aesEncrypt("aes-256-gcm", key, pt)
  const wrongKey = randomBytes(32)
  let threw = false
  try {
    aesDecrypt("aes-256-gcm", wrongKey, ct)
  } catch (e) {
    threw = true
  }
  assert(threw, "decryption with wrong key throws")
}

{
  const key = randomBytes(32)
  const pt = utf8.encode("tamper test")
  const ct = aesEncrypt("aes-256-gcm", key, pt)
  const ctArr = Array.from(ct.toArray())
  ctArr[14] ^= 0xff
  const tampered = ByteBuffer.fromArray(ctArr)
  let threw = false
  try {
    aesDecrypt("aes-256-gcm", key, tampered)
  } catch (e) {
    threw = true
  }
  assert(threw, "decryption of tampered ciphertext throws")
}

{
  const key = randomBytes(32)
  const pt = utf8.encode("empty")
  const ct = aesEncrypt("aes-256-gcm", key, pt)
  assert(ct.length > 0, "ciphertext non-empty for short plaintext")
}

{
  let threw = false
  try {
    aesEncrypt("aes-256-gcm", randomBytes(16), utf8.encode("x"))
  } catch (e) {
    threw = true
  }
  assert(threw, "aes-256-gcm with 16-byte key throws (wrong key size)")
}

{
  let threw = false
  try {
    aesEncrypt("aes-128-gcm", randomBytes(32), utf8.encode("x"))
  } catch (e) {
    threw = true
  }
  assert(threw, "aes-128-gcm with 32-byte key throws (wrong key size)")
}

{
  let threw = false
  try {
    aesEncrypt("aes-256-gcm", randomBytes(24), utf8.encode("x"))
  } catch (e) {
    threw = true
  }
  assert(threw, "aes-256-gcm with 24-byte key throws (invalid key size)")
}

{
  let threw = false
  try {
    aesEncrypt("aes-128-gcm", randomBytes(8), utf8.encode("x"))
  } catch (e) {
    threw = true
  }
  assert(threw, "aes-128-gcm with 8-byte key throws (invalid key size)")
}

{
  let threw = false
  try {
    aesEncrypt("chacha20-poly1305" as any, randomBytes(32), utf8.encode("x"))
  } catch (e) {
    threw = true
  }
  assert(threw, "unsupported algo chacha20-poly1305 throws")
}

{
  let threw = false
  try {
    aesEncrypt("aes-256-cbc" as any, randomBytes(32), utf8.encode("x"))
  } catch (e) {
    threw = true
  }
  assert(threw, "unsupported algo aes-256-cbc throws")
}

{
  let threw = false
  try {
    aesEncrypt("invalid" as any, randomBytes(32), utf8.encode("x"))
  } catch (e) {
    threw = true
  }
  assert(threw, "invalid algo throws")
}

{
  const key = randomBytes(32)
  const pt = utf8.encode("")
  const ct = aesEncrypt("aes-256-gcm", key, pt)
  const dec = aesDecrypt("aes-256-gcm", key, ct)
  assert(dec.toString() === "", "empty plaintext roundtrip")
}

{
  const key = randomBytes(32)
  const pt = utf8.encode("a")
  const ct = aesEncrypt("aes-256-gcm", key, pt)
  const dec = aesDecrypt("aes-256-gcm", key, ct)
  assert(dec.toString() === "a", "single char roundtrip")
}

{
  const key = randomBytes(32)
  let longPt = ""
  for (let i = 0; i < 1000; i++) longPt += "x"
  const pt = utf8.encode(longPt)
  const ct = aesEncrypt("aes-256-gcm", key, pt)
  const dec = aesDecrypt("aes-256-gcm", key, ct)
  assert(dec.toString() === longPt, "1000-char roundtrip")
}

{
  const key = randomBytes(32)
  const pt = utf8.encode("hello world")
  const ct1 = aesEncrypt("aes-256-gcm", key, pt)
  const ct2 = aesEncrypt("aes-256-gcm", key, pt)
  assert(
    ct1.toArray().join(",") !== ct2.toArray().join(","),
    "same plaintext produces different ciphertext (random nonce)",
  )
}

console.log("ALL CRYPTO AES TESTS PASSED")
