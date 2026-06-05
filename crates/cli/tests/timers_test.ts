// setTimeout basic
let timeoutFired = false
setTimeout(() => { timeoutFired = true }, 10)

// setTimeout with 0ms delay
let zeroFired = false
setTimeout(() => { zeroFired = true }, 0)

// clearTimeout before fire
let clearedFired = false
const clearedId = setTimeout(() => { clearedFired = true }, 5)
clearTimeout(clearedId)

// setInterval fires multiple times
let intervalCount = 0
const intervalId = setInterval(() => { intervalCount++ }, 10)

// clearInterval after some ticks
setTimeout(() => { clearInterval(intervalId) }, 50)

// nested setTimeout
let nestedFired = false
setTimeout(() => {
  setTimeout(() => { nestedFired = true }, 5)
}, 5)

// clearTimeout on already fired (should not crash)
const onceId = setTimeout(() => {}, 1)
setTimeout(() => { clearTimeout(onceId) }, 20)

// setInterval with larger interval
let largeIntervalCount = 0
const largeId = setInterval(() => { largeIntervalCount++ }, 30)
setTimeout(() => { clearInterval(largeId) }, 80)

// wait for all timers
setTimeout(() => {
  console.assert(timeoutFired, "setTimeout should fire")
  console.assert(zeroFired, "setTimeout 0ms should fire")
  console.assert(!clearedFired, "cleared timeout should NOT fire")
  console.assert(intervalCount >= 2, `interval should fire >=2 times, got ${intervalCount}`)
  console.assert(nestedFired, "nested setTimeout should fire")
  console.assert(largeIntervalCount >= 1, `large interval should fire >=1 times, got ${largeIntervalCount}`)
  console.assert(largeIntervalCount <= 3, `large interval should fire <=3 times, got ${largeIntervalCount}`)
  console.log("ALL TIMER TESTS PASSED")
}, 100)
