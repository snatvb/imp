import impfs from "imp:fs"

const dir = import.meta.dirname + "/test_dir"
await impfs.mkdir(dir)
console.assert((await impfs.exists(dir)) === true, "mkdir creates directory")

const metadata = await impfs.metadata(dir)
console.assert(metadata.isDirectory === true, "metadata shows isDirectory")
console.assert(typeof metadata.isFile === "boolean", "metadata has isFile")
console.assert(typeof metadata.size === "number", "metadata has size")
console.log("PASS: mkdir/metadata")

await impfs.remove(dir)
console.assert((await impfs.exists(dir)) === false, "remove deletes directory")
console.log("PASS: remove/exists")

const fileMeta = await impfs.metadata(import.meta.dirname + "/text.txt")
console.assert(fileMeta.isFile === true, "file metadata shows isFile")
console.assert(fileMeta.isDirectory === false, "file metadata shows not directory")
console.assert(fileMeta.size > 0, "file has size")
console.log("PASS: metadata on file")

console.assert(
  (await impfs.exists(import.meta.dirname + "/DOES_NOT_EXIST")) === false,
  "exists returns false for missing",
)
console.log("PASS: exists missing")

const writePath = import.meta.dirname + "/test_write.txt"
const written = await impfs.writeFile(writePath, "hello world")
console.assert(written === 11, "writeFile returns bytes written")
const readBack = await impfs.readFile(writePath, "utf8")
console.assert(readBack === "hello world", "writeFile content matches")
console.log("PASS: writeFile string")

const written2 = await impfs.writeFile(writePath, " appended", "a")
console.assert(written2 === 9, "writeFile append returns bytes")
const readBack2 = await impfs.readFile(writePath, "utf8")
console.assert(readBack2 === "hello world appended", "writeFile append content matches")
console.log("PASS: writeFile append")

const buf = new ByteBuffer(5)
const arr = buf.toArray()
arr[0] = 72
arr[1] = 101
arr[2] = 108
arr[3] = 108
arr[4] = 111
await impfs.writeFile(writePath, buf)
const readBack3 = await impfs.readFile(writePath, "utf8")
console.assert(readBack3 === "Hello", "writeFile ByteBuffer content matches")
console.log("PASS: writeFile ByteBuffer")

const rsStr = RsString.fromString("RsString content")
await impfs.writeFile(writePath, rsStr)
const readBack4 = await impfs.readFile(writePath, "utf8")
console.assert(readBack4 === "RsString content", "writeFile RsString content matches")
console.log("PASS: writeFile RsString")

await impfs.remove(writePath)

console.log("ALL IMP:FS TESTS PASSED")
