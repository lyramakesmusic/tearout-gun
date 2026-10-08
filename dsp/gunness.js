// How gun-like a render is: distance of its feature vector from the fitted presets' distribution.
// Features separate guns from kicks: guns are mid-dominant with sustained, moving, partial-rich mids;
// kicks are sub-dominant with mids that die in a few ms.
import { fft, SR } from './engine.js';

const N = 2048, HOP = 256, win = Float64Array.from({ length: N }, (_, i) => 0.5 - 0.5 * Math.cos((2 * Math.PI * i) / N));
const bin = (f) => Math.round((f * N) / SR);

export function features(x) {
  x = x.subarray(0, Math.floor(0.6 * SR));
  const re = new Float64Array(N), im = new Float64Array(N), fr = [];
  for (let o = 0; o + N <= x.length; o += HOP) {
    for (let i = 0; i < N; i++) { re[i] = x[o + i] * win[i]; im[i] = 0; }
    fft(re, im, false);
    const p = new Float64Array(N / 2); for (let k = 0; k < N / 2; k++) p[k] = re[k] ** 2 + im[k] ** 2 + 1e-18;
    fr.push(p);
  }
  const bandE = (p, a, b) => { let s = 0; for (let k = bin(a); k < bin(b); k++) s += p[k]; return s; };
  const tot = fr.map((p) => bandE(p, 30, 16000)), emax = Math.max(...tot), act = fr.filter((_, i) => tot[i] > emax * 1e-3);
  const q = (a, f) => { const s = [...a].sort((x, y) => x - y); return s[Math.floor(f * (s.length - 1))]; };
  const db = (v) => 10 * Math.log10(v + 1e-18);
  // noisiness, movement
  const flat = act.map((p) => { let lg = 0, ar = 0; const a = bin(400), b = bin(10000); for (let k = a; k < b; k++) { lg += Math.log(p[k]); ar += p[k]; } return Math.exp(lg / (b - a)) / (ar / (b - a)); });
  const cen = act.map((p) => { let a = 0, b = 0; for (let k = bin(100); k < bin(16000); k++) { a += k * p[k]; b += p[k]; } return Math.log2((a / b) * SR / N); });
  // partial prominence on the long-term spectrum
  const lt = new Float64Array(N / 2); for (const p of act) for (let k = 0; k < N / 2; k++) lt[k] += p[k];
  const ldb = Array.from(lt, db); let pk = 0, c = 0;
  for (let k = bin(400); k < bin(8000); k++) { const w = Math.max(1, Math.round(k * 0.115)); const seg = ldb.slice(k - w, k + w + 1).sort((a, b) => a - b); pk += Math.max(0, ldb[k] - seg[seg.length >> 1]); c++; }
  // mid band envelope: sustain and late energy
  const mid = fr.map((p) => bandE(p, 300, 3000)), mpk = Math.max(...mid), ip = mid.indexOf(mpk);
  let j = ip; while (j < mid.length - 1 && mid[j] > mpk * 0.01) j++;
  const t60 = Math.round(0.06 * SR / HOP);
  const early = mid.slice(0, t60).reduce((a, b) => a + b, 0), late = mid.slice(t60).reduce((a, b) => a + b, 0);
  // crest over active region
  let peak = 0, ss = 0; const n = Math.min(x.length, act.length * HOP); for (let i = 0; i < n; i++) { peak = Math.max(peak, Math.abs(x[i])); ss += x[i] ** 2; }
  return {
    noisy: q(flat, 0.5), peaky: pk / c, move: q(cen, 0.9) - q(cen, 0.1),
    crest: 20 * Math.log10(peak / Math.sqrt(ss / Math.max(1, n))),
    harsh: db(bandE(lt, 2500, 6000)) - db(bandE(lt, 500, 2000)),
    subMid: db(bandE(lt, 35, 90)) - db(bandE(lt, 300, 3000)),
    midSustain: ((j - ip) * HOP / SR) * 1000,
    midLate: db(late) - db(early),
  };
}

export const FEATURE_KEYS = ['noisy', 'peaky', 'move', 'crest', 'harsh', 'subMid', 'midSustain', 'midLate'];

// ref: { median: {k: v}, mad: {k: v} }. Lower is more gun-like. Mean of clipped robust z-scores.
export function gunScore(f, ref) {
  let s = 0;
  for (const k of FEATURE_KEYS) s += Math.min(4, Math.abs(f[k] - ref.median[k]) / (1.4826 * ref.mad[k] + 1e-9));
  return s / FEATURE_KEYS.length;
}
