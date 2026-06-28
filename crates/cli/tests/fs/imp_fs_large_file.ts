import { resolve } from "path"

import { open } from "imp:fs"

const fixture = (name: string) => resolve(import.meta.dirname, "fixtures", name)

const largeFilePath = fixture("large_test.bin")
const fileSize = 1024 * 1024 // 1MB
const pattern = "ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789"

// --- test: large file with small chunks ---
{
  const chunkSize = 4096
  const fh = await open(largeFilePath, chunkSize)
  let totalRead = 0
  let chunk
  let chunkCount = 0

  while ((chunk = await fh.read()) !== undefined) {
    chunkCount++
    totalRead += chunk.length
    const str = chunk.toString()
    for (let i = 0; i < str.length; i++) {
      const expected = pattern[(totalRead - chunk.length + i) % pattern.length]
      if (str[i] !== expected) {
        console.error(`Data corruption at offset ${totalRead - chunk.length + i}: expected ${expected}, got ${str[i]}`)
        throw new Error("Data corruption detected")
      }
    }
  }

  assert(totalRead === fileSize, `Expected ${fileSize} bytes, got ${totalRead}`)
  assert(
    chunkCount === Math.ceil(fileSize / chunkSize),
    `Expected ${Math.ceil(fileSize / chunkSize)} chunks, got ${chunkCount}`,
  )
  console.log(`PASS: large file small chunks (${chunkCount} chunks, ${totalRead} bytes)`)

  await fh.close()
}

// --- test: large file with large chunks ---
{
  const chunkSize = 65536 // 64KB
  const fh = await open(largeFilePath, chunkSize)
  let totalRead = 0
  let chunk
  let chunkCount = 0

  while ((chunk = await fh.read()) !== undefined) {
    chunkCount++
    totalRead += chunk.length
  }

  assert(totalRead === fileSize, `Expected ${fileSize} bytes, got ${totalRead}`)
  assert(
    chunkCount === Math.ceil(fileSize / chunkSize),
    `Expected ${Math.ceil(fileSize / chunkSize)} chunks, got ${chunkCount}`,
  )
  console.log(`PASS: large file large chunks (${chunkCount} chunks, ${totalRead} bytes)`)

  await fh.close()
}

// --- test: very small chunks ---
{
  const chunkSize = 128 // 128 bytes
  const fh = await open(largeFilePath, chunkSize)
  let totalRead = 0
  let chunk
  let chunkCount = 0

  while ((chunk = await fh.read()) !== undefined) {
    chunkCount++
    totalRead += chunk.length
  }

  assert(totalRead === fileSize, `Expected ${fileSize} bytes, got ${totalRead}`)
  assert(
    chunkCount === Math.ceil(fileSize / chunkSize),
    `Expected ${Math.ceil(fileSize / chunkSize)} chunks, got ${chunkCount}`,
  )
  console.log(`PASS: large file tiny chunks (${chunkCount} chunks, ${totalRead} bytes)`)

  await fh.close()
}

// --- test: seek in large file ---
{
  const fh = await open(largeFilePath, 4096)

  const midPoint = Math.floor(fileSize / 2)
  const pos = await fh.seek(midPoint, "start")
  assert(pos === midPoint, `Expected position ${midPoint}, got ${pos}`)

  const chunk = await fh.read()
  assert(chunk !== undefined, "Should read chunk after seek")
  assert(chunk.length === 4096, `Expected 4096 bytes, got ${chunk.length}`)

  const str = chunk.toString()
  for (let i = 0; i < str.length; i++) {
    const expected = pattern[(midPoint + i) % pattern.length]
    if (str[i] !== expected) {
      console.error(`Data corruption at offset ${midPoint + i}: expected ${expected}, got ${str[i]}`)
      throw new Error("Data corruption detected after seek")
    }
  }

  console.log(`PASS: seek in large file (offset ${midPoint})`)

  await fh.close()
}

// --- test: multiple reads, verify zero-copy doesn't corrupt ---
{
  const chunkSize = 8192
  const fh = await open(largeFilePath, chunkSize)

  const chunks = []
  for (let i = 0; i < 3; i++) {
    const chunk = await fh.read()
    if (chunk === undefined) break
    chunks.push(chunk)
  }

  for (let i = 0; i < chunks.length; i++) {
    const chunk = chunks[i]
    assert(chunk !== undefined, "chunk exists")
    const str = chunk.toString()
    const offset = i * chunkSize
    for (let j = 0; j < str.length; j++) {
      const expected = pattern[(offset + j) % pattern.length]
      if (str[j] !== expected) {
        console.error(`Zero-copy corruption: chunk ${i}, offset ${offset + j}: expected ${expected}, got ${str[j]}`)
        throw new Error("Zero-copy data corruption")
      }
    }
  }

  console.log(`PASS: zero-copy integrity (${chunks.length} chunks retained)`)

  await fh.close()
}

console.log("ALL LARGE FILE TESTS PASSED")
