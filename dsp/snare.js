// Snare engine: layers (hit, click, tone, noise, metal, clap) -> room -> bus; the hit lands on top of the bus clipper.
import {
  SR, rng, biquad, lr4, shape, envFollow, ottCore, snap, makeIR, convolveStereo, normRms, rmsOf, dbToLin, midiHz, TAU,
} from './engine.js';

// attack -> hold -> decay (dec = time to -60 dB). gate 0 = exponential, 1 = squared ramp that ends at dec.
function aenv(n, sr, att, hold, dec, gate) {
  const e = new Float32Array(n), A = (att * sr) / 1000, H = A + (hold * sr) / 1000, D = Math.max(1, (dec * sr) / 1000);
  for (let i = 0; i < n; i++) {
    if (i < A) { e[i] = i / A; continue; }
    const t = i < H ? 0 : (i - H) / D;
    const ex = Math.exp(-6.9 * t), lin = t >= 1 ? 0 : (1 - t) * (1 - t);
    e[i] = (1 - gate) * ex + gate * lin;
  }
  return e;
}
const active = (att, hold, dec, sr) => Math.ceil(((att + hold + dec * 0.35) * sr) / 1000);
const blep = (t, dt) => (t < dt ? ((t /= dt), t + t - t * t - 1) : t > 1 - dt ? ((t = (t - 1) / dt), t * t + t + t + 1) : 0);
const baseHz = (p, oct, tune) => midiHz(12 * (oct + 1) + (Math.round(p.note) % 12) + tune);
// shell register: octave k spans E(k)..D#(k+1), so octave 3 puts the body at 165–311 Hz
export const shellMidi = (note, oct) => 12 * (oct + 1) + 4 + ((((Math.round(note) - 4) % 12) + 12) % 12);
// time-varying SVF lowpass from f0 toward f1 with time constant ms
function sweepLP(x, f0, f1, ms, sr, k = 1.3) {
  const l0 = Math.log(f0), l1 = Math.log(f1), ct = ms / 1000;
  let ic1 = 0, ic2 = 0, a1 = 0, a2 = 0, a3 = 0;
  for (let i = 0; i < x.length; i++) {
    if ((i & 15) === 0) { const f = Math.exp(l1 + (l0 - l1) * Math.exp(-(i / sr) / ct)); const g = Math.tan((Math.PI * Math.min(f, sr * 0.45)) / sr); a1 = 1 / (1 + g * (g + k)); a2 = g * a1; a3 = g * a2; }
    const v3 = x[i] - ic2, v1 = a1 * ic1 + a2 * v3, v2 = ic2 + a2 * ic1 + a3 * v3; ic1 = 2 * v1 - ic1; ic2 = 2 * v2 - ic2; x[i] = v2;
  }
  return x;
}
const delayed = (x, ms, sr) => { const d = Math.floor((ms * sr) / 1000); if (d <= 0) return x; const y = new Float32Array(x.length); y.set(x.subarray(0, x.length - d), d); return y; };

// ---------------------------------------------------------------- tone
const SHELL = [1.59, 2.14, 2.3, 2.65, 2.92];
function renderTone(p, n, sr, r) {
  const f0 = midiHz(shellMidi(p.note, Math.round(p.sn_t_oct)) + p.sn_t_tune), D = p.sn_t_pitch / 12, pt = p.sn_t_pitch_ms / 1000, C = p.sn_t_click / 12, ct = p.sn_t_click_ms / 1000;
  const rd = p.sn_t_round, rn = 1 / (1 + 0.45 * rd);
  const env = aenv(n, sr, p.sn_t_att, p.sn_t_hold, p.sn_t_dec, p.sn_t_curve);
  const wave = Math.round(p.sn_t_wave), x = new Float32Array(n), pg = dbToLin(p.sn_t_partials);
  const pc = SHELL.map(() => r() * TAU), pdk = SHELL.map((q) => 6.9 / ((p.sn_t_dec * 0.5 * sr) / 1000) * q);
  let ph = 0.25, pm = 0; // 90° start: the hit begins at full level
  for (let i = 0; i < n; i++) {
    const t = i / sr, f = Math.min(f0 * Math.pow(2, D * Math.exp(-t / pt) + C * Math.exp(-t / ct)), sr * 0.45), dt = f / sr;
    pm += (TAU * f * p.sn_t_fm_ratio) / sr;
    const mod = p.sn_t_fm > 0.001 ? (p.sn_t_fm * env[i] * Math.sin(pm)) / TAU : 0;
    ph += dt; if (ph >= 1) ph -= 1;
    let q = ph + mod; q -= Math.floor(q);
    let v;
    if (wave === 0) v = Math.sin(TAU * q);
    else if (wave === 1) v = 1 - 4 * Math.abs(q - 0.5);
    else if (wave === 2) v = 2 * q - 1 - blep(q, dt);
    else v = (q < 0.5 ? 1 : -1) + blep(q, dt) - blep((q + 0.5) % 1, dt);
    if (rd > 0.001) v = (v + rd * (0.35 * Math.sin(3 * TAU * q) + 0.1 * Math.sin(5 * TAU * q))) * rn; // toward a rounded square
    if (p.sn_t_even > 0.001) v += p.sn_t_even * (0.3 * Math.sin(2 * TAU * q) + 0.08 * Math.sin(4 * TAU * q)); // even harmonics (breathy/hollow vs reedy)
    if (pg > 0.001) for (let k = 0; k < SHELL.length; k++) v += (pg / (k + 1)) * Math.sin(TAU * f * SHELL[k] * t + pc[k]) * Math.exp(-i * pdk[k]);
    if (p.sn_t_grit > 0.001) v *= 1 + p.sn_t_grit * 1.5 * (r() * 2 - 1);
    x[i] = v;
  }
  // clip the oscillator itself (flat-topped body), drive falling with the envelope; then the amp envelope,
  // so the clipper squares the wave without stretching the decay
  const de = p.sn_t_drive_env;
  for (let i = 0; i < n; i++) x[i] *= 0.5 * dbToLin(p.sn_t_drive * (1 - de + de * env[i]));
  shape(x, 0.02, 0.4, p.sn_t_fold);
  for (let i = 0; i < n; i++) x[i] *= env[i];
  normRms(x, 0.25, active(p.sn_t_att, p.sn_t_hold, p.sn_t_dec, sr));
  return x;
}

// ---------------------------------------------------------------- click
function renderClick(p, n, sr, r) {
  const x = new Float32Array(n), type = Math.round(p.sn_c_type), L = Math.max(8, Math.floor((p.sn_c_len * sr) / 1000)), f = p.sn_c_f;
  if (type === 0) { // pitch-swept sine click
    let ph = 0;
    for (let i = 0; i < Math.min(n, L * 4); i++) { const t = i / L, fr = 60 + (f - 60) * Math.exp(-t * 3); ph += (TAU * Math.min(fr, sr * 0.45)) / sr; x[i] = Math.sin(ph) * Math.exp(-t * 1.5); }
  } else if (type === 1) { // random sine bleeps scattered across the window
    const N = Math.round(p.sn_c_n), W = L * 3;
    for (let b = 0; b < N; b++) {
      const st = b === 0 ? 0 : Math.floor(Math.pow(r(), 1.5) * W), bl = Math.floor((0.002 + 0.012 * r()) * sr * Math.sqrt(p.sn_c_len / 4));
      const fr = Math.min(f * Math.pow(2, (r() - 0.5) * p.sn_c_spread), sr * 0.45), a = b === 0 ? 1 : 0.4 + 0.6 * r();
      for (let i = 0; i < bl && st + i < n; i++) x[st + i] += a * Math.sin((TAU * fr * i) / sr) * Math.sin((Math.PI * i) / bl);
    }
  } else if (type === 2) { // saw zap, f down three octaves
    let ph = 0;
    for (let i = 0; i < Math.min(n, L * 3); i++) { const t = i / L, fr = Math.min(f * Math.pow(2, -3 * Math.min(1, t)), sr * 0.45), dt = fr / sr; ph += dt; if (ph >= 1) ph -= 1; x[i] = (2 * ph - 1 - blep(ph, dt)) * Math.exp(-t * 1.2); }
  } else if (type === 3) { // noise burst
    for (let i = 0; i < Math.min(n, L * 4); i++) x[i] = (r() * 2 - 1) * Math.exp(-(i / L) * 2);
    biquad(x, 'hp', f * 0.35, 0.707, 0, sr); biquad(x, 'peak', f, 1, 6, sr);
  } else { // snap: two short bandpassed bursts
    for (const o of [0, Math.floor(L * 1.6)]) for (let i = 0; i < L * 3 && o + i < n; i++) x[o + i] += (r() * 2 - 1) * Math.exp(-(i / L) * 2.5) * (o ? 0.8 : 1);
    biquad(x, 'bp', f, 1.4, 0, sr); biquad(x, 'bp', f, 1.4, 0, sr);
  }
  shape(x, p.sn_c_drive, 0.5);
  normRms(x, 0.25, L * 2);
  return x;
}

// ---------------------------------------------------------------- noise
// raw noise of a given type, one channel
function noiseSrc(type, n, sr, r, rate, ringF) {
  const x = new Float32Array(n);
  if (type === 1) { // pink (Kellet)
    let b0 = 0, b1 = 0, b2 = 0, b3 = 0, b4 = 0, b5 = 0, b6 = 0;
    for (let i = 0; i < n; i++) { const w = r() * 2 - 1; b0 = 0.99886 * b0 + w * 0.0555179; b1 = 0.99332 * b1 + w * 0.0750759; b2 = 0.969 * b2 + w * 0.153852; b3 = 0.8665 * b3 + w * 0.3104856; b4 = 0.55 * b4 + w * 0.5329522; b5 = -0.7616 * b5 - w * 0.016898; x[i] = (b0 + b1 + b2 + b3 + b4 + b5 + b6 + w * 0.5362) * 0.11; b6 = w * 0.115926; }
  } else if (type === 2) { // geiger: Poisson clicks, each a tiny decaying impulse
    const pr = rate / sr, tail = Math.max(2, Math.floor(sr / 9000)); let k = 0, amp = 0;
    for (let i = 0; i < n; i++) { if (r() < pr) { amp = r() < 0.5 ? -1 : 1; k = tail; } x[i] = k > 0 ? amp * (k / tail) : 0; if (k > 0) k--; }
  } else if (type === 3) { // crackle: sparse impulses of random size, resonant pops
    const pr = rate / sr;
    for (let i = 0; i < n; i++) if (r() < pr) x[i] = (r() * 2 - 1) * Math.pow(r(), 0.5) * 3;
    biquad(x, 'bp', 2500, 0.7, 0, sr);
  } else if (type === 4) { // crushed: sample-and-hold at rate, 4-bit
    const step = Math.max(1, sr / rate); let h = 0, c = 0;
    for (let i = 0; i < n; i++) { if (c <= 0) { h = Math.round((r() * 2 - 1) * 7) / 7; c += step; } c--; x[i] = h; }
  } else if (type === 5) { // digital: short-period LFSR clocked at rate (metallic NES noise)
    let reg = 1 + Math.floor(r() * 32766), c = 0; const step = Math.max(1, sr / rate);
    for (let i = 0; i < n; i++) { if (c <= 0) { const bit = (reg ^ (reg >> 6)) & 1; reg = (reg >> 1) | (bit << 14); c += step; } c--; x[i] = reg & 1 ? 1 : -1; }
  } else if (type === 6) { // ring: white noise ring-modulated by a sine at the peak freq
    for (let i = 0; i < n; i++) x[i] = (r() * 2 - 1) * Math.sin((TAU * ringF * i) / sr) * 1.4;
  } else for (let i = 0; i < n; i++) x[i] = r() * 2 - 1;
  return x;
}
function renderNoise(p, n, sr, seed) {
  const type = Math.round(p.sn_n_type), env = aenv(n, sr, p.sn_n_att, p.sn_n_hold, p.sn_n_dec, p.sn_n_curve);
  if (p.sn_n_flutter > 0.001) { // a second peak a little later (two-burst wash)
    const g = Math.floor((p.sn_n_flutter_ms * sr) / 1000), e0 = env.slice(), f = p.sn_n_flutter;
    for (let i = 0; i < n; i++) env[i] = (e0[i] * (1 - 0.5 * f) + f * (i >= g ? e0[i - g] : 0)) / (1 + 0.5 * f);
  }
  // tuned ring: the note's pitch class (plus interval) placed at 600–1200 Hz, and its octave
  let ringF = midiHz(24 + ((Math.round(p.note) + Math.round(p.sn_n_ring_st)) % 12)); while (ringF < 600) ringF *= 2;
  const mid = noiseSrc(type, n, sr, rng(seed * 31 + 1), p.sn_n_rate, p.sn_n_pk_f), side = noiseSrc(type, n, sr, rng(seed * 31 + 2), p.sn_n_rate, p.sn_n_pk_f * 1.01);
  // rattle: slow random amplitude grain (snare wires buzzing)
  let rat = null;
  if (p.sn_n_rattle > 0.001) {
    const rr = rng(seed * 31 + 3); rat = new Float32Array(n); let v = 0, tgt = 0, c = 0; const per = sr / p.sn_n_rattle_f;
    for (let i = 0; i < n; i++) { if (c <= 0) { tgt = rr(); c += per * (0.5 + rr()); } c--; v += (tgt - v) * 0.02; rat[i] = 1 - p.sn_n_rattle + p.sn_n_rattle * 2 * v; }
  }
  const out = [mid, side].map((x) => {
    lr4(x, 'hp', p.sn_n_hp, sr);
    biquad(x, 'lp', p.sn_n_lp, 0.707, 0, sr);
    if (p.sn_n_close > 0.01) sweepLP(x, p.sn_n_lp, Math.max(400, p.sn_n_lp * Math.pow(0.08, p.sn_n_close)), p.sn_n_close_ms, sr);
    if (p.sn_n_pk_g > 0.05) biquad(x, 'peak', p.sn_n_pk_f, 1.2, p.sn_n_pk_g, sr);
    if (p.sn_n_ring > 0.05) { biquad(x, 'peak', ringF, p.sn_n_ring_q, p.sn_n_ring, sr); biquad(x, 'peak', ringF * 2, p.sn_n_ring_q, p.sn_n_ring - 6, sr); }
    for (let i = 0; i < n; i++) x[i] *= env[i] * (rat ? rat[i] : 1);
    normRms(x, 0.25, active(p.sn_n_att, p.sn_n_hold, p.sn_n_dec, sr));
    shape(x, p.sn_n_drive, p.sn_n_hard, p.sn_n_fold);
    lr4(x, 'hp', p.sn_n_hp * 0.6, sr); // drive regrows lows; keep the wash above the shell
    return x;
  });
  const w = p.sn_n_width, L = new Float32Array(n), R = new Float32Array(n);
  for (let i = 0; i < n; i++) { const m = out[0][i], s = out[1][i] - out[0][i]; L[i] = m + 0.5 * w * s; R[i] = m - 0.5 * w * s; }
  const g = 0.25 / (Math.sqrt((rmsOf(L, active(p.sn_n_att, p.sn_n_hold, p.sn_n_dec, sr)) ** 2 + rmsOf(R, active(p.sn_n_att, p.sn_n_hold, p.sn_n_dec, sr)) ** 2) / 2) || 1);
  for (let i = 0; i < n; i++) { L[i] *= g; R[i] *= g; }
  return [L, R];
}

// ---------------------------------------------------------------- metal
const BELL = [1, 2.756, 5.404, 8.933, 13.34];
const MEAS = [1, 1.49, 2.28, 3.65, 6.77, 10.6]; // measured snare shell, wires off
function metalChannel(p, n, sr, r, side) {
  const type = Math.round(p.sn_m_type), f0 = baseHz(p, Math.round(p.sn_m_oct), p.sn_m_tune) * (side ? 1 + 0.004 * p.sn_m_width : 1);
  const x = new Float32Array(n), D = Math.max(1, (p.sn_m_dec * sr) / 1000), inh = p.sn_m_spread, br = p.sn_m_bright;
  const bank = (ratios, decK) => {
    ratios.forEach((q, k) => {
      const f = f0 * q, amp = Math.pow(1 / (k + 1), 1.5 - 1.3 * br); if (f > sr * 0.45) return;
      const R = Math.exp(-6.9 / (D * decK(k))), c1 = 2 * R * Math.cos((TAU * f) / sr), c2 = -R * R; let y1 = 0, y2 = 0;
      for (let i = 0; i < n; i++) { const ex = i < 64 ? (r() * 2 - 1) : 0; const y = (1 - R) * ex * 8 + c1 * y1 + c2 * y2; y2 = y1; y1 = y; x[i] += amp * y; }
    });
  };
  if (type === 0) { // modal: blend harmonic -> measured-shell ratios
    bank(MEAS.map((q, k) => Math.pow(k + 1, 1 - inh) * Math.pow(q, inh) * (1 + (r() - 0.5) * 0.01)), (k) => 1 / (1 + 0.25 * k));
  } else if (type === 4) { // bell / free bar
    bank(BELL.map((q) => 1 + (q - 1) * (0.6 + 0.8 * inh)), (k) => 1 / (1 + 0.4 * k));
  } else if (type === 1) { // comb: noise burst into a feedback comb at f0
    const d = sr / f0, fb = Math.pow(0.001, d / D), N = 1 << 14, buf = new Float32Array(N); let w = 0, lp = 0;
    const damp = 0.15 + 0.8 * (1 - br);
    for (let i = 0; i < n; i++) {
      const rp = w - d + N * 2, i0 = Math.floor(rp), fr = rp - i0, del = buf[i0 & (N - 1)] * (1 - fr) + buf[(i0 + 1) & (N - 1)] * fr;
      lp += (del - lp) * (1 - damp * 0.7); const y = (i < 96 ? r() * 2 - 1 : 0) + fb * (inh * del + (1 - inh) * lp);
      buf[w & (N - 1)] = y; w++; x[i] = y;
    }
  } else if (type === 2) { // unison saws + octave
    const V = 7;
    for (let v = 0; v < V; v++) {
      const det = Math.pow(2, ((v - (V - 1) / 2) / ((V - 1) / 2)) * (0.15 + inh * 0.6) / 12) * (v === V - 1 ? 2 : 1); let ph = r();
      const f = Math.min(f0 * det, sr * 0.45), dt = f / sr;
      for (let i = 0; i < n; i++) { ph += dt; if (ph >= 1) ph -= 1; x[i] += (2 * ph - 1 - blep(ph, dt)) / V; }
    }
    biquad(x, 'hp', f0 * 0.9, 0.707, 0, sr); biquad(x, 'lp', Math.min(18000, f0 * (3 + 20 * br)), 0.707, 0, sr);
    for (let i = 0; i < n; i++) x[i] *= Math.exp((-6.9 * i) / D);
  } else { // 808 metal: six squares, two bandpasses
    const sc = f0 / 540, fr = [205.3, 304.4, 369.6, 522.7, 540, 800].map((f) => f * sc * (1 + (inh - 0.5) * 0.2));
    for (let i = 0; i < n; i++) { let s = 0; for (const f of fr) s += ((i * f) / sr) % 1 < 0.5 ? 1 : -1; x[i] = (s / 6) * Math.exp((-6.9 * i) / D); }
    const a = x.slice(); biquad(x, 'bp', 3440 * sc * 4, 3, 0, sr); biquad(a, 'bp', 7100 * sc * 4 * (0.5 + br), 3, 0, sr);
    for (let i = 0; i < n; i++) x[i] += 0.7 * a[i];
  }
  const S = Math.floor((p.sn_m_att * sr) / 1000);
  for (let i = 0; i < Math.min(S, n); i++) x[i] *= 0.5 - 0.5 * Math.cos((Math.PI * i) / S);
  shape(x, p.sn_m_drive, 0.3);
  biquad(x, 'hp', 200, 0.707, 0, sr);
  normRms(x, 0.25, Math.min(n, Math.ceil(D * 0.35) + S));
  return x;
}
function renderMetal(p, n, sr, seed) {
  // fixed excitation and detune: the ring is the same on every hit
  const a = metalChannel(p, n, sr, rng(4101), false);
  if (p.sn_m_width < 0.01) return [a, a.slice()];
  return [a, metalChannel(p, n, sr, rng(4102), true)];
}

// ---------------------------------------------------------------- clap
// fixed, slightly irregular burst spacing: every hit identical (the noise texture still varies with the seed)
const CLAP_JIT = [0, 1.07, 0.95, 1.03, 0.98, 1.05];
function renderClap(p, n, sr, r) {
  const x = new Float32Array(n), N = Math.round(p.sn_k_n), gap = (p.sn_k_gap * sr) / 1000, tail = (p.sn_k_dec * sr) / 1000;
  for (let b = 0; b < N; b++) {
    const o = Math.floor(b * gap * CLAP_JIT[b]), last = b === N - 1, len = last ? Math.floor(tail * 1.2) : Math.floor(gap * 1.2);
    for (let i = 0; i < len && o + i < n; i++) x[o + i] += (r() * 2 - 1) * (last ? Math.exp((-6.9 * i) / tail) : Math.exp(-i / (gap * 0.3)));
  }
  biquad(x, 'bp', p.sn_k_f, p.sn_k_q, 0, sr); biquad(x, 'hp', p.sn_k_f * 0.4, 0.707, 0, sr);
  shape(x, p.sn_k_drive, 0.4);
  normRms(x, 0.25, Math.min(n, Math.floor(N * gap + tail * 0.35)));
  return x;
}

// ---------------------------------------------------------------- width
// symmetric stereo: a decorrelated copy (allpass cascade) of everything above the width highpass becomes the side
// signal, faded in after the hit, so the transient and the lows stay mono and the tail opens up
function widen(L, R, p, sr, r) {
  if (p.sn_w_amt < 0.005) return;
  const n = L.length, m = new Float32Array(n); for (let i = 0; i < n; i++) m[i] = 0.5 * (L[i] + R[i]);
  const hi = lr4(m.slice(), 'hp', p.sn_w_hp, sr), s = hi.slice();
  for (let k = 0; k < 10; k++) biquad(s, 'ap', 300 * Math.pow(40, r()), 0.4 + 1.6 * r(), 0, sr);
  const g = rmsOf(hi) / (rmsOf(s) || 1), F = Math.max(1, (p.sn_w_fade * sr) / 1000), a = p.sn_w_amt * g;
  for (let i = 0; i < n; i++) { const w = a * (i < F ? 0.5 - 0.5 * Math.cos((Math.PI * i) / F) : 1); L[i] += w * s[i]; R[i] -= w * s[i]; }
}

// ---------------------------------------------------------------- hit
// a full-scale noise burst: flat for its length, then a fast fall; tilted dark (lowpass) or bright (highpass); peak 1
function renderHit(p, n, sr, r) {
  const hold = Math.ceil((p.sn_h_len * sr) / 1000), tau = hold / 2, m = Math.min(n, hold + Math.ceil(5 * tau)), x = new Float32Array(n);
  const s0 = Math.floor(0.00025 * sr); for (let i = 0; i < m; i++) x[i + s0 < n ? i + s0 : n - 1] = (i === 0 ? 1 : r() * 2 - 1) * (i < hold ? 1 : Math.exp(-(i - hold) / tau));
  const c = p.sn_h_color;
  if (c < 0) biquad(x, 'lp', 16000 * 2 ** (c * 4), 0.707, 0, sr); else if (c > 0.02) biquad(x, 'hp', 100 * 2 ** (c * 5), 0.707, 0, sr);
  if (p.sn_h_drive > 0.05) { const g = dbToLin(p.sn_h_drive); for (let i = 0; i < Math.min(n, m + s0); i++) x[i] = Math.tanh(x[i] * g); }
  let pk = 1e-9; for (let i = 0; i < Math.min(n, m + s0); i++) pk = Math.max(pk, Math.abs(x[i])); for (let i = 0; i < n; i++) x[i] /= pk;
  return x;
}

// ---------------------------------------------------------------- render
const LAYERS = [['hit', 'sn_h_lvl'], ['click', 'sn_c_lvl'], ['tone', 'sn_t_lvl'], ['noise', 'sn_n_lvl'], ['metal', 'sn_m_lvl'], ['clap', 'sn_k_lvl']];
const TIMED = /^sn_(t_(att|hold|dec|pitch_ms)|n_(att|hold|dec|close_ms|flutter_ms)|c_len|m_(att|dec)|k_(gap|dec)|r_size|b_whip_ms)$/;
export function renderSnare(p0, opts = {}) {
  const sr = opts.sr || SR, seed = p0.seed | 0, n = Math.floor((p0.len * sr) / 1000);
  const p = { ...p0 }, tm = p0.sn_b_time || 1;
  if (Math.abs(tm - 1) > 1e-3) for (const k in p) if (TIMED.test(k)) p[k] *= tm;
  const lay = {};
  const mono = (x) => [x, x];
  if (p.sn_h_lvl > -59) lay.hit = mono(renderHit(p, n, sr, rng(seed * 7 + 1)));
  if (p.sn_c_lvl > -59) lay.click = mono(delayed(renderClick(p, n, sr, rng(seed * 13 + 3)), p.sn_c_delay, sr));
  if (p.sn_t_lvl > -59) lay.tone = mono(delayed(renderTone(p, n, sr, rng(seed * 19 + 11)), p.sn_t_delay, sr));
  if (p.sn_n_lvl > -59) lay.noise = renderNoise(p, n, sr, seed).map((x) => delayed(x, p.sn_n_delay, sr));
  if (p.sn_m_lvl > -59) lay.metal = renderMetal(p, n, sr, seed).map((x) => delayed(x, p.sn_m_delay, sr));
  if (p.sn_k_lvl > -59) lay.clap = mono(delayed(renderClap(p, n, sr, rng(seed * 43 + 5)), p.sn_k_delay, sr));
  for (const [k, id] of LAYERS) if (lay[k]) { const g = dbToLin(p[id]); const done = new Set(); for (const x of lay[k]) if (!done.has(x)) { done.add(x); for (let i = 0; i < x.length; i++) x[i] *= g; } }
  if (opts.extra) for (const [k, v] of Object.entries(opts.extra)) if (v) lay[k] = v; // dropped sample layers
  const P = 0, N = n;
  const dry = [new Float32Array(n), new Float32Array(n)];
  const solo = opts.solo;
  for (const [k, v] of Object.entries(lay)) {
    if ((solo && solo !== k) || k === 'hit') continue; // the hit lands after the bus clipper
    for (let c = 0; c < 2; c++) for (let i = 0; i < n; i++) dry[c][i] += v[c][i];
  }
  const L = new Float32Array(N), R = new Float32Array(N);
  L.set(dry[0], P); R.set(dry[1], P);
  // room: send of everything, highpassed
  if (p.sn_r_lvl > -59 && (!solo || solo === 'room')) {
    const send = new Float32Array(N); for (let i = 0; i < N; i++) send[i] = 0.5 * (L[i] + R[i]);
    if (solo === 'room') { // the room stem hears the whole kit
      for (const [k, v] of Object.entries(lay)) if (k !== 'hit') for (let i = 0; i < n; i++) send[P + i] += 0.5 * (v[0][i] + v[1][i]);
      L.fill(0); R.fill(0);
    }
    const [hL, hR] = makeIR({ rev_type: p.sn_r_type, rev_size: p.sn_r_size, rev_char: p.sn_r_char, rev_tone: p.sn_r_tone, rev_pre: 2, seed: p.seed }, sr);
    const [cL, cR] = convolveStereo(send, hL, hL, N); // mono room: the snare sits dead center
    for (const x of [cL, cR]) { biquad(x, 'hp', p.sn_r_hp, 0.707, 0, sr); biquad(x, 'hp', p.sn_r_hp, 0.707, 0, sr); }
    const rs = rmsOf(send), rr = Math.sqrt((rmsOf(cL) ** 2 + rmsOf(cR) ** 2) / 2), g = rr > 1e-9 ? (rs / rr) * dbToLin(p.sn_r_lvl) : 0;
    for (let i = 0; i < N; i++) { L[i] += cL[i] * g; R[i] += cR[i] * g; }
  }
  widen(L, R, p, sr, rng(seed * 53 + 9));
  const gains = snareBus(L, R, p, sr, opts.fixed, P, !solo || solo === 'hit' ? lay.hit : null);
  return { L, R, sr, layers: lay, gains, pre: P };
}

// peaking EQ whose centre glides from f0 to f1 (time constant ms), from the hit onward
function tvPeak(x, f0, f1, ms, gainDb, q, sr, start) {
  const A = Math.pow(10, gainDb / 40), l0 = Math.log(f0), l1 = Math.log(f1);
  let b0 = 1, b1 = 0, b2 = 0, a1 = 0, a2 = 0, x1 = 0, x2 = 0, y1 = 0, y2 = 0;
  for (let i = 0; i < x.length; i++) {
    if ((i & 31) === 0) {
      const t = Math.max(0, i - start) / sr, f = Math.exp(l1 + (l0 - l1) * Math.exp(-t / (ms / 1000)));
      const w = (TAU * Math.min(f, sr * 0.45)) / sr, cw = Math.cos(w), al = Math.sin(w) / (2 * q), a0 = 1 + al / A;
      b0 = (1 + al * A) / a0; b1 = (-2 * cw) / a0; b2 = (1 - al * A) / a0; a1 = (-2 * cw) / a0; a2 = (1 - al / A) / a0;
    }
    const xi = x[i], y = b0 * xi + b1 * x1 + b2 * x2 - a1 * y1 - a2 * y2; x2 = x1; x1 = xi; y2 = y1; y1 = y; x[i] = y;
  }
}
function snareBus(L, R, p, sr, fixed, start = 0, hit = null) {
  if (p.sn_b_dip > 0.001) { // short notch right after the click: snappier
    const a = start + Math.floor(0.003 * sr), w = Math.floor(0.009 * sr);
    for (let i = 0; i < w && a + i < L.length; i++) { const g = 1 - 0.6 * p.sn_b_dip * Math.sin((Math.PI * i) / w); L[a + i] *= g; R[a + i] *= g; }
  }
  if (p.sn_b_whip > 0.05) for (const x of [L, R]) tvPeak(x, p.sn_b_whip_f0, p.sn_b_whip_f1, p.sn_b_whip_ms, p.sn_b_whip, 1.4, sr, start);
  for (const x of [L, R]) {
    lr4(x, 'hp', p.sn_b_hp, sr);
    if (Math.abs(p.sn_b_low) > 0.05) biquad(x, 'peak', 220, 0.8, p.sn_b_low, sr);
    if (Math.abs(p.sn_b_mid) > 0.05) biquad(x, 'peak', p.sn_b_mid_f, 1, p.sn_b_mid, sr);
    if (Math.abs(p.sn_b_high) > 0.05) biquad(x, 'hs', 5000, 0.7, p.sn_b_high, sr);
  }
  ottCore(L, R, p.sn_b_ott, 0.6, 0.6, sr);
  snap(L, R, p.sn_b_snap, sr);
  let pk = 0; for (let i = 0; i < L.length; i++) pk = Math.max(pk, Math.abs(L[i]), Math.abs(R[i]));
  const pre = fixed ? fixed.pre : pk > 0 ? 0.7 / pk : 1; for (let i = 0; i < L.length; i++) { L[i] *= pre; R[i] *= pre; }
  shape(L, p.sn_b_drive, p.sn_b_hard); shape(R, p.sn_b_drive, p.sn_b_hard);
  shape(L, 3, 0.2); shape(R, 3, 0.2);
  // hit: on top of the clipped body, level re the body's clipped peak
  pk = 0; for (let i = 0; i < L.length; i++) pk = Math.max(pk, Math.abs(L[i]), Math.abs(R[i]));
  const hp = fixed?.hp ?? pk;
  if (hit) for (let i = 0; i < L.length; i++) { L[i] += hit[0][i] * hp; R[i] += hit[1][i] * hp; }
  pk = 0; for (let i = 0; i < L.length; i++) pk = Math.max(pk, Math.abs(L[i]), Math.abs(R[i]));
  const g = fixed ? fixed.g : pk > 0 ? 0.966 / pk : 1; for (let i = 0; i < L.length; i++) { L[i] *= g; R[i] *= g; }
  // fade the last 5 ms
  const F = Math.floor(0.005 * sr); for (let i = 0; i < F; i++) { const k = L.length - 1 - i; if (k < 0) break; L[k] *= i / F; R[k] *= i / F; }
  return { pre, g, hp };
}
