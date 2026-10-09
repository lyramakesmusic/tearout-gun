// Inputs for the snare inverse model: a 64×48 log-mel image and a 180-bin log-frequency pitch spectrum, both from the onset.
import { fft, SR, SPEC, fromNorm } from './engine.js';
import { logMel } from './fitcore.js';
export const FRAMES = 64, BANDS = 48, HOP = 480, PBINS = 180, PF0 = 70, PER_OCT = 36;
export function trim(x) { let pk = 0; for (const v of x) pk = Math.max(pk, Math.abs(v)); let i = 0; while (i < x.length && Math.abs(x[i]) < pk * 0.03) i++; return x.subarray(Math.max(0, i - 24)); }
export function melOf(x) {
  const t = trim(x), y = new Float32Array(1024 + FRAMES * HOP); y.set(t.subarray(0, y.length));
  const M = logMel(y, SR, 1024, HOP, BANDS, 30, 16000), d = M.data.subarray(0, FRAMES * BANDS); let mx = -1e9; for (const v of d) mx = Math.max(mx, v);
  return Uint8Array.from(d, (v) => Math.round(Math.max(0, Math.min(255, ((v - mx + 80) / 80) * 255))));
}
export function pitchOf(x) {
  const t = trim(x), N = 16384, re = new Float64Array(N), im = new Float64Array(N), a = Math.floor(0.01 * SR), b = Math.min(t.length, Math.floor(0.2 * SR));
  for (let i = a; i < b; i++) re[i - a] = t[i] * (0.5 - 0.5 * Math.cos((2 * Math.PI * (i - a)) / (b - a)));
  fft(re, im, false);
  const out = new Float32Array(PBINS); let mx = -1e9;
  for (let k = 0; k < PBINS; k++) { const f = PF0 * 2 ** (k / PER_OCT), j = (f * N) / SR, j0 = Math.floor(j), w = j - j0; const m = (1 - w) * Math.hypot(re[j0], im[j0]) + w * Math.hypot(re[j0 + 1], im[j0 + 1]); out[k] = 20 * Math.log10(m + 1e-9); mx = Math.max(mx, out[k]); }
  return Uint8Array.from(out, (v) => Math.round(Math.max(0, Math.min(255, ((v - mx + 60) / 60) * 255))));
}
// model output (continuous knobs in [0,1], then one logit block per switch knob) → patch values
export function decodeInv(meta, out, base) {
  const P = { ...base }, K = meta.knobs, byId = Object.fromEntries(SPEC.map((s) => [s.id, s]));
  meta.cont.forEach((i, j) => { const s = byId[K[i].id]; if (s) P[s.id] = fromNorm(s, out[j]); });
  let o = meta.cont.length;
  for (const [i, n] of meta.ints) { let b = 0; for (let k = 1; k < n; k++) if (out[o + k] > out[o + b]) b = k; o += n; const id = K[i].id; if (id === 'note_pc') P.note = 24 + b; else P[id] = K[i].min + b; }
  return P;
}
