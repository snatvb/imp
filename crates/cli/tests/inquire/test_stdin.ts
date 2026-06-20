import { readLine, readAll } from "imp:sys/stdin"

{
  const line = await readLine()
  console.assert(line.toString() === "hello", "readLine should return 'hello'")
  console.assert(line.valueOf() === "hello", "readLine valueOf should work")
}

{
  const all = await readAll()
  const str = all.toString().replace(/\r\n/g, "\n")
  console.assert(str === "world\n", "readAll should return 'world\\n'")
}

console.log("ALL STDIN TESTS PASSED")
