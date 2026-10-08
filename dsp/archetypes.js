// Sound families for the randomizer: a base patch plus the knobs that vary within the family.
import { SPEC_BY_ID, defaults, toNorm, fromNorm, render } from './engine.js';
import { thirdOct } from '../web/worker.js';

const COMMON = { syn_lvl: -60, spice_lvl: -60, cr_mix: 1, mst_sub_in: 0, body_width: 0.6 };

export const ARCHETYPES = {
  gun: {
    base: {
      sub_lvl: -3, sub_dive: 60, sub_tau: 15, sub_drive: 4, sub_hard: 0.3, sub_noise: -60, sub_hold: 260, sub_rel: 20, sub_dec: 600,
      tr_lvl: 0, tr_color: 0.3, tr_hp: 800, tr_pk_f: 3000, tr_pk_g: 6, tr_click: -60, tr_click_f0: 9000, tr_click_tau: 2, tr_hold: 45, tr_rel: 2, tr_dec: 80, tr_drive: 18,
      body_lvl: -2, body_noise: 0, body_color: 0, body_modes: -10, body_mode_f: 500, body_metal: 0.6, body_q: 40, body_comb: -14, body_comb_f: 200, body_fm: -60,
      body_flt_type: 0, body_flt_f0: 14000, body_flt_f1: 2200, body_flt_tau: 80, body_flt_res: 0.25, body_flt_mix: 1, body_hp: 300,
      body_hold: 300, body_rel: 6, body_dec: 400, body_shape: 1, body_drive: 20, body_fold: 0.15,
      cr_drive1: 16, cr_disp_n: 8, cr_disp_f: 1500, cr_conv: 0.5, cr_conv_type: 0, cr_conv_len: 30, cr_drive2: 10, cr_rack: 1, cr_rack_depth: 0.5,
      rev_lvl: -14, rev_type: 0, rev_size: 300, mst_ott: 0.5, mst_drive: 8,
    },
    vary: {
      note: [24, 35], sub_dive: [40, 100], sub_tau: [8, 70], tr_hold: [12, 60], tr_hp: [150, 2000], tr_pk_f: [1000, 8000], tr_color: [-0.8, 0.6], tr_att: [0, 1], tr_drive: [10, 40], body_att: [1, 12], mst_snap: [4, 9], tr_shape: [0, 2],
      tr_rel: [1, 30], tr_close: [700, 6000], tr_close_ms: [15, 120], body_dec: [60, 600], mst_sub_in: [0, 0.9], mst_drive: [6, 22], cr_mix: [0, 1],
      body_hold: [180, 450], body_noise: [-6, 3], body_modes: [-24, -4], body_mode_f: [200, 2000], body_metal: [0.2, 1], body_flt_f1: [1200, 5000], body_flt_tau: [40, 220], body_flt_type: [0, 1], cr_rack: [0, 2], body_comb: [-40, -4], body_comb_f: [80, 800],
      body_shape: [0, 2], body_drive: [10, 32], body_drop: [0, 24], cr_disp_n: [0, 20], cr_disp_f: [300, 5000], cr_conv: [0.2, 0.9], cr_conv_type: [0, 4], rev_size: [150, 600], rev_type: [0, 4], rev_char: [0, 1], rev_tone: [3000, 16000], rev_lvl: [-24, -8], body_mode_set: [0, 6], spice_type: [0, 4],
    },
    // the transient owns the onset: its level rides relative to the body
    rel: { tr_lvl: ['body_lvl', 4, 14] },
    maybe: [
      { prob: 0.3, set: {}, vary: { tr_zap: [-12, 0], tr_zap_f0: [6000, 16000], tr_zap_f1: [150, 1500], tr_zap_ms: [10, 60] } },
      { prob: 0.15, set: {}, vary: { tr_stutter: [2, 4], tr_stut_ms: [8, 30], tr_dec: [5, 20] } },
      {
      prob: 0.35, set: { syn_shape: 1 }, tie: { syn_hold: 'body_hold' },
      vary: { syn_lvl: [-14, -3], syn_wave: [0, 3], syn_oct: [1, 3], syn_detune: [0.1, 0.7], syn_pitch: [0, 24], syn_pitch_ms: [10, 80], syn_flt_f1: [800, 4000], syn_drive: [4, 20] },
    }],
  },
  snare: {
    rel: { tr_lvl: ['body_lvl', 2, 10] },
    base: {
      // shell: the sine generator tuned up to ~150–250 Hz with a small drop
      sub_lvl: -4, sub_tune: 26, sub_dive: 14, sub_tau: 12, sub_k: 1, sub_hold: 200, sub_dec: 90, sub_rel: 20, sub_drive: 3, sub_hard: 0.3, sub_noise: -60, sub_shape: 0,
      tr_lvl: 0, tr_color: 0.2, tr_hp: 600, tr_lp: 18000, tr_pk_f: 2500, tr_pk_g: 4, tr_click: -60, tr_hold: 14, tr_dec: 30, tr_rel: 4, tr_drive: 14, tr_shape: 0,
      // wires: broadband noise wash, core around 1–3 kHz, smooth exponential decay
      body_lvl: 0, body_noise: 0, body_color: 0.1, body_modes: -20, body_mode_set: 1, body_mode_f: 220, body_metal: 0.2, body_q: 20, body_nmodes: 6,
      body_comb: -60, body_fm: -60, body_flt_type: 0, body_flt_f0: 18000, body_flt_f1: 7000, body_flt_tau: 180, body_flt_res: 0.1, body_flt_mix: 0.6, body_hp: 1000,
      body_att: 0.5, body_hold: 600, body_dec: 70, body_rel: 20, body_shape: 0, body_drive: 12, body_fold: 0.05, body_width: 0.6,
      cr_mix: 0.5, cr_drive1: 10, cr_disp_n: 2, cr_conv: 0.35, cr_conv_type: 4, cr_conv_len: 25, cr_drive2: 6, cr_rack: 1, cr_rack_depth: 0.35,
      rev_lvl: -12, rev_type: 0, rev_size: 400, mst_ott: 0.35, mst_drive: 6, mst_sub_in: 0.5,
    },
    vary: {
      note: [24, 35], sub_tune: [6, 24], sub_tau: [6, 30], tr_delay: [0, 0], sub_delay: [0, 6], body_delay: [4, 25], sub_hold: [100, 240], sub_drive: [4, 16], sub_lvl: [-9, -3], sub_dive: [12, 30], mst_drive: [10, 18], mst_hard: [0.4, 0.8],
      tr_hp: [400, 2000], tr_hold: [6, 20], tr_color: [-0.5, 0.3], mst_snap: [2, 6], body_dec: [35, 90], body_hp: [300, 800], body_mode_set: [0, 3], body_flt_tau: [40, 120], body_noise: [-3, 4], body_modes: [-30, -8],
      body_mode_f: [150, 400], body_drive: [10, 26], body_flt_f1: [1800, 4500], body_color: [-0.6, -0.1], cr_mix: [0.4, 0.9], cr_rack: [1, 2], cr_conv_type: [0, 4], rev_type: [0, 4], rev_size: [120, 350], rev_lvl: [-30, -18],
    },
  },
  ping: {
    rel: { tr_lvl: ['body_lvl', -10, -2] },
    base: {
      sub_lvl: -60,
      tr_lvl: -6, tr_color: 0.4, tr_hp: 1200, tr_hold: 12, tr_dec: 15, tr_rel: 2, tr_click: -60, tr_drive: 14, tr_shape: 0,
      // clean metal: many high-Q inharmonic partials, almost no noise
      body_lvl: 0, body_noise: -40, body_color: 0.3, body_modes: 0, body_mode_set: 4, body_mode_f: 800, body_metal: 0.9, body_q: 300, body_nmodes: 20, body_mode_tilt: -1,
      body_comb: -60, body_fm: -60, body_flt_mix: 0, body_hp: 400, body_att: 0.2, body_hold: 600, body_dec: 140, body_rel: 20, body_shape: 0, body_drive: 8, body_fold: 0,
      spice_lvl: -60,
      cr_mix: 0.2, cr_drive1: 8, cr_disp_n: 0, cr_conv: 0.2, cr_conv_type: 1, cr_conv_len: 20, cr_drive2: 4, cr_rack: 0, cr_rack_depth: 0.3,
      rev_lvl: -18, rev_type: 1, rev_size: 250, mst_ott: 0.2, mst_drive: 4, mst_sub_in: 0, mst_snap: 3,
    },
    vary: {
      note: [24, 35], body_mode_set: [0, 6], body_mode_f: [400, 1600], body_q: [150, 400], body_metal: [0.6, 1], body_nmodes: [14, 24], body_mode_tilt: [-1, 4],
      body_dec: [40, 110], body_drive: [2, 14], body_key: [0, 1], tr_hold: [6, 20], tr_hp: [150, 800], mst_drive: [6, 12],
      cr_mix: [0, 0.4], rev_type: [0, 4], rev_size: [120, 400], rev_lvl: [-26, -14],
    },
  },
  chug: {
    // a gun's distorted-noise texture with a ^ envelope: it chugs in instead of slamming
    base: {
      sub_lvl: 0, sub_tune: 0, sub_dive: 14, sub_tau: 10, sub_k: 1, sub_hold: 300, sub_dec: 2000, sub_rel: 14, sub_shape: 0,
      sub_drive: 22, sub_hard: 0.6, sub_noise: -18, sub_noise_hp: 900, sub_delay: 25, sub_swell: 40,
      tr_lvl: -60,
      body_lvl: 0, body_noise: 0, body_color: 0, body_modes: -10, body_mode_f: 500, body_metal: 0.6, body_q: 40, body_comb: -14, body_comb_f: 200, body_fm: -60,
      body_flt_type: 0, body_flt_f0: 12000, body_flt_f1: 2000, body_flt_tau: 90, body_flt_res: 0.25, body_flt_mix: 1, body_hp: 250,
      body_att: 2, body_swell: 60, body_hold: 300, body_dec: 110, body_rel: 10, body_shape: 0, body_drive: 22, body_fold: 0.15, body_width: 0.5,
      syn_lvl: -60,
      cr_mix: 0.8, cr_drive1: 16, cr_disp_n: 8, cr_disp_f: 1500, cr_conv: 0.4, cr_conv_type: 0, cr_conv_len: 30, cr_drive2: 10, cr_rack: 0, cr_rack_depth: 0.3,
      rev_lvl: -22, rev_type: 0, rev_size: 200, mst_ott: 0.2, mst_drive: 12, mst_hard: 0.6, mst_sub_in: 0.5, mst_snap: 0, mst_swell: 50, mst_fall: 110,
    },
    vary: {
      note: [24, 35], body_swell: [35, 110], sub_swell: [15, 80], sub_delay: [5, 50], body_hold: [200, 400], body_dec: [60, 170], mst_ott: [0.05, 0.3], cr_rack: [0, 1], mst_swell: [30, 90], mst_fall: [60, 220],
      sub_hold: [180, 450], sub_drive: [14, 32], sub_noise: [-30, -10], body_noise: [-6, 3], body_modes: [-24, -4], body_mode_f: [200, 1500], body_metal: [0.2, 1],
      body_flt_f1: [1200, 5000], body_flt_tau: [40, 220], body_drive: [12, 32], body_comb: [-40, -6], body_comb_f: [80, 800],
      cr_disp_n: [0, 20], cr_conv: [0.2, 0.8], cr_conv_type: [0, 4], mst_sub_in: [0.2, 0.7], mst_drive: [8, 20], rev_type: [0, 4], rev_size: [120, 400],
    },
    maybe: [{ prob: 0.35, set: { syn_shape: 2 }, tie: { syn_hold: 'body_hold', syn_swell: 'body_swell' }, vary: { syn_lvl: [-14, -4], syn_wave: [0, 3], syn_oct: [0, 2], syn_flt_f1: [800, 4000], syn_drive: [10, 28], syn_detune: [0.1, 0.6] } }],
  },
};

// Roll a patch from a family: base values, varied knobs uniform in their normalized range, fresh seed.
export function rollArchetype(name, r = Math.random, locked = new Set(), P = defaults()) {
  const a = ARCHETYPES[name], Q = { ...P, engine: 0 };
  const fresh = { ...defaults(), ...COMMON, ...a.base };
  for (const [k, v] of Object.entries(fresh)) { const g = SPEC_BY_ID[k]?.g; if (!locked.has(g) && !g?.startsWith('sample')) Q[k] = v; }
  for (const [k, [lo, hi]] of Object.entries(a.vary)) {
    const s = SPEC_BY_ID[k]; if (locked.has(s.g)) continue;
    const u0 = toNorm(s, lo), u1 = toNorm(s, hi);
    Q[k] = fromNorm(s, u0 + r() * (u1 - u0));
  }
  for (const [k, src] of Object.entries(a.tie || {})) if (!locked.has(SPEC_BY_ID[k].g)) Q[k] = Q[src];
  // relative levels: { target: [source, lo dB, hi dB] }
  for (const [k, [src, lo, hi]] of Object.entries(a.rel || {})) if (!locked.has(SPEC_BY_ID[k].g)) Q[k] = Math.min(6, Q[src] + lo + r() * (hi - lo));
  // optional extra layers: { prob, set, vary }
  for (const opt of a.maybe || []) {
    if (r() >= opt.prob || [...Object.keys(opt.set), ...Object.keys(opt.vary)].some((k) => locked.has(SPEC_BY_ID[k].g))) continue;
    Object.assign(Q, opt.set);
    for (const [k, [lo, hi]] of Object.entries(opt.vary)) { const s = SPEC_BY_ID[k], u0 = toNorm(s, lo), u1 = toNorm(s, hi); Q[k] = fromNorm(s, u0 + r() * (u1 - u0)); }
    if (opt.tie) for (const [k, src] of Object.entries(opt.tie)) Q[k] = Q[src];
  }
  if (!locked.has('global')) Q.seed = 1 + Math.floor(r() * 9999);
  return balance(Q, locked, TARGETS[name]);
}

// 1–8 kHz average, dB re the loudest third-octave band. The reference median is −17.4; ping sits a little hotter, chug darker.
const TARGET_HIGH = -17.4;
export const TARGETS = { gun: -17.4, snare: -6.6, ping: -5.1, chug: -25.3 }; // snare/ping/chug: medians of Lyra's labeled examples
function highBand(P) {
  const o = render({ ...P, b_shots: 1 });
  const t = thirdOct(Float32Array.from(o.L, (v, i) => 0.5 * (v + o.R[i])), o.sr);
  const v = t.db.filter((_, i) => t.centers[i] >= 1000 && t.centers[i] < 8100);
  return v.reduce((a, b) => a + b, 0) / v.length;
}
// Move the tops (transient + body) until the 1–8 kHz band sits on the target curve.
export function balance(P, locked = new Set(), target = TARGET_HIGH) {
  if (locked.has('tr') && locked.has('body')) return P;
  const Q = { ...P };
  for (let it = 0; it < 3; it++) {
    const d = Math.max(-18, Math.min(18, target - highBand(Q)));
    if (Math.abs(d) < 1) break;
    for (const k of ['tr_lvl', 'body_lvl', 'spice_lvl']) if (!locked.has(k.split('_')[0]) && Q[k] > -59) Q[k] = Math.max(-58, Math.min(6, Q[k] + d));
  }
  return Q;
}

// The starting patch: the gun family's base, balanced against the target curve.
export function initPatch() {
  const a = ARCHETYPES.gun;
  const P = { ...defaults(), ...COMMON, ...a.base, tr_lvl: a.base.body_lvl + 4, tr_close: 3000, tr_close_ms: 50, body_att: 3, seed: 1 };
  return balance(P, new Set(), TARGETS.gun);
}
