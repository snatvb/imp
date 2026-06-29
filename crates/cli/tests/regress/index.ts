import { join } from "path"

import { run } from "imp:subprocess"

const imp = process.argv[0] as string
const testsDir = import.meta.dirname + "/.."

// --- smoke base ---
import "../smoke/some_module.ts"
import "../smoke/test_meta"
import "../smoke/test_path"
import "../smoke/test_fs_promises"
import "../smoke/test_imp_fs"
// --- modules ---
import "../time/index"
import "../parsers/index"
import "../encoding/index.ts"
import "../env/index.ts"
import "../hash/index.ts"
import "../crypto/index.ts"
import "../url/index"
import "../rs_string/index"
import "../subprocess/index.ts"
import "../signal/test_signal"
// --- clap (full) ---
import "../clap/index"
// --- fs (full) ---
import "../fs/index"
// --- import tests ---
import "../test_text_import"
import "../test_json_import"
import "../test_dynamic_import"
import "../test_top_level_await"
import "../test_import_failures"
import "../test_import_types"
// --- process (safe) ---
import "../process/test_process"

// --- process exit via subprocess ---
{
  const exitTest = join(testsDir, "process", "test_process_exit.ts")
  const r = await run(imp, ["run", exitTest])
  assert(r.code === 42, `process.exit(42) should exit with code 42, got ${r.code}`)
  console.log("PASS: process.exit(42)")
}

{
  const exitListenerTest = join(testsDir, "process", "test_process_exit_listener.ts")
  const r = await run(imp, ["run", exitListenerTest])
  assert(r.code === 99, `process.exit(99) should exit with code 99, got ${r.code}`)
  assert(String(r.stdout).includes("exit listener called with code: 99"), "exit listener output")
  console.log("PASS: process.exit(99) with listener")
}

// --- network (slow, may fail if httpbin is down) ---
import "../fetch/index"
// --- stdin/inject ---
import "../inquire/index"

console.log("ALL REGRESS TESTS PASSED")
