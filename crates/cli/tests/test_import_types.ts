assert(typeof import.meta.filename === "string", "meta available in import tests")

// extension fallback — no with attribute
import jsonByExt from "./data/config.json"
assert(typeof jsonByExt === "object", "json by extension is object")
assert(jsonByExt.name === "imp", "json by extension name")

import textByExt from "./data/readme.txt"
assert(typeof textByExt === "string", "txt by extension is string")

// with { type: "text" } on .md file — any extension works
import mdAsText from "./data/readme.md" with { type: "text" }
assert(typeof mdAsText === "string", "md as text is string")
assert(mdAsText.includes("# Markdown File"), "md content preserved")

// with { type: "text" } on extensionless file
import noExt from "./data/noext" with { type: "text" }
assert(typeof noExt === "string", "extensionless is string")
assert(noExt.includes("no extension file content"), "extensionless content")

// with { type: "json" } explicit
import jsonExplicit from "./data/config.json" with { type: "json" }
assert(typeof jsonExplicit === "object", "explicit json is object")
assert(jsonExplicit.version === "0.1.0", "explicit json version")

console.log("ALL IMPORT TYPES TESTS PASSED")
