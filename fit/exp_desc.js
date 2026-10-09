// Per-descriptor distribution gap between rolls and unseen real snares: Wasserstein-1 / real IQR, and median shift.
import { readFileSync, readdirSync } from 'fs';
import { readWav } from './lib.js';
import { describe, DESC } from './descriptors.js';
const to48 = (x, sr) => { if (sr === 48000) return x; const q = sr / 48000, n = Math.floor(x.length / q), y = new Float32Array(n); for (let i = 0; i < n; i++) { const t = i * q, j = Math.floor(t); y[i] = x[j] + (t - j) * ((x[j + 1] ?? x[j]) - x[j]); } return y; };
const J = JSON.parse(readFileSync('out/gp/rollab/list.json', 'utf8')), corpus = JSON.parse(readFileSync('analysis/snares/corpus.json', 'utf8'));
const real = J.held.filter((_, i) => i % 3 === 0).map((i) => describe(to48(readWav(corpus[i].path).mono, readWav(corpus[i].path).sr)));
const by = {}; for (const x of J.items) (by[x.method] ||= []).push(describe(readWav(x.wav).mono));
const q = (a, p) => a[Math.floor(p * (a.length - 1))], sorted = (rows, k) => rows.map((d) => d[k]).filter(Number.isFinite).sort((a, b) => a - b);
function w1(a, b) { let s = 0; for (let i = 0; i < 100; i++) s += Math.abs(q(a, (i + 0.5) / 100) - q(b, (i + 0.5) / 100)); return s / 100; }
const methods = Object.keys(by);
console.log(`${real.length} real refs\n${'descriptor'.padEnd(16)} ${'real p10/50/90'.padEnd(24)} ` + methods.map((m) => `${m}: W1/IQR  med`.padEnd(22)).join(''));
const tot = Object.fromEntries(methods.map((m) => [m, 0]));
for (const k of DESC) {
  const R = sorted(real, k), iqr = q(R, 0.75) - q(R, 0.25) || 1, f = (v) => (Math.abs(v) >= 100 ? v.toFixed(0) : v.toFixed(2));
  let line = `${k.padEnd(16)} ${[0.1, 0.5, 0.9].map((p) => f(q(R, p))).join(' / ').padEnd(24)} `;
  for (const m of methods) { const A = sorted(by[m], k), g = w1(A, R) / iqr; tot[m] += g; line += `${g.toFixed(2).padStart(6)} ${f(q(A, 0.5)).padStart(8)}`.padEnd(22); }
  console.log(line);
}
console.log('sum'.padEnd(41) + methods.map((m) => tot[m].toFixed(2).padStart(6).padEnd(22)).join(''));
