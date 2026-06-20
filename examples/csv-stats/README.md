# csv-stats

Process N CSV files **in parallel** with `Promise.all`, compute per-file
statistics (min/max/mean/total) for numeric columns, and aggregate across all
files grouped by product and region.

## What it shows

- `imp:parsers` — CSV parse + DataFrame-like aggregation
- `Promise.all` — **concurrent file processing** (imp's killer feature)
- `imp:fs` — `readFile` with `utf8` encoding
- `Record<string, T>` accumulators for group-by
- Top-level `await`
- `performance.now()` timing

## Run

```bash
imp run main.ts
```

The script processes 4 fixture files (Q1–Q4 sales, 30 rows each = 120 total
rows) concurrently and reports per-file + aggregate stats.

## Expected output

```
=== CSV Stats — Parallel (Promise.all) ===
Processed 4 files in <50ms

--- sales-q1.csv (30 rows) ---
  units:   min=15  max=53  mean=33.8  total=1014
  revenue: min=900  max=2300  mean=1631.67  total=48950
  profit:  min=180  max=1380  mean=571.67  total=17150

...

=== Aggregate across all files ===
  total rows:    120
  total revenue: 244200
  total profit:  82920
  margin:        33.9%

By product:
  Widget   revenue=96800  profit=31600
  Gadget   revenue=76200  profit=29000
  Gizmo    revenue=43200  profit=21600

By region:
  North    revenue=71200  profit=23800
  East     revenue=62800  profit=21600
  South    revenue=58200  profit=19600
  West     revenue=52000  profit=17920
```

## Why this matters

The `Promise.all` line processes 4 files concurrently. The async I/O overlaps —
filesystem reads run in parallel rather than sequentially. On a real workload
with slow disks or remote files, the speedup can be 3–4×.

```ts
const results = await Promise.all(files.map(async (f) => processFile(f)))
```

That's the entire parallelism story in imp — no thread pools, no worker
threads, no `worker_threads` boilerplate. Tokio underneath schedules the
concurrent reads on its async runtime.
