import { readFile as fsReadFile, open as fsOpen } from "fs/promises";
import { bench, printResult, fixture } from "./harness.mjs";

function bufferToStr(buf) {
  let s = "";
  for (let i = 0; i < buf.length; i++) {
    s += String.fromCharCode(buf[i]);
  }
  return s;
}

async function main() {
  console.log("=== FileHandle Benchmark (Node.js) ===");
  console.log("");

  const total = 11;

  {
    const iters = 100;
    const r = await bench("readFile 1MB (baseline)", iters, async () => {
      await fsReadFile(fixture("large_test.bin"));
    });
    printResult(1, total, "readFile 1MB (baseline)", iters, Math.floor(iters / 10), r);
  }

  {
    const iters = 1000;
    const r = await bench("open + close", iters, async () => {
      const fh = await fsOpen(fixture("hello.txt"));
      await fh.close();
    });
    printResult(2, total, "open + close", iters, Math.floor(iters / 10), r);
  }

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

  {
    const iters = 1000;
    const r = await bench("seek start/current/end", iters, async () => {
      const fh = await fsOpen(fixture("hello.txt"));
      const stat = await fh.stat();
      const size = stat.size;
      const dummy = Buffer.alloc(0);

      await fh.read(dummy, 0, 0, 0);
      await fh.read(dummy, 0, 0, 3);
      await fh.read(dummy, 0, 0, size - 3);

      await fh.close();
    });
    printResult(7, total, "seek start/current/end", iters, Math.floor(iters / 10), r);
  }

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
