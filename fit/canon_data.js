// Rewrite inverse-model training labels into the canonical pitch gauge. usage: node fit/canon_data.js <src dir> <dst dir>  (mel/pitch/meta are linked)
import { readFileSync, writeFileSync, readdirSync, mkdirSync, symlinkSync, existsSync } from 'fs';
import { resolve } from 'path';
import { defaults, fromNorm } from '../dsp/engine.js';
import { vec, KNOBS } from './gen_data.js';
import { canonSnare } from './canon.js';
const [src, dst] = process.argv.slice(2); mkdirSync(dst, { recursive: true });
let n = 0, moved = 0, skip = 0;
for (const f of readdirSync(src).filter((f) => /^[ct]\d+\.params\.f32$/.test(f))) {
  const b = f.replace('.params.f32', ''), D = KNOBS.length + 1, V = new Float32Array(readFileSync(`${src}/${f}`).buffer.slice(0)), O = new Float32Array(V.length);
  for (let i = 0; i < V.length / D; i++) {
    const P = { ...defaults(), engine: 1 }; KNOBS.forEach((s, k) => (P[s.id] = fromNorm(s, V[i * D + k]))); P.note = 24 + Math.round(V[i * D + D - 1] * 11);
    const Q = canonSnare(P); n++; if (Q === P) skip++; else if (Q.note !== P.note) moved++;
    O.set(vec(Q), i * D);
  }
  writeFileSync(`${dst}/${f}`, Buffer.from(O.buffer));
  for (const e of ['mel.u8', 'pitch.u8', 'meta.json']) if (!existsSync(`${dst}/${b}.${e}`)) symlinkSync(resolve(`${src}/${b}.${e}`), `${dst}/${b}.${e}`);
}
console.log(n, 'rows,', moved, 'note moved,', skip, 'left as-is (octave out of range)');
