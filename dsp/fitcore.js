// Fitting math shared by offline scripts and the browser worker: multi-resolution log-mel, losses, CMA-ES.
import { fft } from './engine.js';
// ---------------------------------------------------------------- log-mel
const melCache = new Map();
function melBank(nfft, sr, nmel, fmin, fmax) {
  const key = `${nfft}|${sr}|${nmel}|${fmin}`;
  if (melCache.has(key)) return melCache.get(key);
  const hz2mel = (f) => 2595 * Math.log10(1 + f / 700), mel2hz = (m) => 700 * (10 ** (m / 2595) - 1);
  const pts = Array.from({ length: nmel + 2 }, (_, i) => mel2hz(hz2mel(fmin) + (i * (hz2mel(fmax) - hz2mel(fmin))) / (nmel + 1)));
  const bins = nfft / 2 + 1, bank = [];
  for (let m = 0; m < nmel; m++) {
    const w = new Float32Array(bins), [a, c, d] = [pts[m], pts[m + 1], pts[m + 2]];
    for (let k = 0; k < bins; k++) { const f = (k * sr) / nfft; w[k] = f > a && f < d ? (f <= c ? (f - a) / (c - a) : (d - f) / (d - c)) : 0; }
    // area-normalize so band energy is comparable across widths
    const s = w.reduce((x, y) => x + y, 0) || 1; for (let k = 0; k < bins; k++) w[k] /= s;
    const lo = w.findIndex((x) => x > 0);
    const hi = lo < 0 ? 0 : bins - [...w].reverse().findIndex((x) => x > 0);
    bank.push({ w, lo: Math.max(0, lo), hi });
  }
  melCache.set(key, bank);
  return bank;
}
const winCache = new Map();
const hann = (n) => { if (!winCache.has(n)) winCache.set(n, Float64Array.from({ length: n }, (_, i) => 0.5 - 0.5 * Math.cos((2 * Math.PI * i) / n))); return winCache.get(n); };

// returns { frames, nmel, data: Float32Array(frames*nmel) } in dB
export function logMel(x, sr, nfft = 1024, hop = 256, nmel = 64, fmin = 30, fmax = 16000) {
  const bank = melBank(nfft, sr, nmel, Math.max(fmin, (3 * sr) / nfft), fmax), w = hann(nfft);
  const frames = Math.max(1, Math.floor((x.length - nfft) / hop) + 1), data = new Float32Array(frames * nmel);
  const re = new Float64Array(nfft), im = new Float64Array(nfft), pw = new Float64Array(nfft / 2 + 1);
  for (let f = 0; f < frames; f++) {
    for (let i = 0; i < nfft; i++) { re[i] = (x[f * hop + i] || 0) * w[i]; im[i] = 0; }
    fft(re, im, false);
    for (let k = 0; k <= nfft / 2; k++) pw[k] = re[k] * re[k] + im[k] * im[k];
    for (let m = 0; m < nmel; m++) {
      const { w: bw, lo, hi } = bank[m]; let s = 0;
      for (let k = lo; k < hi; k++) s += bw[k] * pw[k];
      data[f * nmel + m] = 10 * Math.log10(s + 1e-12);
    }
  }
  return { frames, nmel, data };
}

export const RES = [[2048, 512, 80], [1024, 256, 64], [256, 64, 32]];
export function melStack(x, sr, fmax = 16000) { return RES.map(([n, h, m]) => logMel(x, sr, n, h, fmax < 8000 ? Math.round(m * 0.6) : m, 30, fmax)); }

// Gain-invariant L1 between log-mels. Both clamped to a floor relative to the target's max,
// the offset between them is the median difference over target-active cells, removed before L1.
export function melLoss(A, T, floorDb = 70) {
  let total = 0, offsets = [];
  for (let r = 0; r < T.length; r++) {
    const a = A[r], t = T[r], n = Math.min(a.frames, t.frames) * t.nmel;
    let tmax = -Infinity; for (let i = 0; i < n; i++) tmax = Math.max(tmax, t.data[i]);
    const fl = tmax - floorDb, diffs = [];
    for (let i = 0; i < n; i++) if (t.data[i] > fl + 20) diffs.push(t.data[i] - a.data[i]);
    diffs.sort((p, q) => p - q);
    const off = diffs.length ? diffs[diffs.length >> 1] : 0; offsets.push(off);
    let s = 0;
    for (let i = 0; i < n; i++) { const tv = Math.max(t.data[i], fl), av = Math.max(a.data[i] + off, fl); s += Math.abs(tv - av); }
    total += s / n;
  }
  return { loss: total / T.length, gainDb: offsets[1] };
}

// ---------------------------------------------------------------- sub pitch track (zero crossings, monotone branch)
// Returns semitone values (re 1 Hz... as 12*log2(hz)) sampled on a 0.5 ms grid over the first maxMs after onset.
export function pitchTrack(x, sr, maxMs = 160) {
  let pk = 0; for (const v of x) pk = Math.max(pk, Math.abs(v));
  let i0 = 0; while (i0 < x.length && Math.abs(x[i0]) < 0.02 * pk) i0++;
  const end = Math.min(x.length - 1, i0 + Math.floor((maxMs / 1000) * sr * 1.5));
  const zc = [];
  for (let i = i0; i < end; i++) if (x[i] < 0 && x[i + 1] >= 0) zc.push(i + -x[i] / (x[i + 1] - x[i] + 1e-12) - i0);
  const t = [], hz = [];
  for (let k = 1; k < zc.length; k++) { t.push(zc[k] / sr); hz.push(sr / (zc[k] - zc[k - 1])); }
  if (hz.length < 6) return null;
  const tail = hz.slice(Math.floor(hz.length * 0.6)).sort((a, b) => a - b)[Math.floor(hz.length * 0.2)];
  const kt = [], kh = []; let cur = Infinity;
  for (let k = 0; k < hz.length; k++) if (hz[k] <= 1.15 * cur && hz[k] > 0.6 * tail) { kt.push(t[k]); kh.push(12 * Math.log2(hz[k])); cur = hz[k]; }
  const grid = new Float32Array(Math.floor(maxMs * 2)).fill(NaN);
  let j = 0;
  for (let g = 0; g < grid.length; g++) {
    const tg = g / 2000;
    while (j < kt.length - 1 && kt[j + 1] < tg) j++;
    if (tg >= kt[0] && j < kt.length - 1) { const f = (tg - kt[j]) / (kt[j + 1] - kt[j]); grid[g] = kh[j] + f * (kh[j + 1] - kh[j]); }
  }
  return grid;
}
// Robust variant for distorted / hissy subs: track several zero-phase-lowpassed copies; at each grid
// point use the most-lowpassed copy whose cutoff is >= 2.5x the estimate (fundamental in passband,
// harmonics and hiss attenuated). Then keep the monotone (non-rising) branch.
const CUTS = [Infinity, 12000, 6000, 3000, 1500, 700, 300];
function zpLowpass(x, fc, sr) {
  const y = Float32Array.from(x), a = Math.exp((-2 * Math.PI * fc) / sr);
  for (let p = 0; p < 3; p++) {
    let s = 0; for (let i = 0; i < y.length; i++) { s = (1 - a) * y[i] + a * s; y[i] = s; }
    s = 0; for (let i = y.length - 1; i >= 0; i--) { s = (1 - a) * y[i] + a * s; y[i] = s; }
  }
  return y;
}
function rawGrid(x, sr, maxMs, i0) {
  const end = Math.min(x.length - 1, i0 + Math.floor((maxMs / 1000) * sr * 1.2));
  const zc = [];
  for (let i = i0; i < end; i++) if (x[i] < 0 && x[i + 1] >= 0) zc.push(i + -x[i] / (x[i + 1] - x[i] + 1e-12) - i0);
  const grid = new Float32Array(Math.floor(maxMs * 2)).fill(NaN);
  let k = 1;
  for (let g = 0; g < grid.length; g++) {
    const ts = (g / 2000) * sr;
    while (k < zc.length - 1 && zc[k] < ts) k++;
    if (k < zc.length && zc[k - 1] <= ts + 1e-9 && zc[k] >= ts) grid[g] = 12 * Math.log2(sr / (zc[k] - zc[k - 1]));
  }
  return grid;
}
export function pitchTrackRobust(x, sr, maxMs = 300) {
  let pk = 0; for (const v of x) pk = Math.max(pk, Math.abs(v));
  let i0 = 0; while (i0 < x.length && Math.abs(x[i0]) < 0.02 * pk) i0++;
  const seg = x.subarray(0, Math.min(x.length, i0 + Math.floor((maxMs / 1000) * sr * 1.3)));
  const grids = CUTS.map((c) => rawGrid(c === Infinity ? seg : zpLowpass(seg, c, sr), sr, maxMs, i0));
  const out = new Float32Array(grids[0].length).fill(NaN);
  for (let g = 0; g < out.length; g++) {
    for (let j = CUTS.length - 1; j >= 0; j--) {
      const v = grids[j][g];
      if (Number.isFinite(v) && 2 ** (v / 12) * 2.5 <= CUTS[j]) { out[g] = v; break; }
    }
  }
  // monotone branch, walking backward from the tail: values may only rise (with 1 st slack) going back in time
  let cur = -Infinity;
  for (let g = out.length - 1; g >= 0; g--) {
    if (!Number.isFinite(out[g])) continue;
    if (out[g] >= cur - 1) cur = Math.max(cur, out[g]); else out[g] = NaN;
  }
  return out;
}

// mean absolute semitone error over grid points defined in the target (missing in ours = 24 st penalty)
export function pitchLoss(A, T) {
  if (!T) return 0; if (!A) return 24;
  let s = 0, c = 0;
  for (let g = 0; g < T.length; g++) if (Number.isFinite(T[g])) { c++; s += Number.isFinite(A[g]) ? Math.min(12, Math.abs(A[g] - T[g])) : 6; }
  return c ? s / c : 0;
}

// ---------------------------------------------------------------- CMA-ES (Hansen), box [0,1]^d via clipping + penalty
function randn(r) { let u = 0, v = 0; while (u === 0) u = r(); v = r(); return Math.sqrt(-2 * Math.log(u)) * Math.cos(2 * Math.PI * v); }
function eigSym(A) { // Jacobi
  const n = A.length, a = A.map((r) => r.slice()), V = a.map((_, i) => a.map((_, j) => (i === j ? 1 : 0)));
  for (let sweep = 0; sweep < 60; sweep++) {
    let off = 0; for (let i = 0; i < n; i++) for (let j = i + 1; j < n; j++) off += a[i][j] ** 2;
    if (off < 1e-20) break;
    for (let p = 0; p < n; p++) for (let q = p + 1; q < n; q++) {
      if (Math.abs(a[p][q]) < 1e-15) continue;
      const th = (a[q][q] - a[p][p]) / (2 * a[p][q]), t = Math.sign(th || 1) / (Math.abs(th) + Math.sqrt(th * th + 1));
      const c = 1 / Math.sqrt(t * t + 1), s = t * c;
      for (let k = 0; k < n; k++) { const akp = a[k][p], akq = a[k][q]; a[k][p] = c * akp - s * akq; a[k][q] = s * akp + c * akq; }
      for (let k = 0; k < n; k++) { const apk = a[p][k], aqk = a[q][k]; a[p][k] = c * apk - s * aqk; a[q][k] = s * apk + c * aqk; }
      for (let k = 0; k < n; k++) { const vkp = V[k][p], vkq = V[k][q]; V[k][p] = c * vkp - s * vkq; V[k][q] = s * vkp + c * vkq; }
    }
  }
  return { d: a.map((r, i) => Math.max(1e-20, r[i])), V };
}

export function cmaes(f, x0, { sigma = 0.25, maxEvals = 2000, seed = 1, lambda, onGen } = {}) {
  const r = (() => { let a = seed >>> 0 || 1; return () => { a |= 0; a = (a + 0x6d2b79f5) | 0; let t = Math.imul(a ^ (a >>> 15), 1 | a); t = (t + Math.imul(t ^ (t >>> 7), 61 | t)) ^ t; return ((t ^ (t >>> 14)) >>> 0) / 4294967296; }; })();
  const n = x0.length, lam = lambda || 4 + Math.floor(3 * Math.log(n)), mu = Math.floor(lam / 2);
  let w = Array.from({ length: mu }, (_, i) => Math.log(mu + 0.5) - Math.log(i + 1)); const ws = w.reduce((a, b) => a + b); w = w.map((x) => x / ws);
  const mueff = 1 / w.reduce((a, b) => a + b * b, 0);
  const cc = (4 + mueff / n) / (n + 4 + (2 * mueff) / n), cs = (mueff + 2) / (n + mueff + 5);
  const c1 = 2 / ((n + 1.3) ** 2 + mueff), cmu = Math.min(1 - c1, (2 * (mueff - 2 + 1 / mueff)) / ((n + 2) ** 2 + mueff));
  const damps = 1 + 2 * Math.max(0, Math.sqrt((mueff - 1) / (n + 1)) - 1) + cs, chiN = Math.sqrt(n) * (1 - 1 / (4 * n) + 1 / (21 * n * n));
  let m = x0.slice(), pc = Array(n).fill(0), ps = Array(n).fill(0), C = m.map((_, i) => m.map((_, j) => (i === j ? 1 : 0)));
  let B = C.map((r) => r.slice()), D = Array(n).fill(1), evals = 0, gen = 0, best = { x: x0.slice(), f: Infinity };
  const clip = (x) => x.map((v) => Math.min(1, Math.max(0, v)));
  while (evals < maxEvals) {
    const pop = [];
    for (let k = 0; k < lam; k++) {
      const z = Array.from({ length: n }, () => randn(r));
      const y = B.map((row) => row.reduce((s, b, j) => s + b * D[j] * z[j], 0));
      const x = m.map((mi, i) => mi + sigma * y[i]);
      const xc = clip(x), pen = x.reduce((s, v, i) => s + (v - xc[i]) ** 2, 0);
      const fx = f(xc) + 10 * pen; evals++;
      if (fx < best.f) best = { x: xc, f: fx };
      pop.push({ x, y, f: fx });
    }
    pop.sort((a, b) => a.f - b.f);
    const old = m.slice();
    m = Array.from({ length: n }, (_, i) => pop.slice(0, mu).reduce((s, p, k) => s + w[k] * p.x[i], 0));
    const yw = m.map((v, i) => (v - old[i]) / sigma);
    // C^{-1/2} yw
    const bt = B[0].map((_, j) => B.reduce((s, row, i) => s + row[j] * yw[i], 0) / D[j]);
    const cinv = B.map((row) => row.reduce((s, b, j) => s + b * bt[j], 0));
    ps = ps.map((v, i) => (1 - cs) * v + Math.sqrt(cs * (2 - cs) * mueff) * cinv[i]);
    const psn = Math.sqrt(ps.reduce((s, v) => s + v * v, 0));
    const hsig = psn / Math.sqrt(1 - (1 - cs) ** (2 * (gen + 1))) / chiN < 1.4 + 2 / (n + 1) ? 1 : 0;
    pc = pc.map((v, i) => (1 - cc) * v + hsig * Math.sqrt(cc * (2 - cc) * mueff) * yw[i]);
    for (let i = 0; i < n; i++) for (let j = 0; j <= i; j++) {
      let rk = 0; for (let k = 0; k < mu; k++) rk += w[k] * pop[k].y[i] * pop[k].y[j];
      C[i][j] = C[j][i] = (1 - c1 - cmu) * C[i][j] + c1 * (pc[i] * pc[j] + (1 - hsig) * cc * (2 - cc) * C[i][j]) + cmu * rk;
    }
    sigma *= Math.exp((cs / damps) * (psn / chiN - 1));
    sigma = Math.min(sigma, 0.6);
    const e = eigSym(C); B = e.V; D = e.d.map(Math.sqrt);
    gen++;
    if (onGen) onGen({ gen, evals, best: best.f, genBest: pop[0].f, sigma });
    if (sigma * Math.max(...D) < 1e-4) break;
  }
  return best;
}

// ---------------------------------------------------------------- long-term average spectrum
// 8192-pt Welch average in dB, lightly smoothed (1/24 octave) above 60 Hz. Noise-robust, partial-sensitive.
export function ltas(x, sr, nfft = 8192, hop = 2048) {
  const w = hann(nfft), re = new Float64Array(nfft), im = new Float64Array(nfft), acc = new Float64Array(nfft / 2 + 1);
  let frames = 0;
  for (let o = 0; o + nfft <= Math.max(x.length, nfft); o += hop) {
    for (let i = 0; i < nfft; i++) { re[i] = (x[o + i] || 0) * w[i]; im[i] = 0; }
    fft(re, im, false);
    for (let k = 0; k <= nfft / 2; k++) acc[k] += re[k] * re[k] + im[k] * im[k];
    frames++;
  }
  const k0 = Math.ceil((60 * nfft) / sr), k1 = Math.floor((16000 * nfft) / sr), out = [];
  for (let k = k0; k <= k1; k++) {
    const half = Math.max(0, Math.floor(k * (2 ** (1 / 48) - 1)));
    let s = 0, c = 0; for (let j = k - half; j <= k + half; j++) { s += acc[j]; c++; }
    out.push(10 * Math.log10(s / c / frames + 1e-14));
  }
  return Float32Array.from(out);
}
// gain-invariant L1 on LTAS (offset = median diff over bins within 60 dB of target max)
export function ltasLoss(a, t, floorDb = 60) {
  let tmax = -Infinity; for (const v of t) tmax = Math.max(tmax, v);
  const fl = tmax - floorDb, d = [];
  for (let i = 0; i < t.length; i++) if (t[i] > fl) d.push(t[i] - a[i]);
  d.sort((p, q) => p - q); const off = d.length ? d[d.length >> 1] : 0;
  let s = 0; for (let i = 0; i < t.length; i++) s += Math.abs(Math.max(t[i], fl) - Math.max(a[i] + off, fl));
  return s / t.length;
}

// Tonal balance: mean |dB| over 27 third-octave bands (39 Hz–16 kHz), each side normalized to its loudest band.
// Equal weight per band = log-frequency weighting; sees sub-vs-mids balance that median-offset losses wash out.
export { thirdOct } from '../web/worker.js';
import { thirdOct } from '../web/worker.js';
import { thirdOct as _to } from '../web/worker.js';
export function tobLoss(x, tTob, sr) {
  const a = _to(x, sr).db; let s = 0;
  for (let i = 0; i < a.length; i++) s += Math.abs(Math.max(a[i], -60) - Math.max(tTob.db[i], -60));
  return s / a.length;
}
