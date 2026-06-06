import impfs from "imp:fs";

const dir = import.meta.dirname + "/test_dir";
await impfs.mkdir(dir);
console.assert(await impfs.exists(dir) === true, "mkdir creates directory");

const metadata = await impfs.metadata(dir);
console.assert(metadata.isDirectory === true, "metadata shows isDirectory");
console.assert(typeof metadata.isFile === "boolean", "metadata has isFile");
console.assert(typeof metadata.size === "number", "metadata has size");
console.log("PASS: mkdir/metadata");

await impfs.remove(dir);
console.assert(await impfs.exists(dir) === false, "remove deletes directory");
console.log("PASS: remove/exists");

const fileMeta = await impfs.metadata(import.meta.dirname + "/text.txt");
console.assert(fileMeta.isFile === true, "file metadata shows isFile");
console.assert(fileMeta.isDirectory === false, "file metadata shows not directory");
console.assert(fileMeta.size > 0, "file has size");
console.log("PASS: metadata on file");

console.assert(await impfs.exists(import.meta.dirname + "/DOES_NOT_EXIST") === false, "exists returns false for missing");
console.log("PASS: exists missing");

console.log("ALL IMP:FS TESTS PASSED");
