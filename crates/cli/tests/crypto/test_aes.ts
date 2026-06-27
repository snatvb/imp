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
  assert(dec.toString() === "hello world", `aes roundtrip: "${dec.toString()}"`)
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

console.log("ALL CRYPTO AES TESTS PASSED")
