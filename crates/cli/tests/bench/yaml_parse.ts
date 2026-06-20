import { yaml } from "imp:parsers"

import { bench, printResult } from "./harness"

const SMALL_YAML = "name: test\nvalue: 42\nactive: true\n"
const MEDIUM_YAML = `users:
  - id: 1
    name: Alice
    email: alice@example.com
    roles:
      - admin
      - user
  - id: 2
    name: Bob
    email: bob@example.com
    roles:
      - user
  - id: 3
    name: Charlie
    email: charlie@example.com
    roles: []
meta:
  total: 3
  page: 1
  perPage: 10
`

function generateLargeYaml(): string {
  const obj = {
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
  return yaml.stringify(obj).toString()
}

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
  console.log("=== YAML.parse Benchmark ===")
  console.log("")

  const LARGE_YAML = generateLargeYaml()

  const total = 6

  {
    const iters = 10000
    const r1 = await bench("yaml.parse small", iters, async () => {
      yaml.parse(SMALL_YAML)
    })
    printResult(1, total, "yaml.parse small", iters, Math.floor(iters / 10), r1)
  }

  {
    const iters = 2000
    const r1 = await bench("yaml.parse medium", iters, async () => {
      yaml.parse(MEDIUM_YAML)
    })
    printResult(2, total, "yaml.parse medium", iters, Math.floor(iters / 10), r1)
  }

  {
    const iters = 100
    const r1 = await bench("yaml.parse large (1000 items)", iters, async () => {
      yaml.parse(LARGE_YAML)
    })
    printResult(3, total, "yaml.parse large (1000 items)", iters, Math.floor(iters / 10), r1)
  }

  console.log("=== YAML.stringify Benchmark ===")
  console.log("")

  {
    const iters = 20000
    const r1 = await bench("yaml.stringify small", iters, async () => {
      yaml.stringify(SMALL_OBJ)
    })
    printResult(4, total, "yaml.stringify small", iters, Math.floor(iters / 10), r1)
  }

  {
    const iters = 5000
    const r1 = await bench("yaml.stringify medium", iters, async () => {
      yaml.stringify(MEDIUM_OBJ)
    })
    printResult(5, total, "yaml.stringify medium", iters, Math.floor(iters / 10), r1)
  }

  {
    const iters = 100
    const r1 = await bench("yaml.stringify large (1000 items)", iters, async () => {
      yaml.stringify(LARGE_OBJ)
    })
    printResult(6, total, "yaml.stringify large (1000 items)", iters, Math.floor(iters / 10), r1)
  }

  console.log("=== Done ===")
}

main()
