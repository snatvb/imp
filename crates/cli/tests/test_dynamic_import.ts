import "./helper.ts"

const { default: helper } = await import("./helper.ts")
assert(typeof helper === "function", "dynamic import returns module with function")

const result = helper()
assert(result === 42, "helper returns 42")

console.log("ALL DYNAMIC IMPORT TESTS PASSED")
