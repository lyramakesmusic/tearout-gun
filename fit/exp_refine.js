// Refinement strategies from the same kNN start, equal render budget; metric: sound error vs truth at the same seed.
import { readFileSync, appendFileSync } from 'fs';
import { defaults, render, SPEC_BY_ID, toNorm, fromNorm } from '../dsp/engine.js';
import { targetFrom, mono, cma, liveIds, bank, nearest } from './lab.js';
process.env.LOSS = 'v2';
const B = +(process.argv[2] || 800), BANK = bank(1500), ks = [0, 3, 6, 9, 12];
// sensitivity at P: loss change for a ±0.08 move of each knob (2 renders per knob)
function sensitivity(t, P, ids) {
  const f0 = t.loss(P);
  return ids.map((id) => { const s = SPEC_BY_ID[id], u = toNorm(s, P[id]); return [id, Math.max(Math.abs(t.loss({ ...P, [id]: fromNorm(s, Math.min(1, u + 0.08)) }) - f0), Math.abs(t.loss({ ...P, [id]: fromNorm(s, Math.max(0, u - 0.08)) }) - f0))]; }).sort((a, b) => b[1] - a[1]);
}
// coordinate descent: for each knob (most sensitive first) try a shrinking set of offsets, keep improvements, cycle
function coord(t, P, order, budget) {
  let best = { P, f: t.loss(P) }, step = 0.2, used = 1;
  while (used < budget && step > 0.01) {
    for (const id of order) {
      if (used >= budget) break;
      const s = SPEC_BY_ID[id], u = toNorm(s, best.P[id]);
      for (const d of [step, -step]) { const Q = { ...best.P, [id]: fromNorm(s, u + d) }, f = t.loss(Q); used++; if (f < best.f) { best = { P: Q, f }; break; } }
    }
    step *= 0.6;
  }
  return best.P;
}
const rows = [];
for (const k of ks) {
  const T = { ...defaults(), ...JSON.parse(readFileSync(`out/recov/t${k}.truth.json`, 'utf8')) }, tgt = targetFrom(mono(render(T)));
  const start = nearest(BANK, tgt.E, 8).map((b) => ({ P: { ...b.P, seed: 1 }, f: tgt.loss({ ...b.P, seed: 1 }) })).sort((a, b) => a.f - b.f)[0].P;
  const truthT = targetFrom(mono(render({ ...T, seed: 1 })));
  const runs = {
    'CMA all live': () => cma(tgt, start, liveIds(start), B - 8, 0.12).P,
    'CMA top-30 sensitive': () => { const S = sensitivity(tgt, start, liveIds(start)); return cma(tgt, start, S.slice(0, 30).map(([id]) => id), B - 8 - 2 * S.length, 0.12).P; },
    'coordinate descent': () => { const S = sensitivity(tgt, start, liveIds(start)); return coord(tgt, start, S.map(([id]) => id), B - 8 - 2 * S.length); },
  };
  for (const [name, fn] of Object.entries(runs)) { const P = fn(), e = truthT.loss({ ...P, seed: 1 }); rows.push({ name, e }); console.error(`t${k} ${name.padEnd(22)} sound err ${e.toFixed(2)}`); }
}
const med = (a) => { const s = [...a].sort((x, y) => x - y); return s[s.length >> 1]; };
const out = [...new Set(rows.map((r) => r.name))].map((n) => `${n.padEnd(22)} median sound err ${med(rows.filter((r) => r.name === n).map((r) => r.e)).toFixed(2)}  all ${rows.filter((r) => r.name === n).map((r) => r.e.toFixed(2)).join(' ')}`).join('\n');
console.log(`budget ${B}\n${out}`); appendFileSync('out/exp_refine.log', `\n${new Date().toISOString()}\n${out}\n`);
