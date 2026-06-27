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
  const buf = randomBytes(1)
  assert(buf.length === 1, `randomBytes(1).length = ${buf.length}`)
}

{
  const buf = randomBytes(1024)
  assert(buf.length === 1024, `randomBytes(1024).length = ${buf.length}`)
}

{
  const a = randomBytes(16)
  const b = randomBytes(16)
  let differs = false
  for (let i = 0; i < 16; i++) {
    if (a.toArray()[i] !== b.toArray()[i]) {
      differs = true
      break
    }
  }
  assert(differs, "two randomBytes(16) should differ (statistical)")
}

{
  let threw = false
  try {
    randomBytes(-1)
  } catch (e) {
    threw = true
  }
  assert(threw, "randomBytes(-1) throws")
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
  const h = randomHex(1)
  assert(h.length === 2, `randomHex(1).length = ${h.length}`)
}

{
  const h = randomHex(32)
  assert(h.length === 64, `randomHex(32).length = ${h.length}`)
}

{
  let threw = false
  try {
    randomHex(-1)
  } catch (e) {
    threw = true
  }
  assert(threw, "randomHex(-1) throws")
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
  const ids = new Set<string>()
  for (let i = 0; i < 100; i++) {
    ids.add(randomUUID())
  }
  assert(ids.size === 100, `100 UUIDs all unique, got ${ids.size}`)
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

{
  const v = randomInt(5, 6)
  assert(v === 5, `randomInt(5,6) = ${v}`)
}

{
  const v = randomInt(0, 1)
  assert(v === 0, `randomInt(0,1) = ${v}`)
}

{
  const v = randomInt(-10, 10)
  assert(v >= -10, `randomInt(-10,10) >= -10: ${v}`)
  assert(v < 10, `randomInt(-10,10) < 10: ${v}`)
}

{
  const v = randomInt(-5, -1)
  assert(v >= -5, `randomInt(-5,-1) >= -5: ${v}`)
  assert(v < -1, `randomInt(-5,-1) < -1: ${v}`)
}

{
  let threw = false
  try {
    randomInt(10, 10)
  } catch (e) {
    threw = true
  }
  assert(threw, "randomInt(10,10) throws (min >= max)")
}

{
  let threw = false
  try {
    randomInt(20, 5)
  } catch (e) {
    threw = true
  }
  assert(threw, "randomInt(20,5) throws (min > max)")
}

{
  let threw = false
  try {
    randomInt(0, 0)
  } catch (e) {
    threw = true
  }
  assert(threw, "randomInt(0,0) throws")
}

console.log("ALL CRYPTO RANDOM TESTS PASSED")
