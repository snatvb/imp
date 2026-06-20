import { yaml } from "imp:parsers"

{
  const input = "name: test\nvalue: 42\nnested:\n  a: true\n  b: null\n"
  const parsed = yaml.parse(input) as any
  console.assert(RsString.equals(parsed.name, "test"), "name should be test")
  console.assert(parsed.value === 42, "value should be 42")
  console.assert(parsed.nested.a === true, "nested.a should be true")
  console.assert(parsed.nested.b === null, "nested.b should be null")
}

{
  const obj = { x: 1, y: [1, 2, 3], z: { nested: "value" } }
  const str = yaml.stringify(obj).toString()
  console.assert(str.includes("x: 1"), "stringify should contain x: 1")
  console.assert(str.includes("y:"), "stringify should contain y:")
  console.assert(str.includes("- 1"), "stringify should contain - 1")
}

{
  const arr = [1, 2, 3]
  const str = yaml.stringify(arr).toString()
  console.assert(str.includes("- 1"), "array stringify should contain - 1")
  console.assert(str.includes("- 2"), "array stringify should contain - 2")
  console.assert(str.includes("- 3"), "array stringify should contain - 3")
}

{
  const input = "- 1\n- 2\n- 3\n"
  const parsed = yaml.parse(input) as any[]
  console.assert(Array.isArray(parsed), "should be array")
  console.assert(parsed.length === 3, "length should be 3")
  console.assert(parsed[0] === 1 && parsed[1] === 2 && parsed[2] === 3, "array values")
}

{
  const input = "float: 3.14\nnegative: -10\n"
  const parsed = yaml.parse(input) as any
  console.assert(parsed.float === 3.14, "float should work")
  console.assert(parsed.negative === -10, "negative should work")
}

{
  let error = false
  try {
    yaml.parse("invalid:\n  - yaml\n  bad: indent")
  } catch (e) {
    error = true
  }
  console.assert(error, "invalid YAML should throw error")
}

{
  let error = false
  try {
    yaml.parse("")
  } catch (e) {
    error = true
  }
  console.assert(!error, "empty YAML should not throw error")
}

{
  let error = false
  try {
    yaml.parse('key: "unclosed string')
  } catch (e) {
    error = true
  }
  console.assert(error, "unclosed string should throw error")
}

{
  const set = new Set([1, 2, 3])
  const str = yaml.stringify(set as any).toString()
  const parsed = yaml.parse(str) as any[]
  console.assert(Array.isArray(parsed), "set should become array")
  console.assert(parsed.length === 3, "set length should be 3")
}

{
  const map = new Map([
    ["a", 1],
    ["b", 2],
  ])
  const str = yaml.stringify(map as any).toString()
  const parsed = yaml.parse(str) as any
  console.assert(parsed.a === 1, "map key a")
  console.assert(parsed.b === 2, "map key b")
}

{
  const date = new Date("2025-01-01T00:00:00.000Z")
  const str = yaml.stringify(date as any).toString()
  console.assert(str.includes("2025-01-01"), "date should be serialized")
}

{
  const regexp = /hello/gi
  const str = yaml.stringify(regexp as any).toString()
  console.assert(str.includes("hello"), "regexp should be serialized")
}

{
  const obj = { fn: () => {}, value: 42 }
  const str = yaml.stringify(obj).toString()
  const parsed = yaml.parse(str) as any
  console.assert(parsed.fn === undefined, "function should be omitted")
  console.assert(parsed.value === 42, "other values should work")
}

{
  const input = 'text: "Привет мир"\n'
  const parsed = yaml.parse(input) as any
  console.assert(RsString.equals(parsed.text, "Привет мир"), "cyrillic should parse")
}

{
  const input = 'emoji: "😀🚀🎉"\n'
  const parsed = yaml.parse(input) as any
  console.assert(RsString.equals(parsed.emoji, "😀🚀🎉"), "emoji should parse")
}

{
  const obj = { mixed: "hello мир こんにちは 🌍" }
  const str = yaml.stringify(obj).toString()
  const parsed = yaml.parse(str) as any
  console.assert(RsString.equals(parsed.mixed, "hello мир こんにちは 🌍"), "unicode roundtrip")
}

{
  const emptyObj = {}
  const str = yaml.stringify(emptyObj).toString()
  const parsed = yaml.parse(str) as any
  console.assert(typeof parsed === "object" && parsed !== null, "empty object roundtrip yaml")
}

{
  const emptyArr: any[] = []
  const str = yaml.stringify(emptyArr).toString()
  const parsed = yaml.parse(str) as any[]
  console.assert(Array.isArray(parsed) && parsed.length === 0, "empty array roundtrip yaml")
}

console.log("ALL PARSERS YAML TESTS PASSED")
