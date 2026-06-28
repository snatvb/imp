import clap from "imp:clap"

const parser = new clap.Parser()
  .name("test")
  .version("1.0")
  .about("Test parser")
  .arg({ name: "name", short: "n", long: "name", help: "Your name", action: "set" })
  .arg({ name: "verbose", short: "v", long: "verbose", action: "count" })
  .arg({ name: "output", short: "o", long: "output", action: "set" })
  .arg({ name: "debug", short: "d", long: "debug", action: "flag" })
  .arg({ name: "files", action: "append" })

const result = parser.parse(["-n", "Alice", "-vvv", "-o", "out.txt", "--debug", "file1.txt", "file2.txt"])

assert(result.type === "ok", "type should be ok")
if (result.type === "ok") {
  assert(result.name === "Alice", "name should be Alice")
  assert(result.verbose === 3, "verbose should be 3")
  assert(result.output === "out.txt", "output should be out.txt")
  assert(result.debug === true, "debug should be true")
  assert(result.files.length === 2, "files should have 2 elements")
  assert(result.files[0] === "file1.txt", "files[0] should be file1.txt")
  assert(result.files[1] === "file2.txt", "files[1] should be file2.txt")
}

console.log("ALL CLAP TESTS PASSED")
console.log(clap.args)
