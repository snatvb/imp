# scaffold

Interactive project generator using `imp:inq` prompts. Demonstrates how to
build a real CLI tool with imp.

## What it shows

- `imp:inq` — `prompt`, `select`, `confirm`
- `imp:fs` — `mkdir`, `writeFile`
- `path.resolve` + `process.cwd()`
- Conditional logic based on user choices
- Top-level `await`

## Run

```bash
imp run main.ts
```

You'll be asked:

- Project name?
- Project type? (lib / cli / web / api)
- Language? (TypeScript / JavaScript)
- Initialize git?
- Add test scaffold?

The tool creates a new directory with a starter file structure.

## Example session

```
Project name? my-tool
Project type? cli
Language? TypeScript
Initialize git? Yes
Add test scaffold? Yes

Creating my-tool in C:\Users\you\my-tool...

Created:
  main.ts
  test.ts
  .gitignore
  README.md

Next: cd my-tool && imp run main.ts
```

## Compile to a distributable binary

`imp compile` embeds the runtime + your script into a single native binary
that runs without Node.js installed.

```bash
# Windows
imp compile main.ts scaffold.exe
.\scaffold.exe

# macOS / Linux
imp compile main.ts scaffold
./scaffold
```

A **single ~11MB binary** you can ship to anyone. No Node.js runtime, no
dependencies, no install step. On Windows, `imp compile main.ts scaffold`
also works and auto-appends `.exe`; on macOS and Linux, the output keeps the
exact name you pass.

This is the killer demo: write your dev tool in 60 lines of TypeScript,
distribute it as a single binary to non-technical users.

## Why this matters

The standard Node.js workflow for shipping a CLI tool is:

1. Write code
2. `npm install` (50-200MB of `node_modules`)
3. `npm run build`
4. Package with `pkg` or `nexe` (build artifacts, hacks)
5. Hope the user has the right Node version

The imp workflow is:

1. Write code
2. `imp compile main.ts mytool.exe`

That's it. Ship the binary.
