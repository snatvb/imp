function testUrlWithRsString() {
  const rs = RsString.fromString("https://example.com/path?q=1#hash");
  const u = new URL(rs);
  console.assert(u.hostname === "example.com", "URL(RsString): hostname");
  console.assert(u.pathname === "/path", "URL(RsString): pathname");
  console.assert(u.search === "?q=1", "URL(RsString): search");
  console.assert(u.hash === "#hash", "URL(RsString): hash");
}

function testUrlWithRsStringBase() {
  const rsInput = RsString.fromString("/relative");
  const rsBase = RsString.fromString("https://example.com");
  const u = new URL(rsInput, rsBase);
  console.assert(u.href === "https://example.com/relative", "URL(RsString, RsString): href");
}

function testUrlCanParseRsString() {
  const rs = RsString.fromString("https://example.com");
  console.assert(URL.canParse(rs), "URL.canParse(RsString): valid");
  console.assert(!URL.canParse(RsString.fromString("not a url")), "URL.canParse(RsString): invalid");
}

function testUrlParseRsString() {
  const rs = RsString.fromString("https://example.com/path");
  const u = URL.parse(rs);
  console.assert(u !== null, "URL.parse(RsString): not null");
  console.assert(u!.hostname === "example.com", "URL.parse(RsString): hostname");
}

function testFetchWithUrlObject() {
  const u = new URL("https://httpbin.org/get");
  const req = new Request(u);
  console.assert(req.url === "https://httpbin.org/get", "Request(URL): url");
}

function testFetchWithUrlObjectAndInit() {
  const u = new URL("https://httpbin.org/post");
  const req = new Request(u, { method: "POST", body: "data" });
  console.assert(req.method === "POST", "Request(URL, init): method");
  console.assert(req.url === "https://httpbin.org/post", "Request(URL, init): url");
}

async function testFetchCallWithUrl() {
  const u = new URL("https://httpbin.org/get");
  const r = await fetch(u);
  console.assert(r.ok, "fetch(URL): ok");
  console.assert(r.status === 200, "fetch(URL): status 200");
}

async function testFetchFileWithUrl() {
  const { writeFile, remove } = await import("imp:fs");
  const path = `${process.cwd()}\\_test_fetch_url_obj.txt`;
  await writeFile(path, "url obj content");
  const u = new URL(`file:///${path}`);
  const r = await fetch(u);
  const t = await r.text();
  console.assert(t === "url obj content", "fetch(URL, file://): content");
  await remove(path);
}

async function testMain() {
  testUrlWithRsString();
  console.log("PASS: URL with RsString");

  testUrlWithRsStringBase();
  console.log("PASS: URL with RsString base");

  testUrlCanParseRsString();
  console.log("PASS: URL.canParse with RsString");

  testUrlParseRsString();
  console.log("PASS: URL.parse with RsString");

  testFetchWithUrlObject();
  console.log("PASS: Request with URL");

  testFetchWithUrlObjectAndInit();
  console.log("PASS: Request with URL and init");

  await testFetchCallWithUrl();
  console.log("PASS: fetch(URL)");

  await testFetchFileWithUrl();
  console.log("PASS: fetch(URL, file://)");

  console.log("ALL URL+RSSTRING TESTS PASSED");
}

testMain();
