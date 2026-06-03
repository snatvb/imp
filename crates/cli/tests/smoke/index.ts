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
console.log("join     ", "cwd/t.txt, foo/bar/baz", path.join(import.meta.dirname, "text.txt"), path.join("foo", "bar", "baz/file.txt"))

console.log("=== path.sep / delimiter ===")
console.log("sep/delim", path.sep, path.delimiter)

console.log("=== path.basename ===")
console.log("basename ", "filename            ", path.basename(import.meta.filename))
console.log("basename ", "with suffix ts      ", path.basename(import.meta.filename, "ts"))

console.log("=== path.normalize ===")
console.log("normalize", "/foo/bar//baz/asdf/quux/..", path.normalize("/foo/bar//baz/asdf/quux/.."))

console.log("=== path.parse ===")
console.log("prase    ", "C:\\path\\dir\\file.txt  ", path.parse('C:\\path\\dir\\file.txt'))

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


console.log("")
console.log("=== path.win32 ===")
console.log("")
console.log("win32.sep/delim", path.win32.sep, path.win32.delimiter)
console.log("win32.resolve  ", "/abs                ", path.win32.resolve("/asd"))
console.log("win32.join     ", "foo\\bar\\baz        ", path.win32.join("foo", "bar", "baz/file.txt"))
console.log("win32.basename ", "with suffix ts      ", path.win32.basename("C:\\path\\dir\\file.txt", ".txt"))
console.log("win32.dirname  ", "C:\\path\\dir\\file   ", path.win32.dirname("C:\\path\\dir\\file.txt"))
console.log("win32.extname  ", "file.txt            ", path.win32.extname("file.txt"))
console.log("win32.extname  ", "file                ", path.win32.extname("file"))
console.log("win32.normalize", "C:\\foo\\bar\\..\\baz  ", path.win32.normalize("C:\\foo\\bar\\..\\baz"))
console.log("win32.isAbs    ", "C:/foo/..           ", path.win32.isAbsolute("C:/foo/.."))
console.log("win32.isAbs    ", "/foo/bar            ", path.win32.isAbsolute("/foo/bar"))
console.log("win32.format   ", "dir+base            ", path.win32.format({ dir: 'C:\\path', base: 'file.txt' }))
console.log("win32.parse    ", "C:\\path\\dir\\file   ", path.win32.parse('C:\\path\\dir\\file.txt'))
console.log("win32.relative ", "C:\\a\\b\\c → C:\\a\\b\\d", path.win32.relative("C:\\a\\b\\c", "C:\\a\\b\\d"))

console.log("")
console.log("=== path.posix ===")
console.log("")
console.log("posix.sep/delim", path.posix.sep, path.posix.delimiter)
console.log("posix.resolve  ", "/abs                ", path.posix.resolve("/asd"))
console.log("posix.join     ", "foo/bar/baz         ", path.posix.join("foo", "bar", "baz/file.txt"))
console.log("posix.basename ", "with suffix ts      ", path.posix.basename("/path/dir/file.txt", ".txt"))
console.log("posix.dirname  ", "/path/dir/file      ", path.posix.dirname("/path/dir/file.txt"))
console.log("posix.extname  ", "file.txt            ", path.posix.extname("file.txt"))
console.log("posix.extname  ", "file                ", path.posix.extname("file"))
console.log("posix.normalize", "/foo/bar/../baz     ", path.posix.normalize("/foo/bar/../baz"))
console.log("posix.isAbs    ", "/foo/bar            ", path.posix.isAbsolute("/foo/bar"))
console.log("posix.isAbs    ", "qux/                ", path.posix.isAbsolute("qux/"))
console.log("posix.format   ", "dir+base            ", path.posix.format({ dir: '/home/user', base: 'file.txt' }))
console.log("posix.parse    ", "/path/dir/file      ", path.posix.parse('/path/dir/file.txt'))
console.log("posix.relative ", "/a/b/c → /a/b/d     ", path.posix.relative("/a/b/c", "/a/b/d"))

console.log("")
console.log("=== path.relative ===")
console.log("relative ", "/data/orandea/test/aaa → /data/orandea/impl/bbb  ", path.relative("/data/orandea/test/aaa", "/data/orandea/impl/bbb"))
console.log("relative ", "same path                                      ", path.relative("/a/b/c", "/a/b/c"))
console.log("relative ", "subdir                                         ", path.relative("/a/b/c", "/a/b/c/d"))
console.log("relative ", "parent                                         ", path.relative("/a/b/c", "/a/b"))
console.log("relative ", "sibling                                        ", path.relative("/a/b/c", "/a/b/d"))
console.log("relative ", "C:\\orandea\\test\\aaa → C:\\orandea\\impl\\bbb    ", path.relative("C:\\orandea\\test\\aaa", "C:\\orandea\\impl\\bbb"))
console.log("relative ", "C: → D: (different drive)                     ", path.relative("C:\\foo", "D:\\bar"))

console.log("=== path.format ===")
console.log("format   ", "dir+base            ", path.format({ root: '/ignored', dir: '/home/user/dir', base: 'file.txt' }))
console.log("format   ", "root+base           ", path.format({ root: '/', base: 'file.txt', ext: 'ignored' }))
console.log("format   ", "root+name+ext       ", path.format({ root: '/', name: 'file', ext: '.txt' }))
console.log("format   ", "root+name+ext (no .)", path.format({ root: '/', name: 'file', ext: 'txt' }))

console.log("=== objects ===")
console.log({ a: 1, b: "hello", c: true, d: null, e: undefined })
console.log({ nested: { foo: [1, 2, { bar: "baz" }] } })
console.log({ date: new Date(), err: new Error("test") })

console.log("=== Map ===")
const m = new Map()
m.set("x", 10)
m.set("y", { z: 20 })
console.log(m, "| size:", m.size)

console.log("=== Set ===")
const s = new Set([1, 2, 3, "four", { obj: true }])
console.log(s, "| size:", s.size)

console.log("=== fs ===")
console.log("fs read  ", await fs.readFile(import.meta.dirname + "/text.txt", "utf8"))
