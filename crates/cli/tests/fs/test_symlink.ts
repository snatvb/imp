import { mkdir, symlink, link, writeFile, readFile, metadata, remove } from "imp:fs"

const tmpDir = import.meta.dirname + ".tmp_symlink"
await mkdir(tmpDir, { recursive: true })

const isWindows = process.platform === "win32"
let canSymlink = true

{
  const testTarget = tmpDir + "/perm_test.txt"
  const testLink = tmpDir + "/perm_test_link.txt"
  await writeFile(testTarget, "test")
  try {
    await symlink(testTarget, testLink)
    await remove(testLink)
  } catch {
    canSymlink = false
  }
  await remove(testTarget)
}

if (canSymlink) {
  const target = tmpDir + "/file_target.txt"
  const linkPath = tmpDir + "/file_link.txt"
  await writeFile(target, "hello")
  await symlink(target, linkPath)

  const st = await metadata(linkPath)
  assert(st.isSymbolicLink, "symlink => isSymbolicLink")

  const content = await readFile(linkPath, "utf8")
  assert(content === "hello", "read through symlink")

  await remove(linkPath)
  await remove(target)
}

if (canSymlink) {
  const targetDir = tmpDir + "/dir_target"
  const linkDir = tmpDir + "/dir_link"
  await mkdir(targetDir, { recursive: true })
  await writeFile(targetDir + "/file.txt", "inside dir")
  await symlink(targetDir, linkDir)

  const st = await metadata(linkDir)
  assert(st.isSymbolicLink, "dir symlink => isSymbolicLink")

  const content = await readFile(linkDir + "/file.txt", "utf8")
  assert(content === "inside dir", "read through dir symlink")

  await remove(linkDir)
  await remove(targetDir, { recursive: true })
}

{
  const target = tmpDir + "/hard_target.txt"
  const linkPath = tmpDir + "/hard_link.txt"
  await writeFile(target, "data")
  await link(target, linkPath)

  const st = await metadata(linkPath)
  assert(!st.isSymbolicLink, "hard link => not symlink")

  const content = await readFile(linkPath, "utf8")
  assert(content === "data", "read through hard link")

  await remove(linkPath)
  await remove(target)
}

if (canSymlink) {
  let threw = false
  try {
    await symlink(tmpDir + "/NOPE.txt", tmpDir + "/bad_link.txt")
  } catch (e) {
    threw = true
  }
  assert(threw, "symlink non-existent target throws")
}

{
  let threw = false
  try {
    await link(tmpDir + "/NOPE.txt", tmpDir + "/bad_hard.txt")
  } catch (e) {
    threw = true
  }
  assert(threw, "link non-existent target throws")
}

if (canSymlink) {
  const target = tmpDir + "/existing_target.txt"
  const linkPath = tmpDir + "/existing_link.txt"
  await writeFile(target, "a")
  await symlink(target, linkPath)

  let threw = false
  try {
    await symlink(target, linkPath)
  } catch (e) {
    threw = true
  }
  assert(threw, "symlink to existing path throws")

  await remove(linkPath)
  await remove(target)
}

await remove(tmpDir)
console.log("ALL SYMLINK TESTS PASSED")
