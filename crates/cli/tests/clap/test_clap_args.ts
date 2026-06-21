import clap from "imp:clap"

assert(Array.isArray(clap.args), "args should be an array")
assert(clap.args.length > 0, "args should not be empty")

{
  const parser = new clap.Parser()
    .name("test")
    .arg({ name: "name", short: "n", long: "name", action: "set" })
    .arg({ name: "verbose", short: "v", long: "verbose", action: "count" })

  const result = parser.parse(clap.args)
  assert(String(result.type) === "result", "type should be result")
  if (result.type === "result") {
    assert(result.name === "Alice", "name should be Alice")
    assert(result.verbose === 3, "verbose should be 3")
  }
}

{
  const parser = new clap.Parser()
    .name("test")
    .arg({ name: "name", short: "n", long: "name", action: "set" })
    .arg({ name: "verbose", short: "v", long: "verbose", action: "count" })

  const result = parser.parse(clap.args)
  assert(String(result.type) === "result", "type should be result")
  if (result.type === "result") {
    assert(result.name === "Alice", "name should be Alice")
    assert(result.verbose === 3, "verbose should be 3")
  }
}

console.log("ALL CLAP ARGS TESTS PASSED")
