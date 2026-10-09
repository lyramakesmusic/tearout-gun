// Self-training round: fit library snares from the inverse model's guess (+ pitch estimate), refined by coordinate descent.
// prep:  node fit/lib_fit.js prep          → out/gp/lib/list.json + mel.u8 (eval snares held out)
// fit:   SHARD=k/N node fit/lib_fit.js fit <budget>  → out/gp/lib/fits_k.jsonl (one fit per line, streamed)
import { readFileSync, writeFileSync, appendFileSync, mkdirSync, existsSync } from 'fs';
import { defaults, fromNorm, SR } from '../dsp/engine.js';
import { readWav } from './lib.js';
import { melOf, KNOBS, FRAMES, BANDS } from './gen_data.js';
import { target, refineCD } from './lab.js';
import { shellPitch } from '../dsp/fitstages.js';
process.env.LOSS = 'v2';
const D = 'out/gp/lib'; mkdirSync(D, { recursive: true });
const to48 = (x, sr) => { if (sr === SR) return x; const q = sr / SR, n = Math.floor(x.length / q), y = new Float32Array(n); for (let i = 0; i < n; i++) { const t = i * q, j = Math.floor(t); y[i] = x[j] + (t - j) * ((x[j + 1] ?? x[j]) - x[j]); } return y; };
const [mode, budget] = process.argv.slice(2);
if (mode === 'prep') {
  const held = new Set(JSON.parse(readFileSync('out/gp/eval/real.json', 'utf8')).map((x) => x.wav));
  const C = JSON.parse(readFileSync('analysis/snares/corpus.json', 'utf8')).filter((c) => !held.has(c.path) && !/roll|buzz|fill|loop|reverse/i.test(c.path));
  const MEL = new Uint8Array(C.length * FRAMES * BANDS), ok = [];
  C.forEach((c, i) => { try { const w = readWav(c.path); MEL.set(melOf(to48(w.mono, w.sr)), ok.length * FRAMES * BANDS); ok.push(c.path); } catch {} });
  writeFileSync(`${D}/mel.u8`, Buffer.from(MEL.buffer, 0, ok.length * FRAMES * BANDS)); writeFileSync(`${D}/list.json`, JSON.stringify(ok)); console.log(ok.length, 'library snares');
} else {
  const [sk, sn] = (process.env.SHARD || '0/1').split('/').map(Number), B = +(budget || 600);
  const list = JSON.parse(readFileSync(`${D}/list.json`, 'utf8')), PRED = JSON.parse(readFileSync(`${D}/pred.json`, 'utf8')), out = `${D}/fits_${sk}.jsonl`;
  const done = new Set(existsSync(out) ? readFileSync(out, 'utf8').trim().split('\n').filter(Boolean).map((l) => JSON.parse(l).file) : []);
  list.forEach((file, i) => {
    if (i % sn !== sk || done.has(file)) return;
    try {
      const t = target(file), v = PRED[i], P = { ...defaults(), engine: 1, len: 700, seed: 1 };
      KNOBS.forEach((s, k) => (P[s.id] = fromNorm(s, v[k])));
      const midi = Math.round(69 + 12 * Math.log2(shellPitch(t.T0) / 440)); P.note = 24 + (((midi % 12) + 12) % 12); P.sn_t_oct = Math.max(2, Math.min(6, Math.floor((midi - 4) / 12) - 1));
      const start = t.loss(P), r = refineCD(t, P, B);
      appendFileSync(out, JSON.stringify({ file, start, loss: r.f, lenMs: t.lenMs, params: r.P }) + '\n');
    } catch (e) { appendFileSync(`${D}/errors.txt`, `${file}\t${e.message}\n`); }
  });
}
