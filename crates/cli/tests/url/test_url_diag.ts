console.log("console works:", typeof console.log)
console.log("URL type:", typeof URL)
console.log("URLSearchParams type:", typeof URLSearchParams)
if (typeof URL !== "undefined") {
  const u = new URL("https://example.com/path?q=1#hash")
  console.log("href:", u.href)
  console.log("protocol:", u.protocol)
  console.log("hostname:", u.hostname)
  console.log("pathname:", u.pathname)
  console.log("search:", u.search)
  console.log("hash:", u.hash)
  console.log("URL TEST OK")
} else {
  console.log("URL NOT DEFINED")
}
