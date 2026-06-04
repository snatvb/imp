import { open } from "imp:fs";
import { resolve } from "path";

const fixture = (name: string) => resolve(import.meta.dirname, "fixtures", name);

function bytesToStr(buf: ArrayBuffer): string {
  const view = new Uint8Array(buf);
  let s = "";
  for (let i = 0; i < view.length; i++) {
    s += String.fromCharCode(view[i]);
  }
  return s;
}

// --- test: basic read ---
{
  const fh = await open(fixture("hello.txt"), 64);
  const chunk = await fh.read();
  console.assert(chunk !== undefined, "read returns chunk");
  console.assert(chunk instanceof ArrayBuffer, "read returns ArrayBuffer");
  console.assert(chunk.byteLength === 11, `chunk size is 11, got ${chunk.byteLength}`);
  const text = bytesToStr(chunk);
  console.assert(text === "hello world", `content: "${text}"`);

  const eof = await fh.read();
  console.assert(eof === undefined, "read returns undefined at EOF");

  await fh.close();
  console.log("PASS: basic read");
}

// --- test: multiple reads (buffer reuse) ---
{
  const fh = await open(fixture("hello.txt"), 5);
  const c1 = await fh.read();
  console.assert(c1 !== undefined, "first read");
  console.assert(c1.byteLength === 5, `first chunk size 5, got ${c1.byteLength}`);

  const c2 = await fh.read();
  console.assert(c2 !== undefined, "second read");
  console.assert(c2.byteLength === 5, `second chunk size 5, got ${c2.byteLength}`);

  const c3 = await fh.read();
  console.assert(c3 !== undefined, "third read");
  console.assert(c3.byteLength === 1, `third chunk size 1, got ${c3.byteLength}`);

  const eof = await fh.read();
  console.assert(eof === undefined, "EOF after all chunks");

  const full = bytesToStr(c1) + bytesToStr(c2) + bytesToStr(c3);
  console.assert(full === "hello world", `reassembled: "${full}"`);

  await fh.close();
  console.log("PASS: multiple reads");
}

// --- test: zero-copy independence ---
{
  const fh = await open(fixture("hello.txt"), 5);
  const c1 = await fh.read();
  const c2 = await fh.read();

  const t1 = bytesToStr(c1);
  const t2 = bytesToStr(c2);
  console.assert(t1 === "hello", `first: "${t1}"`);
  console.assert(t2 === " worl", `second: "${t2}"`);

  await fh.close();
  console.log("PASS: zero-copy independence");
}

// --- test: seek ---
{
  const fh = await open(fixture("hello.txt"), 64);
  const pos = await fh.seek(6, "start");
  console.assert(pos === 6, `seek start returns 6, got ${pos}`);

  const chunk = await fh.read();
  const text = bytesToStr(chunk);
  console.assert(text === "world", `after seek: "${text}"`);

  await fh.close();
  console.log("PASS: seek");
}

// --- test: seek current ---
{
  const fh = await open(fixture("hello.txt"), 64);
  await fh.seek(3, "start");
  const pos = await fh.seek(2, "current");
  console.assert(pos === 5, `seek current returns 5, got ${pos}`);

  const chunk = await fh.read();
  const text = bytesToStr(chunk);
  console.assert(text === " world", `after seek current: "${text}"`);

  await fh.close();
  console.log("PASS: seek current");
}

// --- test: close is idempotent ---
{
  const fh = await open(fixture("hello.txt"), 64);
  await fh.close();
  await fh.close();
  await fh.close();
  console.log("PASS: close idempotent");
}

// --- test: read after close errors ---
{
  const fh = await open(fixture("hello.txt"), 64);
  await fh.close();
  let threw = false;
  try {
    await fh.read();
  } catch (e) {
    threw = true;
  }
  console.assert(threw === true, "read after close throws");
  console.log("PASS: read after close");
}

// --- test: seek after close errors ---
{
  const fh = await open(fixture("hello.txt"), 64);
  await fh.close();
  let threw = false;
  try {
    await fh.seek(0, "start");
  } catch (e) {
    threw = true;
  }
  console.assert(threw === true, "seek after close throws");
  console.log("PASS: seek after close");
}

// --- test: open non-existent file ---
{
  let threw = false;
  try {
    await open(fixture("DOES_NOT_EXIST.txt"), 64);
  } catch (e) {
    threw = true;
  }
  console.assert(threw === true, "open non-existent throws");
  console.log("PASS: open non-existent");
}

// --- test: seek end ---
{
  const fh = await open(fixture("hello.txt"), 64);
  const pos = await fh.seek(-5, "end");
  console.assert(pos === 6, `seek end returns 6, got ${pos}`);

  const chunk = await fh.read();
  const text = bytesToStr(chunk);
  console.assert(text === "world", `after seek end: "${text}"`);

  await fh.close();
  console.log("PASS: seek end");
}

// --- test: larger file with bigger chunk ---
{
  const fh = await open(fixture("readfile.bin"), 32);
  let all = "";
  let chunk;
  while ((chunk = await fh.read()) !== undefined) {
    all += bytesToStr(chunk);
  }
  console.assert(all === "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz\n", `larger file: "${all}"`);
  await fh.close();
  console.log("PASS: larger file");
}

// --- test: invalid whence ---
{
  const fh = await open(fixture("hello.txt"), 64);
  let threw = false;
  try {
    await fh.seek(0, "invalid");
  } catch (e) {
    threw = true;
  }
  console.assert(threw === true, "invalid whence throws");
  await fh.close();
  console.log("PASS: invalid whence");
}

console.log("ALL IMP:FS FILEHANDLE TESTS PASSED");
