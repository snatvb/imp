import "./some_module"
import fs from "fs/promises"
import path from "path"
console.log("hello folks!")

console.log("=== meta ===")
console.log("Running from", process.cwd())
console.log("meta", Object.keys(import.meta), import.meta.dirname)

console.log("=== path.resolve ===")
console.log("resolve  ", "cwd/t.txt           ", path.resolve(import.meta.dirname, "text.txt"))
console.log("resolve  ", "/abs                ", path.resolve("/asd"))

console.log("=== path.join ===")
console.log("join     ", "cwd/t.txt, foo/bar/baz", path.join(import.meta.dirname, "text.txt"), path.join("foo", "bar", "baz"))

console.log("=== path.sep / delimiter ===")
console.log("sep/delim", path.sep, path.delimiter)

console.log("=== path.basename ===")
console.log("basename ", "filename            ", path.basename(import.meta.filename))
console.log("basename ", "with suffix ts      ", path.basename(import.meta.filename, "ts"))

console.log("=== path.isAbsolute ===")
console.log("isAbs    ", "/foo/bar            ", path.isAbsolute("/foo/bar"))
console.log("isAbs    ", "/baz/..             ", path.isAbsolute("/baz/.."))
console.log("isAbs    ", "//server            ", path.isAbsolute("//server"))
console.log("isAbs    ", "C:/foo/..           ", path.isAbsolute("C:/foo/.."))
console.log("isAbs    ", "C:\\foo\\..           ", path.isAbsolute("C:\\foo\\.."))
console.log("isAbs    ", "qux/                ", path.isAbsolute("qux/"))
console.log("isAbs    ", "bar\\baz             ", path.isAbsolute("bar\\baz"))
console.log("isAbs    ", ".                   ", path.isAbsolute("."))
console.log("isAbs    ", "(empty)             ", path.isAbsolute(""))

console.log("=== path.format ===")
console.log("format   ", "dir+base            ", path.format({ root: '/ignored', dir: '/home/user/dir', base: 'file.txt' }))
console.log("format   ", "root+base           ", path.format({ root: '/', base: 'file.txt', ext: 'ignored' }))
console.log("format   ", "root+name+ext       ", path.format({ root: '/', name: 'file', ext: '.txt' }))
console.log("format   ", "root+name+ext (no .)", path.format({ root: '/', name: 'file', ext: 'txt' }))

console.log("=== fs ===")
console.log("fs read  ", await fs.readFile(import.meta.dirname + "/text.txt", "utf8"))
