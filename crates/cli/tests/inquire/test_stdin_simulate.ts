import { injectKeys } from "imp:sys/input_simulate"
import { readLine } from "imp:sys/stdin"

setTimeout(async () => {
  await injectKeys(["h", "e", "l", "l", "o", "Enter"])
}, 100)

const line = await readLine()
console.assert(line.toString() === "hello", "readLine with injectKeys should return 'hello'")

console.log("ALL STDIN SIMULATE TESTS PASSED")
