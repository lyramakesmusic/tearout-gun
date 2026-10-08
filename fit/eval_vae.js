// Does the gun space produce guns? Prior samples' gun-ness vs the dataset; per-knob traversal effects.
import { readFileSync } from 'fs';
import { render, rng, defaults } from '../dsp/engine.js';
import { decode } from '../dsp/vae.js';
import { features, gunScore, FEATURE_KEYS } from '../dsp/gunness.js';
const vae = JSON.parse(readFileSync('web/vae.json', 'utf8')), ref = JSON.parse(readFileSync('web/gunness.json', 'utf8'));
const r = rng(5), g = () => { let u = 0; while (!u) u = r(); return Math.sqrt(-2 * Math.log(u)) * Math.cos(2 * Math.PI * r()); };
const feat = (P) => { const o = render({ ...defaults(), ...P, seed: 7 }); return features(Float32Array.from(o.L, (v, i) => 0.5 * (v + o.R[i]))); };
const med = (a) => { const s = [...a].sort((x, y) => x - y); return s[s.length >> 1]; };
const q = (a, f) => { const s = [...a].sort((x, y) => x - y); return s[Math.floor(f * (s.length - 1))]; };
for (const sd of [1, 1.5]) {
  const sc = Array.from({ length: 40 }, () => gunScore(feat(decode(vae, Array.from({ length: 8 }, () => sd * g()))), ref));
  console.log(`z ~ N(0, ${sd}²): gun-ness median ${med(sc).toFixed(2)}  p90 ${q(sc, 0.9).toFixed(2)}  share ≤0.9: ${(sc.filter((x) => x <= 0.9).length / sc.length * 100).toFixed(0)}%`);
}
console.log('reference: dataset accept threshold 0.90, fitted presets 0.80, psy kick 1.52, unfiltered rolls ~1.0');
// traversal: each knob -2 -> +2, others 0; report the features that move most (in units of the reference spread)
for (let d = 0; d < 8; d++) {
  const lo = feat(decode(vae, Array.from({ length: 8 }, (_, i) => (i === d ? -2 : 0)))), hi = feat(decode(vae, Array.from({ length: 8 }, (_, i) => (i === d ? 2 : 0))));
  const eff = FEATURE_KEYS.map((k) => [k, (hi[k] - lo[k]) / (1.4826 * ref.mad[k] + 1e-9)]).sort((a, b) => Math.abs(b[1]) - Math.abs(a[1]));
  const zl = decode(vae, Array.from({ length: 8 }, (_, i) => (i === d ? -2 : 0))), zh = decode(vae, Array.from({ length: 8 }, (_, i) => (i === d ? 2 : 0)));
  const pk = Object.keys(zl).filter((k) => typeof zl[k] === 'number').map((k) => [k, zh[k] - zl[k], zl[k], zh[k]]);
  const { SPEC_BY_ID, toNorm } = await import('../dsp/engine.js');
  const top = pk.map(([k, , a, b]) => [k, toNorm(SPEC_BY_ID[k], b) - toNorm(SPEC_BY_ID[k], a), a, b]).sort((x, y) => Math.abs(y[1]) - Math.abs(x[1])).slice(0, 5);
  console.log(`z${d}: ` + eff.slice(0, 3).map(([k, v]) => `${k} ${v > 0 ? '+' : ''}${v.toFixed(1)}σ`).join(', ') + '  | params: ' + top.map(([k, , a, b]) => `${k} ${(+a).toFixed(1)}→${(+b).toFixed(1)}`).join(', '));
}
