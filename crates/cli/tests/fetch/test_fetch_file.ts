import { writeFile, remove } from "imp:fs"

const TMP_DIR = process.cwd()

async function testBasicFileRead() {
  const filePath = `${TMP_DIR}\\_test_fetch_basic.txt`
  await writeFile(filePath, "hello file fetch")
  const r = await fetch(`file:///${filePath}`)
  console.assert(r.ok, "Basic file read: ok")
  console.assert(r.status === 200, "Basic file read: status 200")
  const t = await r.text()
  console.assert(t === "hello file fetch", "Basic file read: content matches")
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
  console.assert(rJson.headers.get("content-type") === "application/json", "Content-Type: .json")

  const rHtml = await fetch(`file:///${htmlPath}`)
  console.assert(rHtml.headers.get("content-type") === "text/html", "Content-Type: .html")

  const rCss = await fetch(`file:///${cssPath}`)
  console.assert(rCss.headers.get("content-type") === "text/css", "Content-Type: .css")

  const rJs = await fetch(`file:///${jsPath}`)
  console.assert(rJs.headers.get("content-type") === "application/javascript", "Content-Type: .js")

  const rTxt = await fetch(`file:///${txtPath}`)
  console.assert(rTxt.headers.get("content-type") === "text/plain", "Content-Type: .txt")

  const rUnknown = await fetch(`file:///${unknownPath}`)
  console.assert(rUnknown.headers.get("content-type") === "application/octet-stream", "Content-Type: unknown ext")

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
  console.assert(j.key === "value", "JSON file: key=value")
  console.assert(j.num === 42, "JSON file: num=42")
  await remove(filePath)
}

async function testFileNotFound() {
  try {
    await fetch("file:///C:\\nonexistent_path_12345.txt")
    console.assert(false, "File not found: should have thrown")
  } catch (e) {
    console.log("PASS: File not found throws")
  }
}

async function testAbortBeforeFileFetch() {
  const ctrl = new AbortController()
  ctrl.abort()
  try {
    await fetch("file:///C:\\some_file.txt", { signal: ctrl.signal })
    console.assert(false, "Abort before file fetch: should have thrown")
  } catch (e) {
    console.assert((e as Error).name === "AbortError", "Abort before file fetch: must be AbortError")
    console.log("PASS: Abort before file fetch throws")
  }
}

async function testResponseProperties() {
  const filePath = `${TMP_DIR}\\_test_fetch_props.txt`
  await writeFile(filePath, "props test")
  const url = `file:///${filePath}`
  const r = await fetch(url)
  console.assert(r.status === 200, "Response: status=200")
  console.assert(r.statusText === "OK", "Response: statusText=OK")
  console.assert(r.url === url, "Response: url matches input")
  console.assert(r.ok, "Response: ok=true")
  await remove(filePath)
}

async function testPercentEncodedPath() {
  const filePath = `${TMP_DIR}\\_test_fetch space file.txt`
  await writeFile(filePath, "spaced file content")
  const encoded = encodeURI(`file:///${filePath}`)
  const r = await fetch(encoded)
  const t = await r.text()
  console.assert(t === "spaced file content", "Percent-encoded path: content matches")
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

  console.log("ALL FILE FETCH TESTS PASSED")
}

main()
