/**
 * Generates the cross-language fixtures: random operations run with the JavaScript library (the reference),
 * written with their results so the Rust tests can assert the same strings.
 *
 * Run it from the `javascript/` folder (it uses its `tsx`):
 *
 *   node_modules/.bin/tsx ../rust/tests/fixtures/generate.ts
 *
 * Outputs (tab separated, one operation per line):
 * - `positions.tsv`: `system  operation  argument1  argument2  expected`
 * - `list.tsv`:      `system  scenario  operation  arguments  result  state`
 */
import { writeFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
import Staquia, {
  Digits,
  InvalidCharacterError,
  LexicalList,
  OverflowError,
  SystemMismatchError,
  ThresholdReachedError,
} from "../../../javascript/src/export.js";
import type { IPosition, IPositionResult, LexicalError } from "../../../javascript/src/export.js";

const here = dirname(fileURLToPath(import.meta.url));

/** The systems, the Rust test creates the same ones by index. */
const SYSTEMS: [string, string, string, number, number, number][] = [
  ["!", "a", "z", 5, 15, 0.15],
  ["!", "a", "z", 4, 12, 0.15],
  ["!", "°", "ұ", 5, 15, 0.15],
  ["!", "a", "z", 2, 3, 0.15],
  ["0", "1", "9", 3, 6, 0.3],
  ["!", "a", "c", 3, 8, 0.1],
  ["!", "a", "z", 1, 31, 0],
];

/** A seeded random generator (mulberry32), so the fixtures are reproducible. */
const random = (() => {
  let seed = 0x5ea9c1a;
  return () => {
    seed |= 0;
    seed = (seed + 0x6d2b79f5) | 0;
    let t = Math.imul(seed ^ (seed >>> 15), 1 | seed);
    t = (t + Math.imul(t ^ (t >>> 7), 61 | t)) ^ t;
    return ((t ^ (t >>> 14)) >>> 0) / 4294967296;
  };
})();
const integer = (max: number) => Math.floor(random() * max);
const pick = <T>(array: T[]) => array[integer(array.length)]!;

/** Encodes a result: the position string, or the kind of error. */
const encode = (result: IPositionResult): string => {
  if (result.ok) return result.value.toString();
  const error = result.error;
  if (error instanceof ThresholdReachedError) return `!threshold:${error.position.toString()}`;
  if (error instanceof OverflowError) return "!overflow";
  if (error instanceof InvalidCharacterError) return `!invalid:${error.index}`;
  if (error instanceof SystemMismatchError) return "!mismatch";
  throw new Error(`Unknown error ${error}`);
};

/** The position of a result, also when it reached the threshold (it is still valid). */
const usable = (result: IPositionResult): IPosition | undefined => {
  if (result.ok) return result.value;
  if (result.error instanceof ThresholdReachedError) return result.error.position;
  return undefined;
};

const positions: string[] = [];
const lists: string[] = [];

SYSTEMS.forEach(([zero, first, last, ln, lz, factor], s) => {
  const system = Staquia.create(zero, first, last, ln, lz, factor);
  const lmax = ln + lz;
  const base = last.charCodeAt(0) - first.charCodeAt(0) + 2;
  const character = (digit: number) => (digit === 0 ? zero : String.fromCharCode(first.charCodeAt(0) + digit - 1));
  const record = (operation: string, a: string, b: string, result: IPositionResult) => {
    positions.push([s, operation, a, b, encode(result)].join("\t"));
    const position = usable(result);
    if (position) pool.push(position);
  };
  const pool: IPosition[] = [];
  const start = system.start();
  record("start", "", "", start);

  // Forge: valid strings of every length (also too long ones) and some invalid characters
  for (let i = 0; i < 250; i++) {
    const length = integer(lmax + 3);
    let string = "";
    for (let k = 0; k < length; k++) {
      const r = random();
      if (r < 0.02) string += "~";
      else if (r < 0.15) string += zero;
      else if (r < 0.25) string += pick([first, last, character(Math.ceil(base / 2))]);
      else string += character(1 + integer(base - 1));
    }
    record("forge", string, "", system.forge(string));
  }
  // The edges of N
  record("forge", last.repeat(ln), "", system.forge(last.repeat(ln)));
  record("forge", zero.repeat(ln), "", system.forge(zero.repeat(ln)));

  // Next and previous of random positions, also chained
  for (let i = 0; i < 300; i++) {
    let reference = pick(pool);
    const steps = 1 + integer(4);
    for (let k = 0; k < steps; k++) {
      const operation = random() < 0.5 ? "next" : "previous";
      const result = operation === "next" ? system.next(reference) : system.previous(reference);
      record(operation, reference.toString(), "", result);
      const next = usable(result);
      if (!next) break;
      reference = next;
    }
  }
  record("next", last.repeat(ln), "", system.next(system.forge(last.repeat(ln))));
  record("previous", zero.repeat(ln), "", system.previous(system.forge(zero.repeat(ln))));

  // Core of random pairs, also equal and swapped ones
  for (let i = 0; i < 500; i++) {
    const a = pick(pool);
    const b = random() < 0.03 ? a : pick(pool);
    record("core", a.toString(), b.toString(), system.core(a, b));
  }
  // Moving again and again into the same gap until it is exhausted
  for (let i = 0; i < 40; i++) {
    let a = pick(pool);
    let b = random() < 0.5 ? usable(system.next(a)) ?? pick(pool) : pick(pool);
    for (let k = 0; k < 40; k++) {
      const result = system.core(a, b);
      record("core", a.toString(), b.toString(), result);
      const c = usable(result);
      if (!c) break;
      if (random() < 0.5) a = c;
      else b = c;
    }
  }

  // From digits: positions rebuilt from their digits, and random numbers
  for (let i = 0; i < 150; i++) {
    let digits: number[];
    if (random() < 0.5) {
      const position = pick(pool);
      const number = position.digits();
      digits = [];
      for (let k = number.upperBound; k >= number.lowerBound; k--) digits.push(number.get(k));
    } else {
      const length = 1 + integer(lmax + 1);
      digits = Array.from({ length }, () => integer(base));
    }
    const number = new Digits();
    number._map = new Map();
    number.lowerBound = 0;
    number.upperBound = digits.length - 1;
    digits.forEach((digit, k) => number.set(digits.length - 1 - k, digit));
    record("from", digits.join(","), "", system.from(number));
  }

  // Positions of another system can't be mixed
  const other = Staquia.create(...SYSTEMS[(s + 1) % SYSTEMS.length]!);
  const foreign = other.start();
  if (foreign.ok) {
    record("mismatch", pick(pool).toString(), String((s + 1) % SYSTEMS.length), system.core(pick(pool), foreign.value));
  }
});

// Lists: random operations, the list state written after each one
interface Task {
  id: number;
  position: string;
}
const kind = (error: LexicalError) => error.type;
for (const s of [0, 3, 5]) {
  const system = Staquia.create(...SYSTEMS[s]!);
  for (let scenario = 0; scenario < 6; scenario++) {
    const list = new LexicalList<number, Task>(system);
    let counter = 1;
    const state = () => {
      const array = list.toArray();
      if (!array.ok) throw array.error;
      return array.value.map((item) => `${item.id}=${item.position.toString()}`).join(",");
    };
    const ids = () => {
      const array = list.toArray();
      if (!array.ok) throw array.error;
      return array.value.map((item) => item.id);
    };
    const write = (operation: string, args: string, result: string) => {
      lists.push([s, scenario, operation, args, result, state()].join("\t"));
    };
    for (let step = 0; step < 220; step++) {
      const r = random();
      const current = ids();
      if (r < 0.25 || current.length < 2) {
        const id = random() < 0.05 && current.length ? pick(current) : counter++;
        const result = list.push({ id, position: "" });
        write("push", String(id), result.ok ? "ok" : `err:${kind(result.error)}`);
      } else if (r < 0.37) {
        const id = random() < 0.05 ? pick(current) : counter++;
        const result = list.unshift({ id, position: "" });
        write("unshift", String(id), result.ok ? "ok" : `err:${kind(result.error)}`);
      } else if (r < 0.45) {
        const id = random() < 0.1 ? 100000 : pick(current);
        const result = list.delete(id);
        write("delete", String(id), result.ok ? "ok" : `err:${kind(result.error)}`);
      } else if (r < 0.85) {
        const id = pick(current);
        const rest = current.filter((x) => x !== id);
        let previous: number | undefined;
        let next: number | undefined;
        if (random() < 0.9) {
          // A valid place, often the same gap to exhaust it
          const index = random() < 0.4 ? Math.min(1, rest.length) : integer(rest.length + 1);
          previous = rest[index - 1];
          next = rest[index];
        } else {
          previous = random() < 0.8 ? pick(current) : undefined;
          next = random() < 0.8 ? pick(current) : undefined;
        }
        const result = list.move(id, previous, next);
        write("move", [id, previous ?? "", next ?? ""].join(","), result.ok ? "ok" : `err:${kind(result.error)}`);
      } else if (r < 0.9) {
        const result = list.sort((a, b) => (a.position.less(b.position) ? -1 : 1));
        if (!result.ok) break;
        const affected = result.value[0].map((item) => item.id).sort((a, b) => a - b);
        write("rebalance", "", `ok:${affected.join(",")}`);
      } else {
        // The stored rows, shuffled and partly broken
        const rows: Task[] = current.map((id) => ({ id, position: list.find(id)!.position.toString() }));
        const untrusted = random() < 0.75;
        if (untrusted) {
          for (let k = rows.length - 1; k > 0; k--) {
            const j = integer(k + 1);
            [rows[k], rows[j]] = [rows[j]!, rows[k]!];
          }
          for (const row of rows) {
            const x = random();
            if (x < 0.1) row.position = "";
            else if (x < 0.15) row.position = "A";
            else if (x < 0.3) row.position = pick(rows).position;
          }
        } else if (random() < 0.3 && rows.length > 1) {
          [rows[0], rows[1]] = [rows[1]!, rows[0]!];
        }
        const args = rows.map((row) => `${row.id}=${row.position}`).join(",");
        const result = list.set(rows, { untrusted });
        const outcome = result.ok
          ? `ok:${result.value[0]
              .map((item) => item.id)
              .sort((a, b) => a - b)
              .join(",")}`
          : `err:${kind(result.error)}`;
        write(untrusted ? "set_untrusted" : "set", args, outcome);
      }
    }
  }
}

writeFileSync(join(here, "positions.tsv"), positions.join("\n") + "\n");
writeFileSync(join(here, "list.tsv"), lists.join("\n") + "\n");
console.log(`positions.tsv: ${positions.length} lines, list.tsv: ${lists.length} lines`);
