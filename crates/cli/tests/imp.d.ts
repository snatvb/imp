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

declare function setTimeout(callback: () => void, delay?: number | Duration): number;
declare function clearTimeout(id: number): void;
declare function setInterval(callback: () => void, delay?: number | Duration): number;
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

type RsString = string & {
  readonly length: number;

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
};

declare namespace RsString {
  function fromString(value?: any): RsString;
  function fromCharCode(...codes: number[]): RsString;
  function fromCodePoint(...points: number[]): RsString;
  function equals(a: RsString | string, b: RsString | string, caseInsensitive?: boolean): boolean;
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
  seek(offset: number, whence: JsString): Promise<number>;
  close(): Promise<void>;
  [Symbol.dispose](): void;
}

declare class WriteHandle {
  write(data: ByteBuffer | string | ArrayBuffer): Promise<number>;
  writeFrom(buffer: ByteBuffer, offset?: number, length?: number): Promise<number>;
  flush(): Promise<void>;
  seek(offset: number, whence: JsString): Promise<number>;
  close(): Promise<void>;
  [Symbol.dispose](): void;
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
  interface ArgOptions<N extends string = string, C extends string = string> {
    name: N;
    short?: string;
    long?: string;
    help?: string;
    exclusive?: boolean;
    required?: boolean;
    action?: "set" | "append" | "count" | "flag" | "set_false" | "help" | "help_short" | "help_long" | "version";
    choices?: readonly C[];
    num_args?: number | [number] | [number, number];
  }

  type ArgValueKind<O extends ArgOptions> =
    O['action'] extends 'count' ? number :
    O['action'] extends 'flag' | 'set_false' ? boolean :
    O['action'] extends 'append' ? (O['choices'] extends readonly (infer C)[] ? C[] : RsString[]) :
    O['action'] extends 'help' | 'help_short' | 'help_long' | 'version' ? never :
    O['num_args'] extends [number, number] | [number] ? (O['choices'] extends readonly (infer C)[] ? C[] : RsString[]) :
    O['choices'] extends readonly (infer C)[] ? C :
    RsString | undefined;

  interface ParseResultSuccess<T> {
    type: "result";
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

  type ParseResult<T = {}> = (ParseResultSuccess<T> & T) | ParseResultHelp | ParseResultVersion | ParseResultError;

  const args: readonly RsString[];

  class Parser<T = {}> {
    constructor();
    name(name: string): Parser<T>;
    version(version: string): Parser<T>;
    about(about: string): Parser<T>;
    arg<const O extends ArgOptions>(opts: O): Parser<T & { [K in O['name']]: ArgValueKind<O> }>;
    parse(args: readonly JsString[]): ParseResult<T>;
  }

  export { Parser, args };
}

declare module "imp:fs" {
  function open(path: JsString, chunkSize: number): Promise<FileHandle>;
  function openWrite(path: JsString, flags?: 'w' | 'a' | 'rw', chunkSize?: number): Promise<WriteHandle>;
  function readFile(path: JsString): Promise<ArrayBuffer>;
  function readFile(path: JsString, encoding: "buffer" | "null"): Promise<ArrayBuffer>;
  function readFile(path: JsString, encoding: string): Promise<RsString>;
  function writeFile(path: JsString, data: JsString | ByteBuffer | ArrayBuffer, flag?: 'w' | 'a'): Promise<number>;
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
    openWrite: typeof openWrite;
    readFile: typeof readFile;
    writeFile: typeof writeFile;
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
  export { open, openWrite, readFile, writeFile, mkdir, metadata, metadataBatch, remove, removeAll, exists, walk, glob, globStream, FileHandle, WriteHandle, FsStats, WalkIterator, WalkOptions };
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
  function confirm(question: JsString, byDefault?: boolean): Promise<boolean>;

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
  function readFile(path: string, encoding: string): Promise<string>;
  function readFile(path: string): Promise<ArrayBuffer>;
  function writeFile(path: string, data: JsString | ByteBuffer | ArrayBuffer, flag?: 'w' | 'a'): Promise<number>;
  function glob(pattern: string, options?: { cwd?: string }): AsyncIterableIterator<string>;

  const _default: {
    readFile: typeof readFile;
    writeFile: typeof writeFile;
    glob: typeof glob;
  };
  export default _default;
  export { readFile, writeFile, glob };
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
  export { readLine, readAll };
}

interface ParseOptions {
  nativeStrings?: boolean;
}

declare module "imp:parsers" {
  export const json: {
    parse(input: JsString, options?: ParseOptions): unknown;
    stringify(value: unknown): RsString;
  };
  export const yaml: {
    parse(input: JsString, options?: ParseOptions): unknown;
    stringify(value: unknown): RsString;
  };
  export const xml: {
    parse(input: JsString, options?: ParseOptions): unknown;
    stringify(value: unknown, root: JsString): RsString;
  };
  export const toml: {
    parse(input: JsString, options?: ParseOptions): unknown;
    stringify(value: unknown): RsString;
  };
  export const ron: {
    parse(input: JsString, options?: ParseOptions): unknown;
    stringify(value: unknown): RsString;
  };
  export const csv: {
    parse(input: JsString, options?: ParseOptions): unknown[];
    stringify(value: unknown[]): RsString;
  };
  export const msgpack: {
    parse(input: ByteBuffer, options?: ParseOptions): unknown;
    stringify(value: unknown): ByteBuffer;
  };
}

declare function fetch(input: string | URL, init?: RequestInit): Promise<Response>;

interface RequestInit {
  method?: string;
  headers?: Headers | Record<string, string>;
  body?: string;
  signal?: AbortSignal;
}

declare class Headers {
  constructor(init?: Record<string, string> | [string, string][]);
  get(name: string): string | null;
  set(name: string, value: string): void;
  append(name: string, value: string): void;
  has(name: string): boolean;
  delete(name: string): void;
  entries(): [string, string][];
  keys(): string[];
  values(): string[];
  forEach(cb: (value: string, key: string) => void): void;
}

declare class Request {
  constructor(input: string | URL, init?: RequestInit);
  readonly method: string;
  readonly url: string;
  readonly headers: Headers;
  readonly body: string | null;
  readonly signal: AbortSignal | null;
  clone(): Request;
}

declare class Response {
  readonly status: number;
  readonly statusText: string;
  readonly ok: boolean;
  readonly headers: Headers;
  readonly url: string;
  readonly type: string;
  text(): Promise<string>;
  json(): Promise<any>;
  arrayBuffer(): ArrayBuffer;
  clone(): Response;
}

declare class DOMException extends Error {
  constructor(message?: string, name?: string);
  readonly name: string;
  readonly code: number;
}

declare class AbortController {
  readonly signal: AbortSignal;
  abort(reason?: any): void;
}

declare class AbortSignal {
  readonly aborted: boolean;
  readonly reason: string;
  static timeout(ms: number): AbortSignal;
}

declare class URLSearchParams {
  constructor(init?: string);
  append(name: string, value: string): void;
  delete(name: string): void;
  get(name: string): string | null;
  getAll(name: string): string[];
  has(name: string): boolean;
  set(name: string, value: string): void;
  sort(): void;
  readonly size: number;
  toString(): string;
  keys(): string[];
  values(): string[];
  entries(): string[][];
  forEach(callback: (value: string, key: string) => void): void;
}

declare class URL {
  constructor(input: string | RsString, base?: string | RsString);
  href: string;
  readonly origin: string;
  protocol: string;
  username: string;
  password: string;
  host: string;
  hostname: string;
  port: string;
  pathname: string;
  search: string;
  readonly searchParams: URLSearchParams;
  hash: string;
  toString(): string;
  toJSON(): string;
  static canParse(input: string | RsString, base?: string | RsString): boolean;
  static parse(input: string | RsString, base?: string | RsString): URL | null;
}

declare class Duration {
  constructor();
  static zero(): Duration;
  static nanos(n: number): Duration;
  static micros(n: number): Duration;
  static millis(n: number): Duration;
  static seconds(n: number): Duration;
  static minutes(n: number): Duration;
  static hours(n: number): Duration;
  static days(n: number): Duration;
  static weeks(n: number): Duration;
  static parse(input: string): Duration;
  asNanos(): number;
  asMicros(): number;
  asMillis(): number;
  asSeconds(): number;
  asMinutes(): number;
  asHours(): number;
  asDays(): number;
  add(other: Duration): Duration;
  sub(other: Duration): Duration;
  mul(n: number): Duration;
  neg(): Duration;
  abs(): Duration;
  isZero(): boolean;
  isNegative(): boolean;
  eq(other: Duration): boolean;
  lt(other: Duration): boolean;
  lte(other: Duration): boolean;
  gt(other: Duration): boolean;
  gte(other: Duration): boolean;
  toString(): string;
}

declare class ImpDate {
  constructor();
  static today(): ImpDate;
  static fromYmd(year: number, month: number, day: number): ImpDate;
  static fromIso(input: string): ImpDate;
  static fromTimestamp(ms: number): ImpDate;
  getYear(): number;
  getMonth(): number;
  getDay(): number;
  getDayOfWeek(): number;
  getDayOfYear(): number;
  addDays(d: Duration): ImpDate;
  addMonths(n: number): ImpDate;
  addYears(n: number): ImpDate;
  daysBetween(other: ImpDate): Duration;
  toIso(): string;
  toJsDate(): Date;
  toString(): string;
  equals(other: ImpDate): boolean;
}

declare class ImpTime {
  constructor();
  static fromHms(hour: number, minute: number, second: number): ImpTime;
  static fromHmsNano(hour: number, minute: number, second: number, nano: number): ImpTime;
  getHour(): number;
  getMinute(): number;
  getSecond(): number;
  getNano(): number;
  add(d: Duration): ImpTime;
  toIso(): string;
  toJsDate(): Date;
  toString(): string;
  equals(other: ImpTime): boolean;
}

declare class ImpDateTime {
  constructor();
  static now(): ImpDateTime;
  static fromTimestamp(ms: number): ImpDateTime;
  static fromIso(input: string): ImpDateTime;
  getYear(): number;
  getMonth(): number;
  getDay(): number;
  getHour(): number;
  getMinute(): number;
  getSecond(): number;
  getNano(): number;
  getDate(): ImpDate;
  add(d: Duration): ImpDateTime;
  sub(d: Duration): ImpDateTime;
  diff(other: ImpDateTime): Duration;
  format(fmt: string): string;
  toIso(): string;
  toJsDate(): Date;
  toString(): string;
  equals(other: ImpDateTime): boolean;
}

declare class ImpLocalDateTime {
  constructor();
  static nowLocal(): ImpLocalDateTime;
  static fromTimestamp(ms: number): ImpLocalDateTime;
  static fromIso(input: string): ImpLocalDateTime;
  getYear(): number;
  getMonth(): number;
  getDay(): number;
  getHour(): number;
  getMinute(): number;
  getSecond(): number;
  getNano(): number;
  add(d: Duration): ImpLocalDateTime;
  sub(d: Duration): ImpLocalDateTime;
  diff(other: ImpLocalDateTime): Duration;
  format(fmt: string): string;
  toIso(): string;
  toUtc(): ImpDateTime;
  toJsDate(): Date;
  toString(): string;
  equals(other: ImpLocalDateTime): boolean;
}

declare module "imp:time" {
  export const Duration: typeof Duration;
  export const ImpDate: typeof ImpDate;
  export const ImpTime: typeof ImpTime;
  export const ImpDateTime: typeof ImpDateTime;
  export const ImpLocalDateTime: typeof ImpLocalDateTime;
  export default {
    Duration, ImpDate, ImpTime, ImpDateTime, ImpLocalDateTime,
  };
}
