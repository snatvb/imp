import { toml } from "imp:parsers"

{
  const input = 'name = "test"\nvalue = 42\n'
  const parsed = toml.parse(input) as any
  console.assert(RsString.equals(parsed.name, "test"), "name should be test")
  console.assert(parsed.value === 42, "value should be 42")
}

{
  const input = '[nested]\na = true\nb = "hello"\n'
  const parsed = toml.parse(input) as any
  console.assert(parsed.nested.a === true, "nested.a should be true")
  console.assert(RsString.equals(parsed.nested.b, "hello"), "nested.b should be hello")
}

{
  const input = "array = [1, 2, 3]\n"
  const parsed = toml.parse(input) as any
  console.assert(Array.isArray(parsed.array), "array should be array")
  console.assert(parsed.array.length === 3, "array length should be 3")
  console.assert(parsed.array[0] === 1 && parsed.array[1] === 2 && parsed.array[2] === 3, "array values")
}

{
  const input = "float = 3.14\nnegative = -10\n"
  const parsed = toml.parse(input) as any
  console.assert(parsed.float === 3.14, "float should work")
  console.assert(parsed.negative === -10, "negative should work")
}

{
  const obj = { name: "test", value: 42 }
  const str = toml.stringify(obj).toString()
  console.assert(str.includes('name = "test"'), "stringify should contain name")
  console.assert(str.includes("value = 42"), "stringify should contain value")
}

{
  const obj = { nested: { a: true, b: "hello" } }
  const str = toml.stringify(obj).toString()
  console.assert(str.includes("[nested]"), "stringify should contain [nested]")
  console.assert(str.includes("a = true"), "stringify should contain a = true")
  console.assert(str.includes('b = "hello"'), "stringify should contain b = hello")
}

{
  let error = false
  try {
    toml.parse("invalid = = =")
  } catch (e) {
    error = true
  }
  console.assert(error, "invalid TOML should throw error")
}

{
  let error = false
  try {
    toml.parse("")
  } catch (e) {
    error = true
  }
  console.assert(!error, "empty TOML should not throw error")
}

{
  let error = false
  try {
    toml.parse('key = "unclosed string')
  } catch (e) {
    error = true
  }
  console.assert(error, "unclosed string should throw error")
}

{
  const set = new Set([1, 2, 3])
  const obj = { items: set }
  const str = toml.stringify(obj).toString()
  console.assert(str.includes("1"), "set should be serialized")
  console.assert(str.includes("2"), "set should contain 2")
  console.assert(str.includes("3"), "set should contain 3")
}

{
  const map = new Map([
    ["a", 1],
    ["b", 2],
  ])
  const str = toml.stringify(map as any).toString()
  console.assert(str.includes("a = 1"), "map key a")
  console.assert(str.includes("b = 2"), "map key b")
}

{
  const date = new Date("2025-01-01T00:00:00.000Z")
  const obj = { created: date }
  const str = toml.stringify(obj).toString()
  console.assert(str.includes("2025-01-01"), "date should be serialized as native datetime")
}

{
  const regexp = /hello/gi
  const obj = { pattern: regexp }
  const str = toml.stringify(obj).toString()
  console.assert(str.includes("hello"), "regexp should be serialized")
}

{
  const obj = { fn: () => {}, value: 42 }
  const str = toml.stringify(obj).toString()
  console.assert(!str.includes("fn"), "function should be omitted")
  console.assert(str.includes("value = 42"), "other values should work")
}

{
  const input = "big = 3000000000\n"
  const parsed = toml.parse(input) as any
  console.assert(parsed.big === 3000000000, "large integer should not truncate")
}

{
  const input = "neg = -3000000000\n"
  const parsed = toml.parse(input) as any
  console.assert(parsed.neg === -3000000000, "large negative integer should not truncate")
}

{
  const input = "over = 2147483648\n"
  const parsed = toml.parse(input) as any
  console.assert(parsed.over === 2147483648, "i32+1 should not truncate")
}

{
  let error = false
  try {
    toml.stringify((() => {}) as any)
  } catch (e) {
    error = true
  }
  console.assert(error, "top-level function should throw error")
}

{
  const emptyObj = {}
  const str = toml.stringify(emptyObj).toString()
  console.assert(RsString.equals(str.trim(), ""), "empty object toml should be empty")
}

{
  const emptyArr: any[] = []
  const obj = { items: emptyArr }
  const str = toml.stringify(obj).toString()
  console.assert(str.includes("items = []"), "empty array in toml")
}

{
  const input = "created = 2025-01-01T00:00:00Z\n"
  const parsed = toml.parse(input) as any
  console.assert(
    typeof parsed.created.valueOf?.() === "string" || typeof parsed.created === "string",
    "datetime should parse as string",
  )
  console.assert(parsed.created.includes("2025-01-01"), "datetime should contain date")

  const str = toml.stringify({ created: parsed.created }).toString()
  console.assert(str.includes("2025-01-01"), "datetime should roundtrip through toml")
}

console.log("ALL PARSERS TOML TESTS PASSED")
