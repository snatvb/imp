import config from "./data/config.json" with { type: "json" }
assert(typeof config === "object", "config is object")
assert(config.name === "imp", "config.name is imp")
assert(config.version === "0.1.0", "config.version is 0.1.0")
assert(Array.isArray(config.features), "config.features is array")
assert(config.features.length === 3, "config.features has 3 items")

console.log("ALL JSON IMPORT TESTS PASSED")
