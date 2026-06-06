import { open, readFile } from "imp:fs";
import { bench, printResult, fixture } from "./harness";

async function main() {
  console.log("=== FileHandle Benchmark ===");
  console.log("");

  const total = 17;

  {
    const iters = 100;
    const r = await bench("readFile 1MB (baseline)", iters, async () => {
      await readFile(fixture("large_test.bin"), "buffer");
    });
    printResult(1, total, "readFile 1MB (baseline)", iters, Math.floor(iters / 10), r);
  }

  {
    const iters = 10000;
    const r = await bench("open + close", iters, async () => {
      const fh = await open(fixture("hello.txt"), 64);
      await fh.close();
    });
    printResult(2, total, "open + close", iters, Math.floor(iters / 10), r);
  }

  {
    const iters = 10000;
    const r = await bench("read small 11B, 5B chunks", iters, async () => {
      const fh = await open(fixture("hello.txt"), 5);
      while ((await fh.read()) !== undefined) {}
      await fh.close();
    });
    printResult(3, total, "read small 11B, 5B chunks", iters, Math.floor(iters / 10), r);
  }

  {
    const iters = 100;
    const r = await bench("read large 1MB, 128B chunks", iters, async () => {
      const fh = await open(fixture("large_test.bin"), 128);
      while ((await fh.read()) !== undefined) {}
      await fh.close();
    });
    printResult(4, total, "read large 1MB, 128B chunks", iters, Math.floor(iters / 10), r);
  }

  {
    const iters = 1000;
    const r = await bench("read large 1MB, 4KB chunks", iters, async () => {
      const fh = await open(fixture("large_test.bin"), 4096);
      while ((await fh.read()) !== undefined) {}
      await fh.close();
    });
    printResult(5, total, "read large 1MB, 4KB chunks", iters, Math.floor(iters / 10), r);
  }

  {
    const iters = 1000;
    const r = await bench("read large 1MB, 64KB chunks", iters, async () => {
      const fh = await open(fixture("large_test.bin"), 65536);
      while ((await fh.read()) !== undefined) {}
      await fh.close();
    });
    printResult(6, total, "read large 1MB, 64KB chunks", iters, Math.floor(iters / 10), r);
  }

  {
    const iters = 10000;
    const r = await bench("seek start/current/end", iters, async () => {
      const fh = await open(fixture("hello.txt"), 64);
      await fh.seek(0, "start");
      await fh.seek(3, "current");
      await fh.seek(-3, "end");
      await fh.close();
    });
    printResult(7, total, "seek start/current/end", iters, Math.floor(iters / 10), r);
  }

  {
    const iters = 100;
    const r = await bench("read + ByteBuffer.toString() 4KB", iters, async () => {
      const fh = await open(fixture("large_test.bin"), 4096);
      let chunk;
      while ((chunk = await fh.read()) !== undefined) {
        chunk.toString();
      }
      await fh.close();
    });
    printResult(8, total, "read + ByteBuffer.toString() 4KB", iters, Math.floor(iters / 10), r);
  }

  {
    const iters = 100;
    const r = await bench("read + ByteBuffer.toStr() 4KB", iters, async () => {
      const fh = await open(fixture("large_test.bin"), 4096);
      let chunk;
      while ((chunk = await fh.read()) !== undefined) {
        chunk.toStr();
      }
      await fh.close();
    });
    printResult(9, total, "read + ByteBuffer.toStr() 4KB", iters, Math.floor(iters / 10), r);
  }

  {
    const iters = 10000;
    const r = await bench("read + close цикл", iters, async () => {
      const fh = await open(fixture("hello.txt"), 64);
      await fh.read();
      await fh.close();
    });
    printResult(10, total, "read + close цикл", iters, Math.floor(iters / 10), r);
  }

  {
    const iters = 10000;
    const r = await bench("ByteBuffer.length access", iters, async () => {
      const fh = await open(fixture("hello.txt"), 64);
      let chunk;
      while ((chunk = await fh.read()) !== undefined) {
        const _len = chunk.length;
      }
      await fh.close();
    });
    printResult(11, total, "ByteBuffer.length access", iters, Math.floor(iters / 10), r);
  }

  {
    const iters = 10000;
    const r = await bench("readInto small 11B, 5B buffer", iters, async () => {
      const fh = await open(fixture("hello.txt"), 5);
      const buf = new ByteBuffer(5);
      while ((await fh.readInto(buf)) !== undefined) {}
      await fh.close();
    });
    printResult(12, total, "readInto small 11B, 5B buffer", iters, Math.floor(iters / 10), r);
  }

  {
    const iters = 100;
    const r = await bench("readInto large 1MB, 128B buffer", iters, async () => {
      const fh = await open(fixture("large_test.bin"), 128);
      const buf = new ByteBuffer(128);
      while ((await fh.readInto(buf)) !== undefined) {}
      await fh.close();
    });
    printResult(13, total, "readInto large 1MB, 128B buffer", iters, Math.floor(iters / 10), r);
  }

  {
    const iters = 1000;
    const r = await bench("readInto large 1MB, 4KB buffer", iters, async () => {
      const fh = await open(fixture("large_test.bin"), 4096);
      const buf = new ByteBuffer(4096);
      while ((await fh.readInto(buf)) !== undefined) {}
      await fh.close();
    });
    printResult(14, total, "readInto large 1MB, 4KB buffer", iters, Math.floor(iters / 10), r);
  }

  {
    const iters = 1000;
    const r = await bench("readInto large 1MB, 64KB buffer", iters, async () => {
      const fh = await open(fixture("large_test.bin"), 65536);
      const buf = new ByteBuffer(65536);
      while ((await fh.readInto(buf)) !== undefined) {}
      await fh.close();
    });
    printResult(15, total, "readInto large 1MB, 64KB buffer", iters, Math.floor(iters / 10), r);
  }

  {
    const iters = 100;
    const r = await bench("readInto + ByteBuffer.toString() 4KB", iters, async () => {
      const fh = await open(fixture("large_test.bin"), 4096);
      const buf = new ByteBuffer(4096);
      while ((await fh.readInto(buf)) !== undefined) {
        buf.toString();
      }
      await fh.close();
    });
    printResult(16, total, "readInto + ByteBuffer.toString() 4KB", iters, Math.floor(iters / 10), r);
  }

  {
    const iters = 100;
    const r = await bench("readInto + ByteBuffer.toStr() 4KB", iters, async () => {
      const fh = await open(fixture("large_test.bin"), 4096);
      const buf = new ByteBuffer(4096);
      while ((await fh.readInto(buf)) !== undefined) {
        buf.toStr();
      }
      await fh.close();
    });
    printResult(17, total, "readInto + ByteBuffer.toStr() 4KB", iters, Math.floor(iters / 10), r);
  }

  console.log("=== Done ===");
}

main();
