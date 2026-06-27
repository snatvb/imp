import { msgpack } from "imp:parsers"

{
  const data = { name: "test", value: 42 }
  const buf = msgpack.stringify(data)
  assert(buf instanceof ByteBuffer, "stringify should return ByteBuffer")
  assert(buf.length > 0, "buffer should not be empty")
}

{
  const data = { name: "test", value: 42 }
  const buf = msgpack.stringify(data)
  const parsed = msgpack.parse(buf) as any
  assert(RsString.equals(parsed.name, "test"), "name should be test")
  assert(parsed.value === 42, "value should be 42")
}

{
  const data = { nested: { a: true, b: "hello" } }
  const buf = msgpack.stringify(data)
  const parsed = msgpack.parse(buf) as any
  assert(parsed.nested.a === true, "nested.a should be true")
  assert(RsString.equals(parsed.nested.b, "hello"), "nested.b should be hello")
}

{
  const data = { array: [1, 2, 3] }
  const buf = msgpack.stringify(data)
  const parsed = msgpack.parse(buf) as any
  assert(Array.isArray(parsed.array), "array should be array")
  assert(parsed.array.length === 3, "array length should be 3")
  assert(parsed.array[0] === 1 && parsed.array[1] === 2 && parsed.array[2] === 3, "array values")
}

{
  const data = { float: 3.14, negative: -10 }
  const buf = msgpack.stringify(data)
  const parsed = msgpack.parse(buf) as any
  assert(parsed.float === 3.14, "float should work")
  assert(parsed.negative === -10, "negative should work")
}

{
  const data = [1, "two", false, null]
  const buf = msgpack.stringify(data)
  const parsed = msgpack.parse(buf) as any[]
  assert(Array.isArray(parsed), "should be array")
  assert(parsed.length === 4, "length should be 4")
  assert(parsed[0] === 1, "first element should be 1")
  assert(RsString.equals(parsed[1], "two"), "second element should be two")
  assert(parsed[2] === false, "third element should be false")
  assert(parsed[3] === null, "fourth element should be null")
}

{
  const data = { empty: "", zero: 0, null: null }
  const buf = msgpack.stringify(data)
  const parsed = msgpack.parse(buf) as any
  assert(RsString.equals(parsed.empty, ""), "empty string should work")
  assert(parsed.zero === 0, "zero should work")
  assert(parsed.null === null, "null should work")
}

{
  const large = new Array(1000).fill(0).map((_, i) => i)
  const buf = msgpack.stringify(large)
  const parsed = msgpack.parse(buf) as number[]
  assert(parsed.length === 1000, "large array length should be 1000")
  assert(parsed[0] === 0 && parsed[999] === 999, "large array values")
}

{
  let error = false
  try {
    const invalidBuf = ByteBuffer.fromArray([0xc1])
    msgpack.parse(invalidBuf)
  } catch (e) {
    error = true
  }
  assert(error, "invalid msgpack should throw error")
}

{
  let error = false
  try {
    const emptyBuf = new ByteBuffer(0)
    msgpack.parse(emptyBuf)
  } catch (e) {
    error = true
  }
  assert(error, "empty buffer should throw error")
}

console.log("ALL PARSERS MSGPACK TESTS PASSED")
