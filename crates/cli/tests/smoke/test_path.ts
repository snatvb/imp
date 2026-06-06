import path, { join, resolve, isAbsolute, relative, format, parse, normalize, basename, dirname, extname, toNamespacedPath, sep, delimiter, win32, posix } from "path";

console.assert(typeof join === "function", "join is function");
console.assert(typeof resolve === "function", "resolve is function");
console.assert(typeof isAbsolute === "function", "isAbsolute is function");
console.assert(typeof relative === "function", "relative is function");
console.assert(typeof format === "function", "format is function");
console.assert(typeof parse === "function", "parse is function");
console.assert(typeof normalize === "function", "normalize is function");
console.assert(typeof basename === "function", "basename is function");
console.assert(typeof dirname === "function", "dirname is function");
console.assert(typeof extname === "function", "extname is function");
console.assert(typeof toNamespacedPath === "function", "toNamespacedPath is function");
console.assert(typeof sep === "string", "sep is string");
console.assert(typeof delimiter === "string", "delimiter is string");
console.assert(typeof win32 === "object", "win32 is object");
console.assert(typeof posix === "object", "posix is object");
console.log("PASS: named imports");

console.assert(join("/foo", "bar").includes("foo"), "join contains foo");
console.log("PASS: join");

console.assert(isAbsolute(resolve("/foo", "bar")), "resolve returns absolute");
console.log("PASS: resolve");

console.assert(isAbsolute("/foo") || isAbsolute("C:\\foo"), "absolute path detected");
console.assert(!isAbsolute("foo/bar"), "relative path detected");
console.assert(!isAbsolute("./foo"), "dot-relative not absolute");
console.log("PASS: isAbsolute");

console.assert(sep === "\\" || sep === "/", "sep is valid");
console.assert(delimiter === ";" || delimiter === ":", "delimiter is valid");
console.log("PASS: sep/delimiter");

console.assert(basename("/foo/bar.txt") === "bar.txt", "basename extracts filename");
console.assert(basename("/foo/bar.txt", ".txt") === "bar", "basename with suffix");
console.log("PASS: basename");

console.assert(dirname("/foo/bar/baz.txt").includes("bar"), "dirname extracts directory");
console.log("PASS: dirname");

console.assert(extname("file.txt") === "txt", "extname extracts extension");
console.assert(extname("file") === "", "extname empty for no extension");
console.assert(extname("file.tar.gz") === "gz", "extname last extension");
console.log("PASS: extname");

console.assert(!normalize("/foo/bar//baz/asdf/quux/..").includes("//"), "normalize removes double slashes");
console.log("PASS: normalize");

const parsed = parse("C:\\path\\dir\\file.txt");
console.assert(typeof parsed === "object", "parse returns object");
console.assert(parsed.base === "file.txt", "parse base correct");
console.assert(parsed.ext === ".txt", "parse ext correct");
console.log("PASS: parse");

console.assert(format({ dir: "/home/user/dir", base: "file.txt" }).includes("file.txt"), "format combines parts");
console.log("PASS: format");

console.assert(relative("/data/orandea/test/aaa", "/data/orandea/impl/bbb").includes(".."), "relative contains parent ref");
console.log("PASS: relative");

console.assert(typeof toNamespacedPath("C:\\foo") === "string", "toNamespacedPath returns string");
console.log("PASS: toNamespacedPath");

// win32
console.assert(typeof win32.resolve === "function", "win32.resolve is function");
console.assert(typeof win32.join === "function", "win32.join is function");
console.assert(typeof win32.basename === "function", "win32.basename is function");
console.assert(typeof win32.dirname === "function", "win32.dirname is function");
console.assert(typeof win32.extname === "function", "win32.extname is function");
console.assert(typeof win32.normalize === "function", "win32.normalize is function");
console.assert(typeof win32.isAbsolute === "function", "win32.isAbsolute is function");
console.assert(typeof win32.format === "function", "win32.format is function");
console.assert(typeof win32.parse === "function", "win32.parse is function");
console.assert(typeof win32.relative === "function", "win32.relative is function");
console.assert(win32.sep === "\\", "win32.sep is backslash");
console.assert(win32.delimiter === ";", "win32.delimiter is semicolon");
console.assert(win32.join("foo", "bar", "baz").includes("bar"), "win32.join works");
console.assert(win32.isAbsolute("C:\\foo"), "win32.isAbsolute detects absolute");
console.assert(!win32.isAbsolute("foo\\bar"), "win32.isAbsolute detects relative");
console.log("PASS: win32");

// posix
console.assert(typeof posix.resolve === "function", "posix.resolve is function");
console.assert(typeof posix.join === "function", "posix.join is function");
console.assert(typeof posix.basename === "function", "posix.basename is function");
console.assert(typeof posix.dirname === "function", "posix.dirname is function");
console.assert(typeof posix.extname === "function", "posix.extname is function");
console.assert(typeof posix.normalize === "function", "posix.normalize is function");
console.assert(typeof posix.isAbsolute === "function", "posix.isAbsolute is function");
console.assert(typeof posix.format === "function", "posix.format is function");
console.assert(typeof posix.parse === "function", "posix.parse is function");
console.assert(typeof posix.relative === "function", "posix.relative is function");
console.assert(posix.sep === "/", "posix.sep is slash");
console.assert(posix.delimiter === ":", "posix.delimiter is colon");
console.assert(posix.join("foo", "bar", "baz").includes("bar"), "posix.join works");
console.assert(posix.isAbsolute("/foo"), "posix.isAbsolute detects absolute");
console.assert(!posix.isAbsolute("foo/bar"), "posix.isAbsolute detects relative");
console.log("PASS: posix");

console.assert(typeof path.resolve === "function", "path.resolve is function");
console.assert(typeof path.win32 === "object", "path.win32 exists");
console.assert(typeof path.posix === "object", "path.posix exists");
console.log("PASS: path default");

console.log("ALL PATH TESTS PASSED");
