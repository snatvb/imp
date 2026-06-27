import { randomBytes, randomHex, randomUUID, randomInt } from "imp:crypto"

{
  const buf = randomBytes(16)
  assert(buf instanceof ByteBuffer, "randomBytes(16) is ByteBuffer")
  assert(buf.length === 16, `randomBytes(16).length = ${buf.length}`)
}

{
  const buf = randomBytes(0)
  assert(buf instanceof ByteBuffer, "randomBytes(0) is ByteBuffer")
  assert(buf.length === 0, `randomBytes(0).length = ${buf.length}`)
}

{
  const buf = randomBytes(64)
  assert(buf.length === 64, `randomBytes(64).length = ${buf.length}`)
}

{
  const h = randomHex(16)
  assert(typeof h === "string", "randomHex is string")
  assert(h.length === 32, `randomHex(16).length = ${h.length} (expected 32)`)
  assert(/^[0-9a-f]+$/.test(h), `randomHex valid hex: ${h}`)
}

{
  const h = randomHex(0)
  assert(h === "", `randomHex(0) = "${h}"`)
}

{
  const id = randomUUID()
  assert(typeof id === "string", "randomUUID is string")
  assert(/^[0-9a-f]{8}-[0-9a-f]{4}-4[0-9a-f]{3}-[89ab][0-9a-f]{3}-[0-9a-f]{12}$/.test(id), `randomUUID format: ${id}`)
}

{
  const a = randomUUID()
  const b = randomUUID()
  assert(a !== b, `randomUUID unique: ${a} !== ${b}`)
}

{
  const v = randomInt(0, 100)
  assert(typeof v === "number", "randomInt is number")
  assert(v >= 0, `randomInt(0,100) >= 0: ${v}`)
  assert(v < 100, `randomInt(0,100) < 100: ${v}`)
}

{
  const v = randomInt(10, 11)
  assert(v === 10, `randomInt(10,11) = ${v}`)
}

console.log("ALL CRYPTO RANDOM TESTS PASSED")
