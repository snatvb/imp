import { writeFile, remove, mkdir } from "imp:fs"

const TMP_DIR = import.meta.dirname + "\\.tmp"
await mkdir(TMP_DIR, { recursive: true })

function createMockDest() {
  const chunks: any[] = []
  const dest = {
    locked: false,
    getWriter() {
      dest.locked = true
      return {
        write(chunk: any) {
          chunks.push(chunk)
          return Promise.resolve()
        },
        close() {
          dest.locked = false
          return Promise.resolve()
        },
        releaseLock() {
          dest.locked = false
        },
      }
    },
  }
  return { dest, chunks }
}

async function testPipeToBasic() {
  const filePath = `${TMP_DIR}\\_test_pipe.txt`
  await writeFile(filePath, "pipe me")
  const r = await fetch(`file:///${filePath}`)
  const body = r.body
  const { dest, chunks } = createMockDest()
  await body.pipeTo(dest)
  assert(chunks.length === 1, "pipeTo: got 1 chunk")
  assert(chunks[0] instanceof ArrayBuffer, "pipeTo: chunk is ArrayBuffer")
  const bytes = new Uint8Array(chunks[0])
  let text = ""
  for (let i = 0; i < bytes.length; i++) {
    text += String.fromCharCode(bytes[i])
  }
  assert(text === "pipe me", "pipeTo: content matches")
  await remove(filePath)
  console.log("PASS: pipeTo basic")
}

async function testTeeBasic() {
  const filePath = `${TMP_DIR}\\_test_tee.txt`
  await writeFile(filePath, "tee me")
  const r = await fetch(`file:///${filePath}`)
  const body = r.body
  const branches = await body.tee()
  assert(branches.length === 2, "tee: returns 2 branches")

  const reader1 = branches[0].getReader()
  const result1 = await reader1.read()
  assert(result1.done === false, "tee branch1: not done")
  assert(result1.value instanceof ArrayBuffer, "tee branch1: is ArrayBuffer")
  const bytes1 = new Uint8Array(result1.value)
  let text1 = ""
  for (let i = 0; i < bytes1.length; i++) {
    text1 += String.fromCharCode(bytes1[i])
  }
  assert(text1 === "tee me", "tee branch1: content matches")

  const reader2 = branches[1].getReader()
  const result2 = await reader2.read()
  assert(result2.done === false, "tee branch2: not done")
  const bytes2 = new Uint8Array(result2.value)
  let text2 = ""
  for (let i = 0; i < bytes2.length; i++) {
    text2 += String.fromCharCode(bytes2[i])
  }
  assert(text2 === "tee me", "tee branch2: content matches")

  await remove(filePath)
  console.log("PASS: tee basic")
}

async function testReaderDispose() {
  const filePath = `${TMP_DIR}\\_test_dispose.txt`
  await writeFile(filePath, "dispose test")
  const r = await fetch(`file:///${filePath}`)
  const body = r.body

  assert(!body.locked, "dispose: not locked before")
  {
    using reader = body.getReader()
    assert(body.locked, "dispose: locked inside block")
    const result = await reader.read()
    assert(result.done === false, "dispose: can read")
  }
  assert(!body.locked, "dispose: unlocked after")

  await remove(filePath)
  console.log("PASS: reader Symbol.dispose")
}

async function main() {
  await testPipeToBasic()
  await testTeeBasic()
  await testReaderDispose()
  console.log("ALL STREAM TESTS PASSED")
}

main()
