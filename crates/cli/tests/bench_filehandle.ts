import { open } from "imp:fs";
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
  console.log("=== FileHandle Benchmark ===");
  console.log("");

  const total = 8;

  // [1] open + close
  {
    const iters = 10000;
    const r = await bench("open + close", iters, async () => {
      const fh = await open(fixture("hello.txt"), 64);
      await fh.close();
    });
    printResult(1, total, "open + close", iters, Math.floor(iters / 10), r);
  }

  // [2] read small file (11B, 5B chunks)
  {
    const iters = 10000;
    const r = await bench("read small 11B, 5B chunks", iters, async () => {
      const fh = await open(fixture("hello.txt"), 5);
      while ((await fh.read()) !== undefined) {}
      await fh.close();
    });
    printResult(2, total, "read small 11B, 5B chunks", iters, Math.floor(iters / 10), r);
  }

  // [3] read large (1MB, 128B chunks)
  {
    const iters = 100;
    const r = await bench("read large 1MB, 128B chunks", iters, async () => {
      const fh = await open(fixture("large_test.bin"), 128);
      while ((await fh.read()) !== undefined) {}
      await fh.close();
    });
    printResult(3, total, "read large 1MB, 128B chunks", iters, Math.floor(iters / 10), r);
  }

  // [4] read large (1MB, 4KB chunks)
  {
    const iters = 1000;
    const r = await bench("read large 1MB, 4KB chunks", iters, async () => {
      const fh = await open(fixture("large_test.bin"), 4096);
      while ((await fh.read()) !== undefined) {}
      await fh.close();
    });
    printResult(4, total, "read large 1MB, 4KB chunks", iters, Math.floor(iters / 10), r);
  }

  // [5] read large (1MB, 64KB chunks)
  {
    const iters = 1000;
    const r = await bench("read large 1MB, 64KB chunks", iters, async () => {
      const fh = await open(fixture("large_test.bin"), 65536);
      while ((await fh.read()) !== undefined) {}
      await fh.close();
    });
    printResult(5, total, "read large 1MB, 64KB chunks", iters, Math.floor(iters / 10), r);
  }

  // [6] seek (start, current, end)
  {
    const iters = 10000;
    const r = await bench("seek start/current/end", iters, async () => {
      const fh = await open(fixture("hello.txt"), 64);
      await fh.seek(0, "start");
      await fh.seek(3, "current");
      await fh.seek(-3, "end");
      await fh.close();
    });
    printResult(6, total, "seek start/current/end", iters, Math.floor(iters / 10), r);
  }

  // [7] read + bytesToStr (4KB chunks)
  {
    const iters = 100;
    const r = await bench("read + bytesToStr 4KB chunks", iters, async () => {
      const fh = await open(fixture("large_test.bin"), 4096);
      let chunk;
      while ((chunk = await fh.read()) !== undefined) {
        bytesToStr(chunk);
      }
      await fh.close();
    });
    printResult(7, total, "read + bytesToStr 4KB chunks", iters, Math.floor(iters / 10), r);
  }

  // [8] read + close цикл (много коротких файлов)
  {
    const iters = 10000;
    const r = await bench("read + close цикл", iters, async () => {
      const fh = await open(fixture("hello.txt"), 64);
      await fh.read();
      await fh.close();
    });
    printResult(8, total, "read + close цикл", iters, Math.floor(iters / 10), r);
  }

  console.log("=== Done ===");
}

main();
