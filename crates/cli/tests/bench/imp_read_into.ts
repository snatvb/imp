import { open } from "imp:fs"

import { bench, printResult, fixture } from "./harness"

async function main() {
  console.log("=== FileHandle read_into Benchmark ===")
  console.log("")

  const total = 6

  {
    const iters = 10000
    const r = await bench("read_into small 11B, 5B buffer", iters, async () => {
      const fh = await open(fixture("hello.txt"), 5)
      const buf = ByteBuffer.alloc(5)
      while ((await fh.readInto(buf)) !== undefined) {}
      await fh.close()
    })
    printResult(1, total, "read_into small 11B, 5B buffer", iters, Math.floor(iters / 10), r)
  }

  {
    const iters = 100
    const r = await bench("read_into large 1MB, 128B buffer", iters, async () => {
      const fh = await open(fixture("large_test.bin"), 128)
      const buf = ByteBuffer.alloc(128)
      while ((await fh.readInto(buf)) !== undefined) {}
      await fh.close()
    })
    printResult(2, total, "read_into large 1MB, 128B buffer", iters, Math.floor(iters / 10), r)
  }

  {
    const iters = 1000
    const r = await bench("read_into large 1MB, 4KB buffer", iters, async () => {
      const fh = await open(fixture("large_test.bin"), 4096)
      const buf = ByteBuffer.alloc(4096)
      while ((await fh.readInto(buf)) !== undefined) {}
      await fh.close()
    })
    printResult(3, total, "read_into large 1MB, 4KB buffer", iters, Math.floor(iters / 10), r)
  }

  {
    const iters = 1000
    const r = await bench("read_into large 1MB, 64KB buffer", iters, async () => {
      const fh = await open(fixture("large_test.bin"), 65536)
      const buf = ByteBuffer.alloc(65536)
      while ((await fh.readInto(buf)) !== undefined) {}
      await fh.close()
    })
    printResult(4, total, "read_into large 1MB, 64KB buffer", iters, Math.floor(iters / 10), r)
  }

  {
    const iters = 100
    const r = await bench("read_into + toString 4KB buffer", iters, async () => {
      const fh = await open(fixture("large_test.bin"), 4096)
      const buf = ByteBuffer.alloc(4096)
      while ((await fh.readInto(buf)) !== undefined) {
        buf.toString()
      }
      await fh.close()
    })
    printResult(5, total, "read_into + toString 4KB buffer", iters, Math.floor(iters / 10), r)
  }

  {
    const iters = 100
    const r = await bench("read_into + toStr 4KB buffer", iters, async () => {
      const fh = await open(fixture("large_test.bin"), 4096)
      const buf = ByteBuffer.alloc(4096)
      while ((await fh.readInto(buf)) !== undefined) {
        buf.toStr()
      }
      await fh.close()
    })
    printResult(6, total, "read_into + toStr 4KB buffer", iters, Math.floor(iters / 10), r)
  }

  console.log("=== Done ===")
}

main()
