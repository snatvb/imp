import { ron } from "imp:parsers"

{
  const input = '(name: "test", value: 42)'
  const parsed = ron.parse(input) as any
  console.assert(RsString.equals(parsed.name, "test"), "name should be test")
  console.assert(parsed.value === 42, "value should be 42")
}

{
  const input = '(nested: (a: true, b: "hello"))'
  const parsed = ron.parse(input) as any
  console.assert(parsed.nested.a === true, "nested.a should be true")
  console.assert(RsString.equals(parsed.nested.b, "hello"), "nested.b should be hello")
}

{
  const input = "(array: [1, 2, 3])"
  const parsed = ron.parse(input) as any
  console.assert(Array.isArray(parsed.array), "array should be array")
  console.assert(parsed.array.length === 3, "array length should be 3")
  console.assert(parsed.array[0] === 1 && parsed.array[1] === 2 && parsed.array[2] === 3, "array values")
}

{
  const input = "(float: 3.14, negative: -10)"
  const parsed = ron.parse(input) as any
  console.assert(parsed.float === 3.14, "float should work")
  console.assert(parsed.negative === -10, "negative should work")
}

{
  const input = "[1, 2, 3]"
  const parsed = ron.parse(input) as any[]
  console.assert(Array.isArray(parsed), "should be array")
  console.assert(parsed.length === 3, "length should be 3")
  console.assert(parsed[0] === 1 && parsed[1] === 2 && parsed[2] === 3, "array values")
}

{
  const obj = { name: "test", value: 42 }
  const str = ron.stringify(obj).toString()
  console.assert(str.includes("name"), "stringify should contain name")
  console.assert(str.includes("test"), "stringify should contain test")
  console.assert(str.includes("value"), "stringify should contain value")
  console.assert(str.includes("42"), "stringify should contain 42")
}

{
  const obj = { nested: { a: true, b: "hello" } }
  const str = ron.stringify(obj).toString()
  console.assert(str.includes("nested"), "stringify should contain nested")
  console.assert(str.includes("true"), "stringify should contain true")
  console.assert(str.includes("hello"), "stringify should contain hello")
}

{
  let error = false
  try {
    ron.parse("invalid { syntax")
  } catch (e) {
    error = true
  }
  console.assert(error, "invalid RON should throw error")
}

{
  let error = false
  try {
    ron.parse("")
  } catch (e) {
    error = true
  }
  console.assert(error, "empty RON should throw error")
}

{
  let error = false
  try {
    ron.parse('(name: "unclosed)')
  } catch (e) {
    error = true
  }
  console.assert(error, "unclosed string should throw error")
}

{
  const set = new Set([1, 2, 3])
  const str = ron.stringify(set as any).toString()
  const parsed = ron.parse(str) as any[]
  console.assert(Array.isArray(parsed), "set should become array")
  console.assert(parsed.length === 3, "set length should be 3")
}

{
  const map = new Map([
    ["a", 1],
    ["b", 2],
  ])
  const str = ron.stringify(map as any).toString()
  const parsed = ron.parse(str) as any
  console.assert(parsed.a === 1, "map key a")
  console.assert(parsed.b === 2, "map key b")
}

{
  const date = new Date("2025-01-01T00:00:00.000Z")
  const str = ron.stringify(date as any).toString()
  console.assert(str.includes("2025-01-01"), "date should be serialized")
}

{
  const regexp = /hello/gi
  const str = ron.stringify(regexp as any).toString()
  console.assert(str.includes("hello"), "regexp should be serialized")
}

{
  const obj = { fn: () => {}, value: 42 }
  const str = ron.stringify(obj).toString()
  const parsed = ron.parse(str) as any
  console.assert(parsed.fn === undefined, "function should be omitted")
  console.assert(parsed.value === 42, "other values should work")
}

{
  const emptyObj = {}
  const str = ron.stringify(emptyObj).toString()
  const parsed = ron.parse(str) as any
  console.assert(typeof parsed === "object" && parsed !== null, "empty object roundtrip ron")
}

console.log("ALL PARSERS RON TESTS PASSED")
