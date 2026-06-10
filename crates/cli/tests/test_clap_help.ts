import clap from "imp:clap";

const parser = new clap.Parser();
parser.name("test");
parser.version("1.0.0");
parser.about("Test parser");
parser.arg({ name: "name", short: "n", long: "name", help: "Your name", action: "set" });
parser.arg({ name: "verbose", short: "v", long: "verbose", action: "count" });
parser.arg({ name: "output", short: "o", long: "output", action: "set" });
parser.arg({ name: "debug", short: "d", long: "debug", action: "flag" });
parser.arg({ name: "files", action: "append" });

// Тест обычного парсинга
console.log("Test 1: normal parsing");
const result1 = parser.parse(["-n", "Alice", "-vvv"]);
console.assert(result1.type === "result", "type should be result");
console.assert(result1.name === "Alice", "name should be Alice");
console.assert(result1.verbose === 3, "verbose should be 3");
console.log("Test 1 passed");

// Тест help (автогенерация clap)
console.log("Test 2: help");
const result2 = parser.parse(["--help"]);
console.assert(result2.type === "help", "type should be help");
console.assert(result2.message !== undefined, "message should exist");
const helpText = String(result2.message);
console.assert(helpText.includes("Your name"), "message should contain help text");
console.log("Test 2 passed");

// Тест version (автогенерация clap)
console.log("Test 3: version");
const result3 = parser.parse(["--version"]);
console.assert(result3.type === "version", "type should be version");
console.assert(result3.message !== undefined, "message should exist");
const versionText = String(result3.message);
console.assert(versionText.includes("1.0.0"), "message should contain version");
console.log("Test 3 passed");

console.log("ALL CLAP TESTS PASSED");
