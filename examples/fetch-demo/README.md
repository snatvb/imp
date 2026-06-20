# fetch-demo

Single HTTP request with `fetch` + `URL` + JSON parsing. Minimal example of
network I/O in imp.

## What it shows

- `fetch()` global (no import needed)
- `Response.json()` automatic parsing
- `performance.now()` for timing
- `console.assert()` for sanity checks
- Top-level `await`

## Run

```bash
imp run main.ts
```

## Expected output

```
Fetching https://jsonplaceholder.typicode.com/users/1...

User:
  id:       1
  name:     Leanne Graham
  username: Bret
  email:    Sincere@april.biz
  phone:    1-770-736-8031 x56442
  website:  hildegard.org
  company:  Romaguera-Crona
  address:  Gwenborough, Kulas Light

Fetched in ~150ms
```

## Compile to standalone binary

`imp compile` embeds the runtime + your script into a single native binary
that runs without Node.js installed.

```bash
# Windows
imp compile main.ts fetch-user.exe
.\fetch-user.exe

# macOS / Linux
imp compile main.ts fetch-user
./fetch-user
```

The output is a **single ~11MB binary** (runtime + bundled script) — no
Node.js, no `node_modules`, no shared libraries. On Windows, `imp compile
main.ts mytool` also works and auto-appends `.exe`; on macOS and Linux, the
output keeps the exact name you pass.
