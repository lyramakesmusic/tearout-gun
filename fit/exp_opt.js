// Optimizer bake-off at a fixed render budget. Targets: engine renders (exact answer exists) + real snares.
// usage: node fit/exp_opt.js [budget]
import { readFileSync, readdirSync, appendFileSync } from 'fs';
import { defaults, SR } from '../dsp/engine.js';
import { fitTo } from '../dsp/fitstages.js';
import { target, cma, liveIds, bank, nearest, SN } from './lab.js';
import { readWav } from './lib.js';

const B = +(process.argv[2] || 800);
const synth = [0, 3, 6, 9, 12].map((i) => `out/recov/t${i}.wav`);
const real = readdirSync('out/sfits').filter((f) => /^s\d+\.json$/.test(f)).sort().filter((_, i) => i % 40 === 7).map((f) => JSON.parse(readFileSync(`out/sfits/${f}`, 'utf8')).file).slice(0, 5);
const t0 = Date.now(), BANK = bank(1500); console.error(`bank ready ${(Date.now() - t0) / 1000}s`);

const METHODS = {
  // the current staged fitter, scaled to the budget
  staged: (t) => { const r = fitTo(readWavMono(t.path), { ...defaults(), engine: 1 }, { scale: B / 9500 }); return r.P; },
  // one CMA over every live knob from defaults
  flat: (t) => { const P = { ...defaults(), engine: 1 }; return cma(t, P, liveIds(P), B).P; },
  // nearest bank patch, no search
  nn: (t) => nearest(BANK, t.E, 1)[0].P,
  // nearest bank patch by full loss among top-k fingerprint matches, then CMA over its live knobs
  nnRefine: (t) => { const c = nearest(BANK, t.E, 8).map((b) => ({ P: b.P, f: t.loss(b.P) })).sort((a, b) => a.f - b.f)[0]; return cma(t, c.P, liveIds(c.P), B - 8, 0.12).P; },
  // three nearest, short CMA each, keep best
  nn3: (t) => nearest(BANK, t.E, 3).map((b) => cma(t, b.P, liveIds(b.P), Math.floor(B / 3), 0.12)).sort((a, b) => a.f - b.f)[0].P,
};
function readWavMono(p) { const w = readWav(p); if (w.sr === SR) return w.mono; const r = w.sr / SR, n = Math.floor(w.mono.length / r), y = new Float32Array(n); for (let i = 0; i < n; i++) { const t = i * r, j = Math.floor(t); y[i] = w.mono[j] + (t - j) * ((w.mono[j + 1] ?? w.mono[j]) - w.mono[j]); } return y; }

const res = {};
for (const [kind, list] of [['synthetic', synth], ['real', real]]) for (const p of list) {
  const t = target(p);
  for (const [name, fn] of Object.entries(METHODS)) {
    t.reset(); const s = Date.now(); const P = fn(t); const ev = t.evals(); const f = t.loss(P);
    (res[`${kind}|${name}`] = res[`${kind}|${name}`] || []).push(f);
    console.error(`${kind} ${p.split('/').pop().slice(0, 30).padEnd(30)} ${name.padEnd(9)} loss ${f.toFixed(2)}  evals ${ev}  ${((Date.now() - s) / 1000).toFixed(0)}s`);
  }
}
const med = (a) => { const s = [...a].sort((x, y) => x - y); return s[s.length >> 1]; };
const lines = Object.entries(res).map(([k, v]) => `${k.padEnd(20)} median ${med(v).toFixed(2)}  all ${v.map((x) => x.toFixed(2)).join(' ')}`);
console.log(`budget ${B}\n` + lines.join('\n'));
appendFileSync('out/exp_opt.log', `\n${new Date().toISOString()} budget ${B}\n${lines.join('\n')}\n`);
