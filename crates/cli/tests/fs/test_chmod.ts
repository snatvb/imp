import { mkdir, chmod, lchmod, metadata, writeFile, remove } from "imp:fs"

const tmpDir = import.meta.dirname + "/.tmp_chmod"
await mkdir(tmpDir, { recursive: true })

const isWindows = process.platform === "win32"

// --- chmod on a file: writable → readonly → writable ---
{
  const filePath = tmpDir + "/test_file.txt"
  await writeFile(filePath, "hello")

  await chmod(filePath, 0o644)
  {
    const st = await metadata(filePath)
    assert(!st.readonly, "0o644 => not readonly")
  }

  await chmod(filePath, 0o444)
  {
    const st = await metadata(filePath)
    assert(st.readonly, "0o444 => readonly")
  }

  await chmod(filePath, 0o644)
  {
    const st = await metadata(filePath)
    assert(!st.readonly, "back to 0o644 => not readonly")
  }

  await remove(filePath)
}

// --- chmod on a directory ---
{
  const dirPath = tmpDir + "/test_dir"
  await mkdir(dirPath, { recursive: true })

  await chmod(dirPath, 0o755)
  {
    const st = await metadata(dirPath)
    assert(!st.readonly, "dir 0o755 => not readonly")
  }

  await chmod(dirPath, 0o555)
  {
    const st = await metadata(dirPath)
    assert(st.readonly, "dir 0o555 => readonly")
  }

  await chmod(dirPath, 0o755)
  {
    const st = await metadata(dirPath)
    assert(!st.readonly, "dir back to 0o755 => not readonly")
  }

  await remove(dirPath)
}

// --- lchmod on a symlink ---
{
  const targetPath = tmpDir + "/lchmod_target.txt"
  const linkPath = tmpDir + "/lchmod_link.txt"
  await writeFile(targetPath, "content")

  let symlinkCreated = false
  try {
    if (isWindows) {
      const { run } = await import("imp:subprocess")
      const res = await run("cmd", ["/c", "mklink", linkPath, targetPath])
      symlinkCreated = res.success
    } else {
      const { run } = await import("imp:subprocess")
      const res = await run("ln", ["-s", targetPath, linkPath])
      symlinkCreated = res.success
    }
  } catch {
    symlinkCreated = false
  }

  if (symlinkCreated) {
    {
      const st = await metadata(linkPath)
      if (!st.isSymbolicLink) {
        symlinkCreated = false
      }
    }
  }

  if (symlinkCreated) {
    await lchmod(linkPath, 0o444)
    {
      const st = await metadata(linkPath)
      assert(st.isSymbolicLink, "lchmod does not follow symlink")
    }

    await lchmod(linkPath, 0o644)
    {
      const st = await metadata(linkPath)
      assert(st.isSymbolicLink, "lchmod still sees symlink after second call")
    }

    await remove(linkPath)
  }

  await remove(targetPath)
}

// --- multiple mode changes ---
{
  const filePath = tmpDir + "/multi_mode.txt"
  await writeFile(filePath, "data")

  const modes = [0o777, 0o400, 0o644, 0o755, 0o444]
  for (const m of modes) {
    await chmod(filePath, m)
    const st = await metadata(filePath)
    if (isWindows) {
      const expectedReadonly = (m & 0o200) === 0
      assert(
        st.readonly === expectedReadonly,
        `mode 0o${m.toString(8)}: readonly=${st.readonly}, expected=${expectedReadonly}`,
      )
    }
  }

  await remove(filePath)
}

// --- non-existent path throws ---
{
  let threw = false
  try {
    await chmod(tmpDir + "/NOPE.txt", 0o644)
  } catch (e) {
    threw = true
    assert(String(e).includes("ENOENT") || String(e).includes("no such file"), "error mentions ENOENT")
  }
  assert(threw, "chmod on missing path throws")
}

// --- non-existent path throws for lchmod ---
{
  let threw = false
  try {
    await lchmod(tmpDir + "/NOPE_LINK.txt", 0o644)
  } catch (e) {
    threw = true
    assert(String(e).includes("ENOENT") || String(e).includes("no such file"), "error mentions ENOENT")
  }
  assert(threw, "lchmod on missing path throws")
}

await remove(tmpDir)

console.log("ALL CHMOD TESTS PASSED")
