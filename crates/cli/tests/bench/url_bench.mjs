const RUNTIME = typeof process !== "undefined" && process.versions?.node ? "Node.js" : "Imp";

async function bench(label, iterations, fn) {
  const warmup = Math.floor(iterations / 10);
  for (let i = 0; i < warmup; i++) await fn();
  const start = performance.now();
  for (let i = 0; i < iterations; i++) await fn();
  const elapsed = performance.now() - start;
  const avg = elapsed / iterations;
  const ops = iterations / (elapsed / 1000);
  return { avg, ops, elapsed };
}

function printResult(index, total, name, iterations, warmup, r) {
  console.log(`[${index}/${total}] ${name} (${iterations} iters, ${warmup} warmup)`);
  console.log(`  avg: ${r.avg.toFixed(3)}ms | ops/sec: ${r.ops.toFixed(0)} | total: ${r.elapsed.toFixed(1)}ms`);
  console.log("");
}

const URLS = {
  simple: "https://example.com/path?q=1#hash",
  complex: "https://user:pass@www.example.com:8080/a/b/c?x=1&y=2&z=3&foo=bar#section",
  long: "https://example.com/" + "a".repeat(200) + "?" + Array.from({ length: 50 }, (_, i) => `key${i}=val${i}`).join("&"),
};

const SETTERS = {
  href: "https://new.example.com/changed?param=1",
  pathname: "/new/path",
};

async function main() {
  console.log(`=== URL Benchmark (${RUNTIME}) ===`);
  console.log("");

  const total = 12;

  {
    const iters = 100000;
    const r = await bench("URL parse simple", iters, () => { new URL(URLS.simple); });
    printResult(1, total, "URL parse simple", iters, Math.floor(iters / 10), r);
  }

  {
    const iters = 100000;
    const r = await bench("URL parse complex", iters, () => { new URL(URLS.complex); });
    printResult(2, total, "URL parse complex", iters, Math.floor(iters / 10), r);
  }

  {
    const iters = 50000;
    const r = await bench("URL parse long query", iters, () => { new URL(URLS.long); });
    printResult(3, total, "URL parse long query", iters, Math.floor(iters / 10), r);
  }

  {
    const url = new URL(URLS.complex);
    const iters = 200000;
    const r = await bench("property getters x10", iters, () => {
      url.href; url.origin; url.protocol; url.username; url.password;
      url.host; url.hostname; url.port; url.pathname; url.search; url.hash;
    });
    printResult(4, total, "property getters x10", iters, Math.floor(iters / 10), r);
  }

  {
    const iters = 100000;
    const r = await bench("set href (reparse)", iters, () => {
      const u = new URL(URLS.simple);
      u.href = SETTERS.href;
    });
    printResult(5, total, "set href (reparse)", iters, Math.floor(iters / 10), r);
  }

  {
    const iters = 200000;
    const r = await bench("set pathname", iters, () => {
      const u = new URL(URLS.simple);
      u.pathname = SETTERS.pathname;
    });
    printResult(6, total, "set pathname", iters, Math.floor(iters / 10), r);
  }

  {
    const url = new URL(URLS.complex);
    const iters = 200000;
    const r = await bench("URL.toString()", iters, () => { url.toString(); });
    printResult(7, total, "URL.toString()", iters, Math.floor(iters / 10), r);
  }

  {
    const iters = 100000;
    const r = await bench("searchParams.append", iters, () => {
      const u = new URL(URLS.simple);
      u.searchParams.append("k", "v");
    });
    printResult(8, total, "searchParams.append", iters, Math.floor(iters / 10), r);
  }

  {
    const url = new URL(URLS.complex);
    const iters = 200000;
    const r = await bench("searchParams.get", iters, () => { url.searchParams.get("x"); });
    printResult(9, total, "searchParams.get", iters, Math.floor(iters / 10), r);
  }

  {
    const iters = 100000;
    const r = await bench("searchParams.set existing", iters, () => {
      const u = new URL(URLS.complex);
      u.searchParams.set("x", "new");
    });
    printResult(10, total, "searchParams.set existing", iters, Math.floor(iters / 10), r);
  }

  {
    const url = new URL(URLS.complex);
    const iters = 200000;
    const r = await bench("searchParams.has", iters, () => { url.searchParams.has("x"); });
    printResult(11, total, "searchParams.has", iters, Math.floor(iters / 10), r);
  }

  {
    const url = new URL(URLS.long);
    const iters = 100000;
    const r = await bench("searchParams.toString (50 params)", iters, () => { url.searchParams.toString(); });
    printResult(12, total, "searchParams.toString (50 params)", iters, Math.floor(iters / 10), r);
  }

  console.log("=== Done ===");
}

main();
