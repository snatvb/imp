import { base64 } from "imp:encoding"
import { utf8 } from "imp:encoding"

{
  const r = base64.encode("hello")
  assert(r === "aGVsbG8=", `encode "hello" -> ${r}`)
}

{
  const r = base64.encode("hello", { variant: "url", pad: false })
  assert(r === "aGVsbG8", `encode url no-pad -> ${r}`)
}

{
  const r = base64.encode("hi", { variant: "url" })
  assert(r === "aGk=", `encode url default pad -> ${r}`)
}

{
  const r = base64.encode("")
  assert(r === "", `encode empty -> "${r}"`)
}

{
  const r = base64.encode("a")
  assert(r === "YQ==", `encode "a" -> ${r}`)
}

{
  const buf = utf8.encode("hello")
  const r = base64.encode(buf)
  assert(r === "aGVsbG8=", `encode ByteBuffer "hello" -> ${r}`)
}

{
  const r = base64.decode("aGVsbG8=") as ByteBuffer
  assert(r instanceof ByteBuffer, `decode result is ByteBuffer: ${typeof r}`)
  const arr = r.toArray()
  const expected = [104, 101, 108, 108, 111]
  assert(
    arr.length === expected.length && arr.every((v: number, i: number) => v === expected[i]),
    `decode "aGVsbG8=" -> [${arr}] expected [${expected}]`,
  )
}

{
  const r = base64.decode("aGVsbG8=", { mode: "utf8" })
  assert(typeof r === "string", `decode utf8 mode returns string: ${typeof r}`)
  assert(r === "hello", `decode utf8 -> "${r}"`)
}

{
  const r = base64.decode("aGVsbG8") as ByteBuffer
  assert(r instanceof ByteBuffer, `decode no-pad returns ByteBuffer`)
  const arr = r.toArray()
  const expected = [104, 101, 108, 108, 111]
  assert(
    arr.length === expected.length && arr.every((v: number, i: number) => v === expected[i]),
    `decode no-pad bytes -> [${arr}]`,
  )
}

{
  const r = base64.decode("aGVsbG8", { mode: "utf8" })
  assert(r === "hello", `decode no-pad utf8 -> "${r}"`)
}

{
  let threw = false
  try {
    base64.decode("not_valid_base64!@#")
  } catch {
    threw = true
  }
  assert(threw, "decode invalid chars throws")
}

{
  const encoded = base64.encode("hello")
  const decoded = base64.decode(encoded, { mode: "utf8" })
  assert(decoded === "hello", `roundtrip utf8 -> "${decoded}"`)
}

{
  const buf = utf8.encode("test")
  const encoded = base64.encode(buf)
  const decoded = base64.decode(encoded) as ByteBuffer
  const decArr = decoded.toArray()
  const encArr = buf.toArray()
  assert(
    decArr.length === encArr.length &&
      decArr.every((v: number, i: number) => v === encArr[i]),
    `ByteBuffer roundtrip bytes: encoded=${encoded} decoded=[${decArr}]`,
  )
}

console.log("ALL ENCODING BASE64 TESTS PASSED")
