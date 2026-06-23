import { sha256 } from "imp:hash"

{
  const r = sha256("")
  assert(r === "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855", `sha256("") = ${r}`)
}

{
  const r = sha256("hello")
  assert(r === "2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824", `sha256("hello") = ${r}`)
}

{
  const r = sha256("abc")
  assert(r === "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad", `sha256("abc") = ${r}`)
}

{
  const r = sha256("hello", "hex")
  assert(r === "2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824", `sha256 hex = ${r}`)
}

{
  const r = sha256("hello", "base64")
  assert(typeof r === "string", `sha256 base64 is string`)
  assert(r.length > 0, `sha256 base64 non-empty`)
}

{
  const r = sha256("hello", "bytes")
  assert(r instanceof ByteBuffer, `sha256 bytes is ByteBuffer`)
  assert(r.length === 32, `sha256 bytes length ${r.length} (expected 32)`)
}

console.log("ALL HASH SHA256 TESTS PASSED")
