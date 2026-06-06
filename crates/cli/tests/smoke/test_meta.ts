console.assert(typeof import.meta.dirname === "string", "dirname is string");
console.assert(import.meta.dirname.length > 0, "dirname not empty");
console.assert(typeof import.meta.filename === "string", "filename is string");
console.assert(import.meta.filename.endsWith("test_meta.ts"), "filename ends with test_meta.ts");
console.log("PASS: import.meta");

const cwd = process.cwd();
console.assert(typeof cwd === "string", "cwd is string");
console.assert(cwd.length > 0, "cwd not empty");
console.log("PASS: process.cwd");

console.log("ALL META TESTS PASSED");
