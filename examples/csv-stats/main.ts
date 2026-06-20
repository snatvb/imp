import { readFile } from "imp:fs"
import { csv } from "imp:parsers"

type FileStats = {
  file: string
  rows: number
  units: { min: number; max: number; mean: number; total: number }
  revenue: { min: number; max: number; mean: number; total: number }
  profit: { min: number; max: number; mean: number; total: number }
  byProduct: Record<string, { units: number; revenue: number; profit: number }>
  byRegion: Record<string, { units: number; revenue: number; profit: number }>
}

const files = [
  import.meta.dirname + "/fixtures/sales-q1.csv",
  import.meta.dirname + "/fixtures/sales-q2.csv",
  import.meta.dirname + "/fixtures/sales-q3.csv",
  import.meta.dirname + "/fixtures/sales-q4.csv",
]

function summarizeColumn(values: number[]) {
  const min = Math.min(...values)
  const max = Math.max(...values)
  const total = values.reduce((a, b) => a + b, 0)
  const mean = total / values.length
  return { min, max, mean: +mean.toFixed(2), total }
}

async function processFile(path: string): Promise<FileStats> {
  const text = await readFile(path, "utf8")
  const rows = csv.parse(text) as Record<string, any>[]

  const units = rows.map((r) => Number(r.units))
  const revenue = rows.map((r) => Number(r.revenue))
  const profit = rows.map((r) => Number(r.revenue) - Number(r.cost))

  const byProduct: Record<string, { units: number; revenue: number; profit: number }> = {}
  const byRegion: Record<string, { units: number; revenue: number; profit: number }> = {}
  for (const r of rows) {
    const u = Number(r.units)
    const rv = Number(r.revenue)
    const c = Number(r.cost)
    const p = (byProduct[r.product] ??= { units: 0, revenue: 0, profit: 0 })
    p.units += u
    p.revenue += rv
    p.profit += rv - c
    const g = (byRegion[r.region] ??= { units: 0, revenue: 0, profit: 0 })
    g.units += u
    g.revenue += rv
    g.profit += rv - c
  }

  return {
    file: path.split("/").pop()!,
    rows: rows.length,
    units: summarizeColumn(units),
    revenue: summarizeColumn(revenue),
    profit: summarizeColumn(profit),
    byProduct,
    byRegion,
  }
}

const t0 = performance.now()
const results = await Promise.all(files.map((f) => processFile(f)))
const elapsed = (performance.now() - t0).toFixed(1)

console.log("=== CSV Stats — Parallel (Promise.all) ===")
console.log(`Processed ${results.length} files in ${elapsed}ms`)
console.log("")

for (const s of results) {
  console.log(`--- ${s.file} (${s.rows} rows) ---`)
  console.log(`  units:   min=${s.units.min}  max=${s.units.max}  mean=${s.units.mean}  total=${s.units.total}`)
  console.log(`  revenue: min=${s.revenue.min}  max=${s.revenue.max}  mean=${s.revenue.mean}  total=${s.revenue.total}`)
  console.log(`  profit:  min=${s.profit.min}  max=${s.profit.max}  mean=${s.profit.mean}  total=${s.profit.total}`)
  console.log("")
}

console.log("=== Aggregate across all files ===")
const totalRev = results.reduce((a, s) => a + s.revenue.total, 0)
const totalProfit = results.reduce((a, s) => a + s.profit.total, 0)
const totalRows = results.reduce((a, s) => a + s.rows, 0)
console.log(`  total rows:    ${totalRows}`)
console.log(`  total revenue: ${totalRev}`)
console.log(`  total profit:  ${totalProfit}`)
console.log(`  margin:        ${((totalProfit / totalRev) * 100).toFixed(1)}%`)
console.log("")

const productAgg: Record<string, { revenue: number; profit: number }> = {}
const regionAgg: Record<string, { revenue: number; profit: number }> = {}
for (const s of results) {
  for (const [k, v] of Object.entries(s.byProduct)) {
    const p = (productAgg[k] ??= { revenue: 0, profit: 0 })
    p.revenue += v.revenue
    p.profit += v.profit
  }
  for (const [k, v] of Object.entries(s.byRegion)) {
    const p = (regionAgg[k] ??= { revenue: 0, profit: 0 })
    p.revenue += v.revenue
    p.profit += v.profit
  }
}

console.log("By product:")
for (const [k, v] of Object.entries(productAgg).sort((a, b) => b[1].revenue - a[1].revenue)) {
  console.log(`  ${k.padEnd(8)} revenue=${v.revenue.toString().padStart(7)}  profit=${v.profit.toString().padStart(7)}`)
}
console.log("")
console.log("By region:")
for (const [k, v] of Object.entries(regionAgg).sort((a, b) => b[1].revenue - a[1].revenue)) {
  console.log(`  ${k.padEnd(8)} revenue=${v.revenue.toString().padStart(7)}  profit=${v.profit.toString().padStart(7)}`)
}
