import { join } from "path"

import { readFile, writeFile, mkdir, remove } from "imp:fs"
import { sha256 } from "imp:hash"

const tmpDir = import.meta.dirname + "/.tmp/sha256_string_test"
await mkdir(tmpDir, { recursive: true })

const testFile = join(tmpDir, "test.bin")
const content = "hello world"
await writeFile(testFile, content)

// Test 1: sha256 with primitive string (should work)
const hash1 = sha256(content, "hex")
console.log("primitive string hash:", hash1)
assert(typeof hash1 === "string", "sha256 with string returns string")
assert(hash1.length === 64, "sha256 hex is 64 chars")

// Test 2: sha256 with RsString from readFile (currently broken!)
const data = await readFile(testFile, "binary")
console.log("readFile type:", typeof data, data?.constructor?.name)
const hash2 = sha256(data, "hex")
console.log("RsString hash:", hash2)
assert(hash2 === hash1, "hashes match")

await remove(tmpDir)
console.log("ALL SHA256 STRING TESTS PASSED")
