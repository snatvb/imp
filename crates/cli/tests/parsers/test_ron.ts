import { ron } from "imp:parsers"

{
  const input = '(name: "test", value: 42)'
  const parsed = ron.parse(input) as any
  assert(RsString.equals(parsed.name, "test"), "name should be test")
  assert(parsed.value === 42, "value should be 42")
}

{
  const input = '(nested: (a: true, b: "hello"))'
  const parsed = ron.parse(input) as any
  assert(parsed.nested.a === true, "nested.a should be true")
  assert(RsString.equals(parsed.nested.b, "hello"), "nested.b should be hello")
}

{
  const input = "(array: [1, 2, 3])"
  const parsed = ron.parse(input) as any
  assert(Array.isArray(parsed.array), "array should be array")
  assert(parsed.array.length === 3, "array length should be 3")
  assert(parsed.array[0] === 1 && parsed.array[1] === 2 && parsed.array[2] === 3, "array values")
}

{
  const input = "(float: 3.14, negative: -10)"
  const parsed = ron.parse(input) as any
  assert(parsed.float === 3.14, "float should work")
  assert(parsed.negative === -10, "negative should work")
}

{
  const input = "[1, 2, 3]"
  const parsed = ron.parse(input) as any[]
  assert(Array.isArray(parsed), "should be array")
  assert(parsed.length === 3, "length should be 3")
  assert(parsed[0] === 1 && parsed[1] === 2 && parsed[2] === 3, "array values")
}

{
  const obj = { name: "test", value: 42 }
  const str = ron.stringify(obj).toString()
  assert(str.includes("name"), "stringify should contain name")
  assert(str.includes("test"), "stringify should contain test")
  assert(str.includes("value"), "stringify should contain value")
  assert(str.includes("42"), "stringify should contain 42")
}

{
  const obj = { nested: { a: true, b: "hello" } }
  const str = ron.stringify(obj).toString()
  assert(str.includes("nested"), "stringify should contain nested")
  assert(str.includes("true"), "stringify should contain true")
  assert(str.includes("hello"), "stringify should contain hello")
}

{
  let error = false
  try {
    ron.parse("invalid { syntax")
  } catch (e) {
    error = true
  }
  assert(error, "invalid RON should throw error")
}

{
  let error = false
  try {
    ron.parse("")
  } catch (e) {
    error = true
  }
  assert(error, "empty RON should throw error")
}

{
  let error = false
  try {
    ron.parse('(name: "unclosed)')
  } catch (e) {
    error = true
  }
  assert(error, "unclosed string should throw error")
}

{
  const set = new Set([1, 2, 3])
  const str = ron.stringify(set as any).toString()
  const parsed = ron.parse(str) as any[]
  assert(Array.isArray(parsed), "set should become array")
  assert(parsed.length === 3, "set length should be 3")
}

{
  const map = new Map([
    ["a", 1],
    ["b", 2],
  ])
  const str = ron.stringify(map as any).toString()
  const parsed = ron.parse(str) as any
  assert(parsed.a === 1, "map key a")
  assert(parsed.b === 2, "map key b")
}

{
  const date = new Date("2025-01-01T00:00:00.000Z")
  const str = ron.stringify(date as any).toString()
  assert(str.includes("2025-01-01"), "date should be serialized")
}

{
  const regexp = /hello/gi
  const str = ron.stringify(regexp as any).toString()
  assert(str.includes("hello"), "regexp should be serialized")
}

{
  const obj = { fn: () => {}, value: 42 }
  const str = ron.stringify(obj).toString()
  const parsed = ron.parse(str) as any
  assert(parsed.fn === undefined, "function should be omitted")
  assert(parsed.value === 42, "other values should work")
}

{
  const emptyObj = {}
  const str = ron.stringify(emptyObj).toString()
  const parsed = ron.parse(str) as any
  assert(typeof parsed === "object" && parsed !== null, "empty object roundtrip ron")
}

console.log("ALL PARSERS RON TESTS PASSED")
