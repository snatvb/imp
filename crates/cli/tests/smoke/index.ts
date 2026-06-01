import "./some_module"
import { readFile } from "fs/promises"
import { resolve } from "path"
console.log("hello folks!")

console.log("Running from", process.cwd())
console.log("meta", Object.keys(import.meta), import.meta.dirname)
console.log("Path resolve", resolve(import.meta.dirname, "text.txt"), resolve("asd", 123))
console.log("File content:", await readFile(import.meta.dirname + "/text.txt", "utf8"))

