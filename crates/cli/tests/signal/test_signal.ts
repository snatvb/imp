import signal from "imp:signal"

console.log("=== Signal Tests ===")

{
  const pending = signal.pending()
  assert(Array.isArray(pending), "pending() returns array")
  assert(pending.length === 0, "no pending signals initially")
  console.log("Test 1: pending() empty initially OK")
}

{
  const dispose = signal.on("SIGINT", () => {})
  assert(typeof dispose === "function", "on() returns dispose function")
  dispose()
  console.log("Test 2: on() returns dispose OK")
}

{
  const dispose = signal.once("SIGTERM", () => {})
  assert(typeof dispose === "function", "once() returns dispose function")
  dispose()
  console.log("Test 3: once() returns dispose OK")
}

{
  signal.removeAll()
  console.log("Test 4: removeAll() OK")
}

{
  signal.removeAll()
  const before = signal.pending()
  assert(Array.isArray(before), "pending() returns array")
  console.log("Test 5: pending() after removeAll OK")
}

{
  signal.removeAll()
  const d1 = signal.on("SIGINT", () => {})
  const d2 = signal.on("SIGINT", () => {})
  assert(typeof d1 === "function", "first dispose is function")
  assert(typeof d2 === "function", "second dispose is function")
  d1()
  d2()
  console.log("Test 6: multiple handlers on same signal OK")
}

{
  signal.removeAll()
  const d1 = signal.on("SIGINT", () => {})
  const d2 = signal.on("SIGTERM", () => {})
  d1()
  d2()
  console.log("Test 7: handlers on different signals OK")
}

{
  signal.removeAll()
  let called = false
  signal.on("SIGINT", () => {
    called = true
  })
  signal.removeAll("SIGINT")
  console.log("Test 8: removeAll with specific signal OK")
}

{
  signal.removeAll()
  const dispose = signal.on("SIGINT", () => {})
  dispose()
  dispose()
  console.log("Test 9: double dispose is safe OK")
}

signal.removeAll()
await new Promise<void>((r) => setTimeout(r, 50))
console.log("ALL SIGNAL TESTS PASSED")
