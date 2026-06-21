import inq from "imp:inq"
import { injectKeys } from "imp:sys/input_simulate"

{
  let p = inq.confirm("Delete it?")
  await injectKeys(["y", "enter"])
  assert((await p) === true, "confirm yes")
}

{
  let p = inq.confirm("Delete it?")
  await injectKeys(["n", "enter"])
  assert((await p) === false, "confirm no")
}

{
  let p = inq.select("Pick:", ["A", "B", "C"])
  await injectKeys(["down", "enter"])
  assert((await p) === "B", "select B")
}

{
  let p = inq.prompt("Name?")
  await injectKeys(["h", "e", "l", "l", "o", "enter"])
  assert((await p) === "hello", "text hello")
}

console.log("ALL INQ INJECT TESTS PASSED")
