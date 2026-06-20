# log-tail

Tail a log file like `tail -f`. Uses `FileHandle` for buffered streaming and
`setInterval` to poll for new data.

## What it shows

- `imp:fs` — `open` (FileHandle), `exists`
- `setInterval` / `clearInterval` — event loop
- `process.exit(0)` for clean shutdown
- `Duration` / millisecond timing
- Top-level `await`

## Run

```bash
imp run main.ts
```

The script tails `fixtures/sample.log`. Since it's a static file, the script
prints the full content and then exits after 3 seconds. For a real demo,
point it at a growing log file:

```bash
# In one terminal:
imp run main.ts /var/log/myapp.log

# In another:
echo "2025-01-15 10:40:00 INFO  New event" >> /var/log/myapp.log
```

The script polls every 200ms and prints new lines as they appear.

## Expected output

```
Tailing fixtures/sample.log (Ctrl+C to stop)...

10:23:45  [INFO]  Server starting on port 8080
10:23:46  [INFO]  Connected to database
10:23:47  [INFO]  Loaded 142 modules
10:24:12  [INFO]  GET /api/users 200 23ms
10:24:23  [WARN]  Slow query detected: SELECT * FROM orders (340ms)
10:26:11  [ERROR] Failed to send email: SMTP timeout
...

(reached end of fixture file, stopping)
```

## How it works

```ts
const fh = await open(logPath, 4096) // buffered reader, 4KB chunks

const interval = setInterval(async () => {
  const chunk = await fh.read() // reads next 4KB or returns undefined at EOF
  if (!chunk) return
  buffer += chunk.toString()
  // ... split on newlines, emit, keep partial line in buffer
}, 200)
```

The pattern is the same as in Node.js, but with `imp:fs` returning a
disposable `FileHandle` (`using fh = await open(...)` also works).
