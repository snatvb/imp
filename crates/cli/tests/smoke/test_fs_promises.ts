import fs, { readFile, writeFile } from "fs/promises";
import impfs from "imp:fs";

const content = await fs.readFile(import.meta.dirname + "/text.txt", "utf8");
console.assert(typeof content === "string", "readFile returns string");
console.assert(content.includes("this is regular text"), "readFile content correct");
console.log("PASS: fs/promises readFile");

const namedContent = await readFile(import.meta.dirname + "/text.txt", "utf8");
console.assert(typeof namedContent === "string", "named readFile returns string");
console.assert(namedContent.includes("this is regular text"), "named readFile content correct");
console.log("PASS: named readFile");

let threw = false;
try {
  await fs.readFile(import.meta.dirname + "/DOES_NOT_EXIST.txt", "utf8");
} catch (e) {
  threw = true;
  console.assert(String(e).includes("ENOENT") || String(e).includes("not found"), "error message valid");
}
console.assert(threw, "missing file throws");
console.log("PASS: readFile error handling");

const writePath = import.meta.dirname + "/test_write_promises.txt";
const written = await fs.writeFile(writePath, "test content");
console.assert(written === 12, "writeFile returns bytes written");
const content2 = await readFile(writePath, "utf8");
console.assert(content2 === "test content", "writeFile content correct");
console.log("PASS: writeFile fs/promises");

const written2 = await writeFile(writePath, " more", "a");
console.assert(written2 === 5, "named writeFile append returns bytes");
const content3 = await readFile(writePath, "utf8");
console.assert(content3 === "test content more", "named writeFile append correct");
console.log("PASS: named writeFile");

await impfs.remove(writePath);

console.log("ALL FS/PROMISES TESTS PASSED");
