import { timingSafeEqual, randomBytes } from "imp:crypto"

{
  const a = randomBytes(32)
  const b = randomBytes(32)
  const r = timingSafeEqual(a, b)
  assert(r === false, "different buffers not equal")
}

{
  const a = randomBytes(16)
  const b = randomBytes(32)
  assert(timingSafeEqual(a, b) === false, "different lengths not equal")
}

{
  const a = randomBytes(0)
  const b = randomBytes(0)
  assert(timingSafeEqual(a, b) === true, "empty buffers equal")
}

console.log("ALL CRYPTO TIMING_SAFE_EQUAL TESTS PASSED")
