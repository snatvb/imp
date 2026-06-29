import { expandHome } from "imp:fs"

const home = process.homedir
const username = process.env.USER
assert(home.endsWith("/" + username), "homedir ends with username")

// --- ~/foo/bar expands to {home}/foo/bar ---
assert(expandHome("~/foo/bar") === home + "/foo/bar", "~/foo/bar expands")

// --- bare ~ expands to home ---
assert(expandHome("~") === home, "bare ~ expands")

// --- absolute path unchanged ---
assert(expandHome("/usr/bin") === "/usr/bin", "absolute path unchanged")

// --- relative path unchanged ---
assert(expandHome("relative/path") === "relative/path", "relative path unchanged")

// --- ~foo/bar unchanged (not tilde expansion) ---
assert(expandHome("~foo/bar") === "~foo/bar", "~foo/bar unchanged")

// --- ~/ expands to home ---
assert(expandHome("~/") === home, "~/ expands to home")

// --- ~/foo//bar collapses slashes ---
assert(expandHome("~/foo//bar") === home + "/foo//bar", "~/foo//bar keeps double slash")

console.log("ALL EXPAND_HOME TESTS PASSED")
