import { uri } from "imp:encoding"

{
  const r = uri.encode("hello world!")
  console.assert(r === "hello%20world%21", `encode "hello world!" -> ${r}`)
}

{
  const r = uri.encode("unreserved-_.~")
  console.assert(r === "unreserved-_.~", `encode unreserved preserved -> ${r}`)
}

{
  const r = uri.encode("")
  console.assert(r === "", `encode empty -> "${r}"`)
}

{
  const r = uri.encode("a&b=c")
  console.assert(r === "a%26b%3Dc", `encode reserved -> ${r}`)
}

{
  const r = uri.encode("test/path?query=1&x=2")
  console.assert(
    r === "test%2Fpath%3Fquery%3D1%26x%3D2",
    `encode path/query -> ${r}`,
  )
}

{
  const r = uri.encode("привет")
  console.assert(typeof r === "string" && r.length > 0, `encode cyrillic non-empty: ${r}`)
  console.assert(r.indexOf("%") >= 0, `encode cyrillic contains percent: ${r}`)
}

{
  const r = uri.decode("hello%20world%21")
  console.assert(r === "hello world!", `decode -> "${r}"`)
}

{
  const r = uri.decode("a+b")
  console.assert(r === "a+b", `decode + stays + (not space) -> "${r}"`)
}

{
  const r = uri.decode("a%2Bb")
  console.assert(r === "a+b", `decode %2B -> +: "${r}"`)
}

{
  const r = uri.decode("safe-_.~")
  console.assert(r === "safe-_.~", `decode unreserved -> "${r}"`)
}

{
  const r = uri.decode("")
  console.assert(r === "", `decode empty -> "${r}"`)
}

{
  let threw = false
  try {
    uri.decode("abc%")
  } catch {
    threw = true
  }
  console.assert(threw, "decode incomplete percent throws")
}

{
  let threw = false
  try {
    uri.decode("abc%ZZ")
  } catch {
    threw = true
  }
  console.assert(threw, "decode invalid percent hex throws")
}

{
  let threw = false
  try {
    uri.decode("abc%2")
  } catch {
    threw = true
  }
  console.assert(threw, "decode short percent throws")
}

{
  const original = "hello world! привет 🌍"
  const encoded = uri.encode(original)
  const decoded = uri.decode(encoded)
  console.assert(decoded === original, `roundtrip unicode -> "${decoded}"`)
}

console.log("ALL ENCODING URI TESTS PASSED")
