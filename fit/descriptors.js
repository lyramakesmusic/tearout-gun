// Exact, named descriptors of a one-shot (mono, 48 kHz), for comparing distributions of rolls against real sounds.
import { fft, SR } from '../dsp/engine.js';
import { shellPitch } from '../dsp/fitstages.js';
const ms = (n) => (n * 1000) / SR;
function bandEnv(x, lo, hi, hop = 96, N = 512) { // per-hop band energy (linear)
  const out = [], re = new Float64Array(N), im = new Float64Array(N), k0 = Math.ceil((lo * N) / SR), k1 = Math.min(N / 2, Math.floor((hi * N) / SR));
  for (let s = 0; s + N <= x.length; s += hop) { re.fill(0); im.fill(0); for (let i = 0; i < N; i++) re[i] = x[s + i] * (0.5 - 0.5 * Math.cos((2 * Math.PI * i) / N)); fft(re, im, false); let e = 0; for (let k = k0; k <= k1; k++) e += re[k] ** 2 + im[k] ** 2; out.push(e + 1e-20); }
  return out;
}
function decayTo(env, db) { let pk = 0, ip = 0; env.forEach((v, i) => { if (v > pk) { pk = v; ip = i; } }); const thr = pk * 10 ** (db / 10); let i = ip; while (i < env.length && env[i] > thr) i++; return { t: ms(i * 96), peak: ms(ip * 96) }; }
export const DESC = ['pitch_hz', 'attack_ms', 'noise_peak_ms', 'decay20_ms', 'decay40_ms', 'low_decay20_ms', 'high_decay20_ms', 'e_sub', 'e_low', 'e_mid', 'e_pres', 'e_air', 'formant_db', 'centroid_early', 'centroid_late', 'crest_db', 'snap', 'click_hf'];
export function describe(x0) {
  let pk = 0; for (const v of x0) pk = Math.max(pk, Math.abs(v));
  let on = 0; while (on < x0.length && Math.abs(x0[on]) < pk * 0.03) on++;
  const x = x0.subarray(Math.max(0, on - 24)), n = Math.min(x.length, Math.floor(0.8 * SR)), y = x.subarray(0, n);
  let ip = 0; for (let i = 0; i < Math.min(n, SR * 0.05); i++) if (Math.abs(y[i]) > Math.abs(y[ip])) ip = i;
  const bb = bandEnv(y, 20, 20000), lo = bandEnv(y, 20, 400), hi = bandEnv(y, 4000, 12000);
  const d = { pitch_hz: shellPitch(y), attack_ms: ms(ip) };
  d.noise_peak_ms = decayTo(hi, -20).peak; d.decay20_ms = decayTo(bb, -20).t; d.decay40_ms = decayTo(bb, -40).t; d.low_decay20_ms = decayTo(lo, -20).t; d.high_decay20_ms = decayTo(hi, -20).t;
  // long-term spectrum shares
  const N = 8192, re = new Float64Array(N), im = new Float64Array(N), P = new Float64Array(N / 2);
  for (let s = 0; s + N <= Math.max(n, N); s += N / 2) { re.fill(0); im.fill(0); for (let i = 0; i < N && s + i < n; i++) re[i] = y[s + i] * (0.5 - 0.5 * Math.cos((2 * Math.PI * i) / N)); fft(re, im, false); for (let k = 0; k < N / 2; k++) P[k] += re[k] ** 2 + im[k] ** 2; }
  const band = (a, b) => { let e = 0; for (let k = Math.ceil((a * N) / SR); k < (b * N) / SR; k++) e += P[k]; return e + 1e-20; }, tot = band(20, 20000), db = (e) => 10 * Math.log10(e);
  d.e_sub = db(band(20, 100) / tot); d.e_low = db(band(100, 400) / tot); d.e_mid = db(band(400, 2000) / tot); d.e_pres = db(band(2000, 8000) / tot); d.e_air = db(band(8000, 20000) / tot);
  d.formant_db = db(band(1000, 2000) / 1000) - 0.5 * (db(band(500, 1000) / 500) + db(band(2000, 4000) / 2000));
  const cent = (a, b) => { const N2 = 4096, r2 = new Float64Array(N2), i2 = new Float64Array(N2); for (let i = 0; i < N2 && a + i < Math.min(b, n); i++) r2[i] = y[a + i] * (0.5 - 0.5 * Math.cos((2 * Math.PI * i) / N2)); fft(r2, i2, false); let s = 0, w = 0; for (let k = 1; k < N2 / 2; k++) { const m = r2[k] ** 2 + i2[k] ** 2; s += m * Math.log2((k * SR) / N2); w += m; } return s / (w || 1); };
  d.centroid_early = cent(0, Math.floor(0.05 * SR)); d.centroid_late = cent(Math.floor(0.05 * SR), Math.floor(0.2 * SR)); // log2 Hz
  let rms = 0; const m100 = Math.min(n, Math.floor(0.1 * SR)); for (let i = 0; i < m100; i++) rms += y[i] ** 2; rms = Math.sqrt(rms / m100);
  d.crest_db = 20 * Math.log10(pk / (rms + 1e-12));
  let e3 = 0, e30 = 0; for (let i = 0; i < Math.min(n, SR * 0.03); i++) { e30 += y[i] ** 2; if (i < SR * 0.003) e3 += y[i] ** 2; } d.snap = e3 / (e30 + 1e-20);
  const h5 = bandEnv(y.subarray(0, Math.min(n, 2048)), 8000, 20000, 512), b5 = bandEnv(y.subarray(0, Math.min(n, 2048)), 20, 20000, 512); d.click_hf = db(h5[0] / b5[0]);
  return d;
}
