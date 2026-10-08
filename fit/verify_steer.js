// Same knob test as verify_cvae.js, with the closed loop.
import { readFileSync } from 'fs';
import { rng } from '../dsp/engine.js';
import { steer } from '../dsp/cvae.js';
const cv = JSON.parse(readFileSync('web/cvae.json', 'utf8')), ref = JSON.parse(readFileSync('web/gunness.json', 'utf8'));
const spearman = (a, b) => { const rk = (v) => { const o = v.map((x, i) => [x, i]).sort((p, q) => p[0] - q[0]); const r = []; o.forEach(([, i], j) => (r[i] = j)); return r; }; const ra = rk(a), rb = rk(b), n = a.length, m = (n - 1) / 2; let s = 0, sa = 0, sb = 0; for (let i = 0; i < n; i++) { s += (ra[i] - m) * (rb[i] - m); sa += (ra[i] - m) ** 2; sb += (rb[i] - m) ** 2; } return s / Math.sqrt(sa * sb); };
const r = rng(3), g = () => { let u = 0; while (!u) u = r(); return Math.sqrt(-2 * Math.log(u)) * Math.cos(2 * Math.PI * r()); };
const zs = [new Array(cv.z).fill(0), Array.from({ length: cv.z }, g), Array.from({ length: cv.z }, g)];
const levels = [-1.5, -0.75, 0, 0.75, 1.5], pass = [];
cv.cond.forEach((k, ki) => {
  const rhos = [], spans = [], errs = [];
  for (const z of zs) {
    const meas = levels.map((lv) => { const c = new Array(cv.cond.length).fill(0); c[ki] = lv; const b = steer(cv, z, c, ref, { seed: 7 }); errs.push(Math.abs(b.measured[ki] - lv)); return b.measured[ki]; });
    rhos.push(spearman(levels, meas)); spans.push(meas[4] - meas[0]);
  }
  const rho = rhos.reduce((a, b) => a + b, 0) / 3, span = spans.reduce((a, b) => a + b, 0) / 3, me = errs.sort((a, b) => a - b)[errs.length >> 1];
  const ok = rho >= 0.8 && span >= 1; if (ok) pass.push(k);
  console.log(`${k.padEnd(11)} ρ ${rho.toFixed(2)} (${rhos.map((x) => x.toFixed(2)).join(' ')})  moves ${span.toFixed(2)}σ  median |target−measured| ${me.toFixed(2)}σ  ${ok ? 'PASS' : 'fail'}`);
});
console.log('passing:', pass.join(', '));
