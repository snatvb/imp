import { merge, expand } from "imp:env"

{
  const out = merge({ a: "1", b: "2" }) as any
  console.assert(RsString.equals(out.a, "1"), "a=1")
  console.assert(RsString.equals(out.b, "2"), "b=2")
}

{
  const out = merge({ a: "1" }, { b: "2" }) as any
  console.assert(RsString.equals(out.a, "1"), "a from first")
  console.assert(RsString.equals(out.b, "2"), "b from second")
}

{
  const out = merge({ a: "1" }, { a: "2" }) as any
  console.assert(RsString.equals(out.a, "2"), "later overrides earlier")
}

{
  const out = merge({ a: "1" }, { a: "2" }, { a: "3" }) as any
  console.assert(RsString.equals(out.a, "3"), "three sources, last wins")
}

{
  const out = merge({ a: "1" }, { b: "2" }, { c: "3" }) as any
  console.assert(RsString.equals(out.a, "1"), "a from first")
  console.assert(RsString.equals(out.b, "2"), "b from second")
  console.assert(RsString.equals(out.c, "3"), "c from third")
}

{
  const out = merge() as any
  console.assert(Object.keys(out).length === 0, "no args = empty")
}

{
  const out = merge({ a: "1" }, {}) as any
  console.assert(RsString.equals(out.a, "1"), "merge with empty")
}

{
  const r = expand("hello $USER", { USER: "alice" })
  console.assert(r.toString() === "hello alice", "basic expand")
}

{
  const r = expand("hello ${USER}", { USER: "bob" })
  console.assert(r.toString() === "hello bob", "braced expand")
}

{
  const r = expand("hello $UNDEFINED")
  console.assert(r.toString() === "hello $UNDEFINED", "missing keeps literal")
}

{
  let threw = false
  try {
    expand("A=$B", { A: "$B", B: "$A" })
  } catch (e) {
    threw = true
  }
  console.assert(threw, "circular expand throws")
}

{
  const r = expand("plain text", {})
  console.assert(r.toString() === "plain text", "no var = passthrough")
}

{
  const r = expand("a $FOO b $BAR c", { FOO: "x", BAR: "y" })
  console.assert(r.toString() === "a x b y c", "multiple vars")
}

console.log("ALL ENV MERGE EXPAND TESTS PASSED")
