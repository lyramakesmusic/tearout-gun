// Whoosh engine: noise → amp envelope → cutoff sweep (start → peak → end) with wobble → chop → body → width/pass-by → reverb → post.
import { SR, rng, biquad, lr4, shape, ottCore, makeIR, convolveStereo, rmsOf, dbToLin, TAU } from './engine.js';

const ms = (v, sr) => Math.max(1, Math.floor((v * sr) / 1000));
// envelope: curved attack to 1, hold, curved decay to 0 by the end of `dec`
function shapeEnv(n, sr, att, hold, dec, ca, cd) {
  const e = new Float32Array(n), A = (att * sr) / 1000, H = A + (hold * sr) / 1000, D = ms(dec, sr);
  for (let i = 0; i < n; i++) e[i] = i < A ? Math.pow(i / A, ca) : i < H ? 1 : Math.pow(Math.max(0, 1 - (i - H) / D), cd);
  return e;
}
function source(p, n, sr, r) {
  const t = Math.round(p.w_s_type), x = new Float32Array(n);
  if (t === 1) { let b0 = 0, b1 = 0, b2 = 0, b3 = 0, b4 = 0, b5 = 0, b6 = 0; for (let i = 0; i < n; i++) { const w = r() * 2 - 1; b0 = 0.99886 * b0 + w * 0.0555179; b1 = 0.99332 * b1 + w * 0.0750759; b2 = 0.969 * b2 + w * 0.153852; b3 = 0.8665 * b3 + w * 0.3104856; b4 = 0.55 * b4 + w * 0.5329522; b5 = -0.7616 * b5 - w * 0.016898; x[i] = (b0 + b1 + b2 + b3 + b4 + b5 + b6 + w * 0.5362) * 0.11; b6 = w * 0.115926; } }
  else if (t === 2) { let y = 0; for (let i = 0; i < n; i++) { y = 0.98 * y + 0.2 * (r() * 2 - 1); x[i] = y; } }
  else if (t === 3) { const pr = p.w_s_density / sr; for (let i = 0; i < n; i++) if (r() < pr) x[i] = (r() * 2 - 1) * Math.sqrt(r()) * 4; }
  else if (t === 4) { for (let i = 0; i < n; i++) x[i] = (r() * 2 - 1) * 0.3; }
  else for (let i = 0; i < n; i++) x[i] = r() * 2 - 1;
  if (Math.abs(p.w_s_tilt) > 0.01) biquad(x, p.w_s_tilt > 0 ? 'hs' : 'ls', 1500, 0.5, -12 * p.w_s_tilt * (p.w_s_tilt > 0 ? -1 : 1), sr);
  // metal: inharmonic square partials (cymbal), mixed into the source
  if (p.w_s_metal > -59 || t === 4) {
    const g = t === 4 ? 1 : dbToLin(p.w_s_metal), f0 = p.w_s_metal_f, sp = p.w_s_metal_spread, rr = rng(9091);
    // a dozen inharmonic squares (808-cymbal style), smeared by noise so they shimmer instead of ringing as fixed tones
    const fr = [1, 1.483, 1.932, 2.546, 2.630, 3.897, 1.217, 1.722, 2.274, 3.105, 3.512, 4.43].map((k) => f0 * Math.pow(k, 0.4 + 0.6 * sp) * (1 + (rr() - 0.5) * 0.03));
    const am = new Float32Array(n); for (let i = 0; i < n; i++) am[i] = rr() * 2 - 1; biquad(am, 'lp', 900, 0.7, 0, sr);
    const m = new Float32Array(n); for (let i = 0; i < n; i++) { let s = 0; for (const f of fr) s += ((i * f) / sr) % 1 < 0.5 ? 1 : -1; m[i] = (s / 12) * (1 + 6 * am[i]); }
    biquad(m, 'hp', f0 * 0.8, 0.7, 0, sr);
    for (let i = 0; i < n; i++) x[i] += g * m[i];
  }
  return x;
}
// state-variable filter with a per-block cutoff from fc(i)
function svf(x, type, fc, q, sr) {
  const k = 1 / q; let ic1 = 0, ic2 = 0, a1 = 0, a2 = 0, a3 = 0;
  for (let i = 0; i < x.length; i++) {
    if ((i & 15) === 0) { const g = Math.tan((Math.PI * Math.min(fc(i), sr * 0.45)) / sr); a1 = 1 / (1 + g * (g + k)); a2 = g * a1; a3 = g * a2; }
    const v0 = x[i], v3 = v0 - ic2, v1 = a1 * ic1 + a2 * v3, v2 = ic2 + a2 * ic1 + a3 * v3; ic1 = 2 * v1 - ic1; ic2 = 2 * v2 - ic2;
    x[i] = type === 0 ? v2 : type === 1 ? v1 * k : v0 - k * v1 - v2;
  }
  return x;
}

export function renderWhoosh(p, opts = {}) {
  const sr = opts.sr || SR, solo = opts.solo;
  const len = p.w_a_len, n = ms(len + Math.min(p.w_x_size * 0.6, 2500) + 50, sr), N = ms(len, sr);
  const env = shapeEnv(n, sr, Math.min(p.w_a_att, len), p.w_a_hold, Math.max(20, len - Math.min(p.w_a_att, len) - p.w_a_hold), p.w_a_att_curve, p.w_a_dec_curve);
  for (let i = 0; i < n; i++) env[i] *= i < N ? 1 : 0;
  const ch = [0, 1].map((c) => {
    const r = rng((p.seed | 0) * 31 + 7 + c * 1013);
    const x = source(p, n, sr, r), dry = x.slice();
    // cutoff path in log frequency: start → peak (at peak_at × length) → end, plus wobble
    const P = Math.max(1, p.w_f_peak_at * N), ls = Math.log(p.w_f_start), lp = Math.log(p.w_f_peak), le = Math.log(p.w_f_end), lfo = TAU * p.w_m_lfo / sr;
    const fc = (i) => { const u = i < P ? i / P : Math.min(1, (i - P) / Math.max(1, N - P)); const l = i < P ? ls + (lp - ls) * (0.5 - 0.5 * Math.cos(Math.PI * u)) : lp + (le - lp) * (0.5 - 0.5 * Math.cos(Math.PI * u)); return Math.exp(l + p.w_m_lfo_amt * Math.LN2 * Math.sin(lfo * i + c * 0.3)); };
    svf(x, Math.round(p.w_f_type), fc, p.w_f_q, sr);
    const fm = p.w_f_mix; for (let i = 0; i < n; i++) x[i] = (fm * x[i] + (1 - fm) * dry[i] * 0.3) * env[i];
    return x;
  });
  // chop (hype loops): square-ish gate, optional acceleration over the sound
  if (p.w_m_gate_amt > 0.001) {
    let ph = 0; const d = p.w_m_gate_duty;
    for (let i = 0; i < n; i++) { const u = i / N, rate = p.w_m_gate * Math.pow(2, 2 * p.w_m_gate_accel * u); ph += rate / sr; const f = ph % 1, g = f < d ? Math.min(1, f * 200, (d - f) * 200) : 0; const m = 1 - p.w_m_gate_amt * (1 - g); ch[0][i] *= m; ch[1][i] *= m; }
  }
  // stereo: mid = average, side = difference of the two independent noise channels × width (symmetric, centered)
  const L = new Float32Array(n), R = new Float32Array(n), w = p.w_x_width;
  const gs = solo && solo !== 'noise' ? 0 : dbToLin(p.w_s_lvl);
  for (let i = 0; i < n; i++) { const m = 0.5 * (ch[0][i] + ch[1][i]), s = 0.5 * (ch[0][i] - ch[1][i]) * w; L[i] = gs * (m + s); R[i] = gs * (m - s); }
  // pass-by: equal-power pan sweep left → right across the length
  if (p.w_x_pass > 0.005) for (let i = 0; i < n; i++) { const u = Math.min(1, i / N), a = (0.5 + p.w_x_pass * (u - 0.5)) * Math.PI / 2, l = Math.cos(a) * Math.SQRT2, rr = Math.sin(a) * Math.SQRT2; const m = 0.5 * (L[i] + R[i]); L[i] = L[i] * 0.3 + 0.7 * m * l; R[i] = R[i] * 0.3 + 0.7 * m * rr; }
  // body: low swell, sine dropping in pitch, part noise, following the amp envelope's rise
  if (p.w_b_lvl > -59 && (!solo || solo === 'body')) {
    const b = new Float32Array(n), r = rng((p.seed | 0) * 37 + 3), D = ms(p.w_b_dec, sr), on = Math.floor(Math.min(p.w_a_att, len) * sr / 1000);
    let ph = 0; for (let i = 0; i < n; i++) { const t = Math.max(0, i - on) / sr, f = p.w_b_freq * Math.pow(2, (p.w_b_drop / 12) * Math.exp(-t / 0.08)); ph += f / sr; const e = i < on ? Math.pow(i / Math.max(1, on), 3) : Math.exp((-6.9 * (i - on)) / D); b[i] = e * ((1 - p.w_b_noise) * Math.sin(TAU * ph) + p.w_b_noise * (r() * 2 - 1)); }
    lr4(b, 'lp', p.w_b_freq * 4, sr);
    const ref = Math.max(rmsOf(L), 0.05), g = (dbToLin(p.w_b_lvl) * ref) / (rmsOf(b) || 1);
    for (let i = 0; i < n; i++) { L[i] += g * b[i]; R[i] += g * b[i]; }
  }
  // reverb
  if (p.w_x_rev > -59 && (!solo || solo === 'reverb')) {
    const send = Float32Array.from(L, (v, i) => 0.5 * (v + R[i])); if (solo === 'reverb') { L.fill(0); R.fill(0); }
    const [hL, hR] = makeIR({ rev_type: 0, rev_size: p.w_x_size, rev_char: 0.5, rev_tone: 12000, rev_pre: 10, seed: p.seed }, sr);
    const [cL, cR] = convolveStereo(send, hL, hR, n); const g = (rmsOf(send) / (Math.sqrt((rmsOf(cL) ** 2 + rmsOf(cR) ** 2) / 2) || 1)) * dbToLin(p.w_x_rev);
    for (let i = 0; i < n; i++) { L[i] += g * cL[i]; R[i] += g * cR[i]; }
  }
  if (Math.round(p.w_x_reverse) === 1) { L.reverse(); R.reverse(); }
  for (const x of [L, R]) { if (p.w_p_hp > 21) lr4(x, 'hp', p.w_p_hp, sr); if (p.w_p_lp < 19900) lr4(x, 'lp', p.w_p_lp, sr); }
  ottCore(L, R, p.w_p_ott, 0.6, 2, sr);
  let pk = 0; for (let i = 0; i < n; i++) pk = Math.max(pk, Math.abs(L[i]), Math.abs(R[i]));
  const pre = opts.fixed ? opts.fixed.pre : pk > 0 ? 0.7 / pk : 1; for (let i = 0; i < n; i++) { L[i] *= pre; R[i] *= pre; }
  if (p.w_p_drive > 0.05) { shape(L, p.w_p_drive, 0.3); shape(R, p.w_p_drive, 0.3); }
  pk = 0; for (let i = 0; i < n; i++) pk = Math.max(pk, Math.abs(L[i]), Math.abs(R[i]));
  const g = opts.fixed ? opts.fixed.g : pk > 0 ? 0.966 / pk : 1, F = ms(8, sr);
  for (let i = 0; i < n; i++) { const f = i > n - F ? (n - i) / F : 1; L[i] *= g * f; R[i] *= g * f; }
  return { L, R, sr, layers: {}, gains: { pre, g } };
}
