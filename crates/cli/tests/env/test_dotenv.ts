import { parseDotenv } from "imp:env"

{
  const out = parseDotenv("") as any
  assert(typeof out === "object", "empty returns object")
  assert(Object.keys(out).length === 0, "empty returns empty object")
}

{
  const out = parseDotenv("# comment\n; another\n") as any
  assert(Object.keys(out).length === 0, "comments only = empty")
}

{
  const out = parseDotenv("KEY=value\nFOO=bar\n") as any
  assert(RsString.equals(out.KEY, "value"), "KEY=value")
  assert(RsString.equals(out.FOO, "bar"), "FOO=bar")
}

{
  const out = parseDotenv("EMPTY=") as any
  assert("EMPTY" in out, "EMPTY key exists")
  assert(RsString.equals(out.EMPTY, ""), "EMPTY is empty string")
}

{
  const out = parseDotenv("QUOTED=\"hello world\"\nSINGLE='literal'\n") as any
  assert(RsString.equals(out.QUOTED, "hello world"), "double quoted")
  assert(RsString.equals(out.SINGLE, "literal"), "single quoted")
}

{
  const out = parseDotenv('ESC="a\\nb\\tc\\\\d\\"e"') as any
  assert(RsString.equals(out.ESC, 'a\nb\tc\\d"e'), "double-quoted escapes")
}

{
  const out = parseDotenv("SINGLE='$NOT_EXPANDED'") as any
  assert(RsString.equals(out.SINGLE, "$NOT_EXPANDED"), "single-quoted literal $")
}

{
  const out = parseDotenv("DEBUG=true\nNUM=42\n") as any
  assert(RsString.equals(out.DEBUG, "true"), "DEBUG is string 'true'")
  assert(RsString.equals(out.NUM, "42"), "NUM is string '42'")
}

{
  const out = parseDotenv("export FOO=bar\nBAR=baz\n") as any
  assert(RsString.equals(out.FOO, "bar"), "export prefix stripped")
  assert(RsString.equals(out.BAR, "baz"), "BAR without export")
}

{
  const out = parseDotenv("FOO=bar\nFOO=baz") as any
  assert(RsString.equals(out.FOO, "baz"), "last value wins")
}

{
  const out = parseDotenv("URL=postgres://user:pass@host:5432/db") as any
  assert(RsString.equals(out.URL, "postgres://user:pass@host:5432/db"), "URL with special chars")
}

{
  const out = parseDotenv("#\nFOO=bar\n#\nBAZ=qux\n") as any
  assert(RsString.equals(out.FOO, "bar"), "FOO between comments")
  assert(RsString.equals(out.BAZ, "qux"), "BAZ between comments")
}

{
  const out = parseDotenv("FOO=  spaced value  ") as any
  assert(RsString.equals(out.FOO, "spaced value"), "trim unquoted value")
}

{
  const out = parseDotenv("FOO=hello # this is a comment\nBAR=value") as any
  assert(RsString.equals(out.FOO, "hello"), "unquoted stops at #")
  assert(RsString.equals(out.BAR, "value"), "BAR after")
}

{
  const out = parseDotenv('GREETING="Hello $USER"', { USER: "alice" }) as any
  assert(RsString.equals(out.GREETING, "Hello alice"), "var expansion in double-quoted")
}

{
  const out = parseDotenv('MSG="$NOT_SET literal"') as any
  assert(RsString.equals(out.MSG, "$NOT_SET literal"), "missing var keeps literal")
}

{
  const out = parseDotenv("A=$B\nB=$C\nC=end") as any
  assert(RsString.equals(out.A, "end"), "chained var expansion")
  assert(RsString.equals(out.B, "end"), "B expanded")
  assert(RsString.equals(out.C, "end"), "C terminal")
}

{
  const out = parseDotenv('GREETING="Hello ${USER}"', { USER: "bob" }) as any
  assert(RsString.equals(out.GREETING, "Hello bob"), "braced var expansion")
}

{
  const out = parseDotenv('FOO="$BAR"', { BAR: "value" }) as any
  assert(RsString.equals(out.FOO, "value"), "FOO=$BAR expands to value")
}

{
  const out = parseDotenv("FOO=$BAR", { BAR: "value" }) as any
  assert(RsString.equals(out.FOO, "value"), "FOO=$BAR unquoted expansion")
}

{
  let threw = false
  try {
    parseDotenv("A=$B\nB=$A")
  } catch (e) {
    threw = true
  }
  assert(threw, "circular ref throws")
}

{
  const out = parseDotenv("FOO=bar", { expand: false }) as any
  assert(RsString.equals(out.FOO, "bar"), "expand=false still parses")
}

{
  const out = parseDotenv("FOO=$USER", { expand: false, USER: "alice" }) as any
  assert(RsString.equals(out.FOO, "$USER"), "expand=false keeps literal $USER")
}

{
  const out = parseDotenv("ESC=\\$NOT_EXPANDED") as any
  assert(RsString.equals(out.ESC, "$NOT_EXPANDED"), "backslash escapes $ in unquoted")
}

console.log("ALL ENV DOTENV TESTS PASSED")
