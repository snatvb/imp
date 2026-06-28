import { resolve, basename } from "path"

import { readFile, writeFile } from "imp:fs"

const input = import.meta.dirname + "/fixtures/sample.md"

function escape(s: JsString): JsString {
  return s.replace(/&/g, "&amp;").replace(/</g, "&lt;").replace(/>/g, "&gt;")
}

function inline(s: JsString): JsString {
  return escape(s)
    .replace(/`([^`]+)`/g, "<code>$1</code>")
    .replace(/\*\*([^*]+)\*\*/g, "<strong>$1</strong>")
    .replace(/\*([^*]+)\*/g, "<em>$1</em>")
    .replace(/\[([^\]]+)\]\(([^)]+)\)/g, '<a href="$2">$1</a>')
}

function mdToHtml(md: JsString): string {
  const lines = md.split(/\r?\n/)
  const out: string[] = []
  let inCode = false
  let codeBuf: string[] = []
  let inList = false
  let listType: "ul" | "ol" | null = null

  const closeList = () => {
    if (inList && listType) {
      out.push(`</${listType}>`)
      inList = false
      listType = null
    }
  }

  for (const line of lines) {
    if (line.startsWith("```")) {
      closeList()
      if (inCode) {
        out.push(`<pre><code>${escape(codeBuf.join("\n"))}</code></pre>`)
        inCode = false
        codeBuf = []
      } else {
        inCode = true
      }
      continue
    }
    if (inCode) {
      codeBuf.push(line)
      continue
    }

    if (/^---+$/.test(line)) {
      closeList()
      out.push("<hr>")
      continue
    }

    const h6 = line.match(/^######\s+(.*)$/)
    const h5 = line.match(/^#####\s+(.*)$/)
    const h4 = line.match(/^####\s+(.*)$/)
    const h3 = line.match(/^###\s+(.*)$/)
    const h2 = line.match(/^##\s+(.*)$/)
    const h1 = line.match(/^#\s+(.*)$/)
    if (h6) {
      closeList()
      out.push(`<h6>${inline(h6[1]!)}</h6>`)
      continue
    }
    if (h5) {
      closeList()
      out.push(`<h5>${inline(h5[1]!)}</h5>`)
      continue
    }
    if (h4) {
      closeList()
      out.push(`<h4>${inline(h4[1]!)}</h4>`)
      continue
    }
    if (h3) {
      closeList()
      out.push(`<h3>${inline(h3[1]!)}</h3>`)
      continue
    }
    if (h2) {
      closeList()
      out.push(`<h2>${inline(h2[1]!)}</h2>`)
      continue
    }
    if (h1) {
      closeList()
      out.push(`<h1>${inline(h1[1]!)}</h1>`)
      continue
    }

    const ol = line.match(/^\s*\d+\.\s+(.*)$/)
    const ul = line.match(/^\s*[-*]\s+(.*)$/)
    if (ol) {
      if (!inList || listType !== "ol") {
        closeList()
        out.push("<ol>")
        inList = true
        listType = "ol"
      }
      out.push(`  <li>${inline(ol[1]!)}</li>`)
      continue
    }
    if (ul) {
      if (!inList || listType !== "ul") {
        closeList()
        out.push("<ul>")
        inList = true
        listType = "ul"
      }
      out.push(`  <li>${inline(ul[1]!)}</li>`)
      continue
    }

    if (line.trim() === "") {
      closeList()
      continue
    }

    closeList()
    out.push(`<p>${inline(line)}</p>`)
  }

  closeList()
  if (inCode) out.push(`<pre><code>${escape(codeBuf.join("\n"))}</code></pre>`)

  return [
    "<!DOCTYPE html>",
    "<html>",
    '<head><meta charset="utf-8"><title>Document</title></head>',
    "<body>",
    ...out,
    "</body>",
    "</html>",
  ].join("\n")
}

const md = await readFile(input, "utf8")
const html = mdToHtml(md)
const outPath = resolve(import.meta.dirname, `${basename(input, ".md")}.html`)
await writeFile(outPath, html)
console.log(`Converted ${basename(input)} → ${basename(outPath)} (${html.length} bytes)`)
console.log("")
console.log("--- first 20 lines ---")
console.log(html.split("\n").slice(0, 20).join("\n"))
