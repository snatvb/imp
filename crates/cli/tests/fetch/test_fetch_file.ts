import { writeFile, remove, mkdir } from "imp:fs"

const TMP_DIR = import.meta.dirname + "\\.tmp"
await mkdir(TMP_DIR, { recursive: true })

async function testBasicFileRead() {
  const filePath = `${TMP_DIR}\\_test_fetch_basic.txt`
  await writeFile(filePath, "hello file fetch")
  const r = await fetch(`file:///${filePath}`)
  assert(r.ok, "Basic file read: ok")
  assert(r.status === 200, "Basic file read: status 200")
  const t = await r.text()
  assert(t === "hello file fetch", "Basic file read: content matches")
  await remove(filePath)
}

async function testContentType() {
  const jsonPath = `${TMP_DIR}\\_test_fetch_ct.json`
  const htmlPath = `${TMP_DIR}\\_test_fetch_ct.html`
  const cssPath = `${TMP_DIR}\\_test_fetch_ct.css`
  const jsPath = `${TMP_DIR}\\_test_fetch_ct.js`
  const txtPath = `${TMP_DIR}\\_test_fetch_ct.txt`
  const unknownPath = `${TMP_DIR}\\_test_fetch_ct.xyz`

  await writeFile(jsonPath, "{}")
  await writeFile(htmlPath, "<html></html>")
  await writeFile(cssPath, "body{}")
  await writeFile(jsPath, "1")
  await writeFile(txtPath, "text")
  await writeFile(unknownPath, "bin")

  const rJson = await fetch(`file:///${jsonPath}`)
  assert(rJson.headers.get("content-type") === "application/json", "Content-Type: .json")

  const rHtml = await fetch(`file:///${htmlPath}`)
  assert(rHtml.headers.get("content-type") === "text/html", "Content-Type: .html")

  const rCss = await fetch(`file:///${cssPath}`)
  assert(rCss.headers.get("content-type") === "text/css", "Content-Type: .css")

  const rJs = await fetch(`file:///${jsPath}`)
  assert(rJs.headers.get("content-type") === "application/javascript", "Content-Type: .js")

  const rTxt = await fetch(`file:///${txtPath}`)
  assert(rTxt.headers.get("content-type") === "text/plain", "Content-Type: .txt")

  const rUnknown = await fetch(`file:///${unknownPath}`)
  assert(rUnknown.headers.get("content-type") === "application/octet-stream", "Content-Type: unknown ext")

  await remove(jsonPath)
  await remove(htmlPath)
  await remove(cssPath)
  await remove(jsPath)
  await remove(txtPath)
  await remove(unknownPath)
}

async function testJsonFile() {
  const filePath = `${TMP_DIR}\\_test_fetch_json.json`
  await writeFile(filePath, '{"key":"value","num":42}')
  const r = await fetch(`file:///${filePath}`)
  const j = await r.json()
  assert(j.key === "value", "JSON file: key=value")
  assert(j.num === 42, "JSON file: num=42")
  await remove(filePath)
}

async function testFileNotFound() {
  try {
    await fetch("file:///C:\\nonexistent_path_12345.txt")
    assert(false, "File not found: should have thrown")
  } catch (e) {
    console.log("PASS: File not found throws")
  }
}

async function testAbortBeforeFileFetch() {
  const ctrl = new AbortController()
  ctrl.abort()
  try {
    await fetch("file:///C:\\some_file.txt", { signal: ctrl.signal })
    assert(false, "Abort before file fetch: should have thrown")
  } catch (e) {
    assert((e as Error).name === "AbortError", "Abort before file fetch: must be AbortError")
    console.log("PASS: Abort before file fetch throws")
  }
}

async function testResponseProperties() {
  const filePath = `${TMP_DIR}\\_test_fetch_props.txt`
  await writeFile(filePath, "props test")
  const url = `file:///${filePath}`
  const r = await fetch(url)
  assert(r.status === 200, "Response: status=200")
  assert(r.statusText === "OK", "Response: statusText=OK")
  assert(r.url === url, "Response: url matches input")
  assert(r.ok, "Response: ok=true")
  await remove(filePath)
}

async function testPercentEncodedPath() {
  const filePath = `${TMP_DIR}\\_test_fetch space file.txt`
  await writeFile(filePath, "spaced file content")
  const encoded = encodeURI(`file:///${filePath}`)
  const r = await fetch(encoded)
  const t = await r.text()
  assert(t === "spaced file content", "Percent-encoded path: content matches")
  await remove(filePath)
}

async function testBodyIsReadableStream() {
  const filePath = `${TMP_DIR}\\_test_fetch_stream.txt`
  await writeFile(filePath, "stream test data")
  const r = await fetch(`file:///${filePath}`)
  const body = r.body
  assert(body !== null && body !== undefined, "body getter: returns value")
  assert(typeof body.getReader === "function", "body getter: is ReadableStream (has getReader)")
  await remove(filePath)
}

async function testBodyGetReader() {
  const filePath = `${TMP_DIR}\\_test_fetch_reader.txt`
  await writeFile(filePath, "reader test data")
  const r = await fetch(`file:///${filePath}`)
  const body = r.body
  const reader = body.getReader()
  const result = await reader.read()
  assert(result.done === false, "getReader().read(): done=false")
  assert(result.value instanceof ArrayBuffer, "getReader().read(): value is ArrayBuffer")
  assert(result.value.byteLength > 0, "getReader().read(): value has bytes")
  const doneResult = await reader.read()
  assert(doneResult.done === true, "getReader().read(): done=true at end")
  await remove(filePath)
}

async function testBodyStreamLocked() {
  const filePath = `${TMP_DIR}\\_test_fetch_lock.txt`
  await writeFile(filePath, "lock test")
  const r = await fetch(`file:///${filePath}`)
  const body = r.body
  const reader = body.getReader()
  assert(body.locked, "ReadableStream: locked after getReader()")
  try {
    body.getReader()
    assert(false, "ReadableStream: should throw when locked")
  } catch (e) {
    console.log("PASS: getReader on locked stream throws")
  }
  reader.releaseLock()
  assert(!body.locked, "ReadableStream: unlocked after releaseLock()")
  await remove(filePath)
}

async function main() {
  await testBasicFileRead()
  console.log("PASS: Basic file read")

  await testContentType()
  console.log("PASS: Content-Type detection")

  await testJsonFile()
  console.log("PASS: JSON file")

  await testFileNotFound()
  await testAbortBeforeFileFetch()
  console.log("PASS: Negative tests")

  await testResponseProperties()
  console.log("PASS: Response properties")

  await testPercentEncodedPath()
  console.log("PASS: Percent-encoded path")

  await testBodyIsReadableStream()
  console.log("PASS: body is ReadableStream")

  await testBodyGetReader()
  console.log("PASS: body getReader reads data")

  await testBodyStreamLocked()
  console.log("PASS: body stream lock/unlock")

  console.log("ALL FILE FETCH TESTS PASSED")
}

main()
