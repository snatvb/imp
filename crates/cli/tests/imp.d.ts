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

declare module "imp:fs" {
  function open(path: string | RsString, chunkSize: number): Promise<FileHandle>;
  function readFile(path: string | RsString): Promise<ArrayBuffer>;
  function readFile(path: string | RsString, encoding: "buffer" | "null"): Promise<ArrayBuffer>;
  function readFile(path: string | RsString, encoding: string): Promise<RsString>;
  function mkdir(path: string | RsString): Promise<void>;
  function metadata(path: string | RsString): Promise<FsStats>;
  function metadataBatch(paths: Array<string | RsString>): Promise<FsStats[]>;
  function remove(path: string | RsString): Promise<void>;
  function removeAll(paths: Array<string | RsString>): Promise<void>;
  function exists(path: string | RsString): Promise<boolean>;

  const _default: {
    open: typeof open;
    readFile: typeof readFile;
    mkdir: typeof mkdir;
    metadata: typeof metadata;
    metadataBatch: typeof metadataBatch;
    remove: typeof remove;
    removeAll: typeof removeAll;
    exists: typeof exists;
  };
  export default _default;
  export { open, readFile, mkdir, metadata, metadataBatch, remove, removeAll, exists, FileHandle, FsStats };
}

declare module "path" {
  function resolve(...paths: string[]): string;
  function join(...paths: string[]): string;
  function basename(path: string, suffix?: string): string;
  function dirname(path: string): string;
  function extname(path: string): string;
  function normalize(path: string): string;
  function isAbsolute(path: string): boolean;
  function format(pathObject: { dir?: string; root?: string; base?: string; name?: string; ext?: string }): string;
  function parse(path: string): { root: string; dir: string; base: string; name: string; ext: string };
  function relative(from: string, to: string): string;
  function toNamespacedPath(path: string | number): string;

  const sep: string;
  const delimiter: string;

  const win32: {
    resolve(...paths: string[]): string;
    join(...paths: string[]): string;
    basename(path: string, suffix?: string): string;
    dirname(path: string): string;
    extname(path: string): string;
    normalize(path: string): string;
    isAbsolute(path: string): boolean;
    format(pathObject: { dir?: string; root?: string; base?: string; name?: string; ext?: string }): string;
    parse(path: string): { root: string; dir: string; base: string; name: string; ext: string };
    relative(from: string, to: string): string;
    toNamespacedPath(path: string | number): string;
    sep: "\\";
    delimiter: ";";
  };

  const posix: {
    resolve(...paths: string[]): string;
    join(...paths: string[]): string;
    basename(path: string, suffix?: string): string;
    dirname(path: string): string;
    extname(path: string): string;
    normalize(path: string): string;
    isAbsolute(path: string): boolean;
    format(pathObject: { dir?: string; root?: string; base?: string; name?: string; ext?: string }): string;
    parse(path: string): { root: string; dir: string; base: string; name: string; ext: string };
    relative(from: string, to: string): string;
    toNamespacedPath(path: string | number): string;
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

  const _default: {
    readFile: typeof readFile;
  };
  export default _default;
  export { readFile };
}
