import "./helper.ts"

let failed = 0
let passed = 0

async function tryImport(specifier: string): Promise<boolean> {
  try {
    await import(specifier)
    return false
  } catch {
    return true
  }
}

if (await tryImport("./data/bad.json")) {
  console.log("PASS: invalid json throws")
  passed++
} else {
  console.log("FAIL: invalid json should throw")
  failed++
}

if (await tryImport("./data/ghost.txt")) {
  console.log("PASS: nonexistent file throws")
  passed++
} else {
  console.log("FAIL: nonexistent file should throw")
  failed++
}

if (await tryImport("./data/missing.json")) {
  console.log("PASS: nonexistent by extension throws")
  passed++
} else {
  console.log("FAIL: nonexistent by extension should throw")
  failed++
}

console.log(`RESULTS: ${passed} passed, ${failed} failed`)
if (failed > 0) {
  throw new Error(`${failed} failure tests failed`)
}

console.log("ALL FAILURE TESTS PASSED")
