import { walk, glob, globStream } from "imp:fs";
import { resolve } from "path";

const fixture = (name: string) => resolve(import.meta.dirname, "fixtures", name);
const globDir = fixture("glob");


// =============================================================================
// WALK TESTS
// =============================================================================

// --- test: walk returns all entries ---
{
  const results: string[] = [];
  for await (const p of walk(globDir)) {
    results.push(p.toString());
  }
  console.log("walk all:", results);
  console.assert(results.length > 0, "walk returns entries");
  console.log("PASS: walk returns all entries");
}

// --- test: walk filter files only ---
{
  const results: string[] = [];
  for await (const p of walk(globDir, { filter: "files" })) {
    results.push(p.toString());
  }
  console.log("walk files:", results);
  console.assert(results.length > 0, "walk files returns entries");
  console.assert(!results.some(r => r.includes("subdir")), "no directories in files filter");
  console.log("PASS: walk filter files only");
}

// --- test: walk filter directories only ---
{
  const results: string[] = [];
  for await (const p of walk(globDir, { filter: "directories" })) {
    results.push(p.toString());
  }
  console.log("walk dirs:", results);
  console.assert(results.length > 0, "walk dirs returns entries");
  console.log("PASS: walk filter directories only");
}

// --- test: walk dot files excluded by default ---
{
  const results: string[] = [];
  for await (const p of walk(globDir, { filter: "files" })) {
    results.push(p.toString());
  }
  console.log("walk no dot:", results);
  console.assert(!results.some(r => r.includes(".hidden")), "no dot files by default");
  console.log("PASS: walk dot files excluded by default");
}

// --- test: walk dot files included with dot: true ---
{
  const results: string[] = [];
  for await (const p of walk(globDir, { filter: "files", dot: true })) {
    results.push(p.toString());
  }
  console.log("walk with dot:", results);
  console.assert(results.some(r => r.includes(".hidden")), "dot files included with dot: true");
  console.log("PASS: walk dot files included with dot: true");
}

// --- test: walk ignore patterns ---
{
  const results: string[] = [];
  for await (const p of walk(globDir, { ignore: ["**/*.js"] })) {
    results.push(p.toString());
  }
  console.log("walk ignore js:", results);
  console.assert(!results.some(r => r.endsWith(".js")), "no .js files with ignore");
  console.log("PASS: walk ignore patterns");
}

// --- test: walk absolute paths ---
{
  const results: string[] = [];
  for await (const p of walk(globDir, { absolute: true, filter: "files" })) {
    results.push(p.toString());
  }
  console.log("walk absolute:", results);
  console.assert(results.length > 0, "walk absolute returns entries");
  console.log("PASS: walk absolute paths");
}

// =============================================================================
// GLOB TESTS
// =============================================================================

// --- test: glob *.txt ---
{
  const results = await glob(globDir, "**/*.txt");
  const strResults = results.map(r => r.toString());
  console.log("glob *.txt:", strResults);
  console.assert(strResults.length === 2, `expected 2 .txt files, got ${strResults.length}`);
  console.assert(strResults.every(r => r.endsWith(".txt")), "all results are .txt");
  console.log("PASS: glob *.txt");
}

// --- test: glob *.js ---
{
  const results = await glob(globDir, "**/*.js");
  const strResults = results.map(r => r.toString());
  console.log("glob *.js:", strResults);
  console.assert(strResults.length === 2, `expected 2 .js files, got ${strResults.length}`);
  console.assert(strResults.every(r => r.endsWith(".js")), "all results are .js");
  console.log("PASS: glob *.js");
}

// --- test: glob with ignore ---
{
  const results = await glob(globDir, "**/*", { ignore: ["**/*.js"] });
  const strResults = results.map(r => r.toString());
  console.log("glob with ignore:", strResults);
  console.assert(!strResults.some(r => r.endsWith(".js")), "no .js files with ignore");
  console.log("PASS: glob with ignore");
}

// --- test: glob with filter files ---
{
  const results = await glob(globDir, "**/*", { filter: "files" });
  const strResults = results.map(r => r.toString());
  console.log("glob filter files:", strResults);
  console.assert(strResults.length > 0, "glob filter files returns entries");
  console.log("PASS: glob with filter files");
}

// --- test: glob with filter directories ---
{
  const results = await glob(globDir, "**/*", { filter: "directories" });
  const strResults = results.map(r => r.toString());
  console.log("glob filter dirs:", strResults);
  console.assert(strResults.length > 0, "glob filter dirs returns entries");
  console.log("PASS: glob with filter directories");
}

// --- test: glob with dot ---
{
  const results = await glob(globDir, "**/*", { dot: true, filter: "files" });
  const strResults = results.map(r => r.toString());
  console.log("glob with dot:", strResults);
  console.assert(strResults.some(r => r.includes(".hidden")), "dot files included");
  console.log("PASS: glob with dot");
}

// =============================================================================
// GLOBSTREAM TESTS
// =============================================================================

// --- test: globStream basic ---
{
  const results: string[] = [];
  for await (const p of globStream(globDir, "**/*.txt")) {
    results.push(p.toString());
  }
  console.log("globStream *.txt:", results);
  console.assert(results.length === 2, `expected 2 .txt files, got ${results.length}`);
  console.log("PASS: globStream basic");
}

// --- test: globStream with options ---
{
  const results: string[] = [];
  for await (const p of globStream(globDir, "**/*", { filter: "files", ignore: ["**/*.js"] })) {
    results.push(p.toString());
  }
  console.log("globStream with options:", results);
  console.assert(!results.some(r => r.endsWith(".js")), "no .js files");
  console.log("PASS: globStream with options");
}

// --- test: globStream early break ---
{
  const results: string[] = [];
  for await (const p of globStream(globDir, "**/*")) {
    results.push(p.toString());
    if (results.length >= 2) break;
  }
  console.log("globStream early break:", results);
  console.assert(results.length === 2, "early break works");
  console.log("PASS: globStream early break");
}

// =============================================================================
// NEGATIVE SCENARIOS - ERROR HANDLING
// =============================================================================

// --- test: invalid glob pattern ---
{
  let threw = false;
  try {
    await glob(globDir, "[invalid");
  } catch (e) {
    threw = true;
    console.log("invalid pattern error:", String(e));
    console.assert(String(e).includes("pattern") || String(e).includes("glob") || String(e).includes("error"), "error message mentions pattern/glob/error");
  }
  console.assert(threw, "invalid glob pattern throws");
  console.log("PASS: invalid glob pattern throws");
}

// --- test: invalid ignore pattern ---
{
  let threw = false;
  try {
    await glob(globDir, "**/*", { ignore: ["[invalid"] });
  } catch (e) {
    threw = true;
    console.log("invalid ignore error:", String(e));
  }
  console.assert(threw, "invalid ignore pattern throws");
  console.log("PASS: invalid ignore pattern throws");
}

// --- test: invalid filter value ---
{
  let threw = false;
  try {
    await glob(globDir, "**/*", { filter: "invalid" as any });
  } catch (e) {
    threw = true;
    console.log("invalid filter error:", String(e));
    console.assert(String(e).includes("filter"), "error message mentions filter");
  }
  console.assert(threw, "invalid filter value throws");
  console.log("PASS: invalid filter value throws");
}

// --- test: non-existent directory ---
{
  let threw = false;
  try {
    await glob(fixture("DOES_NOT_EXIST_DIR"), "**/*");
  } catch (e) {
    threw = true;
    console.log("non-existent dir error:", String(e));
    console.assert(String(e).includes("ENOENT") || String(e).includes("not found") || String(e).includes("error"), "error message mentions ENOENT/not found/error");
  }
  console.assert(threw, "non-existent directory throws");
  console.log("PASS: non-existent directory throws");
}

// --- test: globStream with invalid pattern ---
{
  let threw = false;
  try {
    const stream = globStream(globDir, "[invalid");
    for await (const _ of stream) {
      // should throw before yielding
    }
  } catch (e) {
    threw = true;
    console.log("globStream invalid pattern error:", String(e));
  }
  console.assert(threw, "globStream with invalid pattern throws");
  console.log("PASS: globStream with invalid pattern throws");
}

// --- test: walk with invalid filter ---
{
  let threw = false;
  try {
    const stream = walk(globDir, { filter: "bad" as any });
    for await (const _ of stream) {
      // should throw
    }
  } catch (e) {
    threw = true;
    console.log("walk invalid filter error:", String(e));
  }
  console.assert(threw, "walk with invalid filter throws");
  console.log("PASS: walk with invalid filter throws");
}

console.log("ALL IMP:FS GLOB/WALK TESTS PASSED");
