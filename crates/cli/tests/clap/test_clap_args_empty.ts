import clap from "imp:clap"

console.assert(Array.isArray(clap.args), "args should be an array")
console.assert(clap.args.length === 0, "args should be empty")

{
  const parser = new clap.Parser().arg({ name: "name", short: "n", long: "name", action: "set" })
  const result = parser.parse(clap.args)
  console.assert(String(result.type) === "result", "empty args should parse as result")
  if (result.type === "result") {
    console.assert(result.name === undefined, "name should be undefined")
  }
}

{
  const parser = new clap.Parser().arg({ name: "name", short: "n", long: "name", action: "set" })
  const result = parser.parse(["--invalid"])
  console.assert(String(result.type) === "error", "type should be error for unknown arg")
  if (result.type === "error") {
    console.assert(String(result.message).includes("--invalid"), "message should mention --invalid")
  }
}

{
  const parser = new clap.Parser().arg({ name: "name", short: "n", long: "name", action: "set", required: true })
  const result = parser.parse([])
  console.assert(String(result.type) === "error", "type should be error for missing required")
}

{
  const parser = new clap.Parser().arg({
    name: "mode",
    short: "m",
    long: "mode",
    action: "set",
    choices: ["fast", "slow"],
  })
  const result = parser.parse(["-m", "turbo"])
  console.assert(String(result.type) === "error", "type should be error for invalid choice")
  if (result.type === "error") {
    console.assert(String(result.message).includes("turbo"), "message should mention invalid value")
  }
}

console.log("ALL CLAP ARGS EMPTY TESTS PASSED")
