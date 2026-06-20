import { resolve } from "path"

import { readFile } from "imp:fs"

console.log(import.meta.dirname)
// Helper to get absolute path relative to this test file
const fixture = (name: string) => resolve(import.meta.dirname, "fixtures", name)

// --- basic utf8 read ---
const utf8 = await readFile(fixture("hello.txt"), "utf8")
console.assert(utf8.toString() === "hello world", "utf8 content")

// --- default encoding (buffer) ---
const buf = await readFile(fixture("hello.txt"))
console.assert(buf instanceof ArrayBuffer, "default returns ArrayBuffer")
console.assert(buf.byteLength === 11, "buffer byteLength")

// --- hex encoding ---
const hex = await readFile(fixture("hello.txt"), "hex")
console.assert(hex.toString() === "68656c6c6f20776f726c64", "hex content")

// --- base64 encoding ---
const b64 = await readFile(fixture("hello.txt"), "base64")
console.assert(b64.toString() === "aGVsbG8gd29ybGQ=", "base64 content")

// --- base64url encoding ---
const b64url = await readFile(fixture("hello.txt"), "base64url")
console.assert(b64url.toString() === "aGVsbG8gd29ybGQ", "base64url content")

// --- ascii encoding ---
const ascii = await readFile(fixture("hello.txt"), "ascii")
console.assert(ascii.toString() === "hello world", "ascii content")

// --- latin1 encoding ---
const latin1 = await readFile(fixture("hello.txt"), "latin1")
console.assert(latin1.toString() === "hello world", "latin1 content")

// --- binary encoding ---
const binary = await readFile(fixture("hello.txt"), "binary")
console.assert(binary.toString() === "hello world", "binary content")

// --- error on nonexistent file ---
let threw = false
try {
  await readFile(fixture("DOES_NOT_EXIST.txt"), "utf8")
} catch (e) {
  threw = true
  console.assert(String(e).includes("ENOENT"), "ENOENT error message")
}
console.assert(threw, "throws on missing file")

console.log("ALL IMP:FS READFILE TESTS PASSED")
