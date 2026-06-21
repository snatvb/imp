import { parseIni } from "imp:env"

{
  const out = parseIni("") as any
  console.assert(typeof out === "object", "empty file returns object")
  console.assert(Object.keys(out).length === 0, "empty file returns empty object")
}

{
  const out = parseIni("; comment only\n# also comment\n") as any
  console.assert(Object.keys(out).length === 0, "comment-only file returns empty object")
}

{
  const out = parseIni("\uFEFF[database]\nhost = localhost\nport = 5432\n") as any
  console.assert(out.database !== undefined, "section exists")
  console.assert(RsString.equals(out.database.host, "localhost"), "host=localhost")
  console.assert(out.database.port === 5432, "port=5432 (number)")
}

{
  const out = parseIni("[database]\nhost=localhost\nport=5432\ndebug=true\ntimeout=1.5\ngreeting=\"hello world\"\n") as any
  console.assert(RsString.equals(out.database.host, "localhost"), "no-space host")
  console.assert(out.database.port === 5432, "no-space port int")
  console.assert(out.database.debug === true, "debug=true")
  console.assert(out.database.timeout === 1.5, "timeout=1.5 float")
  console.assert(RsString.equals(out.database.greeting, "hello world"), "quoted greeting")
}

{
  const out = parseIni("[SECTION]\nKey = value\n") as any
  console.assert(out.section !== undefined, "lowercased section: SECTION -> section")
  console.assert(out.section.key !== undefined, "lowercased key: Key -> key")
  console.assert(RsString.equals(out.section.key, "value"), "value preserved")
}

{
  const out = parseIni("[Sec]\nKey = value\n", { caseSensitive: true }) as any
  console.assert(out.Sec !== undefined, "caseSensitive=true preserves Sec")
  console.assert(out.Sec.Key !== undefined, "caseSensitive=true preserves Key")
}

{
  const out = parseIni("user.name = alice\nuser.age = 30\n") as any
  console.assert(out.user !== undefined, "nested user")
  console.assert(RsString.equals(out.user.name, "alice"), "user.name=alice")
  console.assert(out.user.age === 30, "user.age=30")
}

{
  const out = parseIni("a.b.c = deep\n") as any
  console.assert(out.a !== undefined, "a exists")
  console.assert(out.a.b !== undefined, "a.b exists")
  console.assert(out.a.b.c !== undefined, "a.b.c exists")
  console.assert(RsString.equals(out.a.b.c, "deep"), "a.b.c=deep")
}

{
  const out = parseIni("key = 'single quoted'\n") as any
  console.assert(RsString.equals(out.key, "single quoted"), "single quoted")
}

{
  const out = parseIni('key = "line1\\nline2"\n') as any
  console.assert(RsString.equals(out.key, "line1\nline2"), "double-quoted with \\n escape")
}

{
  const out = parseIni('key = "a\\"b"\n') as any
  console.assert(RsString.equals(out.key, 'a"b'), "double-quoted with \\\" escape")
}

{
  const out = parseIni("key = line1 \\\nline2\n") as any
  console.assert(RsString.equals(out.key, "line1 line2"), "line continuation")
}

{
  const out = parseIni("key = line1 \\\nline2 \\\nline3\n") as any
  console.assert(RsString.equals(out.key, "line1 line2 line3"), "multiple continuations")
}

{
  const out = parseIni('key = """\nmulti\nline\nvalue\n"""\n') as any
  console.assert(RsString.equals(out.key, "multi\nline\nvalue\n"), "triple-quoted multi-line")
}

{
  const out = parseIni("k1 = v1\n; comment\nk2 = v2\n# another\nk3 = v3\n") as any
  console.assert(RsString.equals(out.k1, "v1"), "k1")
  console.assert(RsString.equals(out.k2, "v2"), "k2 after ;")
  console.assert(RsString.equals(out.k3, "v3"), "k3 after #")
}

{
  const out = parseIni("  key  =  spaced value  \n") as any
  console.assert(RsString.equals(out.key, "spaced value"), "whitespace trimmed")
}

{
  const out = parseIni("key1 = v1\nkey2 = v2\nkey3 = v3\n") as any
  console.assert(RsString.equals(out.key1, "v1"), "key1")
  console.assert(RsString.equals(out.key2, "v2"), "key2")
  console.assert(RsString.equals(out.key3, "v3"), "key3")
}

{
  const out = parseIni("[s1]\nkey = v1\n[s2]\nkey = v2\n") as any
  console.assert(RsString.equals(out.s1.key, "v1"), "s1.key")
  console.assert(RsString.equals(out.s2.key, "v2"), "s2.key")
}

{
  const out = parseIni("a.b = 1\na = 2\n") as any
  console.assert(out.a !== undefined, "a defined")
  console.assert(out.a.b === 1 || out.a === 2, "either nested or scalar")
}

{
  const out = parseIni("key = true\nkey2 = TRUE\nkey3 = True\nkey4 = false\nkey5 = FALSE\n") as any
  console.assert(out.key === true, "true lowercase")
  console.assert(out.key2 === true, "TRUE uppercase")
  console.assert(out.key3 === true, "True mixed")
  console.assert(out.key4 === false, "false lowercase")
  console.assert(out.key5 === false, "FALSE uppercase")
}

{
  const out = parseIni("a = -42\nb = 3.14\nc = 0\nd = -1.5e10\n") as any
  console.assert(out.a === -42, "negative int")
  console.assert(Math.abs(out.b - 3.14) < 1e-10, "pi float")
  console.assert(out.c === 0, "zero")
  console.assert(out.d === -15000000000, "scientific notation")
}

{
  const out = parseIni("k1 = v1\nk2 = 1.5\nk3 = true\nk4 = \"unquoted string with # comment\"\n") as any
  console.assert(RsString.equals(out.k4, "unquoted string with # comment"), "comment marker inside quotes")
}

{
  const out = parseIni("a = hello # this is a comment\nb = value\n") as any
  console.assert(RsString.equals(out.a, "hello"), "unquoted # stops at #")
  console.assert(RsString.equals(out.b, "value"), "b after")
}

{
  const out = parseIni("[server]\nhost.address = 127.0.0.1\nhost.port = 8080\n") as any
  console.assert(out.server.host !== undefined, "server.host exists")
  console.assert(RsString.equals(out.server.host.address, "127.0.0.1"), "server.host.address")
  console.assert(out.server.host.port === 8080, "server.host.port")
}

{
  const out = parseIni("k1 = v1\nk1.sub = v2\n") as any
  console.assert(out.k1 !== undefined, "k1 exists after nested")
  console.assert(RsString.equals(out.k1.sub, "v2"), "k1.sub=v2")
}

console.log("ALL ENV INI TESTS PASSED")
