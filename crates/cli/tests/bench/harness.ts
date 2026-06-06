import { resolve } from "path";

export const fixture = (name: string) => resolve(import.meta.dirname, "..", "fixtures", name);
export const repoRoot = resolve(import.meta.dirname, "..", "..", "..", "..");

export async function bench(
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

export function printResult(
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
