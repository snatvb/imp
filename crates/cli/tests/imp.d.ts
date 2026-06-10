type JsString = string | RsString;

interface ImportMeta {
  dirname: string;
  filename: string;
}

interface Console {
  log(...args: any[]): void;
  trace(...args: any[]): void;
  error(...args: any[]): void;
  assert(condition: boolean, ...args: any[]): void;
}

interface Process {
  cwd(): string;
}

interface Performance {
  now(): number;
}

declare function setTimeout(callback: () => void, delay?: number): number;
declare function clearTimeout(id: number): void;
declare function setInterval(callback: () => void, delay?: number): number;
declare function clearInterval(id: number): void;

declare const console: Console;
declare const process: Process;
declare const performance: Performance;

declare class ByteBuffer {
  constructor(size: number);

  static alloc(size: number): ByteBuffer;

  get length(): number;

  toString(): string;
  toStr(): RsString;
  slice(start: number, end?: number): ByteBuffer;
  toArrayBuffer(): ArrayBuffer;
  toArray(): number[];
}

declare class RsString {
  static fromString(value?: any): RsString;
  static fromCharCode(...codes: number[]): RsString;
  static fromCodePoint(...points: number[]): RsString;

  get length(): number;

  at(index?: number): RsString;
  charAt(index: number): RsString;
  charCodeAt(index: number): number;
  codePointAt(index: number): number;

  substring(start: number, end?: number): RsString;
  slice(start: number, end?: number): RsString;
  substr(start: number, length?: number): RsString;

  trim(): RsString;
  trimStart(): RsString;
  trimEnd(): RsString;

  indexOf(search: string, fromIndex?: number): number;
  lastIndexOf(search: string, fromIndex?: number): number;
  includes(search: string): boolean;
  startsWith(search: string): boolean;
  endsWith(search: string): boolean;

  concat(str: string): RsString;
  repeat(count: number): RsString;
  padStart(targetLength: number, padString?: string): RsString;
  padEnd(targetLength: number, padString?: string): RsString;
  toLowerCase(): RsString;
  toUpperCase(): RsString;
  toLocaleLowerCase(): RsString;
  toLocaleUpperCase(): RsString;
  localeCompare(other: string): number;
  normalize(form?: string): RsString;

  replace(search: string | RegExp, replacement: string | ((...args: any[]) => string)): RsString;
  replaceAll(search: string | RegExp, replacement: string | ((...args: any[]) => string)): RsString;
  search(regexp: RegExp): number;
  match(regexp: RegExp): RegExpMatchArray | null;
  matchAll(regexp: RegExp): IterableIterator<RegExpMatchArray>;
  split(separator: string | RegExp, limit?: number): string[];

  toString(): string;
  valueOf(): string;
  [Symbol.toPrimitive](hint: string): string;
  toJSON(): string;
  [Symbol.iterator](): IterableIterator<string>;
}

interface WalkOptions {
  ignore?: JsString[];
  absolute?: boolean;
  dot?: boolean;
  filter?: "all" | "files" | "directories";
}

interface WalkIterator extends AsyncIterable<string> {
  [Symbol.asyncIterator](): WalkIterator;
  next(): Promise<IteratorResult<string>>;
  return(): Promise<IteratorResult<string>>;
}

declare class FileHandle {
  read(): Promise<ByteBuffer | undefined>;
  readInto(buffer: ByteBuffer): Promise<number | undefined>;
  seek(offset: number, whence: string): Promise<number>;
  close(): Promise<void>;
}

interface FsStats {
  readonly isFile: boolean;
  readonly isDirectory: boolean;
  readonly isSymbolicLink: boolean;
  readonly isBlockDevice: boolean;
  readonly isCharacterDevice: boolean;
  readonly isFIFO: boolean;
  readonly isSocket: boolean;

  readonly size: number;

  readonly atimeMs: number;
  readonly mtimeMs: number;
  readonly ctimeMs: number;
  readonly birthtimeMs: number;

  readonly atime: Date;
  readonly mtime: Date;
  readonly ctime: Date;
  readonly birthtime: Date;

  readonly mode: number;
  readonly uid: number;
  readonly gid: number;
  readonly ino: number;
  readonly nlink: number;
  readonly rdev: number;
  readonly blksize: number;
  readonly blocks: number;
  readonly dev: number;

  readonly readonly: boolean;

  readonly archive: boolean;
  readonly hidden: boolean;
  readonly system: boolean;
}

declare module "imp:clap" {
  interface ArgOptions {
    name: string;
    short?: string;
    long?: string;
    help?: string;
    exclusive?: boolean;
    required?: boolean;
    action?: "set" | "append" | "count" | "flag" | "set_false" | "help" | "help_short" | "help_long" | "version";
    choices?: string[];
    num_args?: number | [number] | [number, number];
  }

  interface ParseResultSuccess {
    type: "result";
    [key: string]: any;
  }

  interface ParseResultHelp {
    type: "help";
    message: RsString;
  }

  interface ParseResultVersion {
    type: "version";
    message: RsString;
  }

  interface ParseResultError {
    type: "error";
    message: RsString;
  }

  type ParseResult = ParseResultSuccess | ParseResultHelp | ParseResultVersion | ParseResultError;

  const args: readonly RsString[];

  class Parser {
    constructor();
    name(name: string): void;
    version(version: string): void;
    about(about: string): void;
    arg(options: ArgOptions): void;
    parse(args: string[]): ParseResult;
  }

  export { Parser, args };
}

declare module "imp:fs" {
  function open(path: JsString, chunkSize: number): Promise<FileHandle>;
  function readFile(path: JsString): Promise<ArrayBuffer>;
  function readFile(path: JsString, encoding: "buffer" | "null"): Promise<ArrayBuffer>;
  function readFile(path: JsString, encoding: string): Promise<RsString>;
  function mkdir(path: JsString): Promise<void>;
  function metadata(path: JsString): Promise<FsStats>;
  function metadataBatch(paths: JsString[]): Promise<FsStats[]>;
  function remove(path: JsString): Promise<void>;
  function removeAll(paths: JsString[]): Promise<void>;
  function exists(path: JsString): Promise<boolean>;
  function walk(dir: JsString, options?: WalkOptions): WalkIterator;
  function glob(dir: JsString, pattern: JsString, options?: WalkOptions): Promise<RsString[]>;
  function globStream(dir: JsString, pattern: JsString, options?: WalkOptions): WalkIterator;

  const _default: {
    open: typeof open;
    readFile: typeof readFile;
    mkdir: typeof mkdir;
    metadata: typeof metadata;
    metadataBatch: typeof metadataBatch;
    remove: typeof remove;
    removeAll: typeof removeAll;
    exists: typeof exists;
    walk: typeof walk;
    glob: typeof glob;
    globStream: typeof globStream;
  };
  export default _default;
  export { open, readFile, mkdir, metadata, metadataBatch, remove, removeAll, exists, walk, glob, globStream, FileHandle, FsStats, WalkIterator, WalkOptions };
}

interface DateOptions {
  default?: Date;
  minDate?: Date;
  maxDate?: Date;
  weekStart?: number;
  helpMessage?: string;
}

declare module "imp:inq" {
  function prompt(text: JsString): Promise<string>;
  function select(question: JsString, variants: JsString[]): Promise<string>;
  function multiSelect(question: JsString, variants: JsString[]): Promise<string[]>;
  function password(question: JsString, hidden?: boolean): Promise<string>;
  function passwordWithConfirm(question: JsString, hidden?: boolean): Promise<string>;
  function editor(question: JsString): Promise<string>;
  function dateSelect(question: JsString, options?: DateOptions): Promise<Date>;
  function confirm(question: JsString, byDefault?: boolean): Promise<Date>;

  const _default: {
    prompt: typeof prompt;
    select: typeof select;
    multiSelect: typeof multiSelect;
    password: typeof password;
    passwordWithConfirm: typeof passwordWithConfirm;
    editor: typeof editor;
    dateSelect: typeof dateSelect;
    confirm: typeof confirm;
  };
  export default _default;
  export {
    prompt,
    select,
    multiSelect,
    password,
    passwordWithConfirm,
    editor,
    dateSelect,
    confirm
  };
}

declare module "path" {
  function resolve(...paths: JsString[]): string;
  function join(...paths: JsString[]): string;
  function basename(path: JsString, suffix?: JsString): string;
  function dirname(path: JsString): string;
  function extname(path: JsString): string;
  function normalize(path: JsString): string;
  function isAbsolute(path: JsString): boolean;
  function format(pathObject: { dir?: string; root?: string; base?: string; name?: string; ext?: string }): string;
  function parse(path: JsString): { root: string; dir: string; base: string; name: string; ext: string };
  function relative(from: JsString, to: JsString): string;
  function toNamespacedPath(path: JsString): string;

  const sep: string;
  const delimiter: string;

  const win32: {
    resolve(...paths: JsString[]): string;
    join(...paths: JsString[]): string;
    basename(path: JsString, suffix?: JsString): string;
    dirname(path: JsString): string;
    extname(path: JsString): string;
    normalize(path: JsString): string;
    isAbsolute(path: JsString): boolean;
    format(pathObject: { dir?: string; root?: string; base?: string; name?: string; ext?: string }): string;
    parse(path: JsString): { root: string; dir: string; base: string; name: string; ext: string };
    relative(from: JsString, to: JsString): string;
    toNamespacedPath(path: JsString): string;
    sep: "\\";
    delimiter: ";";
  };

  const posix: {
    resolve(...paths: JsString[]): string;
    join(...paths: JsString[]): string;
    basename(path: JsString, suffix?: JsString): string;
    dirname(path: JsString): string;
    extname(path: JsString): string;
    normalize(path: JsString): string;
    isAbsolute(path: JsString): boolean;
    format(pathObject: { dir?: string; root?: string; base?: string; name?: string; ext?: string }): string;
    parse(path: JsString): { root: string; dir: string; base: string; name: string; ext: string };
    relative(from: JsString, to: JsString): string;
    toNamespacedPath(path: JsString): string;
    sep: "/";
    delimiter: ":";
  };

  const _default: {
    resolve: typeof resolve;
    join: typeof join;
    basename: typeof basename;
    dirname: typeof dirname;
    extname: typeof extname;
    normalize: typeof normalize;
    isAbsolute: typeof isAbsolute;
    format: typeof format;
    parse: typeof parse;
    relative: typeof relative;
    toNamespacedPath: typeof toNamespacedPath;
    sep: typeof sep;
    delimiter: typeof delimiter;
    win32: typeof win32;
    posix: typeof posix;
  };
  export default _default;
  export { resolve, join, basename, dirname, extname, normalize, isAbsolute, format, parse, relative, toNamespacedPath, sep, delimiter, win32, posix };
}

declare module "fs/promises" {
  function readFile(path: string, encoding?: string): Promise<ArrayBuffer | string>;
  function glob(pattern: string, options?: { cwd?: string }): AsyncIterableIterator<string>;

  const _default: {
    readFile: typeof readFile;
    glob: typeof glob;
  };
  export default _default;
  export { readFile, glob };
}

declare module "imp:sys/input_simulate" {
  function injectKeys(keys: string[]): Promise<void>;

  const _default: {
    injectKeys: typeof injectKeys;
  };
  export default _default;
  export { injectKeys };
}

declare module "imp:sys/stdin" {
  function readLine(): Promise<RsString>;
  function readAll(): Promise<ByteBuffer>;

  const _default: {
    readLine: typeof readLine;
    readAll: typeof readAll;
  };
  export default _default;
  export _default;
}
