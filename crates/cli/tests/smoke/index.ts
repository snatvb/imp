import "./some_module"
import fs from "fs/promises"
import path from "path"
console.log("hello folks!")

console.log("Running from", process.cwd())
console.log("meta", Object.keys(import.meta), import.meta.dirname)
console.log("Path resolve", path.resolve(import.meta.dirname, "text.txt"), path.resolve("asd"))
console.log("Path join", path.join(import.meta.dirname, "text.txt"), path.join("foo", "bar", 'baz'))
console.log("Path separator", path.sep)
console.log("File content:", await fs.readFile(import.meta.dirname + "/text.txt", "utf8"))

