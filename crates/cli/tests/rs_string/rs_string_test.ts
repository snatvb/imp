// interning: fromString returns same object for same string
const internA = RsString.fromString("foo")
const internB = RsString.fromString("foo")
const internC = RsString.fromString("bar")
assert(internA === internB, "fromString interning (===)")
assert(internA !== internC, "fromString different strings (!==)")
assert(RsString.fromString(42) === RsString.fromString("42"), "fromString num===str interning")

const s = RsString.fromString("Hello World")

// basic
assert(s.length === 11, "length")
assert(s.toString() === "Hello World", "toString")
assert(typeof s.valueOf() === "string", "valueOf type")
assert(typeof s === "object" && s !== null, "instance of RsString")
assert(Object.getPrototypeOf(s) !== String.prototype, "instance of not String")
assert(typeof s !== "number", "instance of not Number")

// slice methods
assert(s.at(0).toString() === "H", "at(0)")
assert(s.at(-1).toString() === "d", "at(-1)")
assert(s.at(99).toString() === "", "at OOB")
assert(s.charAt(0).toString() === "H", "charAt(0)")
assert(s.charAt(99).toString() === "", "charAt OOB")
assert(s.charCodeAt(0) === 72, "charCodeAt(0)")
assert(s.charCodeAt(99) === -1, "charCodeAt OOB")
assert(s.codePointAt(0) === 72, "codePointAt(0)")
assert(s.substring(0, 5).toString() === "Hello", "substring(0,5)")
assert(s.slice(0, 5).toString() === "Hello", "slice(0,5)")
assert(s.slice(-5).toString() === "World", "slice(-5)")
assert(s.substr(0, 5).toString() === "Hello", "substr(0,5)")
assert(RsString.fromString("  hi  ").trim().toString() === "hi", "trim")
assert(RsString.fromString("  hi  ").trimStart().toString() === "hi  ", "trimStart")
assert(RsString.fromString("  hi  ").trimEnd().toString() === "  hi", "trimEnd")

// search methods
assert(s.indexOf("llo") === 2, "indexOf llo")
assert(s.indexOf("X") === -1, "indexOf X")
assert(s.lastIndexOf("l") === 9, "lastIndexOf l")
assert(s.includes("World") === true, "includes World")
assert(s.startsWith("Hello") === true, "startsWith Hello")
assert(s.endsWith("World") === true, "endsWith World")

// transform methods
assert(s.concat(" !").toString() === "Hello World !", "concat")
assert(RsString.fromString("ab").repeat(3).toString() === "ababab", "repeat")
assert(RsString.fromString("5").padStart(3, "0").toString() === "005", "padStart")
assert(RsString.fromString("5").padEnd(3, "0").toString() === "500", "padEnd")
assert(RsString.fromString("HELLO").toLowerCase().toString() === "hello", "toLowerCase")
assert(RsString.fromString("hello").toUpperCase().toString() === "HELLO", "toUpperCase")

// locale
assert(RsString.fromString("a").localeCompare("a") === 0, "localeCompare same")
assert(RsString.fromString("a").localeCompare("b") === -1, "localeCompare less")
assert(RsString.fromString("b").localeCompare("a") === 1, "localeCompare more")

// regexp methods
assert(RsString.fromString("hello world").replace("world", "JS").toString() === "hello JS", "replace")
assert(RsString.fromString("ab a ab").replace("ab", "X").toString() === "X a ab", "replace first only")
assert(RsString.fromString("ab a ab").replaceAll("ab", "X").toString() === "X a X", "replaceAll")
assert(RsString.fromString("hello123world").search(/[0-9]+/) === 5, "search")

const mr = RsString.fromString("hello123world456").match(/[0-9]+/)
assert(mr !== null && mr[0] === "123", "match")

const sp = RsString.fromString("a,b,c").split(",", undefined)
assert(sp.length === 3, "split length")

// static methods
assert(RsString.fromString(42).toString() === "42", "fromString number")
assert(RsString.fromCharCode(72, 105).toString() === "Hi", "fromCharCode")
assert(RsString.fromCodePoint(128169).toString() === "\u{1F4A9}", "fromCodePoint")

// iterator
let iterCount = 0
for (const ch of RsString.fromString("ABC")) {
  iterCount++
}
assert(iterCount === 3, "iterator yield count")

// toJSON
const jstr = RsString.fromString("test")
assert(JSON.stringify(jstr) === '"test"', "toJSON")

console.log("ALL RSSTRING TESTS PASSED")
