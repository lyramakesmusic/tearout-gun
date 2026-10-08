// "Harsh and flat" in numbers. Per sound (mono, first 600 ms):
//   noisy   : spectral flatness (geo/arith mean of power) over 400 Hz–10 kHz, median over active frames. 1 = white noise.
//   peaky   : LTAS peak prominence — mean over 400 Hz–8 kHz of (spectrum − its 1/3-oct median smoothed), dB, positive part. Partials → high.
//   move    : spectral-centroid range (octaves) across the hit's active frames (p10→p90).
//   crest   : peak/RMS dB over the active region.
//   harsh   : energy 2.5–6 kHz minus energy 0.5–2 kHz, dB.
import { readFileSync, readdirSync } from 'fs';
import { render, rng, fft, SR } from '../dsp/engine.js';
import { rollArchetype } from '../dsp/archetypes.js';
import { readWav } from './lib.js';
import { findStems } from './fit_bus.js';

const K = '/Users/lyra/Music/samples/KFU GUN KIT VOL 2.1/KFU GUN KIT VOL 2.1 GUN SORT';
const N = 2048, HOP = 512, win = Float64Array.from({ length: N }, (_, i) => 0.5 - 0.5 * Math.cos((2 * Math.PI * i) / N));
const bin = (f) => Math.round((f * N) / SR);

export function character(x) {
  x = x.subarray(0, Math.floor(0.6 * SR));
  const frames = [], re = new Float64Array(N), im = new Float64Array(N);
  for (let o = 0; o + N <= x.length; o += HOP) {
    for (let i = 0; i < N; i++) { re[i] = x[o + i] * win[i]; im[i] = 0; }
    fft(re, im, false);
    const p = new Float64Array(N / 2); let e = 0;
    for (let k = 0; k < N / 2; k++) { p[k] = re[k] ** 2 + im[k] ** 2 + 1e-18; e += p[k]; }
    frames.push({ p, e });
  }
  const emax = Math.max(...frames.map((f) => f.e)), act = frames.filter((f) => f.e > emax * 1e-3);
  const lo = bin(400), hi = bin(10000);
  const flat = act.map(({ p }) => { let lg = 0, ar = 0; for (let k = lo; k < hi; k++) { lg += Math.log(p[k]); ar += p[k]; } const n = hi - lo; return Math.exp(lg / n) / (ar / n); }).sort((a, b) => a - b);
  const cen = act.map(({ p }) => { let a = 0, b = 0; for (let k = bin(100); k < bin(16000); k++) { a += k * p[k]; b += p[k]; } return Math.log2((a / b) * SR / N); }).sort((a, b) => a - b);
  const lt = new Float64Array(N / 2); for (const { p } of act) for (let k = 0; k < N / 2; k++) lt[k] += p[k];
  const ldb = Array.from(lt, (v) => 10 * Math.log10(v + 1e-18));
  let pk = 0, c = 0;
  for (let k = bin(400); k < bin(8000); k++) {
    const w = Math.max(1, Math.round(k * 0.115)); const seg = ldb.slice(k - w, k + w + 1).sort((a, b) => a - b);
    pk += Math.max(0, ldb[k] - seg[seg.length >> 1]); c++;
  }
  const band = (a, b) => { let s = 0; for (let k = bin(a); k < bin(b); k++) s += lt[k]; return 10 * Math.log10(s + 1e-18); };
  let peak = 0, ss = 0; const n = act.length * HOP; for (let i = 0; i < Math.min(x.length, n); i++) { peak = Math.max(peak, Math.abs(x[i])); ss += x[i] ** 2; }
  const q = (a, f) => a[Math.floor(f * (a.length - 1))];
  return { noisy: q(flat, 0.5), peaky: pk / c, move: q(cen, 0.9) - q(cen, 0.1), crest: 20 * Math.log10(peak / Math.sqrt(ss / Math.min(x.length, n))), harsh: band(2500, 6000) - band(500, 2000) };
}

if (import.meta.url === `file://${process.argv[1]}`) {
  const mono = (o) => Float32Array.from(o.L, (v, i) => 0.5 * (v + o.R[i]));
  const groups = { kfu: [], presets: [] };
  for (const d of readdirSync(K).filter((d) => d.startsWith('KFU_GUN_') && findStems(`${K}/${d}`).proc)) {
    const w = readWav(findStems(`${K}/${d}`).proc); groups.kfu.push(character(w.ch.length > 1 ? Float32Array.from(w.ch[0], (v, i) => 0.5 * (v + w.ch[1][i])) : w.ch[0]));
  }
  for (const pr of JSON.parse(readFileSync('web/presets.json', 'utf8'))) groups.presets.push(character(mono(render({ ...pr.params, len: 900 }))));
  const r = rng(3);
  for (const a of ['gun', 'snare', 'ping', 'chug']) { groups[a] = []; for (let i = 0; i < 16; i++) groups[a].push(character(mono(render(rollArchetype(a, r))))); }
  const med = (a) => { const s = [...a].sort((x, y) => x - y); return s[s.length >> 1]; };
  console.log('median'.padEnd(9), ['noisy', 'peaky', 'move', 'crest', 'harsh'].map((k) => k.padStart(8)).join(''));
  for (const [g, rows] of Object.entries(groups)) console.log(g.padEnd(9), ['noisy', 'peaky', 'move', 'crest', 'harsh'].map((k) => med(rows.map((x) => x[k])).toFixed(2).padStart(8)).join(''));
  const kq = (k, f) => { const s = groups.kfu.map((x) => x[k]).sort((a, b) => a - b); return s[Math.floor(f * (s.length - 1))].toFixed(2); };
  console.log('kfu p10–p90', ['noisy', 'peaky', 'move', 'crest', 'harsh'].map((k) => `${k} ${kq(k, 0.1)}–${kq(k, 0.9)}`).join('  '));
}
