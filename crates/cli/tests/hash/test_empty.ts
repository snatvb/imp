import hash, { md5, sha1, sha256, sha512, blake3 } from "imp:hash"

assert(hash !== null && typeof hash === "object", "default import is an object")
assert(typeof hash.md5 === "function", "hash.md5 is a function")
assert(typeof hash.sha1 === "function", "hash.sha1 is a function")
assert(typeof hash.sha256 === "function", "hash.sha256 is a function")
assert(typeof hash.sha512 === "function", "hash.sha512 is a function")
assert(typeof hash.blake3 === "function", "hash.blake3 is a function")

assert(typeof md5 === "function", "named md5 export is a function")
assert(typeof sha1 === "function", "named sha1 export is a function")
assert(typeof sha256 === "function", "named sha256 export is a function")
assert(typeof sha512 === "function", "named sha512 export is a function")
assert(typeof blake3 === "function", "named blake3 export is a function")

console.log("ALL HASH EMPTY TESTS PASSED")
