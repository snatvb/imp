//! Human-friendly duration parser.
//!
//! Format: `<number><unit>` tokens separated by whitespace, optional leading `-`.
//! Units: `ns`, `us` (or `μs`), `ms`, `s`, `m` (or `min`), `h`, `d`, `w`.
//!
//! Examples:
//! - `"1h 30m"` → 1.5h
//! - `"-500ms"` → -500ms
//! - `"1d 12h 30m 15s"` → large duration
//! - `"0"` → zero

use chrono::Duration;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParseUnit {
    Nanoseconds,
    Microseconds,
    Milliseconds,
    Seconds,
    Minutes,
    Hours,
    Days,
    Weeks,
}

impl ParseUnit {
    pub fn from_token(s: &str) -> Option<Self> {
        match s {
            "ns" => Some(Self::Nanoseconds),
            "us" | "μs" => Some(Self::Microseconds),
            "ms" => Some(Self::Milliseconds),
            "s" => Some(Self::Seconds),
            "m" | "min" => Some(Self::Minutes),
            "h" => Some(Self::Hours),
            "d" => Some(Self::Days),
            "w" => Some(Self::Weeks),
            _ => None,
        }
    }

    pub fn to_chrono(self) -> i64 {
        match self {
            Self::Nanoseconds => 1,
            Self::Microseconds => 1_000,
            Self::Milliseconds => 1_000_000,
            Self::Seconds => 1_000_000_000,
            Self::Minutes => 60 * 1_000_000_000,
            Self::Hours => 3600 * 1_000_000_000,
            Self::Days => 86_400 * 1_000_000_000,
            Self::Weeks => 604_800 * 1_000_000_000,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParseError {
    Empty,
    BadNumber { pos: usize, found: String },
    BadUnit { pos: usize, found: String },
    MissingUnit { pos: usize, number: String },
    TrailingGarbage { pos: usize, found: char },
    Overflow,
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Empty => write!(f, "empty input"),
            Self::BadNumber { pos, found } => {
                write!(f, "invalid number at position {pos}: {found:?}")
            }
            Self::BadUnit { pos, found } => {
                write!(f, "invalid unit at position {pos}: {found:?}")
            }
            Self::MissingUnit { pos, number } => {
                write!(f, "missing unit after number {number:?} at position {pos}")
            }
            Self::TrailingGarbage { pos, found } => {
                write!(f, "unexpected character {found:?} at position {pos}")
            }
            Self::Overflow => write!(f, "duration overflow"),
        }
    }
}

impl std::error::Error for ParseError {}

/// Parse a human-friendly duration string.
///
/// Accepts both integer (`30s`) and fractional (`1.5h`) numbers. Numbers are
/// collected across whitespace and glued to the next unit. Whitespace between
/// number+unit and between tokens is optional (`30s` and `30 s` both work).
pub fn parse(input: &str) -> Result<Duration, ParseError> {
    let s = input.trim();
    if s.is_empty() {
        return Err(ParseError::Empty);
    }

    let bytes = s.as_bytes();
    let mut i = 0;
    let mut total_nanos: i64 = 0;
    let mut negative = false;

    // Optional leading sign
    if bytes[i] == b'-' {
        negative = true;
        i += 1;
    } else if bytes[i] == b'+' {
        i += 1;
    }

    if i >= bytes.len() {
        return Err(ParseError::MissingUnit {
            pos: 0,
            number: String::new(),
        });
    }

    let mut any = false;
    while i < bytes.len() {
        // skip whitespace
        while i < bytes.len() && bytes[i].is_ascii_whitespace() {
            i += 1;
        }
        if i >= bytes.len() {
            break;
        }

        // parse number (integer or decimal)
        let num_start = i;
        let mut has_dot = false;
        let mut num_end = i;
        while num_end < bytes.len() {
            let c = bytes[num_end];
            if c.is_ascii_digit() {
                num_end += 1;
            } else if c == b'.' && !has_dot {
                has_dot = true;
                num_end += 1;
            } else {
                break;
            }
        }
        if num_end == num_start {
            return Err(ParseError::BadNumber {
                pos: num_start,
                found: (bytes[num_start] as char).to_string(),
            });
        }
        let number_str = &s[num_start..num_end];
        i = num_end;

        // optional whitespace between number and unit
        while i < bytes.len() && bytes[i].is_ascii_whitespace() {
            i += 1;
        }

        // unit — alpha chars
        if i >= bytes.len() {
            // No unit after the number — accept bare zero (e.g. "0", "0.0").
            if let Ok(f) = number_str.parse::<f64>()
                && f == 0.0
            {
                any = true;
                // skip adding to total — zero contribution
                continue;
            }
            return Err(ParseError::MissingUnit {
                pos: num_start,
                number: number_str.to_string(),
            });
        }

        let unit_start = i;
        let mut unit_end = i;
        while unit_end < bytes.len() {
            let c = bytes[unit_end];
            // allow Greek mu (UTF-8 multi-byte)
            if c.is_ascii_alphabetic() || (c >= 0x80) {
                unit_end += 1;
            } else {
                break;
            }
        }
        if unit_end == unit_start {
            return Err(ParseError::BadUnit {
                pos: unit_start,
                found: (bytes[unit_start] as char).to_string(),
            });
        }
        let unit_str = &s[unit_start..unit_end];
        let unit = ParseUnit::from_token(unit_str).ok_or_else(|| ParseError::BadUnit {
            pos: unit_start,
            found: unit_str.to_string(),
        })?;

        i = unit_end;
        any = true;

        // Compute nanoseconds for this token.
        // Use integer math when possible; fall back to float for fractional.
        let nanos = if has_dot {
            let f: f64 = number_str.parse().map_err(|_| ParseError::BadNumber {
                pos: num_start,
                found: number_str.to_string(),
            })?;
            if !f.is_finite() {
                return Err(ParseError::Overflow);
            }
            (f * unit.to_chrono() as f64) as i64
        } else {
            let n: i64 = number_str.parse().map_err(|_| ParseError::BadNumber {
                pos: num_start,
                found: number_str.to_string(),
            })?;
            n.checked_mul(unit.to_chrono())
                .ok_or(ParseError::Overflow)?
        };

        total_nanos = total_nanos.checked_add(nanos).ok_or(ParseError::Overflow)?;
    }

    if !any {
        return Err(ParseError::Empty);
    }

    if negative {
        total_nanos = -total_nanos;
    }

    if total_nanos < 0 {
        // chrono::Duration::from_std only models positive; build manually for negatives
        Ok(Duration::nanoseconds(total_nanos))
    } else {
        Duration::from_std(std::time::Duration::from_nanos(total_nanos as u64))
            .map_err(|_| ParseError::Overflow)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ms(d: Duration) -> i64 {
        d.num_milliseconds()
    }

    #[test]
    fn zero() {
        assert_eq!(parse("0").unwrap(), Duration::zero());
        assert_eq!(parse("0s").unwrap(), Duration::zero());
        assert_eq!(parse("0ms").unwrap(), Duration::zero());
    }

    #[test]
    fn parse_1h_30m() {
        assert_eq!(parse("1h 30m").unwrap(), Duration::minutes(90));
    }

    #[test]
    fn parse_negative() {
        assert_eq!(ms(parse("-500ms").unwrap()), -500);
        assert_eq!(ms(parse("-2h").unwrap()), -7_200_000);
    }

    #[test]
    fn parse_positive_sign() {
        assert_eq!(ms(parse("+1s").unwrap()), 1000);
    }

    #[test]
    fn parse_complex() {
        let d = parse("1d 12h 30m 15s").unwrap();
        assert_eq!(d.num_seconds(), 131415);
    }

    #[test]
    fn parse_fractional() {
        let d = parse("1.5h").unwrap();
        assert_eq!(d.num_minutes(), 90);
    }

    #[test]
    fn parse_no_space() {
        assert_eq!(parse("30s").unwrap(), Duration::seconds(30));
        assert_eq!(parse("100ms").unwrap(), Duration::milliseconds(100));
    }

    #[test]
    fn parse_units_full() {
        assert_eq!(parse("5ns").unwrap(), Duration::nanoseconds(5));
        assert_eq!(parse("5us").unwrap(), Duration::microseconds(5));
        assert_eq!(parse("5μs").unwrap(), Duration::microseconds(5));
        assert_eq!(parse("5ms").unwrap(), Duration::milliseconds(5));
        assert_eq!(parse("5s").unwrap(), Duration::seconds(5));
        assert_eq!(parse("5min").unwrap(), Duration::minutes(5));
        assert_eq!(parse("5h").unwrap(), Duration::hours(5));
        assert_eq!(parse("5d").unwrap(), Duration::days(5));
        assert_eq!(parse("5w").unwrap(), Duration::weeks(5));
    }

    #[test]
    fn parse_with_extra_spaces() {
        assert_eq!(parse("  1h    30m  ").unwrap(), Duration::minutes(90));
    }

    #[test]
    fn parse_empty() {
        assert!(matches!(parse(""), Err(ParseError::Empty)));
        assert!(matches!(parse("   "), Err(ParseError::Empty)));
    }

    #[test]
    fn parse_only_sign() {
        assert!(parse("-").is_err());
        assert!(parse("+").is_err());
    }

    #[test]
    fn parse_no_unit() {
        assert!(matches!(parse("30"), Err(ParseError::MissingUnit { .. })));
        assert!(matches!(parse("30 "), Err(ParseError::MissingUnit { .. })));
    }

    #[test]
    fn parse_dot_in_unit_ok() {
        // "30.s" parses as 30s (decimal point allowed in number, not unit)
        assert_eq!(parse("30.s").unwrap(), Duration::seconds(30));
    }

    #[test]
    fn parse_two_dots_bad_unit() {
        // "1.2.3h" — number is "1.2", leftover ".3h" is bad unit
        assert!(matches!(parse("1.2.3h"), Err(ParseError::BadUnit { .. })));
    }

    #[test]
    fn parse_bad_unit() {
        assert!(matches!(parse("30xs"), Err(ParseError::BadUnit { .. })));
    }

    #[test]
    fn parse_bad_number() {
        assert!(matches!(parse("abc"), Err(ParseError::BadNumber { .. })));
    }

    #[test]
    fn _unused_placeholder() {}

    #[test]
    fn parse_overflow_multiplication() {
        // i64::MAX ns in a single token multiplied by 1_000_000_000 will overflow
        assert!(matches!(
            parse("999999999999999s"),
            Err(ParseError::Overflow)
        ));
    }

    #[test]
    fn parse_sum_overflow() {
        // Each fits, but sum overflows
        let big = "9223372036854775807s"; // i64::MAX seconds
        assert!(matches!(parse(big), Err(ParseError::Overflow)));
    }

    #[test]
    fn parse_tiny_epsilon() {
        // Sub-nanosecond precision is lost (chrono only has ns resolution)
        let d = parse("0.5ns").unwrap();
        assert_eq!(d, Duration::zero());
    }

    #[test]
    fn parse_invalid_utf8_alpha_only() {
        // Symbol chars are not allowed as unit
        assert!(matches!(parse("30!s"), Err(ParseError::BadUnit { .. })));
    }
}
