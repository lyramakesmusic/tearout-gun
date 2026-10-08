// Knob recovery on engine-made targets (truth known): does a seed-averaged loss find the right knobs better?
// metric: mean |normalized knob error| over the truth's live knobs, and the v2 loss of fit vs truth rendered with the same seed
import { readFileSync, appendFileSync } from 'fs';
import { defaults, render, SPEC_BY_ID, toNorm } from '../dsp/engine.js';
import { targetFrom, mono, cma, liveIds, bank, nearest } from './lab.js';
process.env.LOSS = 'v2';
const B = +(process.argv[2] || 800), BANK = bank(1500);
const ks = [0, 3, 6, 9, 12];
const knobErr = (P, T) => { const ids = liveIds(T); return ids.reduce((s, id) => s + Math.abs(toNorm(SPEC_BY_ID[id], P[id]) - toNorm(SPEC_BY_ID[id], T[id])), 0) / ids.length; };
// seed-averaged objective: mean loss over seeds {1, 2}; budget counts renders, so half the CMA iterations
const avgLoss = (t) => ({ ...t, loss: (P) => 0.5 * (t.loss({ ...P, seed: 1 }) + t.loss({ ...P, seed: 2 })) });
const rows = [];
for (const k of ks) {
  const T = { ...defaults(), ...JSON.parse(readFileSync(`out/recov/t${k}.truth.json`, 'utf8')) };
  const tgt = targetFrom(mono(render(T)));
  const start = (t) => nearest(BANK, tgt.E, 8).map((b) => ({ P: { ...b.P, seed: 1 }, f: t.loss({ ...b.P, seed: 1 }) })).sort((a, b) => a.f - b.f)[0].P;
  const runs = {
    'nearest bank patch': () => start(tgt),
    'flat · 1 seed': () => { const P = { ...defaults(), engine: 1, seed: 1 }; return cma(tgt, P, liveIds(P), B).P; },
    'flat · 2-seed avg': () => { const P = { ...defaults(), engine: 1, seed: 1 }; return cma(avgLoss(tgt), P, liveIds(P), B / 2).P; },
    'nn+refine · 1 seed': () => { const P = start(tgt); return cma(tgt, P, liveIds(P), B, 0.12).P; },
    'nn+refine · 2-seed avg': () => { const t2 = avgLoss(tgt), P = start(t2); return cma(t2, P, liveIds(P), B / 2, 0.12).P; },
  };
  for (const [name, fn] of Object.entries(runs)) {
    const P = fn(), ke = knobErr(P, T), same = targetFrom(mono(render({ ...T, seed: 1 }))).loss({ ...P, seed: 1 });
    rows.push({ k, name, ke, same }); console.error(`t${k} ${name.padEnd(24)} knob err ${ke.toFixed(3)}  loss vs truth@same seed ${same.toFixed(2)}`);
  }
}
const med = (a) => { const s = [...a].sort((x, y) => x - y); return s[s.length >> 1]; };
const names = [...new Set(rows.map((r) => r.name))];
const out = names.map((n) => { const R = rows.filter((r) => r.name === n); return `${n.padEnd(24)} knob err ${med(R.map((r) => r.ke)).toFixed(3)}  same-seed loss ${med(R.map((r) => r.same)).toFixed(2)}`; }).join('\n');
console.log(`budget ${B} renders\n${out}`); appendFileSync('out/exp_recov.log', `\n${new Date().toISOString()} budget ${B}\n${out}\n`);
