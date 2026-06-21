import { resolve } from "path"

import { readFile } from "imp:fs"

console.log(import.meta.dirname)
// Helper to get absolute path relative to this test file
const fixture = (name: string) => resolve(import.meta.dirname, "fixtures", name)

// --- basic utf8 read ---
const utf8 = await readFile(fixture("hello.txt"), "utf8")
assert(utf8.toString() === "hello world", "utf8 content")

// --- default encoding (buffer) ---
const buf = await readFile(fixture("hello.txt"))
assert(buf instanceof ArrayBuffer, "default returns ArrayBuffer")
assert(buf.byteLength === 11, "buffer byteLength")

// --- hex encoding ---
const hex = await readFile(fixture("hello.txt"), "hex")
assert(hex.toString() === "68656c6c6f20776f726c64", "hex content")

// --- base64 encoding ---
const b64 = await readFile(fixture("hello.txt"), "base64")
assert(b64.toString() === "aGVsbG8gd29ybGQ=", "base64 content")

// --- base64url encoding ---
const b64url = await readFile(fixture("hello.txt"), "base64url")
assert(b64url.toString() === "aGVsbG8gd29ybGQ", "base64url content")

// --- ascii encoding ---
const ascii = await readFile(fixture("hello.txt"), "ascii")
assert(ascii.toString() === "hello world", "ascii content")

// --- latin1 encoding ---
const latin1 = await readFile(fixture("hello.txt"), "latin1")
assert(latin1.toString() === "hello world", "latin1 content")

// --- binary encoding ---
const binary = await readFile(fixture("hello.txt"), "binary")
assert(binary.toString() === "hello world", "binary content")

// --- error on nonexistent file ---
let threw = false
try {
  await readFile(fixture("DOES_NOT_EXIST.txt"), "utf8")
} catch (e) {
  threw = true
  assert(String(e).includes("ENOENT"), "ENOENT error message")
}
assert(threw, "throws on missing file")

console.log("ALL IMP:FS READFILE TESTS PASSED")
