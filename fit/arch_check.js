import { rollArchetype } from '../dsp/archetypes.js';
import { randomize } from '../dsp/random.js';
import { render, rng, defaults } from '../dsp/engine.js';
import { character } from './character.js';
import { thirdOct } from '../web/worker.js';
import { readFileSync } from 'fs';
const priors = JSON.parse(readFileSync('web/priors.json', 'utf8'));
const r = rng(3), mono = (o) => Float32Array.from(o.L, (v, i) => 0.5 * (v + o.R[i]));
const hb = (m) => { const t = thirdOct(m, 48000); const v = t.db.filter((_, i) => t.centers[i] >= 1000 && t.centers[i] < 8100); return v.reduce((a, b) => a + b, 0) / v.length; };
const roll = { gun: () => rollArchetype('gun', r), snare: () => rollArchetype('snare', r), ping: () => rollArchetype('ping', r), chug: () => rollArchetype('chug', r), kfu: () => randomize(defaults(), priors, new Set(), r) };
for (const [a, f] of Object.entries(roll)) {
  const rows = [], h = [];
  for (let i = 0; i < 12; i++) { const m = mono(render(f())); rows.push(character(m)); h.push(hb(m)); }
  const med = (k) => { const s = rows.map((x) => x[k]).sort((x, y) => x - y); return s[s.length >> 1].toFixed(2); };
  h.sort((x, y) => x - y);
  console.log(a.padEnd(6), ['noisy', 'peaky', 'move', 'crest', 'harsh'].map((k) => `${k} ${med(k)}`).join('  '), ` | 1-8k p10/50/90 ${[h[1], h[6], h[10]].map((x) => x.toFixed(1)).join(' / ')}`);
}
console.log('KFU    noisy 0.23  peaky 1.26  move 2.09  crest 8.56  harsh -1.10   | 1-8k ≈ -17.4');
