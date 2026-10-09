// Which way of rolling new snares stays nearest to real snares it has never seen, while staying diverse?
// Library fits from half the corpus build the prior; the other half's real recordings are the reference.
import { readFileSync, readdirSync } from 'fs';
import { render, defaults, toNorm, fromNorm, rng } from '../dsp/engine.js';
import { rollSnareAnchored, rollSnareFlavor, rollSnareLibrary } from '../dsp/snare_roll.js';
import { SN, SN_INT, mono, embed, edist } from './lab.js';
import { readWav } from './lib.js';
const r = rng(22), to48 = (x, sr) => { if (sr === 48000) return x; const q = sr / 48000, n = Math.floor(x.length / q), y = new Float32Array(n); for (let i = 0; i < n; i++) { const t = i * q, j = Math.floor(t); y[i] = x[j] + (t - j) * ((x[j + 1] ?? x[j]) - x[j]); } return y; };
const lib = readdirSync("out/gp/lib").filter((f) => new RegExp(`^${process.env.SRC || "fits"}_\\d+\\.jsonl$`).test(f)).flatMap((f) => readFileSync(`out/gp/lib/${f}`, 'utf8').trim().split('\n').map(JSON.parse)).filter((x) => (x.v2 ?? x.loss) < 8);
const hash = (s) => { let h = 0; for (const c of s) h = (h * 31 + c.charCodeAt(0)) | 0; return h; };
const train = lib.filter((x) => (hash(x.file) & 1) === 0), heldFiles = lib.filter((x) => (hash(x.file) & 1) === 1).map((x) => x.file);
const REF = heldFiles.filter((_, i) => i % 2 === 0).slice(0, 600).map((f) => { const w = readWav(f); return embed(to48(w.mono, w.sr)); });
console.log(train.length, 'train fits,', REF.length, 'unseen real refs');
const A = train.map((x) => x.params), fam = JSON.parse(readFileSync('web/family_fits.json', 'utf8')).snare_eng;
// PCA over normalized continuous knobs
const X = A.map((P) => SN.map((s) => toNorm(s, P[s.id]))), d = SN.length, mu = new Float64Array(d);
X.forEach((v) => v.forEach((u, j) => (mu[j] += u / X.length)));
const C = Array.from({ length: d }, () => new Float64Array(d));
X.forEach((v) => { for (let i = 0; i < d; i++) for (let j = 0; j < d; j++) C[i][j] += ((v[i] - mu[i]) * (v[j] - mu[j])) / (X.length - 1); });
function jacobi(S) { const n = S.length, a = S.map((r) => Float64Array.from(r)), V = Array.from({ length: n }, (_, i) => Float64Array.from({ length: n }, (_, j) => +(i === j)));
  for (let sweep = 0; sweep < 30; sweep++) { let off = 0; for (let p = 0; p < n; p++) for (let q = p + 1; q < n; q++) off += a[p][q] ** 2; if (off < 1e-14) break;
    for (let p = 0; p < n; p++) for (let q = p + 1; q < n; q++) { if (Math.abs(a[p][q]) < 1e-12) continue; const th = (a[q][q] - a[p][p]) / (2 * a[p][q]), t = Math.sign(th || 1) / (Math.abs(th) + Math.sqrt(th * th + 1)), c = 1 / Math.sqrt(t * t + 1), s = t * c;
      for (let k = 0; k < n; k++) { const akp = a[k][p], akq = a[k][q]; a[k][p] = c * akp - s * akq; a[k][q] = s * akp + c * akq; }
      for (let k = 0; k < n; k++) { const apk = a[p][k], aqk = a[q][k]; a[p][k] = c * apk - s * aqk; a[q][k] = s * apk + c * aqk; }
      for (let k = 0; k < n; k++) { const vkp = V[k][p], vkq = V[k][q]; V[k][p] = c * vkp - s * vkq; V[k][q] = s * vkp + c * vkq; } } }
  const ev = a.map((r, i) => r[i]), o = ev.map((_, i) => i).sort((x, y) => ev[y] - ev[x]); return { ev: o.map((i) => ev[i]), vec: o.map((i) => V.map((r) => r[i])) }; }
const { ev, vec } = jacobi(C), tot = ev.reduce((a, b) => a + b, 0); let cum = 0; const k90 = ev.findIndex((e) => (cum += e) / tot >= 0.9) + 1;
console.log('PCA: top-5 var share', ev.slice(0, 5).map((e) => (e / tot).toFixed(3)).join(' '), '| comps for 90%:', k90, 'of', d);
for (let c = 0; c < 4; c++) console.log(`  gene ${c}:`, vec[c].map((w, j) => [w, SN[j].id]).sort((x, y) => Math.abs(y[0]) - Math.abs(x[0])).slice(0, 5).map(([w, id]) => `${w > 0 ? '+' : '-'}${id.replace('sn_', '')}`).join(' '));
const gauss = () => Math.sqrt(-2 * Math.log(r() + 1e-12)) * Math.cos(2 * Math.PI * r()), pick = (a) => a[Math.floor(r() * a.length)];
const fromVec = (u, base) => { const P = { ...defaults(), ...base, engine: 1 }; SN.forEach((s, j) => (P[s.id] = fromNorm(s, Math.max(0, Math.min(1, u[j]))))); return P; };
const pcaSample = (k, scale = 1) => { const u = Float64Array.from(mu); for (let c = 0; c < k; c++) { const z = gauss() * Math.sqrt(ev[c]) * scale; for (let j = 0; j < d; j++) u[j] += z * vec[c][j]; } return fromVec(u, pick(A)); };
const GROUPS = [...new Set([...SN, ...SN_INT].map((s) => s.g))];
const S0 = {
  'app before (fam ±0.08 75% / flavors)': () => (r() < 0.75 ? rollSnareAnchored(defaults(), fam, r, new Set(), 0.08) : rollSnareFlavor(defaults(), r)),
  'app now (library 80% / fam / flavors)': () => (r() < 0.8 ? rollSnareLibrary(defaults(), A, r) : r() < 0.5 ? rollSnareAnchored(defaults(), fam, r) : rollSnareFlavor(defaults(), r)),
};
const S = process.env.ONLY ? S0 : {
  'library fits, no jitter': () => ({ ...pick(A) }),
  'library anchors ±0.05': () => rollSnareAnchored(defaults(), A, r, new Set(), 0.05),
  'crossover + blend': () => { const a = pick(A), b = pick(A), P = { ...a }; for (const g of GROUPS) if (r() < 0.5) for (const s of [...SN, ...SN_INT]) if (s.g === g) P[s.id] = b[s.id]; const t = 0.2 * r(); for (const s of SN) P[s.id] = fromNorm(s, (1 - t) * toNorm(s, P[s.id]) + t * toNorm(s, (r() < 0.5 ? a : b)[s.id])); return P; },
  uniform: () => { const P = { ...defaults(), engine: 1 }; for (const s of [...SN, ...SN_INT]) P[s.id] = fromNorm(s, r()); return P; },
  'flavors (current mix)': () => rollSnareFlavor(defaults(), r),
  'family anchors ±0.12 (current)': () => rollSnareAnchored(defaults(), fam, r, new Set(), 0.12),
  'library anchors ±0.12': () => rollSnareAnchored(defaults(), A, r, new Set(), 0.12),
  'library anchors ±0.25': () => rollSnareAnchored(defaults(), A, r, new Set(), 0.25),
  'gaussian, full cov': () => pcaSample(d),
  [`gaussian, top ${k90} genes`]: () => pcaSample(k90),
  'crossover (layer-wise)': () => { const a = pick(A), b = pick(A), P = { ...a }; for (const g of GROUPS) if (r() < 0.5) for (const s of [...SN, ...SN_INT]) if (s.g === g) P[s.id] = b[s.id]; return P; },
  'midpoint blend': () => { const a = pick(A), b = pick(A), t = 0.3 + 0.4 * r(); return fromVec(SN.map((s) => (1 - t) * toNorm(s, a[s.id]) + t * toNorm(s, b[s.id])), r() < 0.5 ? a : b); },
};
const N = +(process.env.N || 150), nn = (E) => Math.min(...REF.map((R) => edist(E, R)));
const loo = REF.slice(0, 150).map((E, i) => Math.min(...REF.filter((_, j) => j !== i).map((R) => edist(E, R))));
const med = (a) => [...a].sort((x, y) => x - y)[Math.floor(a.length / 2)].toFixed(2), mean = (a) => (a.reduce((x, y) => x + y, 0) / a.length).toFixed(2);
console.log(`\n${'sampler'.padEnd(32)} near-real(med) near-real(mean) diversity  recall(med)`);
console.log(`${'[real vs other real]'.padEnd(32)} ${med(loo).padStart(8)} ${mean(loo).padStart(14)}`);
for (const [name, f] of Object.entries(S)) {
  const E = []; for (let i = 0; i < N; i++) { const P = { ...f(), engine: 1, len: 700, b_shots: 1, note: 24 + Math.floor(r() * 12), seed: 1 + Math.floor(r() * 9999) }; E.push(embed(mono(render(P)))); }
  const near = E.map(nn), div = []; for (let i = 0; i < 300; i++) { const a = Math.floor(r() * N), b = Math.floor(r() * N); if (a !== b) div.push(edist(E[a], E[b])); }
  const rec = REF.slice(0, 200).map((R) => Math.min(...E.map((e) => edist(e, R))));
  console.log(`${name.padEnd(32)} ${med(near).padStart(8)} ${mean(near).padStart(14)} ${mean(div).padStart(9)} ${med(rec).padStart(11)}`);
}
