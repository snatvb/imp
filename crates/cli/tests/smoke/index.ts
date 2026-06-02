import "./some_module"
import fs from "fs/promises"
import path from "path"
console.log("hello folks!")

console.log("Running from", process.cwd())
console.log("meta", Object.keys(import.meta), import.meta.dirname)
console.log("Path resolve", path.resolve(import.meta.dirname, "text.txt"), path.resolve("asd"))
console.log("Path join", path.join(import.meta.dirname, "text.txt"), path.join("foo", "bar", 'baz'))
console.log("Path separator, delimiter", path.sep, path.delimiter)
console.log("Path basename", path.basename(import.meta.filename), path.basename(import.meta.filename, "ts"))
console.log("Path isAbsolute")
console.log("  /foo/bar    ", path.isAbsolute("/foo/bar"))
console.log("  /baz/..     ", path.isAbsolute("/baz/.."))
console.log("  //server    ", path.isAbsolute("//server"))
console.log("  C:/foo/..   ", path.isAbsolute("C:/foo/.."))
console.log("  C:\\foo\\..  ", path.isAbsolute("C:\\foo\\.."))
console.log("  qux/        ", path.isAbsolute("qux/"))
console.log("  bar\\baz     ", path.isAbsolute("bar\\baz"))
console.log("  .           ", path.isAbsolute("."))
console.log("  (empty)     ", path.isAbsolute(""))
console.log("=====")
console.log(
  path.format({
    root: '/ignored',
    dir: '/home/user/dir',
    base: 'file.txt',
  }))
// Returns: '/home/user/dir/file.txt'

console.log(
  path.format({
    root: '/',
    base: 'file.txt',
    ext: 'ignored',
  })
)
// Returns: '/file.txt'

// `name` + `ext` will be used if `base` is not specified.
console.log(
  path.format({
    root: '/',
    name: 'file',
    ext: '.txt',
  })
)
// Returns: '/file.txt'

// The dot will be added if it is not specified in `ext`.
console.log(
  path.format({
    root: '/',
    name: 'file',
    ext: 'txt',
  })
)
// Returns: '/file.txt'
console.log("=====")
console.log("File content:", await fs.readFile(import.meta.dirname + "/text.txt", "utf8"))

