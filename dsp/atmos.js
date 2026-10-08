// Atmosphere engine: sustained layers that each carry their own movement, summed in stereo → phaser/flanger → reverb → post.
import { SR, rng, biquad, lr4, shape, ottCore, makeIR, convolveStereo, rmsOf, dbToLin, midiHz, TAU } from './engine.js';

// smooth random movement in [-1, 1]: cosine-interpolated random points at `rate` Hz
function drift(n, sr, rate, r) {
  const x = new Float32Array(n), step = Math.max(1, Math.floor(sr / Math.max(0.01, rate))); let a = r() * 2 - 1, b = r() * 2 - 1;
  for (let i = 0; i < n; i++) { const k = i % step; if (k === 0 && i) { a = b; b = r() * 2 - 1; } const u = 0.5 - 0.5 * Math.cos((Math.PI * k) / step); x[i] = a + (b - a) * u; }
  return x;
}
const norm = (x, target = 0.2) => { const v = rmsOf(x); if (v > 1e-9) for (let i = 0; i < x.length; i++) x[i] *= target / v; return x; };
// state-variable bandpass with per-block cutoff
function svfBP(x, fc, q, sr) {
  const k = 1 / q; let ic1 = 0, ic2 = 0, a1 = 0, a2 = 0, a3 = 0; const y = new Float32Array(x.length);
  for (let i = 0; i < x.length; i++) {
    if ((i & 31) === 0) { const g = Math.tan((Math.PI * Math.min(Math.max(20, fc(i)), sr * 0.45)) / sr); a1 = 1 / (1 + g * (g + k)); a2 = g * a1; a3 = g * a2; }
    const v3 = x[i] - ic2, v1 = a1 * ic1 + a2 * v3, v2 = ic2 + a2 * ic1 + a3 * v3; ic1 = 2 * v1 - ic1; ic2 = 2 * v2 - ic2; y[i] = v1;
  }
  return y;
}
function pink(n, r) { const x = new Float32Array(n); let b0 = 0, b1 = 0, b2 = 0, b3 = 0, b4 = 0, b5 = 0, b6 = 0; for (let i = 0; i < n; i++) { const w = r() * 2 - 1; b0 = 0.99886 * b0 + w * 0.0555179; b1 = 0.99332 * b1 + w * 0.0750759; b2 = 0.969 * b2 + w * 0.153852; b3 = 0.8665 * b3 + w * 0.3104856; b4 = 0.55 * b4 + w * 0.5329522; b5 = -0.7616 * b5 - w * 0.016898; x[i] = (b0 + b1 + b2 + b3 + b4 + b5 + b6 + w * 0.5362) * 0.11; b6 = w * 0.115926; } return x; }
function colored(n, r, color) { const w = new Float32Array(n), p = pink(n, r); for (let i = 0; i < n; i++) w[i] = r() * 2 - 1; const c = Math.max(-1, Math.min(1, color)); for (let i = 0; i < n; i++) w[i] = c >= 0 ? (1 - c) * p[i] * 3 + c * w[i] : (1 + c) * p[i] * 3; if (c < 0) { let y = 0; const a = 1 - (1 + c) * 0.9 - 0.05; for (let i = 0; i < n; i++) { y = a * y + (1 - a) * w[i] * 4; w[i] = 0.5 * w[i] + 0.5 * y; } } return w; }
const blep = (t, dt) => (t < dt ? ((t /= dt), t + t - t * t - 1) : t > 1 - dt ? ((t = (t - 1) / dt), t * t + t + t + 1) : 0);
const CHORDS = [[0], [0, 7], [0, 12], [0, 3, 7], [0, 4, 7], [0, 5, 7]];

function bed(p, n, sr, r, mv) {
  const src = colored(n, r, p.at_b_color), out = new Float32Array(n), N = Math.round(p.at_b_formants);
  for (let k = 0; k < N; k++) {
    const off = N > 1 ? (k / (N - 1) - 0.5) * p.at_b_spread : 0, d = drift(n, sr, p.at_speed * (0.6 + 0.8 * r()), r), base = p.at_b_freq * Math.pow(2, off);
    const y = svfBP(src, (i) => base * Math.pow(2, 1.2 * p.at_move * d[i]), p.at_b_q, sr);
    for (let i = 0; i < n; i++) out[i] += y[i] / Math.sqrt(N);
  }
  return out;
}
function drone(p, n, sr, r, side) {
  const base = p.note + 12 * Math.round(p.at_d_oct), ch = CHORDS[Math.round(p.at_d_chord)], V = Math.round(p.at_d_voices), wave = Math.round(p.at_d_wave), x = new Float32Array(n);
  for (const iv of ch) for (let v = 0; v < V; v++) {
    const det = V > 1 ? ((v / (V - 1) - 0.5) * 2 * p.at_d_detune) / 100 : 0, f = Math.min(midiHz(base + iv + det + (side ? 0.03 : 0)), sr * 0.45), dt = f / sr; let ph = r();
    for (let i = 0; i < n; i++) { ph += dt; if (ph >= 1) ph -= 1; x[i] += (wave === 1 ? Math.sin(TAU * ph) : wave === 2 ? (ph < 0.5 ? 1 : -1) + blep(ph, dt) - blep((ph + 0.5) % 1, dt) : 2 * ph - 1 - blep(ph, dt)) / (V * ch.length); }
  }
  // resonant lowpass, cutoff drifting slowly
  const d = drift(n, sr, p.at_speed * 0.7, r), k = 2 - 2 * p.at_d_res; let ic1 = 0, ic2 = 0, a1 = 0, a2 = 0, a3 = 0;
  for (let i = 0; i < n; i++) {
    if ((i & 31) === 0) { const fc = Math.min(p.at_d_cut * Math.pow(2, 1.5 * p.at_move * d[i]), sr * 0.45), g = Math.tan((Math.PI * fc) / sr); a1 = 1 / (1 + g * (g + k)); a2 = g * a1; a3 = g * a2; }
    const v3 = x[i] - ic2, v1 = a1 * ic1 + a2 * v3, v2 = ic2 + a2 * ic1 + a3 * v3; ic1 = 2 * v1 - ic1; ic2 = 2 * v2 - ic2; x[i] = v2;
  }
  return x;
}
function noiseBass(p, n, sr, r) {
  const f = midiHz(p.note + 12 * Math.round(p.at_n_oct)), d = sr / f, N = 1 << 15, buf = new Float32Array(N), x = new Float32Array(n), fb = p.at_n_fb, wob = drift(n, sr, p.at_speed, r);
  let w = 0, lp = 0;
  for (let i = 0; i < n; i++) {
    const dd = d * (1 + 0.004 * p.at_move * wob[i]), rp = w - dd + N * 2, i0 = Math.floor(rp), fr = rp - i0, del = buf[i0 & (N - 1)] * (1 - fr) + buf[(i0 + 1) & (N - 1)] * fr;
    lp += (del - lp) * 0.5; const y = (r() * 2 - 1) * 0.3 + fb * lp; buf[w & (N - 1)] = y; w++; x[i] = y;
  }
  norm(x); const sub = dbToLin(p.at_n_sub) * 0.25; let ph = 0;
  for (let i = 0; i < n; i++) { ph += f / sr; x[i] += sub * Math.sin(TAU * ph); }
  shape(x, p.at_n_drive, 0.3); lr4(x, 'lp', p.at_n_lp, sr);
  return x;
}
function crunch(p, n, sr, r) {
  const x = new Float32Array(n), step = sr / p.at_c_rate, q = Math.pow(2, p.at_c_bits - 1), d = drift(n, sr, p.at_speed * 2, r); let h = 0, c = 0;
  for (let i = 0; i < n; i++) { if (c <= 0) { const gate = r() > p.at_c_grit * (0.5 + 0.5 * d[i]) ? 1 : 0; h = (Math.round((r() * 2 - 1) * q) / q) * gate; c += step; } c--; x[i] = h; }
  shape(x, p.at_c_drive, 0.6); lr4(x, 'hp', p.at_c_hp, sr);
  return x;
}
function ship(p, n, sr, r) {
  const x = new Float32Array(n), wob = drift(n, sr, p.at_speed * 0.5, r); let ph = 0, pr = 0;
  for (let i = 0; i < n; i++) {
    const f = p.at_s_hum * Math.pow(2, (p.at_s_wobble * wob[i]) / 12), dt = f / sr; ph += dt; if (ph >= 1) ph -= 1;
    let v = 0.6 * (2 * ph - 1 - blep(ph, dt)) + 0.4 * Math.sin(TAU * 2 * ph); v += 0.15 * (r() * 2 - 1);
    if (p.at_s_ring > 1) { pr += p.at_s_ring / sr; v *= Math.sin(TAU * pr); }
    x[i] = v;
  }
  // sweeping feedback comb
  const sw = drift(n, sr, p.at_speed * 0.4, r), N = 1 << 14, buf = new Float32Array(N); let w = 0;
  for (let i = 0; i < n; i++) { const f = p.at_s_comb * Math.pow(2, p.at_s_sweep * 0.5 * (1 + sw[i]) * p.at_move), dd = sr / f, rp = w - dd + N * 2, i0 = Math.floor(rp), fr = rp - i0; const del = buf[i0 & (N - 1)] * (1 - fr) + buf[(i0 + 1) & (N - 1)] * fr; const y = x[i] + 0.75 * del; buf[w & (N - 1)] = y; w++; x[i] = y; }
  biquad(x, 'hp', 30, 0.707, 0, sr);
  return x;
}
function shimmer(p, n, sr, r) {
  const x = new Float32Array(n), G = Math.floor(p.at_g_density * (n / sr)), L = Math.max(16, Math.floor((p.at_g_len * sr) / 1000));
  for (let g = 0; g < G; g++) {
    const st = Math.floor(r() * n), f = Math.min(midiHz(12 * (Math.round(p.at_g_oct) + 1) + (Math.round(p.note) % 12) + 12 * (r() - 0.5) * p.at_g_spread), sr * 0.45), a = 0.3 + 0.7 * r();
    for (let i = 0; i < L && st + i < n; i++) x[st + i] += a * Math.sin((TAU * f * i) / sr) * Math.sin((Math.PI * i) / L) ** 2;
  }
  return x;
}
const LAYERS = [['bed', 'at_b_lvl', bed], ['drone', 'at_d_lvl', drone], ['noise bass', 'at_n_lvl', noiseBass], ['crunch', 'at_c_lvl', crunch], ['ship', 'at_s_lvl', ship], ['shimmer', 'at_g_lvl', shimmer]];

export function renderAtmos(p, opts = {}) {
  const sr = opts.sr || SR, solo = opts.solo, N = Math.floor((p.at_len * sr) / 1000), tail = Math.floor(Math.min(p.at_x_size * 0.5, 3000) * sr / 1000), n = N + tail;
  const L = new Float32Array(n), R = new Float32Array(n), seed = p.seed | 0;
  LAYERS.forEach(([name, lvl, fn], li) => {
    if (p[lvl] <= -59 || (solo && solo !== name)) return;
    const g = dbToLin(p[lvl]);
    // two decorrelated renders → symmetric stereo (mid + width × side)
    const a = norm(fn(p, N, sr, rng(seed * 101 + li * 7 + 1), false)), b = norm(fn(p, N, sr, rng(seed * 101 + li * 7 + 2), true)), w = p.at_x_width;
    for (let i = 0; i < N; i++) { const m = 0.5 * (a[i] + b[i]), s = 0.5 * (a[i] - b[i]) * w; L[i] += g * (m + s); R[i] += g * (m - s); }
  });
  // fades
  const A = Math.floor((p.at_att * sr) / 1000), F = Math.floor((p.at_rel * sr) / 1000);
  for (let i = 0; i < N; i++) { const e = Math.min(1, i / Math.max(1, A), (N - i) / Math.max(1, F)); const g = e * e * (3 - 2 * e); L[i] *= g; R[i] *= g; }
  // phaser (allpass cascade with a slow sweep) and flanger (short modulated delay with feedback)
  if (p.at_x_phaser > 0.01) for (const [x, off] of [[L, 0], [R, 0.25]]) {
    const dry = x.slice(), y = x; let s = []; for (let k = 0; k < 6; k++) s.push([0, 0]);
    for (let i = 0; i < n; i++) { const f = 400 * Math.pow(8, 0.5 + 0.5 * Math.sin(TAU * (p.at_speed * 0.5 * i / sr + off))), c = (Math.tan((Math.PI * f) / sr) - 1) / (Math.tan((Math.PI * f) / sr) + 1); let v = dry[i]; for (const st of s) { const o = c * v + st[0] - c * st[1]; st[0] = v; st[1] = o; v = o; } y[i] = dry[i] * (1 - 0.5 * p.at_x_phaser) + 0.5 * p.at_x_phaser * v; }
  }
  if (p.at_x_flanger > 0.01) for (const [x, off] of [[L, 0], [R, 0.5]]) {
    const NB = 1 << 12, buf = new Float32Array(NB); let w = 0;
    for (let i = 0; i < n; i++) { const dl = (0.001 + 0.004 * (0.5 + 0.5 * Math.sin(TAU * (p.at_speed * 0.3 * i / sr + off)))) * sr, rp = w - dl + NB * 4, i0 = Math.floor(rp), fr = rp - i0, del = buf[i0 & (NB - 1)] * (1 - fr) + buf[(i0 + 1) & (NB - 1)] * fr; buf[w & (NB - 1)] = x[i] + p.at_x_fb * del; w++; x[i] = x[i] + p.at_x_flanger * del; }
  }
  if (p.at_x_rev > -59 && (!solo || solo === 'reverb')) {
    const send = Float32Array.from(L, (v, i) => 0.5 * (v + R[i])); if (solo === 'reverb') { L.fill(0); R.fill(0); }
    const [hL, hR] = makeIR({ rev_type: 0, rev_size: p.at_x_size, rev_char: 0.4, rev_tone: 10000, rev_pre: 20, seed: p.seed }, sr);
    const [cL, cR] = convolveStereo(send, hL, hR, n), g = (rmsOf(send) / (Math.sqrt((rmsOf(cL) ** 2 + rmsOf(cR) ** 2) / 2) || 1)) * dbToLin(p.at_x_rev);
    for (let i = 0; i < n; i++) { L[i] += g * cL[i]; R[i] += g * cR[i]; }
  }
  for (const x of [L, R]) { if (p.at_p_hp > 21) lr4(x, 'hp', p.at_p_hp, sr); if (p.at_p_lp < 19900) lr4(x, 'lp', p.at_p_lp, sr); }
  ottCore(L, R, p.at_p_ott, 0.5, 3, sr);
  let pk = 0; for (let i = 0; i < n; i++) pk = Math.max(pk, Math.abs(L[i]), Math.abs(R[i]));
  const pre = opts.fixed ? opts.fixed.pre : pk > 0 ? 0.7 / pk : 1; for (let i = 0; i < n; i++) { L[i] *= pre; R[i] *= pre; }
  if (p.at_p_drive > 0.05) { shape(L, p.at_p_drive, 0.2); shape(R, p.at_p_drive, 0.2); }
  pk = 0; for (let i = 0; i < n; i++) pk = Math.max(pk, Math.abs(L[i]), Math.abs(R[i]));
  const g = opts.fixed ? opts.fixed.g : pk > 0 ? 0.891 / pk : 1; for (let i = 0; i < n; i++) { L[i] *= g; R[i] *= g; }
  return { L, R, sr, layers: {}, gains: { pre, g } };
}
