import readme from "./data/readme.txt" with { type: "text" }
assert(typeof readme === "string", "readme is string")
assert(readme.includes("# ImpJS README"), "readme contains heading")
assert(readme.includes("function greet(name)"), "readme contains JS code")
assert(readme.includes("def greet(name):"), "readme contains Python code")
assert(readme.includes("fn greet(name: &str)"), "readme contains Rust code")
assert(readme.includes('"double"'), "readme contains double quotes")
assert(readme.includes("C:\\Users\\test\\file.txt"), "readme contains backslashes")

import md from "./data/readme.md" with { type: "text" }
assert(typeof md === "string", "md is string")
assert(md.includes("# Markdown File"), "md contains heading")
assert(md.includes("**bold**"), "md contains bold")
assert(md.includes("const x = 42"), "md contains code block")

console.log("ALL TEXT IMPORT TESTS PASSED")
