// 808 engine: sub (dive + drift) with FM and an optional noise click → saturation → tone → OTT → EQ → clip.
import { SR, rng, biquad, lr4, shape, ottCore, rmsOf, dbToLin, midiHz, TAU } from './engine.js';

function subVoice(p, n, sr) {
  const base = p.note + 12 * Math.round(p.e8_s_oct) + p.e8_s_tune, td = p.e8_s_dive_ms / 1000, tr = p.e8_s_drift_ms / 1000, tc = p.e8_n_click_ms / 1000;
  const fd = p.e8_f_dec / 1000, x = new Float32Array(n);
  let ph = 0, pm = 0, prevM = 0;
  for (let i = 0; i < n; i++) {
    const t = i / sr;
    const st = base + p.e8_s_dive * Math.exp(-t / td) + p.e8_s_drift * Math.exp(-t / tr) + p.e8_n_click * Math.exp(-t / tc);
    const f = Math.min(midiHz(st), sr * 0.45);
    ph += f / sr; if (ph >= 1) ph -= 1;
    // phase modulation: the modulator runs on its own phase at ratio × the carrier's frequency, with self-feedback
    pm += (f * p.e8_f_ratio) / sr; if (pm >= 1) pm -= 1;
    const I = p.e8_f_amt * (p.e8_f_sustain + (1 - p.e8_f_sustain) * Math.exp(-t / fd));
    let m = 0;
    if (I > 1e-4) { prevM = Math.sin(TAU * pm + p.e8_f_fb * 1.5 * prevM); m = I * prevM; }
    let q = ph + m / TAU; q -= Math.floor(q);
    const sine = Math.sin(TAU * q), tri = 1 - 4 * Math.abs(((q + 0.25) % 1) - 0.5);
    x[i] = (1 - p.e8_s_shape) * sine + p.e8_s_shape * tri;
  }
  return x;
}
// attack → hold → exponential decay (dec = time to −60 dB) → gate at `length` with a raised-cosine release
function ampEnv(p, n, sr) {
  const e = new Float32Array(n), A = (p.e8_s_att * sr) / 1000, H = A + (p.e8_s_hold * sr) / 1000, D = Math.max(1, (p.e8_s_dec * sr) / 1000);
  const G = (p.e8_s_gate * sr) / 1000, R = Math.max(1, (p.e8_s_rel * sr) / 1000);
  for (let i = 0; i < n; i++) {
    let v = i < A ? i / A : i < H ? 1 : Math.exp((-6.9 * (i - H)) / D);
    if (i > G) { const r = (i - G) / R; v *= r >= 1 ? 0 : 0.5 + 0.5 * Math.cos(Math.PI * r); }
    e[i] = v;
  }
  return e;
}
function noiseVoice(p, n, sr, r) {
  const x = new Float32Array(n), D = Math.max(1, (p.e8_n_dec * sr) / 1000);
  for (let i = 0; i < Math.min(n, D * 8); i++) x[i] = (r() * 2 - 1) * Math.exp(-i / D);
  biquad(x, 'hp', p.e8_n_hp, 0.707, 0, sr); biquad(x, 'hp', p.e8_n_hp, 0.707, 0, sr);
  return x;
}
// drive falls over the hit (gritty attack, cleaner tail): input gain follows exp(−t/250 ms) by `drive falls`
function saturate(x, p, sr) {
  const t = Math.round(p.e8_p_type), d = p.e8_p_drive, f = p.e8_p_drive_fall ?? 0;
  if (f > 0.001) for (let i = 0; i < x.length; i++) x[i] *= dbToLin(-d * f * (1 - Math.exp(-i / (0.25 * sr))));
  if (t === 3) { // tube: asymmetric, adds even harmonics
    const g = dbToLin(d), b = 0.3, o = Math.tanh(b);
    for (let i = 0; i < x.length; i++) x[i] = Math.tanh(g * x[i] + b) - o;
    return x;
  }
  return shape(x, d, t === 1 ? 1 : 0, t === 2 ? 0.6 : 0);
}

export function render808(p, opts = {}) {
  const sr = opts.sr || SR, n = Math.floor(((p.e8_s_gate + p.e8_s_rel + 30) * sr) / 1000), solo = opts.solo;
  const env = ampEnv(p, n, sr), sub = subVoice(p, n, sr), nz = noiseVoice(p, n, sr, rng((p.seed | 0) * 29 + 5));
  const gs = solo && solo !== 'sub' ? 0 : dbToLin(p.e8_s_lvl), gn = solo && solo !== 'noise' || p.e8_n_lvl <= -59 ? 0 : dbToLin(p.e8_n_lvl);
  const dry = new Float32Array(n);
  for (let i = 0; i < n; i++) dry[i] = (gs * sub[i] + gn * 0.5 * nz[i]) * env[i];
  // saturation on a peak-normalized copy, so drive means the same thing at any level
  let pk = 0; for (const v of dry) pk = Math.max(pk, Math.abs(v));
  const pre = opts.fixed ? opts.fixed.pre : pk > 0 ? 1 / pk : 1, x = Float32Array.from(dry, (v) => v * pre);
  const wet = saturate(x.slice(), p, sr);
  biquad(wet, 'hp', 12, 0.707, 0, sr); // asymmetric (tube) saturation leaves DC; remove it so the cutoff doesn't thump
  if (p.e8_p_tone < 19900) lr4(wet, 'lp', p.e8_p_tone, sr);
  // clean sub: below the crossover, blend back the undistorted sine at the same level
  if (p.e8_p_clean > 0.001) {
    const lowW = lr4(wet.slice(), 'lp', p.e8_p_xover, sr), lowD = lr4(x.slice(), 'lp', p.e8_p_xover, sr), hiW = lr4(wet, 'hp', p.e8_p_xover, sr);
    const m = rmsOf(lowW) / (rmsOf(lowD) || 1), c = p.e8_p_clean;
    for (let i = 0; i < n; i++) wet[i] = hiW[i] + (1 - c) * lowW[i] + c * m * lowD[i];
  }
  const R = wet.slice();
  ottCore(wet, R, p.e8_p_ott, 0.6, 1, sr);
  if (Math.abs(p.e8_p_low) > 0.05) biquad(wet, 'ls', 80, 0.7, p.e8_p_low, sr);
  if (Math.abs(p.e8_p_mid) > 0.05) biquad(wet, 'peak', p.e8_p_mid_f, 0.9, p.e8_p_mid, sr);
  // final soft clip, then −0.3 dBFS
  shape(wet, p.e8_p_clip, 0.2);
  let pk2 = 0; for (const v of wet) pk2 = Math.max(pk2, Math.abs(v));
  const g = opts.fixed ? opts.fixed.g : pk2 > 0 ? 0.966 / pk2 : 1;
  const F = Math.floor(0.004 * sr);
  for (let i = 0; i < n; i++) wet[i] *= g * (i > n - F ? (n - i) / F : 1);
  return { L: wet, R: wet.slice(), sr, layers: {}, gains: { pre, g } };
}
