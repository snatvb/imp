import { hex } from "imp:encoding"
import { utf8 } from "imp:encoding"

{
  const r = hex.encode("hello")
  assert(r === "68656c6c6f", `encode "hello" -> ${r}`)
}

{
  const r = hex.encode("hi", { uppercase: true })
  assert(r === "6869", `encode uppercase -> ${r}`)
}

{
  const r = hex.encode("")
  assert(r === "", `encode empty -> "${r}"`)
}

{
  const r = hex.encode("A")
  assert(r === "41", `encode "A" -> ${r}`)
}

{
  const buf = utf8.encode("ABC")
  const r = hex.encode(buf)
  assert(r === "414243", `encode ByteBuffer "ABC" -> ${r}`)
}

{
  const buf = utf8.encode("abc")
  const r = hex.encode(buf, { uppercase: true })
  assert(r === "414243", `encode ByteBuffer "abc" uppercase -> ${r}`)
}

{
  const r = hex.decode("deadbeef")
  assert(r instanceof ByteBuffer, `decode returns ByteBuffer: ${typeof r}`)
  const arr = r.toArray()
  const expected = [0xde, 0xad, 0xbe, 0xef]
  assert(
    arr.length === expected.length && arr.every((v: number, i: number) => v === expected[i]),
    `decode "deadbeef" -> [${arr.map((b: number) => "0x" + b.toString(16))}]`,
  )
}

{
  const r = hex.decode("DEADBEEF")
  const arr = r.toArray()
  const expected = [0xde, 0xad, 0xbe, 0xef]
  assert(
    arr.length === expected.length && arr.every((v: number, i: number) => v === expected[i]),
    `decode uppercase "DEADBEEF" -> [${arr}]`,
  )
}

{
  const r = hex.decode("")
  assert(r.length === 0, `decode empty -> length ${r.length}`)
}

{
  let threw = false
  try {
    hex.decode("xyz")
  } catch {
    threw = true
  }
  assert(threw, "decode invalid hex chars throws")
}

{
  let threw = false
  try {
    hex.decode("abc")
  } catch {
    threw = true
  }
  assert(threw, "decode odd-length hex throws")
}

{
  const buf = utf8.encode("test")
  const encoded = hex.encode(buf)
  const decoded = hex.decode(encoded)
  const decArr = decoded.toArray()
  const encArr = buf.toArray()
  assert(
    decArr.length === encArr.length && decArr.every((v: number, i: number) => v === encArr[i]),
    `ByteBuffer roundtrip: encoded=${encoded} decoded=[${decArr}]`,
  )
}

console.log("ALL ENCODING HEX TESTS PASSED")
