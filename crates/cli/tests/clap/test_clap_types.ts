import clap from "imp:clap"

const parser = new clap.Parser().name("test").arg({ name: "name", short: "n", long: "name", action: "set" })

const result = parser.parse(["-n", "Alice"])
assert(result.type === "result", "type should equal 'result' string")
assert(typeof result.type === "string", "type should be typeof string")

const helpParser = new clap.Parser()
  .name("test")
  .about("Test")
  .arg({ name: "name", short: "n", long: "name", action: "set" })

const helpResult = helpParser.parse(["--help"])
assert(helpResult.type === "help", "help type should equal 'help'")
assert(typeof helpResult.type === "string", "help type should be typeof string")

console.log("ALL CLAP TYPES TESTS PASSED")
