import "./some_module"
import { readFile } from "fs/promises"
console.log("hello folks!")

console.log("Running from", process.cwd())
console.log("meta", Object.keys(import.meta), import.meta.dirname)
console.log("File content:", await readFile(import.meta.dirname + "/text.txt", "utf8"))

