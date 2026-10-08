// Decoder for the 8-d gun-space VAE: z -> normalized params -> synth params.
import { SPEC, fromNorm } from './engine.js';

export const VAE_IDS = SPEC.filter((s) => !['len', 'seed'].includes(s.id) && s.g !== 'burst' && !s.g.startsWith('sample')).map((s) => s.id);

// erf (Abramowitz–Stegun 7.1.26) for exact-GELU parity with the trained model
function erf(x) {
  const s = Math.sign(x); x = Math.abs(x);
  const t = 1 / (1 + 0.3275911 * x);
  return s * (1 - (((((1.061405429 * t - 1.453152027) * t) + 1.421413741) * t - 0.284496736) * t + 0.254829592) * t * Math.exp(-x * x));
}
const gelu = (x) => 0.5 * x * (1 + erf(x / Math.SQRT2));

export function decodeNorm(vae, z) {
  let h = Float64Array.from(z);
  vae.layers.forEach((L, li) => {
    const out = new Float64Array(L.b.length);
    for (let i = 0; i < out.length; i++) { let s = L.b[i]; const w = L.W[i]; for (let j = 0; j < h.length; j++) s += w[j] * h[j]; out[i] = s; }
    h = li < vae.layers.length - 1 ? out.map(gelu) : out.map((v) => 1 / (1 + Math.exp(-v)));
  });
  return h;
}

// The decoder's output order is the param list frozen into vae.json at training time.
export function decode(vae, z, base = {}) {
  const u = decodeNorm(vae, z), P = { ...base };
  (vae.ids || VAE_IDS).forEach((id, i) => { const s = SPEC.find((x) => x.id === id); if (s) P[id] = fromNorm(s, u[i]); });
  return P;
}
