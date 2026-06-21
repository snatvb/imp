use super::*;
use std::collections::HashMap;

// ── tokenize_unquoted ────────────────────────────────────────────────

#[test]
fn tokenize_plain_text() {
    let parts = tokenize_unquoted("hello world");
    assert_eq!(parts.len(), 1);
    assert!(matches!(&parts[0], RawPart::Lit(s) if s == "hello world"));
}

#[test]
fn tokenize_bare_var() {
    let parts = tokenize_unquoted("$FOO");
    assert_eq!(parts.len(), 1);
    assert!(matches!(&parts[0], RawPart::Var(s) if s == "FOO"));
}

#[test]
fn tokenize_brace_var() {
    let parts = tokenize_unquoted("${FOO}");
    assert_eq!(parts.len(), 1);
    assert!(matches!(&parts[0], RawPart::BraceVar(s) if s == "FOO"));
}

#[test]
fn tokenize_escaped_dollar() {
    let parts = tokenize_unquoted("\\$FOO");
    assert_eq!(parts.len(), 2);
    assert!(matches!(&parts[0], RawPart::Var(s) if s == "$"));
    assert!(matches!(&parts[1], RawPart::Lit(s) if s == "FOO"));
}

#[test]
fn tokenize_mixed() {
    let parts = tokenize_unquoted("prefix_${FOO}_$BAR");
    assert_eq!(parts.len(), 4);
    assert!(matches!(&parts[0], RawPart::Lit(s) if s == "prefix_"));
    assert!(matches!(&parts[1], RawPart::BraceVar(s) if s == "FOO"));
    assert!(matches!(&parts[2], RawPart::Lit(s) if s == "_"));
    assert!(matches!(&parts[3], RawPart::Var(s) if s == "BAR"));
}

#[test]
fn tokenize_empty() {
    let parts = tokenize_unquoted("");
    assert!(parts.is_empty());
}

#[test]
fn tokenize_dollar_alone() {
    let parts = tokenize_unquoted("$");
    assert_eq!(parts.len(), 1);
    assert!(matches!(&parts[0], RawPart::Lit(s) if s == "$"));
}

#[test]
fn tokenize_brace_empty_name() {
    let parts = tokenize_unquoted("${}");
    assert_eq!(parts.len(), 1);
    assert!(matches!(&parts[0], RawPart::Lit(s) if s == "${}"));
}

#[test]
fn tokenize_unclosed_brace() {
    let parts = tokenize_unquoted("${FOO");
    assert_eq!(parts.len(), 1);
    assert!(matches!(&parts[0], RawPart::Lit(s) if s == "${FOO"));
}

#[test]
fn tokenize_var_with_underscore() {
    let parts = tokenize_unquoted("$_MY_VAR_123");
    assert_eq!(parts.len(), 1);
    assert!(matches!(&parts[0], RawPart::Var(s) if s == "_MY_VAR_123"));
}

#[test]
fn tokenize_escape_backslash_before_dollar() {
    let parts = tokenize_unquoted("\\\\$FOO");
    assert_eq!(parts.len(), 2);
    assert!(matches!(&parts[0], RawPart::Lit(s) if s == "\\\\"));
    assert!(matches!(&parts[1], RawPart::Var(s) if s == "FOO"));
}

// ── unescape_double ──────────────────────────────────────────────────

#[test]
fn unescape_double_newline() {
    assert_eq!(unescape_double("hello\\nworld"), "hello\nworld");
}

#[test]
fn unescape_double_cr() {
    assert_eq!(unescape_double("a\\rb"), "a\rb");
}

#[test]
fn unescape_double_tab() {
    assert_eq!(unescape_double("a\\tb"), "a\tb");
}

#[test]
fn unescape_double_backslash() {
    assert_eq!(unescape_double("a\\\\b"), "a\\b");
}

#[test]
fn unescape_double_quote() {
    assert_eq!(unescape_double("a\\\"b"), "a\"b");
}

#[test]
fn unescape_double_dollar() {
    assert_eq!(unescape_double("a\\$b"), "a$b");
}

#[test]
fn unescape_double_single_quote() {
    assert_eq!(unescape_double("a\\'b"), "a'b");
}

#[test]
fn unescape_double_unknown_escape() {
    assert_eq!(unescape_double("a\\zb"), "a\\zb");
}

#[test]
fn unescape_double_multibyte() {
    assert_eq!(unescape_double("привет"), "привет");
}

#[test]
fn unescape_double_empty() {
    assert_eq!(unescape_double(""), "");
}

#[test]
fn unescape_double_literal_backslash_at_end() {
    assert_eq!(unescape_double("foo\\"), "foo\\");
}

// ── unescape_single_keep ────────────────────────────────────────────

#[test]
fn unescape_single_keep_quote() {
    assert_eq!(unescape_single_keep("it\\'s"), "it's");
}

#[test]
fn unescape_single_keep_backslash() {
    assert_eq!(unescape_single_keep("a\\\\b"), "a\\b");
}

#[test]
fn unescape_single_keep_nothing() {
    assert_eq!(unescape_single_keep("hello"), "hello");
}

// ── parse_value ──────────────────────────────────────────────────────

#[test]
fn parse_value_double_quoted() {
    let (val, quoted) = parse_value("\"hello world\"");
    assert_eq!(val, "hello world");
    assert!(quoted);
}

#[test]
fn parse_value_double_quoted_with_escapes() {
    let (val, quoted) = parse_value("\"line1\\nline2\"");
    assert_eq!(val, "line1\nline2");
    assert!(quoted);
}

#[test]
fn parse_value_single_quoted() {
    let (val, quoted) = parse_value("'hello world'");
    assert_eq!(val, "hello world");
    assert!(quoted);
}

#[test]
fn parse_value_single_quoted_escapes_kept() {
    let (val, quoted) = parse_value("'it\\'s a test'");
    assert_eq!(val, "it\\");
    assert!(quoted);
}

#[test]
fn parse_value_unquoted() {
    let (val, quoted) = parse_value("hello world");
    assert_eq!(val, "hello world");
    assert!(!quoted);
}

#[test]
fn parse_value_unquoted_with_comment() {
    let (val, _quoted) = parse_value("hello # comment");
    assert_eq!(val, "hello");
}

#[test]
fn parse_value_unclosed_double() {
    let (val, quoted) = parse_value("\"hello");
    assert_eq!(val, "\"hello");
    assert!(!quoted);
}

#[test]
fn parse_value_unclosed_single() {
    let (val, quoted) = parse_value("'hello");
    assert_eq!(val, "'hello");
    assert!(!quoted);
}

// ── parse_unquoted ───────────────────────────────────────────────────

#[test]
fn parse_unquoted_hash_comment() {
    assert_eq!(parse_unquoted("value # comment"), "value");
}

#[test]
fn parse_unquoted_semicolon_comment() {
    assert_eq!(parse_unquoted("value ; comment"), "value");
}

#[test]
fn parse_unquoted_no_comment() {
    assert_eq!(parse_unquoted("hello world"), "hello world");
}

#[test]
fn parse_unquoted_trailing_spaces() {
    assert_eq!(parse_unquoted("value  # comment"), "value");
}

#[test]
fn parse_unquoted_only_comment() {
    assert_eq!(parse_unquoted("# just a comment"), "");
}

#[test]
fn parse_unquoted_empty() {
    assert_eq!(parse_unquoted(""), "");
}

#[test]
fn parse_unquoted_trim() {
    assert_eq!(parse_unquoted("  value  "), "value");
}

// ── unescape_unquoted ───────────────────────────────────────────────

#[test]
fn unescape_unquoted_dollar() {
    assert_eq!(unescape_unquoted("price\\$100"), "price$100");
}

#[test]
fn unescape_unquoted_quote() {
    assert_eq!(unescape_unquoted("say\\\"hi\\\""), "say\"hi\"");
}

#[test]
fn unescape_unquoted_backslash() {
    assert_eq!(unescape_unquoted("path\\\\to"), "path\\to");
}

#[test]
fn unescape_unquoted_other_backslash() {
    assert_eq!(unescape_unquoted("a\\zb"), "a\\zb");
}

#[test]
fn unescape_unquoted_multibyte() {
    assert_eq!(unescape_unquoted("привет"), "привет");
}

#[test]
fn unescape_unquoted_empty() {
    assert_eq!(unescape_unquoted(""), "");
}

// ── find_unescaped ──────────────────────────────────────────────────

#[test]
fn find_unescaped_simple() {
    assert_eq!(find_unescaped("hello\"world", 0, '"'), Some(5));
}

#[test]
fn find_unescaped_with_escapes() {
    assert_eq!(find_unescaped("hello\\\"world", 0, '"'), None);
}

#[test]
fn find_unescaped_escaped_then_unescaped() {
    assert_eq!(find_unescaped("a\\\"b\"c", 0, '"'), Some(4));
}

#[test]
fn find_unescaped_not_found() {
    assert_eq!(find_unescaped("hello world", 0, '"'), None);
}

#[test]
fn find_unescaped_start_offset() {
    assert_eq!(find_unescaped("\"hello\"world", 1, '"'), Some(6));
}

#[test]
fn find_unescaped_multibyte() {
    assert_eq!(find_unescaped("привет\"мир", 0, '"'), Some(12));
}

#[test]
fn find_unescaped_empty_after_start() {
    assert_eq!(find_unescaped("\"", 1, '"'), None);
}

// ── parse_raw ────────────────────────────────────────────────────────

#[test]
fn parse_raw_basic() {
    let input = "FOO=bar\nBAZ=qux";
    let map = parse_raw(input).unwrap();
    assert_eq!(map.get("FOO").unwrap(), "bar");
    assert_eq!(map.get("BAZ").unwrap(), "qux");
}

#[test]
fn parse_raw_export() {
    let input = "export FOO=bar\nexport\tBAZ=qux";
    let map = parse_raw(input).unwrap();
    assert_eq!(map.get("FOO").unwrap(), "bar");
    assert_eq!(map.get("BAZ").unwrap(), "qux");
}

#[test]
fn parse_raw_comments_and_empty() {
    let input = "# comment\n\nFOO=bar\n\n# another\nBAZ=qux\n";
    let map = parse_raw(input).unwrap();
    assert_eq!(map.len(), 2);
    assert_eq!(map.get("FOO").unwrap(), "bar");
    assert_eq!(map.get("BAZ").unwrap(), "qux");
}

#[test]
fn parse_raw_bom() {
    let input = "\u{feff}FOO=bar";
    let map = parse_raw(input).unwrap();
    assert_eq!(map.get("FOO").unwrap(), "bar");
}

#[test]
fn parse_raw_crlf() {
    let input = "FOO=bar\r\nBAZ=qux\r\n";
    let map = parse_raw(input).unwrap();
    assert_eq!(map.get("FOO").unwrap(), "bar");
    assert_eq!(map.get("BAZ").unwrap(), "qux");
}

#[test]
fn parse_raw_cr() {
    let input = "FOO=bar\rBAZ=qux";
    let map = parse_raw(input).unwrap();
    assert_eq!(map.get("FOO").unwrap(), "bar");
    assert_eq!(map.get("BAZ").unwrap(), "qux");
}

#[test]
fn parse_raw_empty_key() {
    let input = "=value\nFOO=bar";
    let map = parse_raw(input).unwrap();
    assert_eq!(map.len(), 1);
    assert_eq!(map.get("FOO").unwrap(), "bar");
}

#[test]
fn parse_raw_no_equals() {
    let input = "FOO\nBAR";
    let map = parse_raw(input).unwrap();
    assert!(map.is_empty());
}

#[test]
fn parse_raw_double_quoted_value() {
    let input = "FOO=\"hello world\"";
    let map = parse_raw(input).unwrap();
    assert_eq!(map.get("FOO").unwrap(), "hello world");
}

#[test]
fn parse_raw_single_quoted_value() {
    let input = "FOO='hello world'";
    let map = parse_raw(input).unwrap();
    assert_eq!(map.get("FOO").unwrap(), "hello world");
}

#[test]
fn parse_raw_value_with_hash() {
    let input = "FOO=bar#baz";
    let map = parse_raw(input).unwrap();
    assert_eq!(map.get("FOO").unwrap(), "bar");
}

#[test]
fn parse_raw_empty_value() {
    let input = "FOO=";
    let map = parse_raw(input).unwrap();
    assert_eq!(map.get("FOO").unwrap(), "");
}

#[test]
fn parse_raw_later_key_wins() {
    let input = "FOO=first\nFOO=second";
    let map = parse_raw(input).unwrap();
    assert_eq!(map.get("FOO").unwrap(), "second");
}

// ── parse_with_vars ──────────────────────────────────────────────────

#[test]
fn parse_with_vars_no_expand() {
    let input = "FOO=bar\nBAZ=$FOO";
    let map = parse_with_vars(input, false, &HashMap::new()).unwrap();
    assert_eq!(map.get("FOO").unwrap(), "bar");
    assert_eq!(map.get("BAZ").unwrap(), "$FOO");
}

#[test]
fn parse_with_vars_expand() {
    let input = "FOO=bar\nBAZ=${FOO}_suffix";
    let map = parse_with_vars(input, true, &HashMap::new()).unwrap();
    assert_eq!(map.get("FOO").unwrap(), "bar");
    assert_eq!(map.get("BAZ").unwrap(), "bar_suffix");
}

#[test]
fn parse_with_vars_extra_vars() {
    let input = "BAZ=${EXTRA}_value";
    let mut extra = HashMap::new();
    extra.insert("EXTRA".to_string(), "injected".to_string());
    let map = parse_with_vars(input, true, &extra).unwrap();
    assert_eq!(map.get("BAZ").unwrap(), "injected_value");
}

#[test]
fn parse_with_vars_circular() {
    let input = "A=$B\nB=$A";
    let result = parse_with_vars(input, true, &HashMap::new());
    assert!(result.is_err());
}

#[test]
fn parse_with_vars_double_quoted_expand() {
    let input = "FOO=hello\nBAZ=\"${FOO} world\"";
    let map = parse_with_vars(input, true, &HashMap::new()).unwrap();
    assert_eq!(map.get("BAZ").unwrap(), "hello world");
}

#[test]
fn parse_with_vars_single_quoted_no_expand() {
    let input = "FOO=hello\nBAZ='${FOO} world'";
    let map = parse_with_vars(input, true, &HashMap::new()).unwrap();
    assert_eq!(map.get("BAZ").unwrap(), "hello world");
}

#[test]
fn parse_with_vars_missing_var() {
    let input = "BAZ=${MISSING}";
    let map = parse_with_vars(input, true, &HashMap::new()).unwrap();
    assert_eq!(map.get("BAZ").unwrap(), "$MISSING");
}

// ── expand_value ─────────────────────────────────────────────────────

#[test]
fn expand_value_double_quoted() {
    let mut vars = HashMap::new();
    vars.insert("NAME".to_string(), "Alice".to_string());
    let mut visiting = Vec::new();
    let result = expand_value("\"Hello ${NAME}\"", &vars, &mut visiting).unwrap();
    assert_eq!(result, "Hello Alice");
}

#[test]
fn expand_value_single_quoted() {
    let mut vars = HashMap::new();
    vars.insert("NAME".to_string(), "Alice".to_string());
    let mut visiting = Vec::new();
    let result = expand_value("'${NAME}'", &vars, &mut visiting).unwrap();
    assert_eq!(result, "'${NAME}'");
}

#[test]
fn expand_value_unquoted_literal() {
    let vars = HashMap::new();
    let mut visiting = Vec::new();
    let result = expand_value("hello", &vars, &mut visiting).unwrap();
    assert_eq!(result, "hello");
}

#[test]
fn expand_value_unquoted_var() {
    let mut vars = HashMap::new();
    vars.insert("X".to_string(), "42".to_string());
    let mut visiting = Vec::new();
    let result = expand_value("$X", &vars, &mut visiting).unwrap();
    assert_eq!(result, "42");
}

#[test]
fn expand_value_brace_var() {
    let mut vars = HashMap::new();
    vars.insert("X".to_string(), "42".to_string());
    let mut visiting = Vec::new();
    let result = expand_value("${X}", &vars, &mut visiting).unwrap();
    assert_eq!(result, "42");
}

// ── resolve_dotenv_var ──────────────────────────────────────────────

#[test]
fn resolve_var_simple() {
    let mut vars = HashMap::new();
    vars.insert("FOO".to_string(), "bar".to_string());
    let mut visiting = Vec::new();
    let result = resolve_dotenv_var("FOO", &vars, &mut visiting).unwrap();
    assert_eq!(result, "bar");
}

#[test]
fn resolve_var_missing() {
    let vars = HashMap::new();
    let mut visiting = Vec::new();
    let result = resolve_dotenv_var("MISSING", &vars, &mut visiting).unwrap();
    assert_eq!(result, "$MISSING");
}

#[test]
fn resolve_var_circular() {
    let mut vars = HashMap::new();
    vars.insert("A".to_string(), "$B".to_string());
    vars.insert("B".to_string(), "$A".to_string());
    let mut visiting = Vec::new();
    let result = resolve_dotenv_var("A", &vars, &mut visiting);
    assert!(result.is_err());
}

#[test]
fn resolve_var_chain() {
    let mut vars = HashMap::new();
    vars.insert("A".to_string(), "$B".to_string());
    vars.insert("B".to_string(), "final".to_string());
    let mut visiting = Vec::new();
    let result = resolve_dotenv_var("A", &vars, &mut visiting).unwrap();
    assert_eq!(result, "final");
}

// ── DotenvOptions ───────────────────────────────────────────────────

#[test]
fn dotenv_options_default() {
    let opts = DotenvOptions::default();
    assert!(opts.expand);
    assert!(opts.vars.is_empty());
}

#[test]
fn dotenv_options_custom() {
    let opts = DotenvOptions {
        expand: false,
        vars: HashMap::from([("X".to_string(), "1".to_string())]),
    };
    assert!(!opts.expand);
    assert_eq!(opts.vars.get("X").unwrap(), "1");
}
