console.log("test1")
const u1 = new URL("https://user:pass@example.com:8080/path?q=1#hash")
console.log("protocol:", u1.protocol)
console.log("username:", u1.username)
console.log("password:", u1.password)
console.log("hostname:", u1.hostname)
console.log("port:", u1.port)
console.log("pathname:", u1.pathname)
console.log("search:", u1.search)
console.log("hash:", u1.hash)
console.log("origin:", u1.origin)

console.log("test2 href set")
const u2 = new URL("https://example.com")
u2.href = "https://other.com:9090/foo?bar=1#baz"
console.log("hostname:", u2.hostname)
console.log("port:", u2.port)
console.log("pathname:", u2.pathname)

console.log("test3 searchParams sync")
const u3 = new URL("https://example.com?a=1&b=2")
console.log("get a:", u3.searchParams.get("a"))
u3.searchParams.append("c", "3")
console.log("search after append:", u3.search)
u3.searchParams.delete("a")
console.log("search after delete:", u3.search)
u3.searchParams.set("b", "99")
console.log("search after set:", u3.search)

console.log("test4 search setter sync")
const u4 = new URL("https://example.com?x=1")
u4.search = "?y=2&z=3"
console.log("get y:", u4.searchParams.get("y"))
console.log("get z:", u4.searchParams.get("z"))
console.log("size:", u4.searchParams.size)

console.log("test5 relative URL")
const u5 = new URL("/foo/bar", "https://example.com")
console.log("href:", u5.href)

console.log("test6 canParse")
console.log("canParse valid:", URL.canParse("https://example.com"))
console.log("canParse invalid:", URL.canParse("not a url"))

console.log("test7 standalone URLSearchParams")
const sp1 = new URLSearchParams("a=1&b=2")
console.log("get a:", sp1.get("a"))
console.log("size:", sp1.size)
sp1.sort()
console.log("toString:", sp1.toString())

console.log("test8 combined setters")
const u6 = new URL("https://example.com")
u6.pathname = "/new/path"
u6.hash = "#section"
u6.search = "?key=val"
console.log("href:", u6.href)

console.log("ALL URL TESTS PASSED")
