import { json } from "imp:parsers"

import { bench, printResult } from "./harness"

const SMALL_JSON = '{"name":"test","value":42,"active":true}'
const MEDIUM_JSON =
  '{"users":[{"id":1,"name":"Alice","email":"alice@example.com","roles":["admin","user"]},{"id":2,"name":"Bob","email":"bob@example.com","roles":["user"]},{"id":3,"name":"Charlie","email":"charlie@example.com","roles":[]}],"meta":{"total":3,"page":1,"perPage":10}}'
const LARGE_JSON = JSON.stringify({
  items: Array.from({ length: 1000 }, (_, i) => ({
    id: i,
    name: `item_${i}`,
    description: `Description for item ${i} with some extra text to make it larger`,
    price: Math.round(Math.random() * 10000) / 100,
    inStock: i % 3 !== 0,
    tags: ["tag1", "tag2", "tag3"],
    metadata: {
      createdAt: "2024-01-01T00:00:00Z",
      updatedAt: "2024-06-01T00:00:00Z",
      version: i % 10,
    },
  })),
})

const SMALL_OBJ = { name: "test", value: 42, active: true }
const MEDIUM_OBJ = {
  users: [
    { id: 1, name: "Alice", email: "alice@example.com", roles: ["admin", "user"] },
    { id: 2, name: "Bob", email: "bob@example.com", roles: ["user"] },
    { id: 3, name: "Charlie", email: "charlie@example.com", roles: [] },
  ],
  meta: { total: 3, page: 1, perPage: 10 },
}
const LARGE_OBJ = {
  items: Array.from({ length: 1000 }, (_, i) => ({
    id: i,
    name: `item_${i}`,
    description: `Description for item ${i} with some extra text to make it larger`,
    price: Math.round(Math.random() * 10000) / 100,
    inStock: i % 3 !== 0,
    tags: ["tag1", "tag2", "tag3"],
    metadata: {
      createdAt: "2024-01-01T00:00:00Z",
      updatedAt: "2024-06-01T00:00:00Z",
      version: i % 10,
    },
  })),
}

async function main() {
  console.log("=== JSON.parse vs parsers.json.parse Benchmark ===")
  console.log("")

  const total = 6

  {
    const iters = 50000
    const r1 = await bench("JSON.parse small", iters, async () => {
      JSON.parse(SMALL_JSON)
    })
    const r2 = await bench("parsers.json.parse small", iters, async () => {
      json.parse(SMALL_JSON)
    })
    printResult(1, total, "JSON.parse small", iters, Math.floor(iters / 10), r1)
    printResult(2, total, "parsers.json.parse small", iters, Math.floor(iters / 10), r2)
  }

  {
    const iters = 10000
    const r1 = await bench("JSON.parse medium", iters, async () => {
      JSON.parse(MEDIUM_JSON)
    })
    const r2 = await bench("parsers.json.parse medium", iters, async () => {
      json.parse(MEDIUM_JSON)
    })
    printResult(3, total, "JSON.parse medium", iters, Math.floor(iters / 10), r1)
    printResult(4, total, "parsers.json.parse medium", iters, Math.floor(iters / 10), r2)
  }

  {
    const iters = 500
    const r1 = await bench("JSON.parse large (1000 items)", iters, async () => {
      JSON.parse(LARGE_JSON)
    })
    const r2 = await bench("parsers.json.parse large (1000 items)", iters, async () => {
      json.parse(LARGE_JSON)
    })
    printResult(5, total, "JSON.parse large (1000 items)", iters, Math.floor(iters / 10), r1)
    printResult(6, total, "parsers.json.parse large (1000 items)", iters, Math.floor(iters / 10), r2)
  }

  console.log("=== JSON.stringify vs parsers.json.stringify Benchmark ===")
  console.log("")

  const total2 = 6

  {
    const iters = 100000
    const r1 = await bench("JSON.stringify small", iters, async () => {
      JSON.stringify(SMALL_OBJ)
    })
    const r2 = await bench("parsers.json.stringify small", iters, async () => {
      json.stringify(SMALL_OBJ)
    })
    printResult(1, total2, "JSON.stringify small", iters, Math.floor(iters / 10), r1)
    printResult(2, total2, "parsers.json.stringify small", iters, Math.floor(iters / 10), r2)
  }

  {
    const iters = 20000
    const r1 = await bench("JSON.stringify medium", iters, async () => {
      JSON.stringify(MEDIUM_OBJ)
    })
    const r2 = await bench("parsers.json.stringify medium", iters, async () => {
      json.stringify(MEDIUM_OBJ)
    })
    printResult(3, total2, "JSON.stringify medium", iters, Math.floor(iters / 10), r1)
    printResult(4, total2, "parsers.json.stringify medium", iters, Math.floor(iters / 10), r2)
  }

  {
    const iters = 500
    const r1 = await bench("JSON.stringify large (1000 items)", iters, async () => {
      JSON.stringify(LARGE_OBJ)
    })
    const r2 = await bench("parsers.json.stringify large (1000 items)", iters, async () => {
      json.stringify(LARGE_OBJ)
    })
    printResult(5, total2, "JSON.stringify large (1000 items)", iters, Math.floor(iters / 10), r1)
    printResult(6, total2, "parsers.json.stringify large (1000 items)", iters, Math.floor(iters / 10), r2)
  }

  console.log("=== Done ===")
}

main()
