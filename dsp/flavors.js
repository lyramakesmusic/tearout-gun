// Randomize recipes for the 808 and whoosh engines: a base patch per flavor plus ranges to draw from.
import { SPEC, SPEC_BY_ID, toNorm, fromNorm } from './engine.js';

const pick = (r, a) => a[Math.floor(r() * a.length)];
function draw(Q, ranges, r, keep) {
  for (const [id, [a, b]] of Object.entries(ranges)) { if (keep(id)) continue; const s = SPEC_BY_ID[id]; if (!s) continue; Q[id] = fromNorm(s, toNorm(s, a) + r() * (toNorm(s, b) - toNorm(s, a))); }
}
function roll(P, r, locked, eng, engineId, flavors, name) {
  const F = flavors[name || pick(r, Object.keys(flavors))], keep = (id) => locked.has(SPEC_BY_ID[id]?.g), Q = { ...P, engine: engineId };
  for (const s of SPEC) if (s.eng === eng && !keep(s.id)) Q[s.id] = s.def;
  for (const [k, v] of Object.entries(F.base || {})) if (!keep(k)) Q[k] = v;
  draw(Q, F.vary, r, keep);
  if (F.after) F.after(Q, r, keep);
  if (!locked.has('global')) Q.seed = 1 + Math.floor(r() * 9999);
  return Q;
}

// 808s: base matches a reference trap 808 (96 → 38 Hz in ~60 ms, drift −1.6 st over ~0.5 s, flat ~100 ms, −21 dB at 900 ms, gate ~1 s, 3rd harmonic −14 dB)
export const E808_FLAVORS = {
  trap: { base: { e8_p_type: 3 }, vary: { e8_s_dive: [12, 20], e8_s_dive_ms: [10, 20], e8_s_drift: [0.5, 2.5], e8_s_drift_ms: [120, 260], e8_s_hold: [60, 160], e8_s_dec: [2000, 3400], e8_s_gate: [800, 1400], e8_p_drive: [3, 9], e8_p_drive_fall: [0.3, 0.7], e8_p_tone: [2200, 4500], e8_p_clean: [0, 0.4], e8_p_clip: [0, 2] } },
  clean: { base: { e8_p_type: 3 }, vary: { e8_s_dive: [6, 14], e8_s_dive_ms: [12, 30], e8_s_drift: [0, 1.5], e8_s_hold: [80, 250], e8_s_dec: [2000, 4000], e8_s_gate: [900, 1800], e8_p_drive: [2, 6], e8_p_tone: [1500, 3500], e8_p_clean: [0.5, 1], e8_p_clip: [0, 1.5] } },
  distorted: { base: { e8_p_type: 3 }, vary: { e8_s_dive: [14, 26], e8_s_dive_ms: [8, 18], e8_s_drift: [1, 3], e8_s_hold: [80, 200], e8_s_dec: [1600, 3000], e8_s_gate: [600, 1200], e8_p_drive: [12, 22], e8_p_drive_fall: [0.4, 0.8], e8_p_tone: [2000, 5000], e8_p_clean: [0.3, 0.8], e8_p_ott: [0.1, 0.4], e8_p_clip: [1, 4], e8_f_amt: [0, 1.5], e8_f_ratio: [1, 3] } },
  fm: { base: { e8_p_type: 3 }, vary: { e8_s_dive: [10, 20], e8_s_dive_ms: [10, 20], e8_s_hold: [60, 150], e8_s_dec: [1600, 3000], e8_s_gate: [600, 1200], e8_f_amt: [0.6, 2.2], e8_f_ratio: [1, 3], e8_f_dec: [40, 250], e8_f_sustain: [0, 0.3], e8_f_fb: [0, 0.3], e8_p_drive: [5, 12], e8_p_drive_fall: [0.3, 0.7], e8_p_tone: [2000, 4500], e8_p_clean: [0.3, 0.7] } },
  punchy: { base: { e8_n_lvl: -14 }, vary: { e8_s_dive: [20, 32], e8_s_dive_ms: [6, 12], e8_s_hold: [20, 80], e8_s_dec: [400, 900], e8_s_gate: [350, 700], e8_n_lvl: [-20, -8], e8_n_hp: [2000, 6000], e8_n_dec: [4, 15], e8_n_click: [12, 36], e8_p_drive: [6, 14], e8_p_drive_fall: [0.4, 0.8], e8_p_tone: [2500, 5000], e8_p_clip: [1, 3] } },
};
export const roll808 = (P, r = Math.random, locked = new Set(), name) => { const Q = roll(P, r, locked, '808', 2, E808_FLAVORS, name); if (!locked.has('global')) Q.note = 24 + Math.floor(r() * 12); return Q; };

// whooshes: bell-shaped swells through a moving bandpass, crashes, wind, chopped hype loops, foley swishes, impacts
export const WHOOSH_FLAVORS = {
  // the whoosh before a punch: noise rushing up in level and brightness, landing on the hit, then a quick fall
  whoosh: { base: { w_f_type: 1, w_m_lfo_amt: 0, w_a_hold: 0 }, vary: { w_a_len: [220, 650], w_a_att: [160, 520], w_a_att_curve: [2.5, 4], w_a_dec_curve: [1.5, 3], w_s_type: [0, 1], w_f_start: [250, 700], w_f_peak: [3000, 9000], w_f_end: [1500, 5000], w_f_peak_at: [0.75, 0.92], w_f_q: [0.7, 1.6], w_f_mix: [0.8, 1], w_x_pass: [0, 0.4], w_x_rev: [-30, -18], w_x_size: [200, 700], w_x_width: [0.3, 0.7] },
    // the rise takes 60–85% of the length, so there's always a short fall after the hit
    after: (Q, r, keep) => { if (!keep('w_a_att')) Q.w_a_att = Q.w_a_len * (0.6 + 0.25 * r()); } },
  swish: { base: { w_f_type: 1, w_m_lfo_amt: 0 }, vary: { w_a_len: [150, 450], w_a_att: [40, 180], w_a_att_curve: [1.5, 3], w_s_type: [0, 0], w_f_start: [1500, 4000], w_f_peak: [5000, 12000], w_f_end: [2000, 6000], w_f_peak_at: [0.3, 0.6], w_f_q: [1, 4], w_x_rev: [-26, -16], w_x_size: [200, 700] } },
  crash: { base: { w_f_type: 2, w_a_att: 0, w_a_hold: 0 }, vary: { w_a_len: [1500, 4500], w_a_dec_curve: [1.5, 3.5], w_s_metal: [-6, 2], w_s_metal_f: [1200, 4000], w_s_metal_spread: [0.4, 1], w_s_type: [0, 0], w_f_start: [300, 900], w_f_peak: [300, 900], w_f_end: [600, 2000], w_f_q: [0.5, 0.9], w_x_rev: [-18, -8], w_x_size: [1200, 3500], w_p_lp: [9000, 16000] } },
  'reverse crash': { base: { w_f_type: 2, w_a_att: 0, w_a_hold: 0, w_x_reverse: 1 }, vary: { w_a_len: [1200, 3000], w_a_dec_curve: [1.5, 3], w_s_metal: [-6, 2], w_s_metal_f: [1200, 4000], w_f_start: [400, 900], w_f_peak: [400, 900], w_f_end: [800, 2000], w_f_q: [0.5, 0.9], w_x_rev: [-14, -6], w_x_size: [1500, 3500] } },
  wind: { base: { w_f_type: 1, w_s_type: 1 }, vary: { w_a_len: [2500, 6000], w_a_att: [600, 2000], w_a_att_curve: [1, 2], w_a_dec_curve: [0.6, 1.5], w_f_start: [300, 900], w_f_peak: [600, 2500], w_f_end: [300, 900], w_f_q: [1.2, 4], w_m_lfo: [0.15, 0.6], w_m_lfo_amt: [0.15, 0.5], w_x_width: [0.7, 1], w_x_rev: [-18, -8], w_x_size: [1500, 4000] } },
  'hype loop': { base: { w_f_type: 0, w_m_gate_amt: 1 }, vary: { w_a_len: [1500, 4000], w_a_att: [800, 3000], w_a_att_curve: [1.5, 3], w_a_dec_curve: [2, 4], w_s_type: [0, 1], w_f_start: [300, 900], w_f_peak: [8000, 18000], w_f_end: [8000, 18000], w_f_peak_at: [0.7, 0.95], w_f_q: [1, 4], w_m_gate: [6, 16], w_m_gate_duty: [0.3, 0.6], w_m_gate_accel: [0, 0.8], w_x_rev: [-20, -10], w_p_drive: [0, 10] } },
  impact: { base: { w_f_type: 0, w_a_att: 0, w_b_lvl: 0 }, vary: { w_a_len: [800, 2500], w_a_dec_curve: [1.5, 3.5], w_s_type: [0, 2], w_f_start: [2000, 8000], w_f_peak: [2000, 8000], w_f_end: [200, 800], w_f_q: [0.6, 1.4], w_b_freq: [40, 90], w_b_drop: [6, 24], w_b_noise: [0.1, 0.5], w_b_dec: [300, 1200], w_b_lvl: [-4, 4], w_x_rev: [-14, -6], w_x_size: [1500, 4000], w_p_drive: [4, 16] } },
  foley: { base: { w_s_type: 3, w_f_type: 1 }, vary: { w_a_len: [200, 900], w_a_att: [20, 200], w_s_density: [300, 6000], w_f_start: [800, 3000], w_f_peak: [3000, 10000], w_f_end: [1500, 5000], w_f_q: [0.8, 3], w_x_rev: [-26, -14], w_x_size: [200, 800], w_x_width: [0.3, 0.8] } },
};
// punch whooshes and swishes come up most often
const WHOOSH_WEIGHTS = ['whoosh', 'whoosh', 'whoosh', 'swish', 'swish', 'crash', 'reverse crash', 'wind', 'hype loop', 'impact', 'foley'];
export const rollWhoosh = (P, r = Math.random, locked = new Set(), name) => roll(P, r, locked, 'whoosh', 3, WHOOSH_FLAVORS, name || pick(r, WHOOSH_WEIGHTS));
