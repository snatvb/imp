import env from "imp:env"
import { env as named, parseIni, parseDotenv, expand, merge, loadFile } from "imp:env"

console.assert(typeof env === "object" && env !== null, "default export is an object")
console.assert(typeof (env as any).parseIni === "function", "env.parseIni is a function")
console.assert(typeof (env as any).parseDotenv === "function", "env.parseDotenv is a function")
console.assert(typeof (env as any).expand === "function", "env.expand is a function")
console.assert(typeof (env as any).merge === "function", "env.merge is a function")
console.assert(typeof (env as any).loadFile === "function", "env.loadFile is a function")

console.assert(typeof named === "object", "named env is an object")
console.assert(typeof parseIni === "function", "parseIni is a function")
console.assert(typeof parseDotenv === "function", "parseDotenv is a function")
console.assert(typeof expand === "function", "expand is a function")
console.assert(typeof merge === "function", "merge is a function")
console.assert(typeof loadFile === "function", "loadFile is a function")

console.log("ALL ENV EMPTY TESTS PASSED")

