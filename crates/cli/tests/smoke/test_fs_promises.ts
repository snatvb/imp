import fs, { readFile } from "fs/promises";

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

console.log("ALL FS/PROMISES TESTS PASSED");
