// Inverse model vs kNN bank as fit starts, each refined by coordinate descent at an equal render budget.
// synthetic: sound error vs the truth rendered at the fit's seed; real: v2 loss vs the target.
import { readFileSync, writeFileSync, appendFileSync } from 'fs';
import { render, defaults, fromNorm } from '../dsp/engine.js';
import { target, targetFrom, mono, bank, nearest, refineCD } from './lab.js';
import { KNOBS } from './gen_data.js';
import { shellPitch } from '../dsp/fitstages.js';
process.env.LOSS = 'v2';
const B = +(process.argv[2] || 800), PRED = JSON.parse(readFileSync('out/gp/eval/pred.json', 'utf8')), BANK = bank(1500);
const synth = JSON.parse(readFileSync('out/gp/eval/synthetic.json', 'utf8')), real = JSON.parse(readFileSync('out/gp/eval/real.json', 'utf8'));
const fromVec = (v) => { const P = { ...defaults(), engine: 1, len: 700, seed: 1 }; KNOBS.forEach((s, i) => (P[s.id] = fromNorm(s, v[i]))); P.note = 24 + Math.round(v[KNOBS.length] * 11); return P; };
const rows = [], fits = [];
const only = process.env.ONLY, [sk, sn] = (process.env.SHARD || '0/1').split('/').map(Number);
[...synth.map((x) => ({ ...x, kind: 'synthetic' })), ...real.map((x) => ({ ...x, kind: 'real' }))].forEach((x, i) => {
  if ((only && x.kind !== only) || i % sn !== sk) return;
  const t = target(x.wav), judge = x.truth ? targetFrom(mono(render({ ...x.truth, seed: 1 }))) : t;
  const inv = fromVec(PRED[i]), midi = Math.round(69 + 12 * Math.log2(shellPitch(t.T0) / 440)), invP = { ...inv, note: 24 + (((midi % 12) + 12) % 12), sn_t_oct: Math.max(2, Math.min(6, Math.floor((midi - 4) / 12) - 1)) }, nn = nearest(BANK, t.E, 8).map((b) => ({ P: { ...b.P, seed: 1 }, f: t.loss({ ...b.P, seed: 1 }) })).sort((a, b) => a.f - b.f)[0].P;
  const both = [inv, nn].map((P) => ({ P, f: t.loss(P) })).sort((a, b) => a.f - b.f)[0].P;
  const runs = { 'model only': () => inv, 'model+pitch only': () => invP, 'bank only': () => nn, 'model + CD': () => refineCD(t, inv, B).P, 'model+pitch + CD': () => refineCD(t, invP, B).P, 'bank + CD': () => refineCD(t, nn, B - 8).P, 'best of both + CD': () => refineCD(t, both, B - 10).P };
  const rec = { wav: x.wav, kind: x.kind };
  for (const [name, fn] of Object.entries(runs)) { const P = fn(), e = judge.loss({ ...P, seed: 1 }); rows.push({ kind: x.kind, name, e }); rec[name] = P; console.error(`${x.kind} ${i} ${name.padEnd(18)} ${e.toFixed(2)}`); }
  fits.push(rec);
});
const med = (a) => { const s = [...a].sort((p, q) => p - q); return s[s.length >> 1]; };
const out = ['synthetic', 'real'].flatMap((k) => [...new Set(rows.map((r) => r.name))].map((n) => { const v = rows.filter((r) => r.kind === k && r.name === n).map((r) => r.e); return v.length ? `${k.padEnd(10)} ${n.padEnd(18)} median ${med(v).toFixed(2)}  mean ${(v.reduce((a, b) => a + b, 0) / v.length).toFixed(2)}  n=${v.length}` : null; }).filter(Boolean)).join('\n');
writeFileSync(`out/gp/eval/rows_${process.env.TAG || 'v1'}_${sk}.json`, JSON.stringify(rows)); writeFileSync(`out/gp/eval/fits_${process.env.TAG || 'v1'}_${sk}.json`, JSON.stringify(fits));
if (sn === 1) console.log(`budget ${B}\n${out}`);
