import { timingSafeEqual, randomBytes } from "imp:crypto"

{
  const a = randomBytes(32)
  const b = randomBytes(32)
  assert(timingSafeEqual(a, b) === false, "different random buffers not equal")
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

{
  const a = ByteBuffer.fromArray([1, 2, 3, 4])
  const b = ByteBuffer.fromArray([1, 2, 3, 4])
  assert(timingSafeEqual(a, b) === true, "identical arrays equal")
}

{
  const a = ByteBuffer.fromArray([1, 2, 3, 4])
  const b = ByteBuffer.fromArray([1, 2, 3, 5])
  assert(timingSafeEqual(a, b) === false, "last byte different")
}

{
  const a = ByteBuffer.fromArray([0, 0, 0, 0])
  const b = ByteBuffer.fromArray([0, 0, 0, 0])
  assert(timingSafeEqual(a, b) === true, "all zeros equal")
}

{
  const a = ByteBuffer.fromArray([255, 255, 255, 255])
  const b = ByteBuffer.fromArray([255, 255, 255, 255])
  assert(timingSafeEqual(a, b) === true, "all 0xff equal")
}

{
  const a = ByteBuffer.fromArray([1])
  const b = ByteBuffer.fromArray([1, 2])
  assert(timingSafeEqual(a, b) === false, "length 1 vs 2 not equal")
}

{
  const a = ByteBuffer.fromArray([])
  const b = ByteBuffer.fromArray([1])
  assert(timingSafeEqual(a, b) === false, "empty vs non-empty not equal")
}

{
  const a = ByteBuffer.fromArray([10, 20, 30])
  const b = ByteBuffer.fromArray([10, 20, 31])
  assert(timingSafeEqual(a, b) === false, "off-by-one last byte")
}

{
  const arr = new Array(256).fill(0).map((_, i) => i & 0xff)
  const a = ByteBuffer.fromArray(arr)
  const b = ByteBuffer.fromArray(arr)
  assert(timingSafeEqual(a, b) === true, "256-byte identical buffers equal")
}

console.log("ALL CRYPTO TIMING_SAFE_EQUAL TESTS PASSED")
