import clap from "imp:clap"

// ============================================================
// POSITIVE: basic --long value
// ============================================================
{
  const parser = new clap.Parser()
    .name("test")
    .arg({ name: "name", long: "name", action: "set" })

  const result = parser.parse(["--name", "Alice"])
  assert(result.type === "ok", "type should be ok")
  if (result.type === "ok") {
    assert(result.name === "Alice", "name should be Alice")
  }
}

// ============================================================
// POSITIVE: short -x value
// ============================================================
{
  const parser = new clap.Parser()
    .name("test")
    .arg({ name: "name", short: "n", long: "name", action: "set" })

  const result = parser.parse(["-n", "Bob"])
  assert(result.type === "ok", "type should be ok")
  if (result.type === "ok") {
    assert(result.name === "Bob", "name should be Bob")
  }
}

// ============================================================
// POSITIVE: combined short flags -abc (count mode)
// ============================================================
{
  const parser = new clap.Parser()
    .name("test")
    .arg({ name: "verbose", short: "v", long: "verbose", action: "count" })

  const result = parser.parse(["-vvv"])
  assert(result.type === "ok", "type should be ok")
  if (result.type === "ok") {
    assert(result.verbose === 3, "verbose should be 3")
  }
}

// ============================================================
// POSITIVE: boolean flag (action: "flag") -> true
// ============================================================
{
  const parser = new clap.Parser()
    .name("test")
    .arg({ name: "debug", short: "d", long: "debug", action: "flag" })

  const result = parser.parse(["--debug"])
  assert(result.type === "ok", "type should be ok")
  if (result.type === "ok") {
    assert(result.debug === true, "debug should be true")
  }
}

// ============================================================
// POSITIVE: flag absent -> false (set_false default true, present false)
// ============================================================
{
  const parser = new clap.Parser()
    .name("test")
    .arg({ name: "color", long: "color", action: "set_false" })

  const resultPresent = parser.parse(["--color"])
  assert(resultPresent.type === "ok", "type should be ok")
  if (resultPresent.type === "ok") {
    assert(resultPresent.color === false, "color present should be false")
  }

  const resultAbsent = parser.parse([])
  assert(resultAbsent.type === "ok", "type should be ok when absent")
  if (resultAbsent.type === "ok") {
    assert(resultAbsent.color === true, "color absent should be true (default)")
  }
}

// ============================================================
// POSITIVE: append action - multiple values accumulated
// ============================================================
{
  const parser = new clap.Parser()
    .name("test")
    .arg({ name: "files", action: "append" })

  const result = parser.parse(["file1.txt", "file2.txt", "file3.txt"])
  assert(result.type === "ok", "type should be ok")
  if (result.type === "ok") {
    assert(result.files.length === 3, "files should have 3 elements")
    assert(result.files[0] === "file1.txt", "files[0] should be file1.txt")
    assert(result.files[1] === "file2.txt", "files[1] should be file2.txt")
    assert(result.files[2] === "file3.txt", "files[2] should be file3.txt")
  }
}

// ============================================================
// POSITIVE: choices - valid choice accepted
// ============================================================
{
  const parser = new clap.Parser()
    .name("test")
    .arg({ name: "mode", long: "mode", action: "set", choices: ["fast", "slow", "medium"] })

  const result = parser.parse(["--mode", "fast"])
  assert(result.type === "ok", "type should be ok")
  if (result.type === "ok") {
    assert(result.mode === "fast", "mode should be fast")
  }
}

// ============================================================
// POSITIVE: multiple args together
// ============================================================
{
  const parser = new clap.Parser()
    .name("test")
    .arg({ name: "name", short: "n", long: "name", action: "set" })
    .arg({ name: "verbose", short: "v", long: "verbose", action: "count" })
    .arg({ name: "output", short: "o", long: "output", action: "set" })
    .arg({ name: "debug", short: "d", long: "debug", action: "flag" })

  const result = parser.parse(["-n", "Alice", "-vv", "-o", "out.txt", "--debug"])
  assert(result.type === "ok", "type should be ok")
  if (result.type === "ok") {
    assert(result.name === "Alice", "name should be Alice")
    assert(result.verbose === 2, "verbose should be 2")
    assert(result.output === "out.txt", "output should be out.txt")
    assert(result.debug === true, "debug should be true")
  }
}

// ============================================================
// POSITIVE: num_args - multi-value args (--point 1 2)
// ============================================================
{
  const parser = new clap.Parser()
    .name("test")
    .arg({ name: "point", long: "point", action: "set", num_args: 2 })

  const result = parser.parse(["--point", "10", "20"])
  assert(result.type === "ok", "type should be ok")
  if (result.type === "ok") {
    const point = result.point as unknown as string[]
    assert(point.length === 2, "point should have 2 values")
    assert(point[0] === "10", "point[0] should be 10")
    assert(point[1] === "20", "point[1] should be 20")
  }
}

// ============================================================
// POSITIVE: version action - --version returns type "version"
// ============================================================
{
  const parser = new clap.Parser()
    .name("test")
    .version("2.5.0")
    .arg({ name: "name", long: "name", action: "set" })

  const result = parser.parse(["--version"])
  assert(result.type === "version", "type should be version")
  if (result.type === "version") {
    const msg = String(result.message)
    assert(msg.includes("2.5.0"), "message should contain version number")
  }
}

// ============================================================
// POSITIVE: help action - --help returns type "help"
// ============================================================
{
  const parser = new clap.Parser()
    .name("test")
    .about("My app")
    .arg({ name: "name", long: "name", help: "Your name", action: "set" })

  const result = parser.parse(["--help"])
  assert(result.type === "help", "type should be help")
  if (result.type === "help") {
    const msg = String(result.message)
    assert(msg.includes("Your name"), "message should contain help text")
  }
}

// ============================================================
// POSITIVE: parse([]) with no required args -> ok with undefined
// ============================================================
{
  const parser = new clap.Parser()
    .name("test")
    .arg({ name: "name", long: "name", action: "set" })
    .arg({ name: "verbose", long: "verbose", action: "count" })

  const result = parser.parse([])
  assert(result.type === "ok", "type should be ok")
  if (result.type === "ok") {
    assert(result.name === undefined, "name should be undefined")
    assert(result.verbose === 0, "verbose should be 0")
  }
}

// ============================================================
// NEGATIVE: unknown flag -> error
// ============================================================
{
  const parser = new clap.Parser()
    .name("test")
    .arg({ name: "name", long: "name", action: "set" })

  const result = parser.parse(["--unknown"])
  assert(result.type === "error", "type should be error for unknown flag")
  if (result.type === "error") {
    const msg = String(result.message)
    assert(msg.includes("--unknown"), "message should mention --unknown")
  }
}

// ============================================================
// NEGATIVE: missing required arg -> error
// ============================================================
{
  const parser = new clap.Parser()
    .name("test")
    .arg({ name: "name", long: "name", action: "set", required: true })

  const result = parser.parse([])
  assert(result.type === "error", "type should be error for missing required")
  if (result.type === "error") {
    const msg = String(result.message)
    assert(msg.length > 0, "error message should not be empty")
  }
}

// ============================================================
// NEGATIVE: invalid choice -> error mentions bad value
// ============================================================
{
  const parser = new clap.Parser()
    .name("test")
    .arg({ name: "mode", long: "mode", action: "set", choices: ["fast", "slow"] })

  const result = parser.parse(["--mode", "turbo"])
  assert(result.type === "error", "type should be error for invalid choice")
  if (result.type === "error") {
    const msg = String(result.message)
    assert(msg.includes("turbo"), "message should mention the invalid value 'turbo'")
  }
}

// ============================================================
// NEGATIVE: missing value for set arg (next arg is another flag)
// ============================================================
{
  const parser = new clap.Parser()
    .name("test")
    .arg({ name: "name", long: "name", action: "set" })
    .arg({ name: "verbose", long: "verbose", action: "count" })

  const result = parser.parse(["--name", "--verbose"])
  assert(result.type === "error", "type should be error for missing value")
}

// ============================================================
// NEGATIVE: short flag too many chars in definition -> throw TypeError
// ============================================================
{
  let threw = false
  try {
    new clap.Parser().arg({ name: "bad", short: "xy", long: "bad", action: "set" })
  } catch (e) {
    threw = true
  }
  assert(threw, "should throw TypeError for multi-char short flag")
}

// ============================================================
// EDGE: empty args array with no parser args -> ok
// ============================================================
{
  const parser = new clap.Parser().name("test")
  const result = parser.parse([])
  assert(result.type === "ok", "type should be ok for empty parse")
}

// ============================================================
// EDGE: parser with no args defined -> parse([]) is ok
// ============================================================
{
  const parser = new clap.Parser().name("test").about("no args")
  const result = parser.parse([])
  assert(result.type === "ok", "type should be ok")
}

// ============================================================
// EDGE: parse(clap.args) - parsing runtime args always succeeds
// ============================================================
{
  const parser = new clap.Parser()
    .name("test")
    .arg({ name: "name", long: "name", action: "set" })

  const result = parser.parse(clap.args)
  assert(result.type === "ok", "parsing clap.args should always succeed")
}

console.log("ALL CLAP ARGS TESTS PASSED")
