import { readFile as fsReadFile, open as fsOpen } from "fs/promises";
import { resolve, dirname } from "path";
import { fileURLToPath } from "url";

const __dirname = dirname(fileURLToPath(import.meta.url));
const fixture = (name) => resolve(__dirname, "fixtures", name);

function bufferToStr(buf) {
  let s = "";
  for (let i = 0; i < buf.length; i++) {
    s += String.fromCharCode(buf[i]);
  }
  return s;
}

async function bench(label, iterations, fn) {
  const warmup = Math.floor(iterations / 10);
  for (let i = 0; i < warmup; i++) await fn();

  const start = performance.now();
  for (let i = 0; i < iterations; i++) await fn();
  const elapsed = performance.now() - start;

  const avg = elapsed / iterations;
  const ops = iterations / (elapsed / 1000);
  return { avg, ops, elapsed };
}

function printResult(index, total, name, iterations, warmup, r) {
  console.log(`[${index}/${total}] ${name} (${iterations} iters, ${warmup} warmup)`);
  console.log(`  avg: ${r.avg.toFixed(3)}ms | ops/sec: ${r.ops.toFixed(0)} | total: ${r.elapsed.toFixed(1)}ms`);
  console.log("");
}

async function main() {
  console.log("=== FileHandle Benchmark (Node.js) ===");
  console.log("");

  const total = 11;

  // [1] readFile baseline (1MB)
  {
    const iters = 100;
    const r = await bench("readFile 1MB (baseline)", iters, async () => {
      await fsReadFile(fixture("large_test.bin"));
    });
    printResult(1, total, "readFile 1MB (baseline)", iters, Math.floor(iters / 10), r);
  }

  // [2] open + close
  {
    const iters = 1000;
    const r = await bench("open + close", iters, async () => {
      const fh = await fsOpen(fixture("hello.txt"));
      await fh.close();
    });
    printResult(2, total, "open + close", iters, Math.floor(iters / 10), r);
  }

  // [3] read small file (11B, 5B chunks)
  {
    const iters = 1000;
    const r = await bench("read small 11B, 5B chunks", iters, async () => {
      const fh = await fsOpen(fixture("hello.txt"));
      const buf = Buffer.alloc(5);
      while (true) {
        const { bytesRead } = await fh.read(buf, 0, 5, null);
        if (bytesRead === 0) break;
      }
      await fh.close();
    });
    printResult(3, total, "read small 11B, 5B chunks", iters, Math.floor(iters / 10), r);
  }

  // [4] read large (1MB, 128B chunks)
  {
    const iters = 100;
    const r = await bench("read large 1MB, 128B chunks", iters, async () => {
      const fh = await fsOpen(fixture("large_test.bin"));
      const buf = Buffer.alloc(128);
      while (true) {
        const { bytesRead } = await fh.read(buf, 0, 128, null);
        if (bytesRead === 0) break;
      }
      await fh.close();
    });
    printResult(4, total, "read large 1MB, 128B chunks", iters, Math.floor(iters / 10), r);
  }

  // [5] read large (1MB, 4KB chunks)
  {
    const iters = 1000;
    const r = await bench("read large 1MB, 4KB chunks", iters, async () => {
      const fh = await fsOpen(fixture("large_test.bin"));
      const buf = Buffer.alloc(4096);
      while (true) {
        const { bytesRead } = await fh.read(buf, 0, 4096, null);
        if (bytesRead === 0) break;
      }
      await fh.close();
    });
    printResult(5, total, "read large 1MB, 4KB chunks", iters, Math.floor(iters / 10), r);
  }

  // [6] read large (1MB, 64KB chunks)
  {
    const iters = 1000;
    const r = await bench("read large 1MB, 64KB chunks", iters, async () => {
      const fh = await fsOpen(fixture("large_test.bin"));
      const buf = Buffer.alloc(65536);
      while (true) {
        const { bytesRead } = await fh.read(buf, 0, 65536, null);
        if (bytesRead === 0) break;
      }
      await fh.close();
    });
    printResult(6, total, "read large 1MB, 64KB chunks", iters, Math.floor(iters / 10), r);
  }

  // [7] seek (start, current, end)
  {
    const iters = 1000;
    const r = await bench("seek start/current/end", iters, async () => {
      const fh = await fsOpen(fixture("hello.txt"));
      const stat = await fh.stat();
      const size = stat.size;
      const dummy = Buffer.alloc(0);

      // seek(0, "start")
      await fh.read(dummy, 0, 0, 0);
      // seek(3, "current")
      await fh.read(dummy, 0, 0, 3);
      // seek(-3, "end")
      await fh.read(dummy, 0, 0, size - 3);

      await fh.close();
    });
    printResult(7, total, "seek start/current/end", iters, Math.floor(iters / 10), r);
  }

  // [8] read + Buffer.toString() (4KB chunks)
  {
    const iters = 100;
    const r = await bench("read + Buffer.toString() 4KB", iters, async () => {
      const fh = await fsOpen(fixture("large_test.bin"));
      const buf = Buffer.alloc(4096);
      while (true) {
        const { bytesRead } = await fh.read(buf, 0, 4096, null);
        if (bytesRead === 0) break;
        buf.toString();
      }
      await fh.close();
    });
    printResult(8, total, "read + Buffer.toString() 4KB", iters, Math.floor(iters / 10), r);
  }

  // [9] read + bufferToStr() (4KB chunks) — JS String
  {
    const iters = 100;
    const r = await bench("read + bufferToStr() 4KB", iters, async () => {
      const fh = await fsOpen(fixture("large_test.bin"));
      const buf = Buffer.alloc(4096);
      while (true) {
        const { bytesRead } = await fh.read(buf, 0, 4096, null);
        if (bytesRead === 0) break;
        bufferToStr(buf);
      }
      await fh.close();
    });
    printResult(9, total, "read + bufferToStr() 4KB", iters, Math.floor(iters / 10), r);
  }

  // [10] read + close цикл (много коротких файлов)
  {
    const iters = 1000;
    const r = await bench("read + close цикл", iters, async () => {
      const fh = await fsOpen(fixture("hello.txt"));
      const buf = Buffer.alloc(64);
      await fh.read(buf, 0, 64, null);
      await fh.close();
    });
    printResult(10, total, "read + close цикл", iters, Math.floor(iters / 10), r);
  }

  // [11] buffer.length access
  {
    const iters = 1000;
    const r = await bench("buffer.length access", iters, async () => {
      const fh = await fsOpen(fixture("hello.txt"));
      const buf = Buffer.alloc(64);
      while (true) {
        const { bytesRead } = await fh.read(buf, 0, 64, null);
        if (bytesRead === 0) break;
        const _len = buf.length;
      }
      await fh.close();
    });
    printResult(11, total, "buffer.length access", iters, Math.floor(iters / 10), r);
  }

  console.log("=== Done ===");
}

main();
