import { Duration, ImpDate, ImpTime, ImpDateTime, ImpLocalDateTime } from "imp:time";

// ---------- Duration factories ----------
{
  console.assert(Duration.zero().asMillis() === 0, "Duration.zero()");
  console.assert(Duration.seconds(30).asMillis() === 30_000, "Duration.seconds");
  console.assert(Duration.minutes(2).asSeconds() === 120, "Duration.minutes");
  console.assert(Duration.hours(1).asMinutes() === 60, "Duration.hours");
  console.assert(Duration.days(1).asHours() === 24, "Duration.days");
  console.assert(Duration.weeks(1).asDays() === 7, "Duration.weeks");
  console.assert(Duration.millis(500).asMillis() === 500, "Duration.millis");
  console.assert(Duration.micros(1000).asMillis() === 1, "Duration.micros");
  console.assert(Duration.nanos(1_000_000).asMillis() === 1, "Duration.nanos");

  // negative durations
  console.assert(Duration.seconds(-30).asSeconds() === -30, "negative seconds");
  console.assert(Duration.seconds(-1).isNegative() === true, "isNegative");
  console.assert(Duration.zero().isZero() === true, "isZero");
  console.assert(Duration.seconds(0.5).asMillis() === 500, "fractional seconds");
}

// ---------- Duration arithmetic ----------
{
  const a = Duration.seconds(30);
  const b = Duration.millis(500);
  console.assert(a.add(b).asMillis() === 30_500, "add");
  console.assert(a.sub(b).asMillis() === 29_500, "sub");
  console.assert(a.mul(2).asSeconds() === 60, "mul");
  console.assert(a.neg().asSeconds() === -30, "neg");
  console.assert(Duration.seconds(-5).abs().asSeconds() === 5, "abs");
}

// ---------- Duration compare ----------
{
  const a = Duration.seconds(30);
  const b = Duration.seconds(60);
  console.assert(a.lt(b) === true, "lt");
  console.assert(b.gt(a) === true, "gt");
  console.assert(a.lte(a) === true, "lte self");
  console.assert(a.gte(a) === true, "gte self");
  console.assert(a.eq(Duration.seconds(30)) === true, "eq");
  console.assert(a.eq(b) === false, "neq");
}

// ---------- Duration type-safety ----------
{
  const d = Duration.seconds(30);
  // Calling methods with non-Duration must throw (not silently coerce)
  let threw = false;
  try {
    d.add(5 as unknown as Duration);
  } catch {
    threw = true;
  }
  console.assert(threw === true, "add(non-Duration) must throw TypeError");

  threw = false;
  try {
    d.eq(5 as unknown as Duration);
  } catch {
    threw = true;
  }
  console.assert(threw === true, "eq(non-Duration) must throw");

  // Numbers do NOT auto-coerce — Duration is opaque
  threw = false;
  try {
    Duration.seconds("30" as unknown as number);
  } catch {
    threw = true;
  }
  console.assert(threw === true, "factory(string) must throw");
}

// ---------- Human parse ----------
{
  console.assert(Duration.parse("0").asMillis() === 0, "parse 0");
  console.assert(Duration.parse("0s").asMillis() === 0, "parse 0s");
  console.assert(Duration.parse("1h 30m").asMinutes() === 90, "parse 1h 30m");
  console.assert(Duration.parse("1d 12h 30m 15s").asSeconds() === 131_415, "parse complex");
  console.assert(Duration.parse("1.5h").asMinutes() === 90, "parse fractional");
  console.assert(Duration.parse("-500ms").asMillis() === -500, "parse negative");
  console.assert(Duration.parse("30s").asSeconds() === 30, "parse no space");

  // Failures
  let threw = false;
  try { Duration.parse(""); } catch { threw = true; }
  console.assert(threw, "parse('') must throw");

  threw = false;
  try { Duration.parse("30"); } catch { threw = true; }
  console.assert(threw, "parse('30') must throw (no unit)");

  threw = false;
  try { Duration.parse("30xs"); } catch { threw = true; }
  console.assert(threw, "parse('30xs') must throw (bad unit)");

  threw = false;
  try { Duration.parse("abc"); } catch { threw = true; }
  console.assert(threw, "parse('abc') must throw (bad number)");
}

// ---------- setTimeout accepts both f64 and Duration ----------
{
  // Number ms (existing behavior)
  const id1 = setTimeout(() => {}, 10);
  console.assert(typeof id1 === "number", "setTimeout(id, number)");
  clearTimeout(id1);

  // Duration object
  const id2 = setTimeout(() => {}, Duration.millis(20));
  console.assert(typeof id2 === "number", "setTimeout(id, Duration)");
  clearTimeout(id2);

  // String must throw (no auto-parse)
  let threw = false;
  try {
    setTimeout(() => {}, "100" as unknown as number);
  } catch {
    threw = true;
  }
  console.assert(threw, "setTimeout(id, string) must throw");

  // Negative must throw
  threw = false;
  try {
    setTimeout(() => {}, -1);
  } catch {
    threw = true;
  }
  console.assert(threw, "setTimeout(id, negative) must throw");
}

// ---------- setInterval accepts both ----------
{
  const id1 = setInterval(() => {}, 1000);
  console.assert(typeof id1 === "number", "setInterval(id, number)");
  clearInterval(id1);

  const id2 = setInterval(() => {}, Duration.seconds(1));
  console.assert(typeof id2 === "number", "setInterval(id, Duration)");
  clearInterval(id2);
}

// ---------- ImpDate ----------
{
  const d = ImpDate.fromYmd(2025, 6, 19);
  console.assert(d.getYear() === 2025, "ImpDate.getYear");
  console.assert(d.getMonth() === 6, "ImpDate.getMonth");
  console.assert(d.getDay() === 19, "ImpDate.getDay");
  console.assert(typeof d.getDayOfWeek() === "number", "ImpDate.getDayOfWeek");
  console.assert(typeof d.getDayOfYear() === "number", "ImpDate.getDayOfYear");
  console.assert(d.toIso() === "2025-06-19", "ImpDate.toIso");

  const parsed = ImpDate.fromIso("2025-01-01");
  console.assert(parsed.getYear() === 2025 && parsed.getMonth() === 1, "ImpDate.fromIso");

  // arithmetic
  const tomorrow = d.addDays(Duration.days(1));
  console.assert(tomorrow.getDay() === 20, "ImpDate.addDays");

  const lastMonth = d.addMonths(-1);
  console.assert(lastMonth.getMonth() === 5, "ImpDate.addMonths negative");

  const nextYear = d.addYears(1);
  console.assert(nextYear.getYear() === 2026, "ImpDate.addYears");

  // daysBetween
  const diff = d.daysBetween(ImpDate.fromYmd(2025, 6, 25));
  console.assert(diff.asDays() === 6, "ImpDate.daysBetween");

  // Invalid date
  let threw = false;
  try { ImpDate.fromYmd(2025, 13, 1); } catch { threw = true; }
  console.assert(threw, "fromYmd(2025, 13, 1) must throw");

  // equals
  console.assert(d.equals(ImpDate.fromYmd(2025, 6, 19)) === true, "ImpDate.equals");
  console.assert(d.equals(ImpDate.fromYmd(2025, 6, 20)) === false, "ImpDate.notEquals");

  // toJsDate
  const jsd = d.toJsDate();
  console.assert(typeof jsd.getTime === "function", "ImpDate.toJsDate");
  console.assert(jsd.getUTCFullYear() === 2025, "toJsDate year");
}

// ---------- ImpTime ----------
{
  const t = ImpTime.fromHms(14, 30, 45);
  console.assert(t.getHour() === 14, "ImpTime.getHour");
  console.assert(t.getMinute() === 30, "ImpTime.getMinute");
  console.assert(t.getSecond() === 45, "ImpTime.getSecond");
  console.assert(t.toIso() === "14:30:45", "ImpTime.toIso");

  const t2 = t.add(Duration.minutes(15));
  console.assert(t2.getMinute() === 45, "ImpTime.add");

  const t3 = ImpTime.fromHmsNano(1, 2, 3, 123_456_789);
  console.assert(t3.getNano() === 123_456_789, "ImpTime.fromHmsNano");

  // Invalid time
  let threw = false;
  try { ImpTime.fromHms(25, 0, 0); } catch { threw = true; }
  console.assert(threw, "ImpTime.fromHms(25,0,0) must throw");
}

// ---------- ImpDateTime (UTC) ----------
{
  const now = ImpDateTime.now();
  console.assert(typeof now.getYear() === "number", "ImpDateTime.now().getYear");

  const dt = ImpDateTime.fromTimestamp(0);
  console.assert(dt.getYear() === 1970, "fromTimestamp(0) year");
  console.assert(dt.getMonth() === 1, "fromTimestamp(0) month");
  console.assert(dt.getDay() === 1, "fromTimestamp(0) day");
  console.assert(dt.getHour() === 0, "fromTimestamp(0) hour");
  console.assert(dt.toIso() === "1970-01-01T00:00:00+00:00", "fromTimestamp(0) iso");

  const parsed = ImpDateTime.fromIso("2025-01-01T00:00:00Z");
  console.assert(parsed.getYear() === 2025, "fromIso");

  // Arithmetic
  const future = dt.add(Duration.hours(1));
  console.assert(future.getHour() === 1, "add hours");

  // diff
  const later = ImpDateTime.fromTimestamp(60_000); // 1 min later
  const d = later.diff(dt);
  console.assert(d.asMinutes() === 1, "diff 1 min");

  // format
  console.assert(dt.format("%Y") === "1970", "format %Y");
  console.assert(dt.format("%Y-%m-%d") === "1970-01-01", "format %Y-%m-%d");

  // equals
  console.assert(dt.equals(ImpDateTime.fromTimestamp(0)) === true, "equals");

  // toJsDate
  const jsd = dt.toJsDate();
  console.assert(typeof jsd.getTime === "function", "toJsDate");
}

// ---------- ImpLocalDateTime ----------
{
  const now = ImpLocalDateTime.nowLocal();
  console.assert(typeof now.getYear() === "number", "ImpLocalDateTime.nowLocal");

  const utc = now.toUtc();
  console.assert(typeof utc.getYear() === "number", "toUtc");

  // round trip
  const back = ImpLocalDateTime.fromTimestamp(0);
  console.assert(typeof back.getYear() === "number", "fromTimestamp");

  // diff
  const a = ImpLocalDateTime.fromTimestamp(0);
  const b = ImpLocalDateTime.fromTimestamp(3_600_000);
  const d = b.diff(a);
  console.assert(d.asHours() === 1, "ImpLocalDateTime diff 1h");

  // format
  console.assert(back.format("%Y") === "1970", "ImpLocalDateTime format");
}

console.log("ALL TIME TESTS PASSED");
