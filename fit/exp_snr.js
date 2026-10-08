// Loss signal-to-noise: rise for a 10% knob move (same seed) vs change from a seed swap alone (same knobs).
import { readFileSync } from 'fs';
import { defaults, SPEC_BY_ID, toNorm, fromNorm } from '../dsp/engine.js';
import { target, liveIds } from './lab.js';
const med = (a) => { const s = [...a].sort((x, y) => x - y); return s[s.length >> 1]; };
for (const L of (process.env.LOSSES || 'v1,v2,v3').split(',')) {
  process.env.LOSS = L; const out = [];
  for (const k of [0, 3, 6, 9, 12]) {
    const tr = { ...defaults(), ...JSON.parse(readFileSync(`out/recov/t${k}.truth.json`, 'utf8')) }, t = target(`out/recov/t${k}.wav`);
    const seed = med([1, 2, 3, 4].map((s) => t.loss({ ...tr, seed: tr.seed + s })));
    const ids = liveIds(tr).filter((_, i) => i % 3 === 0);
    const sens = med(ids.map((id) => { const s = SPEC_BY_ID[id], u = toNorm(s, tr[id]); return t.loss({ ...tr, [id]: fromNorm(s, u > 0.5 ? u - 0.1 : u + 0.1) }); }));
    out.push(`t${k} knob ${sens.toFixed(2)} seed ${seed.toFixed(2)} snr ${(sens / seed).toFixed(2)}`);
  }
  console.log(L, '|', out.join(' | '));
}
