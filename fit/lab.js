// Fitting lab: shared pieces for comparing fitting algorithms at a fixed render budget.
import { readFileSync, writeFileSync, existsSync } from 'fs';
import { render, SPEC, SPEC_BY_ID, toNorm, fromNorm, defaults, SR } from '../dsp/engine.js';
import { readWav } from './lib.js';
import { melStack, melLoss, ltas, ltasLoss, thirdOct, tobLoss, logMel, cmaes } from '../dsp/fitcore.js';
import { prepTarget } from '../dsp/fitstages.js';
import { rollSnareAnchored, rollSnareFlavor } from '../dsp/snare_roll.js';

export const SN = SPEC.filter((s) => s.eng === 'snare' && s.scale !== 'int');
export const SN_INT = SPEC.filter((s) => s.eng === 'snare' && s.scale === 'int');
const to48 = (x, sr) => { if (sr === SR) return x; const r = sr / SR, n = Math.floor(x.length / r), y = new Float32Array(n); for (let i = 0; i < n; i++) { const t = i * r, j = Math.floor(t); y[i] = x[j] + (t - j) * ((x[j + 1] ?? x[j]) - x[j]); } return y; };
export const mono = (o) => Float32Array.from(o.L, (v, i) => 0.5 * (v + o.R[i]));

// fingerprint: 40-band log-mel, 20 ms hop, 24 frames, dB re own peak floored at −60 (same recipe as analysis/manifold/embed.py)
export function embed(x) {
  let pk = 0; for (const v of x) pk = Math.max(pk, Math.abs(v)); let on = 0; while (on < x.length && Math.abs(x[on]) < pk * 0.05) on++;
  const y = new Float32Array(2048 + 24 * 960); y.set(x.subarray(Math.max(0, on - 48), Math.max(0, on - 48) + y.length));
  const M = logMel(y, SR, 2048, 960, 40, 60, 16000), d = M.data.subarray(0, 24 * 40); let mx = -1e9; for (const v of d) mx = Math.max(mx, v);
  return Float32Array.from(d, (v) => Math.max(v - mx, -60));
}
// energy-weighted |dB| distance between fingerprints
export function edist(a, b) { let s = 0, w = 0; for (let i = 0; i < a.length; i++) { const g = (Math.max(a[i], b[i]) + 60) / 60; s += g * Math.abs(a[i] - b[i]); w += g; } return s / (w || 1); }

// same onset trim prepTarget applies to targets
export function trim(x) { let pk = 0; for (const v of x) pk = Math.max(pk, Math.abs(v)); let i = 0; while (i < x.length && Math.abs(x[i]) < pk * 0.03) i++; return x.subarray(Math.max(0, i - 24)); }
// moving average over ±k log-spaced LTAS bins (the bins are ~1/48 octave apart, so k=4 ≈ 1/6 octave)
export function smooth(v, k = 4) { const o = new Float32Array(v.length); for (let i = 0; i < v.length; i++) { let s = 0, c = 0; for (let j = Math.max(0, i - k); j <= Math.min(v.length - 1, i + k); j++) { s += 10 ** (v[j] / 10); c++; } o[i] = 10 * Math.log10(s / c); } return o; }
// time-smooth each resolution's mel frames after 40 ms (power average over a ~20 ms window); the attack stays sharp
const HOPS = [512, 256, 64];
export function smoothT(stack, winMs = 10, attMs = 40, fwin = 0) {
  return stack.map((M, r) => {
    const hopMs = (HOPS[r] / SR) * 1000, k = Math.max(1, Math.round(winMs / hopMs)), f0 = Math.ceil(attMs / hopMs), d = new Float32Array(M.data.length);
    for (let f = 0; f < M.frames; f++) for (let b = 0; b < M.nmel; b++) {
      if (f < f0) { d[f * M.nmel + b] = M.data[f * M.nmel + b]; continue; }
      let s = 0, c = 0; for (let g = Math.max(f0, f - k); g <= Math.min(M.frames - 1, f + k); g++) for (let bb = Math.max(0, b - fwin); bb <= Math.min(M.nmel - 1, b + fwin); bb++) { s += 10 ** (M.data[g * M.nmel + bb] / 10); c++; }
      d[f * M.nmel + b] = 10 * Math.log10(s / c);
    }
    return { ...M, data: d };
  });
}
export function target(path) { const w = readWav(path); return targetFrom(to48(w.mono, w.sr), path); }
export function targetFrom(x, path = '') {
  const prep = prepTarget(x);
  const lenMs = Math.min(800, Math.max(200, prep.lenMs)), n = Math.floor((lenMs * SR) / 1000), T0 = new Float32Array(n); T0.set(prep.x.subarray(0, n));
  const T = melStack(T0, SR), TL = smooth(ltas(T0, SR)), TT = thirdOct(T0, SR), E = embed(T0);
  let evals = 0;
  // v1: the production loss. v2: renders onset-trimmed like the target, LTAS smoothed to ~1/6 octave (noise texture stops counting)
  const lossV1 = (P) => { evals++; const o = render({ ...P, len: lenMs, b_shots: 1 }); const m = mono(o).subarray(0, n); return melLoss(melStack(m, SR), T).loss + 0.5 * ltasLoss(ltas(m, SR), ltas(T0, SR)) + 0.3 * tobLoss(m, TT, SR); };
  const lossV2 = (P) => { evals++; const o = render({ ...P, len: lenMs + 60, b_shots: 1 }); const m = new Float32Array(n); m.set(trim(mono(o)).subarray(0, n)); return melLoss(melStack(m, SR), T).loss + 0.5 * ltasLoss(smooth(ltas(m, SR)), TL) + 0.3 * tobLoss(m, TT, SR); };
  // v3: v2 + after the first 40 ms each mel frame is averaged with its neighbours (stochastic texture counts as its average), LTAS ~1/3 octave
  const TS = smoothT(T), TL3 = smooth(ltas(T0, SR), 8);
  const lossV3 = (P) => { evals++; const o = render({ ...P, len: lenMs + 60, b_shots: 1 }); const m = new Float32Array(n); m.set(trim(mono(o)).subarray(0, n)); return melLoss(smoothT(melStack(m, SR)), TS).loss + 0.5 * ltasLoss(smooth(ltas(m, SR), 8), TL3) + 0.3 * tobLoss(m, TT, SR); };
  // v4: texture compared as its envelope: after 30 ms, ±25 ms and ±1 mel band power averaging; the fine (256) resolution only judges the attack
  const W = +(process.env.WIN || 25), FW = +(process.env.FWIN || 1), T4 = smoothT(T, W, 30, FW), attF = Math.ceil(30 / ((64 / SR) * 1000));
  const lossV4 = (P) => { evals++; const o = render({ ...P, len: lenMs + 60, b_shots: 1 }); const m = new Float32Array(n); m.set(trim(mono(o)).subarray(0, n)); const S = smoothT(melStack(m, SR), W, 30, FW);
    const fine = melLoss([{ ...S[2], frames: Math.min(S[2].frames, attF) }], [{ ...T4[2], frames: Math.min(T4[2].frames, attF) }]).loss;
    return melLoss([S[0], S[1]], [T4[0], T4[1]]).loss * (2 / 3) + fine / 3 + 0.5 * ltasLoss(smooth(ltas(m, SR), 8), TL3) + 0.3 * tobLoss(m, TT, SR); };
  const loss = { v1: lossV1, v2: lossV2, v3: lossV3, v4: lossV4 }[process.env.LOSS || 'v2'];
  return { path, lenMs, n, T0, E, loss, evals: () => evals, reset: () => (evals = 0) };
}

// CMA over a list of param ids starting from P; returns { P, f }
export function cma(t, P, ids, ev, sigma = 0.2, seed = 3) {
  const specs = ids.map((id) => SPEC_BY_ID[id]);
  const Q = (u) => { const q = { ...P }; specs.forEach((s, i) => (q[s.id] = fromNorm(s, u[i]))); return q; };
  const b = cmaes((u) => t.loss(Q(u)), specs.map((s) => toNorm(s, P[s.id])), { sigma, maxEvals: ev, seed });
  return { P: Q(b.x), f: b.f };
}
// parameters that matter for a patch: drop knobs of layers that are off
export function liveIds(P) {
  const off = { sn_click: P.sn_c_lvl <= -59, sn_metal: P.sn_m_lvl <= -59, sn_clap: P.sn_k_lvl <= -59, sn_room: P.sn_r_lvl <= -59 };
  return SN.filter((s) => !off[s.g] || s.id.endsWith('_lvl')).map((s) => s.id).filter((id) => !['sn_b_time', 'sn_n_width', 'sn_w_amt', 'sn_w_fade', 'sn_w_hp'].includes(id));
}

// bank of rendered patches near the real-sound prior, with fingerprints (cached)
export function bank(n = 1500, file = 'out/bank.json') {
  if (existsSync(file)) { const b = JSON.parse(readFileSync(file, 'utf8')); return b.map((x) => ({ P: x.P, E: Float32Array.from(x.E) })); }
  const ff = JSON.parse(readFileSync('web/family_fits.json', 'utf8')).snare_eng; let a = 12345;
  const r = () => { a |= 0; a = (a + 0x6d2b79f5) | 0; let t = Math.imul(a ^ (a >>> 15), 1 | a); t = (t + Math.imul(t ^ (t >>> 7), 61 | t)) ^ t; return ((t ^ (t >>> 14)) >>> 0) / 4294967296; };
  const out = [];
  for (let i = 0; i < n; i++) {
    const P = i < ff.length ? { ...defaults(), ...ff[i] } : i % 4 === 0 ? rollSnareFlavor(defaults(), r) : rollSnareAnchored(defaults(), ff, r);
    P.len = 700; out.push({ P, E: embed(mono(render({ ...P, b_shots: 1 }))) });
  }
  writeFileSync(file, JSON.stringify(out.map((x) => ({ P: x.P, E: Array.from(x.E, (v) => Math.round(v * 10) / 10) }))));
  return out;
}
export const nearest = (B, E, k = 5) => B.map((b, i) => [edist(b.E, E), i]).sort((x, y) => x[0] - y[0]).slice(0, k).map(([d, i]) => ({ ...B[i], d }));
