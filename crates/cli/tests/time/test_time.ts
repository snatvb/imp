import { Duration, ImpDate, ImpTime, ImpDateTime, ImpLocalDateTime } from "imp:time"

export {}

// ---------- Duration factories ----------
{
  assert(Duration.zero().asMillis() === 0, "Duration.zero()")
  assert(Duration.seconds(30).asMillis() === 30_000, "Duration.seconds")
  assert(Duration.minutes(2).asSeconds() === 120, "Duration.minutes")
  assert(Duration.hours(1).asMinutes() === 60, "Duration.hours")
  assert(Duration.days(1).asHours() === 24, "Duration.days")
  assert(Duration.weeks(1).asDays() === 7, "Duration.weeks")
  assert(Duration.millis(500).asMillis() === 500, "Duration.millis")
  assert(Duration.micros(1000).asMillis() === 1, "Duration.micros")
  assert(Duration.nanos(1_000_000).asMillis() === 1, "Duration.nanos")

  // negative durations
  assert(Duration.seconds(-30).asSeconds() === -30, "negative seconds")
  assert(Duration.seconds(-1).isNegative() === true, "isNegative")
  assert(Duration.zero().isZero() === true, "isZero")
  assert(Duration.seconds(0.5).asMillis() === 500, "fractional seconds")
}

// ---------- Duration arithmetic ----------
{
  const a = Duration.seconds(30)
  const b = Duration.millis(500)
  assert(a.add(b).asMillis() === 30_500, "add")
  assert(a.sub(b).asMillis() === 29_500, "sub")
  assert(a.mul(2).asSeconds() === 60, "mul")
  assert(a.neg().asSeconds() === -30, "neg")
  assert(Duration.seconds(-5).abs().asSeconds() === 5, "abs")
}

// ---------- Duration compare ----------
{
  const a = Duration.seconds(30)
  const b = Duration.seconds(60)
  assert(a.lt(b) === true, "lt")
  assert(b.gt(a) === true, "gt")
  assert(a.lte(a) === true, "lte self")
  assert(a.gte(a) === true, "gte self")
  assert(a.eq(Duration.seconds(30)) === true, "eq")
  assert(a.eq(b) === false, "neq")
}

// ---------- Duration type-safety ----------
{
  const d = Duration.seconds(30)
  // Calling methods with non-Duration must throw (not silently coerce)
  let threw = false
  try {
    d.add(5 as unknown as Duration)
  } catch {
    threw = true
  }
  assert(threw === true, "add(non-Duration) must throw TypeError")

  threw = false
  try {
    d.eq(5 as unknown as Duration)
  } catch {
    threw = true
  }
  assert(threw === true, "eq(non-Duration) must throw")

  // Numbers do NOT auto-coerce — Duration is opaque
  threw = false
  try {
    Duration.seconds("30" as unknown as number)
  } catch {
    threw = true
  }
  assert(threw === true, "factory(string) must throw")
}

// ---------- Human parse ----------
{
  assert(Duration.parse("0").asMillis() === 0, "parse 0")
  assert(Duration.parse("0s").asMillis() === 0, "parse 0s")
  assert(Duration.parse("1h 30m").asMinutes() === 90, "parse 1h 30m")
  assert(Duration.parse("1d 12h 30m 15s").asSeconds() === 131_415, "parse complex")
  assert(Duration.parse("1.5h").asMinutes() === 90, "parse fractional")
  assert(Duration.parse("-500ms").asMillis() === -500, "parse negative")
  assert(Duration.parse("30s").asSeconds() === 30, "parse no space")

  // Failures
  let threw = false
  try {
    Duration.parse("")
  } catch {
    threw = true
  }
  assert(threw, "parse('') must throw")

  threw = false
  try {
    Duration.parse("30")
  } catch {
    threw = true
  }
  assert(threw, "parse('30') must throw (no unit)")

  threw = false
  try {
    Duration.parse("30xs")
  } catch {
    threw = true
  }
  assert(threw, "parse('30xs') must throw (bad unit)")

  threw = false
  try {
    Duration.parse("abc")
  } catch {
    threw = true
  }
  assert(threw, "parse('abc') must throw (bad number)")
}

// ---------- setTimeout accepts both f64 and Duration ----------
{
  // Number ms (existing behavior)
  const id1 = setTimeout(() => {}, 10)
  assert(typeof id1 === "number", "setTimeout(id, number)")
  clearTimeout(id1)

  // Duration object
  const id2 = setTimeout(() => {}, Duration.millis(20))
  assert(typeof id2 === "number", "setTimeout(id, Duration)")
  clearTimeout(id2)

  // String must throw (no auto-parse)
  let threw = false
  try {
    setTimeout(() => {}, "100" as unknown as number)
  } catch {
    threw = true
  }
  assert(threw, "setTimeout(id, string) must throw")

  // Negative must throw
  threw = false
  try {
    setTimeout(() => {}, -1)
  } catch {
    threw = true
  }
  assert(threw, "setTimeout(id, negative) must throw")
}

// ---------- setInterval accepts both ----------
{
  const id1 = setInterval(() => {}, 1000)
  assert(typeof id1 === "number", "setInterval(id, number)")
  clearInterval(id1)

  const id2 = setInterval(() => {}, Duration.seconds(1))
  assert(typeof id2 === "number", "setInterval(id, Duration)")
  clearInterval(id2)
}

// ---------- ImpDate ----------
{
  const d = ImpDate.fromYmd(2025, 6, 19)
  assert(d.getYear() === 2025, "ImpDate.getYear")
  assert(d.getMonth() === 6, "ImpDate.getMonth")
  assert(d.getDay() === 19, "ImpDate.getDay")
  assert(typeof d.getDayOfWeek() === "number", "ImpDate.getDayOfWeek")
  assert(typeof d.getDayOfYear() === "number", "ImpDate.getDayOfYear")
  assert(d.toIso() === "2025-06-19", "ImpDate.toIso")

  const parsed = ImpDate.fromIso("2025-01-01")
  assert(parsed.getYear() === 2025 && parsed.getMonth() === 1, "ImpDate.fromIso")

  // arithmetic
  const tomorrow = d.addDays(Duration.days(1))
  assert(tomorrow.getDay() === 20, "ImpDate.addDays")

  const lastMonth = d.addMonths(-1)
  assert(lastMonth.getMonth() === 5, "ImpDate.addMonths negative")

  const nextYear = d.addYears(1)
  assert(nextYear.getYear() === 2026, "ImpDate.addYears")

  // daysBetween
  const diff = d.daysBetween(ImpDate.fromYmd(2025, 6, 25))
  assert(diff.asDays() === 6, "ImpDate.daysBetween")

  // Invalid date
  let threw = false
  try {
    ImpDate.fromYmd(2025, 13, 1)
  } catch {
    threw = true
  }
  assert(threw, "fromYmd(2025, 13, 1) must throw")

  // equals
  assert(d.equals(ImpDate.fromYmd(2025, 6, 19)) === true, "ImpDate.equals")
  assert(d.equals(ImpDate.fromYmd(2025, 6, 20)) === false, "ImpDate.notEquals")

  // toJsDate
  const jsd = d.toJsDate()
  assert(typeof jsd.getTime === "function", "ImpDate.toJsDate")
  assert(jsd.getUTCFullYear() === 2025, "toJsDate year")
}

// ---------- ImpTime ----------
{
  const t = ImpTime.fromHms(14, 30, 45)
  assert(t.getHour() === 14, "ImpTime.getHour")
  assert(t.getMinute() === 30, "ImpTime.getMinute")
  assert(t.getSecond() === 45, "ImpTime.getSecond")
  assert(t.toIso() === "14:30:45", "ImpTime.toIso")

  const t2 = t.add(Duration.minutes(15))
  assert(t2.getMinute() === 45, "ImpTime.add")

  const t3 = ImpTime.fromHmsNano(1, 2, 3, 123_456_789)
  assert(t3.getNano() === 123_456_789, "ImpTime.fromHmsNano")

  // Invalid time
  let threw = false
  try {
    ImpTime.fromHms(25, 0, 0)
  } catch {
    threw = true
  }
  assert(threw, "ImpTime.fromHms(25,0,0) must throw")
}

// ---------- ImpDateTime (UTC) ----------
{
  const now = ImpDateTime.now()
  assert(typeof now.getYear() === "number", "ImpDateTime.now().getYear")

  const dt = ImpDateTime.fromTimestamp(0)
  assert(dt.getYear() === 1970, "fromTimestamp(0) year")
  assert(dt.getMonth() === 1, "fromTimestamp(0) month")
  assert(dt.getDay() === 1, "fromTimestamp(0) day")
  assert(dt.getHour() === 0, "fromTimestamp(0) hour")
  assert(dt.toIso() === "1970-01-01T00:00:00+00:00", "fromTimestamp(0) iso")

  const parsed = ImpDateTime.fromIso("2025-01-01T00:00:00Z")
  assert(parsed.getYear() === 2025, "fromIso")

  // Arithmetic
  const future = dt.add(Duration.hours(1))
  assert(future.getHour() === 1, "add hours")

  // diff
  const later = ImpDateTime.fromTimestamp(60_000) // 1 min later
  const d = later.diff(dt)
  assert(d.asMinutes() === 1, "diff 1 min")

  // format
  assert(dt.format("%Y") === "1970", "format %Y")
  assert(dt.format("%Y-%m-%d") === "1970-01-01", "format %Y-%m-%d")

  // equals
  assert(dt.equals(ImpDateTime.fromTimestamp(0)) === true, "equals")

  // toJsDate
  const jsd = dt.toJsDate()
  assert(typeof jsd.getTime === "function", "toJsDate")
}

// ---------- ImpLocalDateTime ----------
{
  const now = ImpLocalDateTime.nowLocal()
  assert(typeof now.getYear() === "number", "ImpLocalDateTime.nowLocal")

  const utc = now.toUtc()
  assert(typeof utc.getYear() === "number", "toUtc")

  // round trip
  const back = ImpLocalDateTime.fromTimestamp(0)
  assert(typeof back.getYear() === "number", "fromTimestamp")

  // diff
  const a = ImpLocalDateTime.fromTimestamp(0)
  const b = ImpLocalDateTime.fromTimestamp(3_600_000)
  const d = b.diff(a)
  assert(d.asHours() === 1, "ImpLocalDateTime diff 1h")

  // format
  assert(back.format("%Y") === "1970", "ImpLocalDateTime format")
}

// ---------- ImpDate.addWeeks ----------
{
  const d = ImpDate.fromYmd(2025, 6, 19)
  const twoWeeksLater = d.addWeeks(Duration.weeks(2))
  assert(twoWeeksLater.getMonth() === 7, "addWeeks crosses month")
  assert(twoWeeksLater.getDay() === 3, "addWeeks correct day")

  const oneWeekBack = d.addWeeks(Duration.weeks(-1))
  assert(oneWeekBack.getDay() === 12, "addWeeks negative")

  // addWeeks also accepts fractional via Duration
  const halfWeek = d.addWeeks(Duration.days(3))
  assert(halfWeek.getDay() === 22, "addWeeks via days Duration")
}

// ---------- AbortSignal.timeout ----------
{
  const sig1 = AbortSignal.timeout(20)
  assert(sig1.aborted === false, "AbortSignal.timeout(number): initially not aborted")
  assert(sig1.reason === "", "AbortSignal.timeout(number): empty reason initially")

  const sig2 = AbortSignal.timeout(Duration.millis(30))
  assert(sig2.aborted === false, "AbortSignal.timeout(Duration): initially not aborted")
  assert(sig2.reason === "", "AbortSignal.timeout(Duration): empty reason initially")

  await new Promise<void>((resolve) => setTimeout(resolve, 100))
  const sig1_post = sig1
  assert(sig1_post.aborted === true, "AbortSignal.timeout(number): fired after delay")
  assert(sig1_post.reason === "The operation timed out", "AbortSignal.timeout(number): reason")
  const sig2_post = sig2
  assert(sig2_post.aborted === true, "AbortSignal.timeout(Duration): fired after delay")
  assert(sig2_post.reason === "The operation timed out", "AbortSignal.timeout(Duration): reason")

  let threw = false
  try {
    AbortSignal.timeout("100" as unknown as number)
  } catch {
    threw = true
  }
  assert(threw, "AbortSignal.timeout(string) must throw")

  threw = false
  try {
    AbortSignal.timeout(-1)
  } catch {
    threw = true
  }
  assert(threw, "AbortSignal.timeout(negative) must throw")
}

console.log("ALL TIME TESTS PASSED")
