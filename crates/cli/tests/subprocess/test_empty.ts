import sub from "imp:subprocess"
import { run } from "imp:subprocess"

assert(typeof sub === "object" && sub !== null, "sub is an object")
assert(typeof (sub as any).run === "function", "sub.run is a function")
assert(typeof run === "function", "named import run is a function")
