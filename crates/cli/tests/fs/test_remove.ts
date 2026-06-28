import { mkdir, writeFile, remove, exists } from "imp:fs"

const tmpDir = import.meta.dirname + ".tmp_remove"
await mkdir(tmpDir, { recursive: true })

// remove existing file returns true
{
  const p = tmpDir + "/file.txt"
  await writeFile(p, "data")
  const removed = await remove(p)
  assert(removed === true, "remove existing file returns true")
  assert(!(await exists(p)), "file is gone after remove")
}

// remove non-existent file returns false (no throw)
{
  const p = tmpDir + "/nope.txt"
  const removed = await remove(p)
  assert(removed === false, "remove non-existent file returns false")
}

// remove already-removed file returns false
{
  const p = tmpDir + "/once.txt"
  await writeFile(p, "x")
  await remove(p)
  const removed = await remove(p)
  assert(removed === false, "double remove returns false second time")
}

// remove directory returns true
{
  const d = tmpDir + "/subdir"
  await mkdir(d, { recursive: true })
  await writeFile(d + "/a.txt", "inside")
  const removed = await remove(d)
  assert(removed === true, "remove directory returns true")
  assert(!(await exists(d)), "directory is gone after remove")
}

// remove empty directory returns true
{
  const d = tmpDir + "/empty"
  await mkdir(d, { recursive: true })
  const removed = await remove(d)
  assert(removed === true, "remove empty directory returns true")
}

// remove non-existent directory returns false
{
  const removed = await remove(tmpDir + "/nope_dir")
  assert(removed === false, "remove non-existent directory returns false")
}

await remove(tmpDir)
console.log("ALL REMOVE TESTS PASSED")
