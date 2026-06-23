import path, {
  join,
  resolve,
  isAbsolute,
  relative,
  format,
  parse,
  normalize,
  basename,
  dirname,
  extname,
  toNamespacedPath,
  sep,
  delimiter,
  win32,
  posix,
} from "path"

assert(typeof join === "function", "join is function")
assert(typeof resolve === "function", "resolve is function")
assert(typeof isAbsolute === "function", "isAbsolute is function")
assert(typeof relative === "function", "relative is function")
assert(typeof format === "function", "format is function")
assert(typeof parse === "function", "parse is function")
assert(typeof normalize === "function", "normalize is function")
assert(typeof basename === "function", "basename is function")
assert(typeof dirname === "function", "dirname is function")
assert(typeof extname === "function", "extname is function")
assert(typeof toNamespacedPath === "function", "toNamespacedPath is function")
assert(typeof sep === "string", "sep is string")
assert(typeof delimiter === "string", "delimiter is string")
assert(typeof win32 === "object", "win32 is object")
assert(typeof posix === "object", "posix is object")
console.log("PASS: named imports")

assert(join("/foo", "bar").includes("foo"), "join contains foo")
console.log("PASS: join")

assert(isAbsolute(resolve("/foo", "bar")), "resolve returns absolute")
console.log("PASS: resolve")

assert(isAbsolute("/foo") || isAbsolute("C:\\foo"), "absolute path detected")
assert(!isAbsolute("foo/bar"), "relative path detected")
assert(!isAbsolute("./foo"), "dot-relative not absolute")
console.log("PASS: isAbsolute")

assert(sep === "\\" || sep === "/", "sep is valid")
assert(delimiter === ";" || delimiter === ":", "delimiter is valid")
console.log("PASS: sep/delimiter")

assert(basename("/foo/bar.txt") === "bar.txt", "basename extracts filename")
assert(basename("/foo/bar.txt", ".txt") === "bar", "basename with suffix")
console.log("PASS: basename")

assert(dirname("/foo/bar/baz.txt").includes("bar"), "dirname extracts directory")
console.log("PASS: dirname")

assert(extname("file.txt") === "txt", "extname extracts extension")
assert(extname("file") === "", "extname empty for no extension")
assert(extname("file.tar.gz") === "gz", "extname last extension")
console.log("PASS: extname")

assert(!normalize("/foo/bar//baz/asdf/quux/..").includes("//"), "normalize removes double slashes")
console.log("PASS: normalize")

const parsed = parse("C:\\path\\dir\\file.txt")
assert(typeof parsed === "object", "parse returns object")
assert(parsed.base === "file.txt", "parse base correct")
assert(parsed.ext === ".txt", "parse ext correct")
console.log("PASS: parse")

assert(format({ dir: "/home/user/dir", base: "file.txt" }).includes("file.txt"), "format combines parts")
console.log("PASS: format")

assert(relative("/data/orandea/test/aaa", "/data/orandea/impl/bbb").includes(".."), "relative contains parent ref")
console.log("PASS: relative")

assert(typeof toNamespacedPath("C:\\foo") === "string", "toNamespacedPath returns string")
console.log("PASS: toNamespacedPath")

// win32
assert(typeof win32.resolve === "function", "win32.resolve is function")
assert(typeof win32.join === "function", "win32.join is function")
assert(typeof win32.basename === "function", "win32.basename is function")
assert(typeof win32.dirname === "function", "win32.dirname is function")
assert(typeof win32.extname === "function", "win32.extname is function")
assert(typeof win32.normalize === "function", "win32.normalize is function")
assert(typeof win32.isAbsolute === "function", "win32.isAbsolute is function")
assert(typeof win32.format === "function", "win32.format is function")
assert(typeof win32.parse === "function", "win32.parse is function")
assert(typeof win32.relative === "function", "win32.relative is function")
assert(win32.sep === "\\", "win32.sep is backslash")
assert(win32.delimiter === ";", "win32.delimiter is semicolon")
assert(win32.join("foo", "bar", "baz").includes("bar"), "win32.join works")
assert(win32.isAbsolute("C:\\foo"), "win32.isAbsolute detects absolute")
assert(!win32.isAbsolute("foo\\bar"), "win32.isAbsolute detects relative")
console.log("PASS: win32")

// posix
assert(typeof posix.resolve === "function", "posix.resolve is function")
assert(typeof posix.join === "function", "posix.join is function")
assert(typeof posix.basename === "function", "posix.basename is function")
assert(typeof posix.dirname === "function", "posix.dirname is function")
assert(typeof posix.extname === "function", "posix.extname is function")
assert(typeof posix.normalize === "function", "posix.normalize is function")
assert(typeof posix.isAbsolute === "function", "posix.isAbsolute is function")
assert(typeof posix.format === "function", "posix.format is function")
assert(typeof posix.parse === "function", "posix.parse is function")
assert(typeof posix.relative === "function", "posix.relative is function")
assert(posix.sep === "/", "posix.sep is slash")
assert(posix.delimiter === ":", "posix.delimiter is colon")
assert(posix.join("foo", "bar", "baz").includes("bar"), "posix.join works")
assert(posix.isAbsolute("/foo"), "posix.isAbsolute detects absolute")
assert(!posix.isAbsolute("foo/bar"), "posix.isAbsolute detects relative")
console.log("PASS: posix")

assert(typeof path.resolve === "function", "path.resolve is function")
assert(typeof path.win32 === "object", "path.win32 exists")
assert(typeof path.posix === "object", "path.posix exists")
console.log("PASS: path default")

console.log("ALL PATH TESTS PASSED")
