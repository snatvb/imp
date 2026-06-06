import { resolve, dirname } from "path";
import { fileURLToPath } from "url";

const __dirname = dirname(fileURLToPath(import.meta.url));
export const fixture = (name) => resolve(__dirname, "..", "fixtures", name);
export const repoRoot = resolve(__dirname, "..", "..", "..", "..");

export async function bench(label, iterations, fn) {
  const warmup = Math.floor(iterations / 10);
  for (let i = 0; i < warmup; i++) await fn();

  const start = performance.now();
  for (let i = 0; i < iterations; i++) await fn();
  const elapsed = performance.now() - start;

  const avg = elapsed / iterations;
  const ops = iterations / (elapsed / 1000);
  return { avg, ops, elapsed };
}

export function printResult(index, total, name, iterations, warmup, r) {
  console.log(`[${index}/${total}] ${name} (${iterations} iters, ${warmup} warmup)`);
  console.log(`  avg: ${r.avg.toFixed(3)}ms | ops/sec: ${r.ops.toFixed(0)} | total: ${r.elapsed.toFixed(1)}ms`);
  console.log("");
}
