assert(typeof process.cwd() === "string", "cwd is string")
assert(process.cwd().length > 0, "cwd not empty")

assert(typeof process.env === "object", "env is object")
assert(typeof process.env.PATH === "string" || typeof process.env.Path === "string", "env has PATH")

assert(Array.isArray(process.argv), "argv is array")
assert(process.argv.length >= 2, "argv has at least exe and file")

assert(typeof process.platform === "string", "platform is string")
assert(["windows", "linux", "macos"].includes(process.platform), "platform is valid")

assert(typeof process.arch === "string", "arch is string")
assert(process.arch.length > 0, "arch not empty")

assert(typeof process.pid === "number", "pid is number")
assert(process.pid > 0, "pid > 0")

assert(typeof process.ppid === "number", "ppid is number")
assert(process.ppid > 0, "ppid > 0")

assert(typeof process.cpuCount === "number", "cpuCount is number")
assert(process.cpuCount >= 1, "cpuCount >= 1")

assert(typeof process.hostname === "string", "hostname is string")
assert(process.hostname.length > 0, "hostname not empty")

assert(typeof process.homedir === "string", "homedir is string")
assert(process.homedir.length > 0, "homedir not empty")

assert(typeof process.version === "string", "version is string")
assert(/^\d+\.\d+\.\d+$/.test(process.version), "version matches semver")

console.log("ALL PROCESS TESTS PASSED")
