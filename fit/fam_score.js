import { render, rng, defaults } from '../dsp/engine.js';
import { roll } from '../dsp/roll.js';
import { features, gunScore, FEATURE_KEYS } from '../dsp/gunness.js';
import { readFileSync } from 'fs';
const famFits = process.env.NOFITS ? null : JSON.parse(readFileSync('web/family_fits.json', 'utf8'));
const fam = JSON.parse(readFileSync('web/family_refs.json', 'utf8')), r = rng(12), mono = (o) => Float32Array.from(o.L, (v, i) => 0.5 * (v + o.R[i]));
for (const m of process.argv.slice(2)) {
  const F = [], S = [];
  for (let i = 0; i < 10; i++) { const f = features(mono(render(roll(m, defaults(), { famRefs: fam, famFits, r, k: 5 })))); F.push(f); S.push(gunScore(f, fam[m])); }
  const med = (k) => { const v = F.map((f) => f[k]).sort((a, b) => a - b); return v[5].toFixed(2); }; S.sort((a, b) => a - b);
  console.log(m.padEnd(6), 'score', S[5].toFixed(2), '|', FEATURE_KEYS.map((k) => `${k} ${med(k)}(${fam[m].median[k].toFixed(1)})`).join(' '));
}
