assert(typeof process.exit === "function", "exit is function")
assert(typeof process.on === "function", "on is function")

process.exit(42)
