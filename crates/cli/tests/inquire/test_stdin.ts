import { readLine, readAll } from "imp:sys/stdin"

{
  const line = await readLine()
  assert(line.toString() === "hello", "readLine should return 'hello'")
  assert(line.valueOf() === "hello", "readLine valueOf should work")
}

{
  const all = await readAll()
  const str = all.toString().replace(/\r\n/g, "\n")
  assert(str === "world\n", "readAll should return 'world\\n'")
}

console.log("ALL STDIN TESTS PASSED")
