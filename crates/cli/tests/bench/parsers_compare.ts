import { json, yaml, xml, toml, ron, csv, msgpack } from "imp:parsers"

import { bench } from "./harness"

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

const SMALL_JSON = JSON.stringify(SMALL_OBJ)
const MEDIUM_JSON = JSON.stringify(MEDIUM_OBJ)
const LARGE_JSON = JSON.stringify(LARGE_OBJ)

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
const LARGE_YAML = yaml.stringify(LARGE_OBJ).toString()

const SMALL_XML = "<root><name>test</name><value>42</value><active>true</active></root>"
const MEDIUM_XML =
  "<root><users><item><id>1</id><name>Alice</name><email>alice@example.com</email><roles><item>admin</item><item>user</item></roles></item><item><id>2</id><name>Bob</name><email>bob@example.com</email><roles><item>user</item></roles></item><item><id>3</id><name>Charlie</name><email>charlie@example.com</email><roles></roles></item></users><meta><total>3</total><page>1</page><perPage>10</perPage></meta></root>"
const LARGE_XML = xml.stringify(LARGE_OBJ, "root").toString()

const SMALL_TOML = 'name = "test"\nvalue = 42\nactive = true\n'
const MEDIUM_TOML = `[[users]]
id = 1
name = "Alice"
email = "alice@example.com"
roles = ["admin", "user"]

[[users]]
id = 2
name = "Bob"
email = "bob@example.com"
roles = ["user"]

[[users]]
id = 3
name = "Charlie"
email = "charlie@example.com"
roles = []

[meta]
total = 3
page = 1
perPage = 10
`
const LARGE_TOML = toml.stringify(LARGE_OBJ).toString()

const SMALL_RON = '(name: "test", value: 42, active: true)'
const MEDIUM_RON =
  '(users: [(id: 1, name: "Alice", email: "alice@example.com", roles: ["admin", "user"]), (id: 2, name: "Bob", email: "bob@example.com", roles: ["user"]), (id: 3, name: "Charlie", email: "charlie@example.com", roles: [])], meta: (total: 3, page: 1, perPage: 10))'
const LARGE_RON = ron.stringify(LARGE_OBJ).toString()

const SMALL_CSV = "name,value,active\ntest,42,true\n"
const MEDIUM_CSV =
  "id,name,email,roles\n1,Alice,alice@example.com,admin;user\n2,Bob,bob@example.com,user\n3,Charlie,charlie@example.com,\n"
const LARGE_CSV = csv.stringify(LARGE_OBJ.items).toString()

const SMALL_MSGPACK_BUF = msgpack.stringify(SMALL_OBJ)
const MEDIUM_MSGPACK_BUF = msgpack.stringify(MEDIUM_OBJ)
const LARGE_MSGPACK_BUF = msgpack.stringify(LARGE_OBJ)

const SMALL_CSV_OBJ = [{ name: "test", value: "42", active: "true" }]
const MEDIUM_CSV_OBJ = [
  { id: "1", name: "Alice", email: "alice@example.com", roles: "admin;user" },
  { id: "2", name: "Bob", email: "bob@example.com", roles: "user" },
  { id: "3", name: "Charlie", email: "charlie@example.com", roles: "" },
]

interface BenchResult {
  avg: number
  ops: number
  elapsed: number
}
interface BenchRow {
  name: string
  parse: BenchResult[]
  stringify: BenchResult[]
}

function pct(baseline: number, value: number): string {
  if (baseline === 0) return "-"
  return ((value / baseline) * 100).toFixed(0) + "%"
}

function fmt(r: BenchResult): string {
  return `${r.avg.toFixed(3)}ms | ${r.ops.toFixed(0)} ops/s`
}

async function main() {
  console.log("=== Parsers Benchmark ===")
  console.log("")

  const iters = {
    parse: { small: 50000, medium: 10000, large: 500 },
    stringify: { small: 100000, medium: 20000, large: 500 },
  }

  const jsonNativeRow: BenchRow = { name: "JSON (native)", parse: [], stringify: [] }
  const jsonImpRsRow: BenchRow = { name: "JSON (imp+RsStr)", parse: [], stringify: [] }
  const jsonImpNativeRow: BenchRow = { name: "JSON (imp+native)", parse: [], stringify: [] }
  const yamlRow: BenchRow = { name: "YAML", parse: [], stringify: [] }
  const xmlRow: BenchRow = { name: "XML", parse: [], stringify: [] }
  const tomlRow: BenchRow = { name: "TOML", parse: [], stringify: [] }
  const ronRow: BenchRow = { name: "RON", parse: [], stringify: [] }
  const csvRow: BenchRow = { name: "CSV", parse: [], stringify: [] }
  const msgpackRow: BenchRow = { name: "MsgPack", parse: [], stringify: [] }

  console.log("--- Parse ---")
  console.log("")

  for (const size of ["small", "medium", "large"] as const) {
    console.log(`[parse ${size}]`)

    const n = iters.parse[size]

    const r1 = await bench(`JSON native parse ${size}`, n, () => {
      JSON.parse(SMALL_JSON)
    })
    jsonNativeRow.parse.push(r1)
    console.log(`  JSON native: ${fmt(r1)}`)

    const r2 = await bench(`JSON imp+RsStr parse ${size}`, n, () => {
      json.parse(SMALL_JSON)
    })
    jsonImpRsRow.parse.push(r2)
    console.log(`  JSON+RsStr:  ${fmt(r2)}`)

    const r2n = await bench(`JSON imp+native parse ${size}`, n, () => {
      json.parse(SMALL_JSON, { nativeStrings: true })
    })
    jsonImpNativeRow.parse.push(r2n)
    console.log(`  JSON+native: ${fmt(r2n)}`)

    const yamlStr = size === "small" ? SMALL_YAML : size === "medium" ? MEDIUM_YAML : LARGE_YAML
    const r3 = await bench(`YAML parse ${size}`, n, () => {
      yaml.parse(yamlStr)
    })
    yamlRow.parse.push(r3)
    console.log(`  YAML:        ${fmt(r3)}`)

    const xmlStr = size === "small" ? SMALL_XML : size === "medium" ? MEDIUM_XML : LARGE_XML
    const r4 = await bench(`XML parse ${size}`, n, () => {
      xml.parse(xmlStr)
    })
    xmlRow.parse.push(r4)
    console.log(`  XML:         ${fmt(r4)}`)

    const tomlStr = size === "small" ? SMALL_TOML : size === "medium" ? MEDIUM_TOML : LARGE_TOML
    const r5 = await bench(`TOML parse ${size}`, n, () => {
      toml.parse(tomlStr)
    })
    tomlRow.parse.push(r5)
    console.log(`  TOML:        ${fmt(r5)}`)

    const ronStr = size === "small" ? SMALL_RON : size === "medium" ? MEDIUM_RON : LARGE_RON
    const r6 = await bench(`RON parse ${size}`, n, () => {
      ron.parse(ronStr)
    })
    ronRow.parse.push(r6)
    console.log(`  RON:         ${fmt(r6)}`)

    const csvStr = size === "small" ? SMALL_CSV : size === "medium" ? MEDIUM_CSV : LARGE_CSV
    const r7 = await bench(`CSV parse ${size}`, n, () => {
      csv.parse(csvStr)
    })
    csvRow.parse.push(r7)
    console.log(`  CSV:         ${fmt(r7)}`)

    const msgpackBuf = size === "small" ? SMALL_MSGPACK_BUF : size === "medium" ? MEDIUM_MSGPACK_BUF : LARGE_MSGPACK_BUF
    const r8 = await bench(`MsgPack parse ${size}`, n, () => {
      msgpack.parse(msgpackBuf)
    })
    msgpackRow.parse.push(r8)
    console.log(`  MsgPack:     ${fmt(r8)}`)

    console.log("")
  }

  console.log("--- Stringify ---")
  console.log("")

  for (const size of ["small", "medium", "large"] as const) {
    console.log(`[stringify ${size}]`)

    const n = iters.stringify[size]

    const obj = size === "small" ? SMALL_OBJ : size === "medium" ? MEDIUM_OBJ : LARGE_OBJ

    const r1 = await bench(`JSON native stringify ${size}`, n, () => {
      JSON.stringify(obj)
    })
    jsonNativeRow.stringify.push(r1)
    console.log(`  JSON native: ${fmt(r1)}`)

    const r2 = await bench(`JSON imp+RsStr stringify ${size}`, n, () => {
      json.stringify(obj)
    })
    jsonImpRsRow.stringify.push(r2)
    console.log(`  JSON+RsStr:  ${fmt(r2)}`)

    const r2n = await bench(`JSON imp+native stringify ${size}`, n, () => {
      json.stringify(obj)
    })
    jsonImpNativeRow.stringify.push(r2n)
    console.log(`  JSON+native: ${fmt(r2n)}`)

    const r3 = await bench(`YAML stringify ${size}`, n, () => {
      yaml.stringify(obj)
    })
    yamlRow.stringify.push(r3)
    console.log(`  YAML:        ${fmt(r3)}`)

    const r4 = await bench(`XML stringify ${size}`, n, () => {
      xml.stringify(obj, "root")
    })
    xmlRow.stringify.push(r4)
    console.log(`  XML:         ${fmt(r4)}`)

    const r5 = await bench(`TOML stringify ${size}`, n, () => {
      toml.stringify(obj)
    })
    tomlRow.stringify.push(r5)
    console.log(`  TOML:        ${fmt(r5)}`)

    const r6 = await bench(`RON stringify ${size}`, n, () => {
      ron.stringify(obj)
    })
    ronRow.stringify.push(r6)
    console.log(`  RON:         ${fmt(r6)}`)

    const csvObj = size === "small" ? SMALL_CSV_OBJ : size === "medium" ? MEDIUM_CSV_OBJ : LARGE_OBJ.items
    const r7 = await bench(`CSV stringify ${size}`, n, () => {
      csv.stringify(csvObj)
    })
    csvRow.stringify.push(r7)
    console.log(`  CSV:         ${fmt(r7)}`)

    const r8 = await bench(`MsgPack stringify ${size}`, n, () => {
      msgpack.stringify(obj)
    })
    msgpackRow.stringify.push(r8)
    console.log(`  MsgPack:     ${fmt(r8)}`)

    console.log("")
  }

  const allRows = [jsonNativeRow, jsonImpRsRow, jsonImpNativeRow, yamlRow, xmlRow, tomlRow, ronRow, csvRow, msgpackRow]
  const sizes = ["small", "medium", "large"] as const

  console.log("=== Summary Table ===")
  console.log("")
  console.log("--- Parse (ops/s, relative to JSON native) ---")
  console.log("")
  console.log("Parser        | Small       | Medium      | Large")
  console.log("--------------|-------------|-------------|-------------")
  for (const row of allRows) {
    const cells = sizes.map((s, i) => {
      const ops = row.parse[i].ops.toFixed(0).padStart(7)
      const pctStr = pct(jsonNativeRow.parse[i].ops, row.parse[i].ops).padStart(4)
      return `${ops} ${pctStr}`
    })
    console.log(`${row.name.padEnd(14)}| ${cells.join(" | ")}`)
  }

  console.log("")
  console.log("--- Stringify (ops/s, relative to JSON native) ---")
  console.log("")
  console.log("Parser        | Small       | Medium      | Large")
  console.log("--------------|-------------|-------------|-------------")
  for (const row of allRows) {
    const cells = sizes.map((s, i) => {
      const ops = row.stringify[i].ops.toFixed(0).padStart(7)
      const pctStr = pct(jsonNativeRow.stringify[i].ops, row.stringify[i].ops).padStart(4)
      return `${ops} ${pctStr}`
    })
    console.log(`${row.name.padEnd(14)}| ${cells.join(" | ")}`)
  }

  console.log("")
  console.log("=== Done ===")
}

main()
