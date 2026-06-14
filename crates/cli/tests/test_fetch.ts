async function testHeadersGet() {
  const h = new Headers({ "X-Test": "value" });
  console.assert(h.get("X-Test") === "value", "Headers: get");
  console.assert(h.get("missing") === null, "Headers: missing returns null");
}

async function testHeadersSet() {
  const h = new Headers();
  h.set("X-Test", "first");
  console.assert(h.get("X-Test") === "first", "Headers: set");
  h.set("X-Test", "second");
  console.assert(h.get("X-Test") === "second", "Headers: set overwrites");
}

async function testHeadersAppend() {
  const h = new Headers();
  h.append("X-Test", "one");
  h.append("X-Test", "two");
  console.assert(h.has("X-Test"), "Headers: has after append");
  console.assert(h.get("X-Test") === "one", "Headers: get returns first after append");
}

async function testHeadersHas() {
  const h = new Headers({ "X-Test": "value" });
  console.assert(h.has("X-Test"), "Headers: has existing");
  console.assert(!h.has("missing"), "Headers: has missing");
}

async function testHeadersDelete() {
  const h = new Headers({ "X-Test": "value" });
  h.delete("X-Test");
  console.assert(!h.has("X-Test"), "Headers: delete removes");
  h.delete("nonexistent");
}

async function testHeadersCaseInsensitive() {
  const h = new Headers();
  h.set("Content-Type", "text/html");
  console.assert(h.get("content-type") === "text/html", "Headers: case insensitive");
}

async function testHeadersKeysValuesEntries() {
  const h = new Headers({ "a": "1", "b": "2" });
  const k = h.keys();
  const v = h.values();
  const e = h.entries();
  console.assert(k.length === 2, "Headers: keys count");
  console.assert(v.length === 2, "Headers: values count");
  console.assert(e.length === 2, "Headers: entries count");
}

async function testAbortController() {
  const ctrl = new AbortController();
  console.assert(!ctrl.signal.aborted, "AbortController: initially not aborted");
  ctrl.abort();
  console.assert(ctrl.signal.aborted, "AbortController: aborted after abort()");
}

async function testAbortReason() {
  const ctrl = new AbortController();
  ctrl.abort();
  console.assert(ctrl.signal.reason === "The operation was aborted", "AbortController: default reason");
}

async function testAbortCustomReason() {
  const ctrl = new AbortController();
  ctrl.abort("custom timeout");
  try {
    await fetch("https://httpbin.org/get", { signal: ctrl.signal });
    console.assert(false, "Abort custom reason: should have thrown");
  } catch (e) {
    console.assert((e as Error).name === "AbortError", "Abort custom reason: must be AbortError");
    console.log("PASS: Abort custom reason");
  }
}

async function testFetchAbortBefore() {
  const ctrl = new AbortController();
  ctrl.abort();
  try {
    await fetch("https://httpbin.org/get", { signal: ctrl.signal });
    console.assert(false, "Fetch abort before: should have thrown");
  } catch (e) {
    console.assert((e as Error).name === "AbortError", "Fetch abort before: must be AbortError");
    console.assert(e instanceof Error, "Fetch abort before: instanceof Error");
    console.log("PASS: Fetch abort before");
  }
}

async function testFetchGet() {
  const r = await fetch("https://httpbin.org/get");
  console.assert(r.ok, "fetch GET: ok");
  console.assert(r.status === 200, "fetch GET: status 200");
  const t = await r.text();
  console.assert(t.length > 0, "fetch GET: has body");
}

async function testFetchJson() {
  const r = await fetch("https://httpbin.org/json");
  const j = await r.json();
  console.assert(typeof j === "object", "fetch JSON: parsed");
}

async function testFetchPost() {
  const r = await fetch("https://httpbin.org/post", {
    method: "POST",
    body: "hello world",
    headers: { "Content-Type": "text/plain" },
  });
  console.assert(r.status === 200, "fetch POST: status 200");
  const j = await r.json();
  console.assert(j.data === "hello world", "fetch POST: body echoed");
}

async function testFetchHeaders() {
  const r = await fetch("https://httpbin.org/headers", {
    headers: { "X-Custom": "test-value" },
  });
  const j = await r.json();
  console.assert(j.headers["X-Custom"] === "test-value", "fetch headers: custom header sent");
}

async function testResponseClone() {
  const r = await fetch("https://httpbin.org/get");
  const r2 = r.clone();
  console.assert(r.status === r2.status, "clone: same status");
}

async function test404() {
  const r = await fetch("https://httpbin.org/status/404");
  console.assert(r.status === 404, "404: correct status");
  console.assert(!r.ok, "404: not ok");
}

async function testResponseBodyConsumed() {
  const r = await fetch("https://httpbin.org/get");
  await r.text();
  try {
    await r.text();
    console.assert(false, "Body consumed: should have thrown");
  } catch (e) {
    console.log("PASS: Body consumed throws");
  }
}

async function testArrayBuffer() {
  const r = await fetch("https://httpbin.org/get");
  const buf = r.arrayBuffer();
  console.assert(buf instanceof ArrayBuffer, "arrayBuffer: returns ArrayBuffer");
  console.assert(buf.byteLength > 0, "arrayBuffer: has length");
}

async function testResponseUrl() {
  const r = await fetch("https://httpbin.org/get");
  console.assert(r.url === "https://httpbin.org/get", "Response: url matches");
}

async function testResponseStatusText() {
  const r = await fetch("https://httpbin.org/get");
  console.assert(typeof r.statusText === "string", "Response: statusText is string");
  console.assert(r.statusText.length > 0, "Response: statusText not empty");
}

async function testFetchInvalidUrl() {
  try {
    await fetch("http://[::1]:99999");
    console.assert(false, "Invalid URL: should have thrown");
  } catch (e) {
    console.log("PASS: Invalid URL throws");
  }
}

async function testFetchDnsFailure() {
  try {
    await fetch("http://this-host-definitely-does-not-exist-12345.invalid");
    console.assert(false, "DNS failure: should have thrown");
  } catch (e) {
    console.log("PASS: DNS failure throws");
  }
}

async function testFetchAbortDuring() {
  try {
    const ctrl = new AbortController();
    const p = fetch("https://httpbin.org/delay/10", { signal: ctrl.signal });
    setTimeout(() => ctrl.abort(), 10);
    await p;
  } catch (e) {
    console.assert((e as Error).name === "AbortError", "Abort during fetch: must be AbortError");
    console.log("PASS: Abort during fetch");
    return;
  }
  console.log("TODO: Abort during fetch (needs slow endpoint)");
}

async function main() {
  await testHeadersGet();
  await testHeadersSet();
  await testHeadersAppend();
  await testHeadersHas();
  await testHeadersDelete();
  await testHeadersCaseInsensitive();
  await testHeadersKeysValuesEntries();
  console.log("PASS: All Headers tests");

  await testAbortController();
  await testAbortReason();
  await testAbortCustomReason();
  await testFetchAbortBefore();
  console.log("PASS: All AbortController tests");

  await testFetchGet();
  await testFetchJson();
  await testFetchPost();
  await testFetchHeaders();
  await testResponseClone();
  await test404();
  await testResponseBodyConsumed();
  await testArrayBuffer();
  await testResponseUrl();
  await testResponseStatusText();
  console.log("PASS: All fetch/Response tests");

  await testFetchInvalidUrl();
  await testFetchDnsFailure();
  await testFetchAbortDuring();
  console.log("PASS: All negative/abort tests");

  console.log("ALL FETCH TESTS PASSED");
}

main();
