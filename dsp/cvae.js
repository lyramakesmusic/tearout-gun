// Feature knobs over the conditional VAE. decode() is the fast open-loop version; steer() closes the loop:
// render, measure the features, push the decoder's condition by the error, repeat, keep the closest.
import { SPEC, fromNorm, render, defaults } from './engine.js';
import { decodeNorm } from './vae.js';
import { features } from './gunness.js';

export function decodeCV(cv, z, c, base = {}) {
  const u = decodeNorm(cv, [...z, ...c]), P = { ...base };
  cv.ids.forEach((id, i) => { const s = SPEC.find((x) => x.id === id); if (s) P[id] = fromNorm(s, u[i]); });
  return P;
}

export function measureC(cv, P, ref) {
  const o = render({ ...defaults(), ...P, b_shots: 1 });
  const f = features(Float32Array.from(o.L, (v, i) => 0.5 * (v + o.R[i])));
  return cv.cond.map((k) => (f[k] - ref.median[k]) / (1.4826 * ref.mad[k] + 1e-9));
}

// Per-knob loop gain: dims the decoder already honors open-loop (metal, length) are left uncorrected,
// since correcting them fights the others. Chosen by fit/verify_steer.js.
export const LOOP_GAIN = { noisy: 0.8, peaky: 0, move: 0.8, crest: 0.8, harsh: 0.8, subMid: 0.8, midSustain: 0, midLate: 0.8 };
export function steer(cv, z, target, ref, base = {}, iters = 4) {
  let cin = target.slice(), best = null;
  const gains = cv.cond.map((k) => LOOP_GAIN[k] ?? 0.8);
  const scored = cv.cond.map((k) => (LOOP_GAIN[k] ?? 0.8) > 0);
  for (let it = 0; it <= iters; it++) {
    const P = decodeCV(cv, z, cin, base), m = measureC(cv, P, ref);
    const err = target.map((t, i) => t - m[i]), e = Math.sqrt(err.reduce((a, b, i) => a + (scored[i] ? b * b : 0), 0) / err.length);
    if (!best || e < best.e) best = { P, e, measured: m };
    if (it === iters) break;
    cin = cin.map((v, i) => Math.max(-4, Math.min(4, v + gains[i] * err[i])));
  }
  return best;
}
