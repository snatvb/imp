async function testHeaders() {
  const h = new Headers({ "X-Test": "value" });
  console.assert(h.get("X-Test") === "value", "Headers: get");
  h.set("X-Test", "overwritten");
  console.assert(h.get("X-Test") === "overwritten", "Headers: set overwrites");
  h.append("X-Test", "appended");
  console.assert(h.get("X-Test") === "overwritten, appended", "Headers: append adds");
  console.assert(h.has("X-Test"), "Headers: has");
  h.delete("X-Test");
  console.assert(!h.has("X-Test"), "Headers: delete removes");
  console.assert(h.get("missing") === null, "Headers: missing returns null");
}

async function testAbortController() {
  const ctrl = new AbortController();
  console.assert(!ctrl.signal.aborted, "AbortController: initially not aborted");
  ctrl.abort();
  console.assert(ctrl.signal.aborted, "AbortController: aborted after abort()");
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
  console.assert(r.ok, "fetch JSON: ok");
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
  const t1 = await r.text();
  const t2 = await r2.text();
  console.assert(t1 === t2, "clone: same body");
}

async function test404() {
  const r = await fetch("https://httpbin.org/status/404");
  console.assert(r.status === 404, "404: correct status");
  console.assert(!r.ok, "404: not ok");
}

async function main() {
  await Promise.all([
    testHeaders(),
    testAbortController(),
    testFetchGet(),
    testFetchJson(),
    testFetchPost(),
    testFetchHeaders(),
    testResponseClone(),
    test404(),
  ]);
  console.log("ALL FETCH TESTS PASSED");
}

main();
