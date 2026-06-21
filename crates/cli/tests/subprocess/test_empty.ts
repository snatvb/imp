import sub from "imp:subprocess"
import { run } from "imp:subprocess"

console.assert(typeof sub === "object" && sub !== null, "sub is an object")
console.assert(typeof (sub as any).run === "function", "sub.run is a function")
console.assert(typeof run === "function", "named import run is a function")
