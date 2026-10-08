// Does each feature knob do what it says? Sweep it, render, measure that feature.
import { readFileSync } from 'fs';
import { render, rng, defaults } from '../dsp/engine.js';
import { decodeNorm } from '../dsp/vae.js';
import { SPEC, fromNorm } from '../dsp/engine.js';
import { features } from '../dsp/gunness.js';
const cv = JSON.parse(readFileSync('web/cvae.json', 'utf8')), ref = JSON.parse(readFileSync('web/gunness.json', 'utf8'));
const dec = (z, c) => { const u = decodeNorm(cv, [...z, ...c]), P = { ...defaults(), seed: 7 }; cv.ids.forEach((id, i) => { const s = SPEC.find((x) => x.id === id); if (s) P[id] = fromNorm(s, u[i]); }); return P; };
const sig = (k) => 1.4826 * ref.mad[k];
const spearman = (a, b) => { const rk = (v) => { const o = v.map((x, i) => [x, i]).sort((p, q) => p[0] - q[0]); const r = []; o.forEach(([, i], j) => (r[i] = j)); return r; }; const ra = rk(a), rb = rk(b), n = a.length, m = (n - 1) / 2; let s = 0, sa = 0, sb = 0; for (let i = 0; i < n; i++) { s += (ra[i] - m) * (rb[i] - m); sa += (ra[i] - m) ** 2; sb += (rb[i] - m) ** 2; } return s / Math.sqrt(sa * sb); };
const r = rng(3), g = () => { let u = 0; while (!u) u = r(); return Math.sqrt(-2 * Math.log(u)) * Math.cos(2 * Math.PI * r()); };
const zs = [new Array(cv.z).fill(0), Array.from({ length: cv.z }, g), Array.from({ length: cv.z }, g)];
const levels = [-1.5, -0.75, 0, 0.75, 1.5];
const report = [];
cv.cond.forEach((k, ki) => {
  const rhos = [], spans = [];
  for (const z of zs) {
    const meas = levels.map((lv) => { const c = new Array(cv.cond.length).fill(0); c[ki] = lv; const o = render(dec(z, c)); return features(Float32Array.from(o.L, (v, i) => 0.5 * (v + o.R[i])))[k]; });
    rhos.push(spearman(levels, meas)); spans.push((meas[4] - meas[0]) / sig(k));
  }
  const rho = rhos.reduce((a, b) => a + b, 0) / rhos.length, span = spans.reduce((a, b) => a + b, 0) / spans.length;
  report.push({ k, rho, span, pass: rho >= 0.8 && span >= 1 });
  console.log(`${k.padEnd(11)} ρ ${rho.toFixed(2)}  (per seed ${rhos.map((x) => x.toFixed(2)).join(' ')})  moves ${span.toFixed(2)}σ for a 3σ knob sweep  ${rho >= 0.8 && span >= 1 ? 'PASS' : 'fail'}`);
});
console.log('passing knobs:', report.filter((x) => x.pass).map((x) => x.k).join(', '));
