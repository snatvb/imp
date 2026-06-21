import enc from "imp:encoding"
import { base64, hex, utf8, uri } from "imp:encoding"

console.assert(enc !== null && typeof enc === "object", "default import is an object")
console.assert(typeof enc.base64 === "object" && enc.base64 !== null, "enc.base64 is an object")
console.assert(typeof enc.hex === "object" && enc.hex !== null, "enc.hex is an object")
console.assert(typeof enc.utf8 === "object" && enc.utf8 !== null, "enc.utf8 is an object")
console.assert(typeof enc.uri === "object" && enc.uri !== null, "enc.uri is an object")
console.assert(typeof enc.base64.encode === "function", "enc.base64.encode is a function")
console.assert(typeof enc.base64.decode === "function", "enc.base64.decode is a function")
console.assert(typeof enc.hex.encode === "function", "enc.hex.encode is a function")
console.assert(typeof enc.hex.decode === "function", "enc.hex.decode is a function")
console.assert(typeof enc.utf8.encode === "function", "enc.utf8.encode is a function")
console.assert(typeof enc.utf8.decode === "function", "enc.utf8.decode is a function")
console.assert(typeof enc.uri.encode === "function", "enc.uri.encode is a function")
console.assert(typeof enc.uri.decode === "function", "enc.uri.decode is a function")

console.assert(base64 === enc.base64, "named base64 matches default.base64")
console.assert(hex === enc.hex, "named hex matches default.hex")
console.assert(utf8 === enc.utf8, "named utf8 matches default.utf8")
console.assert(uri === enc.uri, "named uri matches default.uri")
