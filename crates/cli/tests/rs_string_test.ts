// interning: fromString returns same object for same string
const internA = RsString.fromString("foo")
const internB = RsString.fromString("foo")
const internC = RsString.fromString("bar")
console.assert(internA === internB, "fromString interning (===)")
console.assert(internA !== internC, "fromString different strings (!==)")
console.assert(RsString.fromString(42) === RsString.fromString("42"), "fromString num===str interning")

const s = RsString.fromString("Hello World")

// basic
console.assert(s.length === 11, "length")
console.assert(s.toString() === "Hello World", "toString")
console.assert(typeof s.valueOf() === "string", "valueOf type")
console.assert(typeof s === "object" && s !== null, "instance of RsString")
console.assert(Object.getPrototypeOf(s) !== String.prototype, "instance of not String")
console.assert(typeof s !== "number", "instance of not Number")

// slice methods
console.assert(s.at(0).toString() === "H", "at(0)")
console.assert(s.at(-1).toString() === "d", "at(-1)")
console.assert(s.at(99).toString() === "", "at OOB")
console.assert(s.charAt(0).toString() === "H", "charAt(0)")
console.assert(s.charAt(99).toString() === "", "charAt OOB")
console.assert(s.charCodeAt(0) === 72, "charCodeAt(0)")
console.assert(s.charCodeAt(99) === -1, "charCodeAt OOB")
console.assert(s.codePointAt(0) === 72, "codePointAt(0)")
console.assert(s.substring(0, 5).toString() === "Hello", "substring(0,5)")
console.assert(s.slice(0, 5).toString() === "Hello", "slice(0,5)")
console.assert(s.slice(-5).toString() === "World", "slice(-5)")
console.assert(s.substr(0, 5).toString() === "Hello", "substr(0,5)")
console.assert(RsString.fromString("  hi  ").trim().toString() === "hi", "trim")
console.assert(RsString.fromString("  hi  ").trimStart().toString() === "hi  ", "trimStart")
console.assert(RsString.fromString("  hi  ").trimEnd().toString() === "  hi", "trimEnd")

// search methods
console.assert(s.indexOf("llo") === 2, "indexOf llo")
console.assert(s.indexOf("X") === -1, "indexOf X")
console.assert(s.lastIndexOf("l") === 9, "lastIndexOf l")
console.assert(s.includes("World") === true, "includes World")
console.assert(s.startsWith("Hello") === true, "startsWith Hello")
console.assert(s.endsWith("World") === true, "endsWith World")

// transform methods
console.assert(s.concat(" !").toString() === "Hello World !", "concat")
console.assert(RsString.fromString("ab").repeat(3).toString() === "ababab", "repeat")
console.assert(RsString.fromString("5").padStart(3, "0").toString() === "005", "padStart")
console.assert(RsString.fromString("5").padEnd(3, "0").toString() === "500", "padEnd")
console.assert(RsString.fromString("HELLO").toLowerCase().toString() === "hello", "toLowerCase")
console.assert(RsString.fromString("hello").toUpperCase().toString() === "HELLO", "toUpperCase")

// locale
console.assert(RsString.fromString("a").localeCompare("a") === 0, "localeCompare same")
console.assert(RsString.fromString("a").localeCompare("b") === -1, "localeCompare less")
console.assert(RsString.fromString("b").localeCompare("a") === 1, "localeCompare more")

// regexp methods
console.assert(RsString.fromString("hello world").replace("world", "JS").toString() === "hello JS", "replace")
console.assert(RsString.fromString("ab a ab").replace("ab", "X").toString() === "X a ab", "replace first only")
console.assert(RsString.fromString("ab a ab").replaceAll("ab", "X").toString() === "X a X", "replaceAll")
console.assert(RsString.fromString("hello123world").search(/[0-9]+/) === 5, "search")

const mr = RsString.fromString("hello123world456").match(/[0-9]+/)
console.assert(mr !== null && mr[0] === "123", "match")

const sp = RsString.fromString("a,b,c").split(",", undefined)
console.assert(sp.length === 3, "split length")

// static methods
console.assert(RsString.fromString(42).toString() === "42", "fromString number")
console.assert(RsString.fromCharCode(72, 105).toString() === "Hi", "fromCharCode")
console.assert(RsString.fromCodePoint(128169).toString() === "\u{1F4A9}", "fromCodePoint")

// iterator
let iterCount = 0
for (const ch of RsString.fromString("ABC")) {
  iterCount++
}
console.assert(iterCount === 3, "iterator yield count")

// toJSON
const jstr = RsString.fromString("test")
console.assert(JSON.stringify(jstr) === '"test"', "toJSON")

console.log("ALL RSSTRING TESTS PASSED")
