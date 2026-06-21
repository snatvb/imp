export {}

console.log("start")

await new Promise<void>((resolve) => setTimeout(resolve, 30))
console.log("after first await")

const dt = ImpDateTime.fromTimestamp(0)
await new Promise<void>((resolve) => setTimeout(resolve, 10))
console.log("after second await, dt:", dt.getYear())

const sig = AbortSignal.timeout(5)
await new Promise<void>((resolve) => setTimeout(resolve, 50))
assert(sig.aborted === true, "top-level await + AbortSignal.timeout")

const d = Duration.seconds(2)
await new Promise<void>((resolve) => setTimeout(resolve, 10))
assert(d.asSeconds() === 2, "Duration survives across top-level awaits")

const d2 = Duration.parse("1h 30m")
assert(d2.asMinutes() === 90, "Duration.parse works at top level")

console.log("ALL TOP LEVEL AWAIT TESTS PASSED")
