import inq from "imp:inq"
import { injectKeys } from "imp:sys/input_simulate"

{
  let p = inq.confirm("Delete it?")
  await injectKeys(["y", "enter"])
  console.assert((await p) === true, "confirm yes")
}

{
  let p = inq.confirm("Delete it?")
  await injectKeys(["n", "enter"])
  console.assert((await p) === false, "confirm no")
}

{
  let p = inq.select("Pick:", ["A", "B", "C"])
  await injectKeys(["down", "enter"])
  console.assert((await p) === "B", "select B")
}

{
  let p = inq.prompt("Name?")
  await injectKeys(["h", "e", "l", "l", "o", "enter"])
  console.assert((await p) === "hello", "text hello")
}

console.log("ALL INQ INJECT TESTS PASSED")
