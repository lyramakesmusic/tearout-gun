// Snare reference from the one-shot corpus: gun-ness features (median/MAD) + 1/3-octave target.
import { readFileSync, writeFileSync } from 'fs';
import { features, FEATURE_KEYS } from '../dsp/gunness.js';
import { readWav } from './lib.js';
import { thirdOct } from '../web/worker.js';
const SR = 48000, C = JSON.parse(readFileSync('analysis/snares/corpus.json', 'utf8'));
const to48 = (x, sr) => { if (sr === SR) return x; const r = sr / SR, n = Math.floor(x.length / r), y = new Float32Array(n); for (let i = 0; i < n; i++) { const t = i * r, j = Math.floor(t); y[i] = x[j] + (t - j) * ((x[j + 1] ?? x[j]) - x[j]); } return y; };
const pick = C.filter((_, i) => i % 6 === 0); // ~670 snares, spread across packs
const F = [], T = [];
for (const r of pick) {
  try {
    let x = to48(readWav(r.path).mono, r.sr); let pk = 0; for (const v of x) pk = Math.max(pk, Math.abs(v)); let i = 0; while (i < x.length && Math.abs(x[i]) < pk * 0.03) i++; x = x.subarray(i);
    F.push(features(x)); T.push(thirdOct(x.subarray(0, Math.min(x.length, SR)), SR).db);
  } catch {}
}
const med = (a) => { const s = [...a].sort((x, y) => x - y); return s[s.length >> 1]; };
const ref = { median: {}, mad: {}, n: F.length, source: 'corpus' };
for (const k of FEATURE_KEYS) { const v = F.map((f) => f[k]), m = med(v); ref.median[k] = m; ref.mad[k] = Math.max(med(v.map((x) => Math.abs(x - m))), 1e-3); }
const fam = JSON.parse(readFileSync('web/family_refs.json', 'utf8')); fam.snare_lyra = fam.snare_lyra || fam.snare; fam.snare = ref;
writeFileSync('web/family_refs.json', JSON.stringify(fam));
const tg = JSON.parse(readFileSync('web/targets.json', 'utf8')), m = T[0].map((_, i) => med(T.map((t) => t[i]))), mx = Math.max(...m);
tg.snare = m.map((v) => Math.round((v - mx) * 10) / 10); writeFileSync('web/targets.json', JSON.stringify(tg));
console.log(F.length, 'snares;', FEATURE_KEYS.map((k) => `${k} ${ref.median[k].toFixed(2)}±${ref.mad[k].toFixed(2)}`).join(' '));
console.log('target', tg.snare.join(' '));
