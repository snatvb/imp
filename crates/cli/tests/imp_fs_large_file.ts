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

const largeFilePath = fixture("large_test.bin");
const fileSize = 1024 * 1024; // 1MB
const pattern = "ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";

// --- test: large file with small chunks ---
{
  const chunkSize = 4096;
  const fh = await open(largeFilePath, chunkSize);
  let totalRead = 0;
  let chunk;
  let chunkCount = 0;
  
  while ((chunk = await fh.read()) !== undefined) {
    chunkCount++;
    totalRead += chunk.byteLength;
    // Проверяем паттерн в каждом чанке
    const view = new Uint8Array(chunk);
    for (let i = 0; i < view.length; i++) {
      const expected = pattern.charCodeAt((totalRead - chunk.byteLength + i) % pattern.length);
      if (view[i] !== expected) {
        console.error(`Data corruption at offset ${totalRead - chunk.byteLength + i}: expected ${expected}, got ${view[i]}`);
        throw new Error("Data corruption detected");
      }
    }
  }
  
  console.assert(totalRead === fileSize, `Expected ${fileSize} bytes, got ${totalRead}`);
  console.assert(chunkCount === Math.ceil(fileSize / chunkSize), `Expected ${Math.ceil(fileSize / chunkSize)} chunks, got ${chunkCount}`);
  console.log(`PASS: large file small chunks (${chunkCount} chunks, ${totalRead} bytes)`);
  
  await fh.close();
}

// --- test: large file with large chunks ---
{
  const chunkSize = 65536; // 64KB
  const fh = await open(largeFilePath, chunkSize);
  let totalRead = 0;
  let chunk;
  let chunkCount = 0;
  
  while ((chunk = await fh.read()) !== undefined) {
    chunkCount++;
    totalRead += chunk.byteLength;
  }
  
  console.assert(totalRead === fileSize, `Expected ${fileSize} bytes, got ${totalRead}`);
  console.assert(chunkCount === Math.ceil(fileSize / chunkSize), `Expected ${Math.ceil(fileSize / chunkSize)} chunks, got ${chunkCount}`);
  console.log(`PASS: large file large chunks (${chunkCount} chunks, ${totalRead} bytes)`);
  
  await fh.close();
}

// --- test: very small chunks ---
{
  const chunkSize = 128; // 128 bytes
  const fh = await open(largeFilePath, chunkSize);
  let totalRead = 0;
  let chunk;
  let chunkCount = 0;
  
  while ((chunk = await fh.read()) !== undefined) {
    chunkCount++;
    totalRead += chunk.byteLength;
  }
  
  console.assert(totalRead === fileSize, `Expected ${fileSize} bytes, got ${totalRead}`);
  console.assert(chunkCount === Math.ceil(fileSize / chunkSize), `Expected ${Math.ceil(fileSize / chunkSize)} chunks, got ${chunkCount}`);
  console.log(`PASS: large file tiny chunks (${chunkCount} chunks, ${totalRead} bytes)`);
  
  await fh.close();
}

// --- test: seek in large file ---
{
  const fh = await open(largeFilePath, 4096);
  
  // Идём к середине файла
  const midPoint = Math.floor(fileSize / 2);
  const pos = await fh.seek(midPoint, "start");
  console.assert(pos === midPoint, `Expected position ${midPoint}, got ${pos}`);
  
  // Читаем чанк
  const chunk = await fh.read();
  console.assert(chunk !== undefined, "Should read chunk after seek");
  console.assert(chunk.byteLength === 4096, `Expected 4096 bytes, got ${chunk.byteLength}`);
  
  // Проверяем паттерн
  const view = new Uint8Array(chunk);
  for (let i = 0; i < view.length; i++) {
    const expected = pattern.charCodeAt((midPoint + i) % pattern.length);
    if (view[i] !== expected) {
      console.error(`Data corruption at offset ${midPoint + i}: expected ${expected}, got ${view[i]}`);
      throw new Error("Data corruption detected after seek");
    }
  }
  
  console.log(`PASS: seek in large file (offset ${midPoint})`);
  
  await fh.close();
}

// --- test: multiple reads, verify zero-copy doesn't corrupt ---
{
  const chunkSize = 8192;
  const fh = await open(largeFilePath, chunkSize);
  
  // Читаем первые 3 чанка
  const chunks: ArrayBuffer[] = [];
  for (let i = 0; i < 3; i++) {
    const chunk = await fh.read();
    if (chunk === undefined) break;
    chunks.push(chunk);
  }
  
  // Проверяем, что все чанки независимы и содержат правильные данные
  for (let i = 0; i < chunks.length; i++) {
    const view = new Uint8Array(chunks[i]);
    const offset = i * chunkSize;
    for (let j = 0; j < view.length; j++) {
      const expected = pattern.charCodeAt((offset + j) % pattern.length);
      if (view[j] !== expected) {
        console.error(`Zero-copy corruption: chunk ${i}, offset ${offset + j}: expected ${expected}, got ${view[j]}`);
        throw new Error("Zero-copy data corruption");
      }
    }
  }
  
  console.log(`PASS: zero-copy integrity (${chunks.length} chunks retained)`);
  
  await fh.close();
}

console.log("ALL LARGE FILE TESTS PASSED");
