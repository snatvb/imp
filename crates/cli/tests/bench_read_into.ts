import { open, readFile } from "imp:fs";
import { resolve } from "path";

const fixture = (name: string) => resolve(import.meta.dirname, "fixtures", name);

function bytesToStr(buf: ArrayBuffer): string {
  const view = new Uint8Array(buf);
  let s = "";
  for (let i = 0; i < view.length; i++) {
    s += String.fromCharCode(view[i]);
  }
  return s;
}

async function bench(
  label: string,
  iterations: number,
  fn: () => Promise<void>,
): Promise<{ avg: number; ops: number; elapsed: number }> {
  const warmup = Math.floor(iterations / 10);
  for (let i = 0; i < warmup; i++) await fn();

  const start = performance.now();
  for (let i = 0; i < iterations; i++) await fn();
  const elapsed = performance.now() - start;

  const avg = elapsed / iterations;
  const ops = iterations / (elapsed / 1000);
  return { avg, ops, elapsed };
}

function printResult(
  index: number,
  total: number,
  name: string,
  iterations: number,
  warmup: number,
  r: { avg: number; ops: number; elapsed: number },
) {
  console.log(`[${index}/${total}] ${name} (${iterations} iters, ${warmup} warmup)`);
  console.log(`  avg: ${r.avg.toFixed(3)}ms | ops/sec: ${r.ops.toFixed(0)} | total: ${r.elapsed.toFixed(1)}ms`);
  console.log("");
}

async function main() {
  console.log("=== FileHandle read_into Benchmark ===");
  console.log("");

  const total = 6;

  // [1] read_into small 11B, 5B buffer
  {
    const iters = 10000;
    const r = await bench("read_into small 11B, 5B buffer", iters, async () => {
      const fh = await open(fixture("hello.txt"), 5);
      const buf = ByteBuffer.alloc(5);
      while ((await fh.readInto(buf)) !== undefined) {}
      await fh.close();
    });
    printResult(1, total, "read_into small 11B, 5B buffer", iters, Math.floor(iters / 10), r);
  }

  // [2] read_into large 1MB, 128B buffer
  {
    const iters = 100;
    const r = await bench("read_into large 1MB, 128B buffer", iters, async () => {
      const fh = await open(fixture("large_test.bin"), 128);
      const buf = ByteBuffer.alloc(128);
      while ((await fh.readInto(buf)) !== undefined) {}
      await fh.close();
    });
    printResult(2, total, "read_into large 1MB, 128B buffer", iters, Math.floor(iters / 10), r);
  }

  // [3] read_into large 1MB, 4KB buffer
  {
    const iters = 1000;
    const r = await bench("read_into large 1MB, 4KB buffer", iters, async () => {
      const fh = await open(fixture("large_test.bin"), 4096);
      const buf = ByteBuffer.alloc(4096);
      while ((await fh.readInto(buf)) !== undefined) {}
      await fh.close();
    });
    printResult(3, total, "read_into large 1MB, 4KB buffer", iters, Math.floor(iters / 10), r);
  }

  // [4] read_into large 1MB, 64KB buffer
  {
    const iters = 1000;
    const r = await bench("read_into large 1MB, 64KB buffer", iters, async () => {
      const fh = await open(fixture("large_test.bin"), 65536);
      const buf = ByteBuffer.alloc(65536);
      while ((await fh.readInto(buf)) !== undefined) {}
      await fh.close();
    });
    printResult(4, total, "read_into large 1MB, 64KB buffer", iters, Math.floor(iters / 10), r);
  }

  // [5] read_into + toString 4KB buffer
  {
    const iters = 100;
    const r = await bench("read_into + toString 4KB buffer", iters, async () => {
      const fh = await open(fixture("large_test.bin"), 4096);
      const buf = ByteBuffer.alloc(4096);
      while ((await fh.readInto(buf)) !== undefined) {
        buf.toString();
      }
      await fh.close();
    });
    printResult(5, total, "read_into + toString 4KB buffer", iters, Math.floor(iters / 10), r);
  }

  // [6] read_into + toStr 4KB buffer
  {
    const iters = 100;
    const r = await bench("read_into + toStr 4KB buffer", iters, async () => {
      const fh = await open(fixture("large_test.bin"), 4096);
      const buf = ByteBuffer.alloc(4096);
      while ((await fh.readInto(buf)) !== undefined) {
        buf.toStr();
      }
      await fh.close();
    });
    printResult(6, total, "read_into + toStr 4KB buffer", iters, Math.floor(iters / 10), r);
  }

  console.log("=== Done ===");
}

main();
