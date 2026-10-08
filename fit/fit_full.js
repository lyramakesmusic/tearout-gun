// Fit the whole engine to one finished sound (no stems): staged CMA-ES over param groups,
// loss = multi-res mel + LTAS + third-octave balance on the full mix.
// usage: node fit/fit_full.js <wav> <family> <out.json> [start_ms] [end_ms]
import { writeFileSync } from 'fs';
import { render, SPEC, toNorm, fromNorm, defaults, encodeWav, SR } from '../dsp/engine.js';
import { ARCHETYPES } from '../dsp/archetypes.js';
import { readWav, melStack, melLoss, cmaes, ltas, ltasLoss, thirdOct, tobLoss } from './lib.js';

const [file, family, out, a0, a1] = process.argv.slice(2);
const w = readWav(file);
const to48 = (x, sr) => { if (sr === SR) return x; const r = sr / SR, n = Math.floor(x.length / r), y = new Float32Array(n); for (let i = 0; i < n; i++) { const t = i * r, j = Math.floor(t); y[i] = x[j] + (t - j) * ((x[j + 1] ?? x[j]) - x[j]); } return y; };
let tgt = to48(w.mono, w.sr);
if (a0) tgt = tgt.subarray(Math.floor((+a0 * SR) / 1000), Math.floor((+(a1 || 99999) * SR) / 1000));
const lenMs = Math.min(1500, Math.max(200, Math.round((tgt.length / SR) * 1000)));
const n = Math.floor((lenMs * SR) / 1000);
const T = melStack(tgt.subarray(0, n), SR), TL = ltas(tgt.subarray(0, n), SR), TT = thirdOct(tgt.subarray(0, n), SR);
const loss = (P) => { const o = render({ ...P, len: lenMs, b_shots: 1 }); const m = Float32Array.from(o.L.subarray(0, n), (v, i) => 0.5 * (v + o.R[i])); return melLoss(melStack(m, SR), T).loss + 0.5 * ltasLoss(ltas(m, SR), TL) + 0.3 * tobLoss(m, TT, SR); };

const a = ARCHETYPES[family];
let cur = { ...defaults(), spice_lvl: -60, syn_lvl: -60, ...a.base, len: lenMs, seed: 1 };
const fitIds = (re) => SPEC.filter((s) => re.test(s.id) && s.fit !== false && !s.g.startsWith('sample')).map((s) => s.id);
const STAGES = [
  ['levels+master', fitIds(/^(sub_lvl|tr_lvl|body_lvl|syn_lvl|rev_lvl|mst_|cr_mix)/), 900],
  ['stagger', ['tr_delay', 'sub_delay', 'body_delay', 'tr_lvl', 'body_lvl', 'sub_lvl', 'sub_swell', 'body_swell', 'mst_swell', 'mst_fall'], 600],
  ['body', fitIds(/^body_/), 2400],
  ['transient', fitIds(/^tr_/), 1400],
  ['sub/shell', [...fitIds(/^sub_/), 'note'], 1200],
  ['synth', family === 'chug' ? fitIds(/^syn_/) : [], 1200],
  ['crunch+reverb', fitIds(/^(cr_|rev_)/), 1200],
  ['levels again', fitIds(/^(sub_lvl|tr_lvl|body_lvl|syn_lvl|rev_lvl|mst_|cr_mix)/), 700],
];
const t0 = Date.now();
let base = loss(cur);
console.error(`${file.split('/').pop()} start ${base.toFixed(3)}`);
for (const [name, ids, ev] of STAGES) {
  if (!ids.length) continue;
  const specs = ids.map((id) => SPEC.find((s) => s.id === id));
  const P = (u) => { const q = { ...cur }; specs.forEach((s, i) => (q[s.id] = fromNorm(s, u[i]))); return q; };
  const b = cmaes((u) => loss(P(u)), specs.map((s) => toNorm(s, cur[s.id])), { sigma: 0.2, maxEvals: ev, seed: 3 });
  if (b.f < base) { cur = P(b.x); base = b.f; }
  console.error(`  ${name}: ${base.toFixed(3)} (${((Date.now() - t0) / 1000).toFixed(0)}s)`);
}
writeFileSync(out, JSON.stringify({ file, family, loss: base, params: cur }, null, 1));
const o = render({ ...cur, len: lenMs });
writeFileSync(out.replace(/\.json$/, '.wav'), Buffer.from(encodeWav(o.L, o.R, o.sr)));
