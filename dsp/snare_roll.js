// Snare randomizer: anchor on a fitted snare when there are any, otherwise roll one of the hand-built flavors.
import { SPEC, SPEC_BY_ID, toNorm, fromNorm, inEngine } from './engine.js';

function gauss(r) { let u = 0; while (!u) u = r(); return Math.sqrt(-2 * Math.log(u)) * Math.cos(2 * Math.PI * r()); }
const pick = (r, a) => a[Math.floor(r() * a.length)];

const OFF = { sn_m_lvl: -60, sn_k_lvl: -60 };
// common to every family: noise peaks 5–25 ms after the hit, with a 1–2 kHz formant
const COMMON = { sn_n_att: [5, 25], sn_n_pk_f: [1000, 2200], sn_n_pk_g: [2, 8], sn_t_click: [30, 48], sn_t_click_ms: [1.5, 5] };
// families from the tutorial survey (research/yt_snares/SYNTHESIS.md §3, §5)
export const SNARE_FLAVORS = {
  brostep: { // flat-topped clipped body ~200–230 Hz, noise tail, long high-passed room
    base: { ...OFF, sn_t_wave: 0, sn_t_oct: 3, sn_c_type: 3, sn_r_type: 0 },
    vary: { sn_t_tune: [-3, 3], sn_t_round: [0.6, 1], sn_t_hold: [35, 55], sn_t_dec: [90, 180], sn_t_pitch: [5, 9], sn_t_pitch_ms: [10, 20], sn_t_drive: [16, 28], sn_t_drive_env: [0.2, 0.5],
      sn_n_lvl: [-4, 1], sn_n_hp: [1200, 2000], sn_n_dec: [200, 400], sn_n_drive: [6, 16], sn_c_lvl: [-14, -6], sn_c_len: [5, 10],
      sn_r_lvl: [-16, -10], sn_r_size: [700, 1400], sn_r_hp: [1200, 2000], sn_b_drive: [10, 18], sn_b_ott: [0.4, 0.7] },
  },
  whip: { // VR: triangle ~200 Hz, enveloped drive, peak sweeping up to ~1 kHz
    base: { ...OFF, sn_t_wave: 1, sn_t_oct: 3, sn_c_type: 3 },
    vary: { sn_t_tune: [-2, 4], sn_t_hold: [12, 28], sn_t_dec: [100, 180], sn_t_pitch: [3, 7], sn_t_pitch_ms: [60, 120], sn_t_drive: [12, 24], sn_t_drive_env: [0.6, 1],
      sn_n_lvl: [-4, 0], sn_n_hp: [600, 1000], sn_n_dec: [140, 260], sn_n_drive: [6, 14], sn_c_lvl: [-16, -8],
      sn_b_whip: [6, 12], sn_b_whip_f0: [250, 400], sn_b_whip_f1: [850, 1300], sn_b_whip_ms: [30, 100], sn_b_ott: [0.4, 0.7], sn_b_drive: [8, 16], sn_r_lvl: [-22, -14] },
  },
  riddim: { // kick-like click under dry clipped claps with an "ah" formant, no tail
    base: { ...OFF, sn_t_wave: 0, sn_t_oct: 2, sn_c_type: 0, sn_r_lvl: -60, sn_b_hard: 0.9 },
    vary: { sn_t_hold: [3, 8], sn_t_dec: [40, 80], sn_t_pitch: [18, 30], sn_t_pitch_ms: [5, 10], sn_t_drive: [10, 22], sn_t_lvl: [-6, 0],
      sn_k_lvl: [-2, 3], sn_k_n: [2, 4], sn_k_gap: [6, 11], sn_k_f: [700, 1250], sn_k_q: [1.5, 3], sn_k_dec: [40, 90], sn_k_drive: [16, 28], sn_k_delay: [0, 3],
      sn_n_lvl: [-14, -6], sn_n_dec: [50, 90], sn_n_drive: [12, 24], sn_b_drive: [14, 24] },
  },
  pan: { // tearout: talk, tail, tone — key-snapped narrow ring, distorted, loud
    base: { ...OFF, sn_t_wave: 0, sn_t_oct: 3, sn_c_type: 3, sn_r_type: 4 },
    vary: { sn_t_tune: [-3, 3], sn_t_hold: [20, 35], sn_t_dec: [70, 140], sn_t_pitch: [9, 14], sn_t_pitch_ms: [12, 18], sn_t_drive: [12, 24], sn_t_round: [0.2, 0.6],
      sn_n_lvl: [-2, 2], sn_n_hp: [800, 1300], sn_n_dec: [200, 380], sn_n_pk_f: [1600, 1800], sn_n_pk_g: [4, 8], sn_n_ring: [12, 24], sn_n_ring_st: [0, 7], sn_n_ring_q: [30, 90], sn_n_drive: [14, 28],
      sn_r_lvl: [-16, -10], sn_r_size: [600, 1000], sn_r_hp: [1200, 2000], sn_b_drive: [10, 20], sn_b_ott: [0.4, 0.7] },
  },
  dnb: { // higher rounded-square body, very short, noise enters as the body ends, resonant tail
    base: { ...OFF, sn_t_wave: 0, sn_t_oct: 4, sn_c_type: 3 },
    vary: { sn_t_tune: [-6, 0], sn_t_round: [0.5, 0.9], sn_t_hold: [10, 20], sn_t_dec: [50, 100], sn_t_pitch: [10, 14], sn_t_pitch_ms: [10, 15], sn_t_drive: [6, 14],
      sn_n_att: [10, 25], sn_n_lvl: [-3, 1], sn_n_hp: [1000, 2000], sn_n_dec: [100, 180], sn_n_ring: [8, 18], sn_n_ring_q: [40, 110], sn_n_drive: [4, 10],
      sn_r_lvl: [-28, -20], sn_r_size: [100, 220], sn_b_drive: [4, 10], sn_b_dip: [0, 0.8] },
  },
  trap: { // tuned body 250–600 Hz, clap, resonant ring at +1/+4/+7 st entering late
    base: { sn_t_wave: 0, sn_t_oct: 4, sn_c_type: 4, sn_m_type: 0 },
    vary: { sn_t_hold: [10, 20], sn_t_dec: [70, 130], sn_t_pitch: [5, 9], sn_t_pitch_ms: [8, 12], sn_t_fm: [0, 1.5], sn_t_fm_ratio: [4, 8], sn_t_drive: [2, 10],
      sn_k_lvl: [-6, 0], sn_k_n: [3, 4], sn_k_dec: [120, 260], sn_k_f: [1000, 2000],
      sn_m_lvl: [-10, -3], sn_m_oct: [6, 6], sn_m_tune: [1, 7], sn_m_spread: [0, 0.3], sn_m_dec: [150, 320], sn_m_delay: [3, 8], sn_m_drive: [0, 6],
      sn_n_lvl: [-8, -3], sn_n_dec: [150, 250], sn_r_lvl: [-22, -14], sn_b_drive: [2, 8] },
  },
  chonk: { // big white-noise riddim snare: loud barely-filtered white noise, gated, clipped hard over a short fat body
    base: { ...OFF, sn_n_type: 0, sn_t_wave: 0, sn_t_oct: 3, sn_c_type: 3, sn_b_hard: 1, sn_n_close: 0, sn_r_lvl: -60 },
    vary: { sn_t_lvl: [-4, 0], sn_t_round: [0.6, 1], sn_t_hold: [20, 40], sn_t_dec: [60, 110], sn_t_pitch: [10, 18], sn_t_pitch_ms: [8, 14], sn_t_drive: [16, 28],
      sn_n_lvl: [1, 5], sn_n_hp: [150, 450], sn_n_lp: [12000, 20000], sn_n_att: [0, 6], sn_n_hold: [40, 110], sn_n_dec: [120, 260], sn_n_curve: [0.6, 1], sn_n_pk_f: [900, 1600], sn_n_pk_g: [2, 6], sn_n_drive: [18, 34], sn_n_hard: [0.7, 1],
      sn_c_lvl: [-12, -4], sn_b_drive: [14, 26], sn_b_ott: [0.5, 0.8], sn_b_snap: [0, 3], sn_w_amt: [0.3, 0.7] },
  },
  beep: { // colour bass: pure sine octave pair as a flam, tiny click, hard clip
    base: { sn_m_lvl: -60, sn_k_lvl: -60, sn_t_wave: 0, sn_t_round: 0, sn_c_type: 1, sn_b_hard: 0.9 },
    vary: { sn_t_oct: [4, 5], sn_t_hold: [6, 14], sn_t_dec: [80, 160], sn_t_pitch: [0, 3], sn_t_click: [0, 12],
      sn_c_lvl: [-10, -4], sn_c_f: [2000, 7000], sn_n_lvl: [-12, -6], sn_n_hp: [1800, 3000], sn_n_dec: [50, 90], sn_b_ott: [0.6, 0.8], sn_b_drive: [10, 18], sn_r_lvl: [-26, -16] },
  },
};
// trap / big-room flute: near-sine lead at 0.8–1.2 kHz, soft 20–45 ms attack, a breathy "ch" noise burst on the front
export const FLUTE = {
  base: { ...OFF, sn_t_wave: 0, sn_t_oct: 5, sn_t_click: 0, sn_t_fm: 0, sn_t_partials: -60, sn_t_fold: 0, sn_t_curve: 0, sn_t_drive_env: 0,
    sn_n_type: 1, sn_n_att: 0, sn_n_close: 0, sn_n_ring: 0, sn_n_flutter: 0, sn_n_rattle: 0, sn_c_lvl: -60, sn_b_whip: 0, sn_b_dip: 0, sn_b_mid: 0, sn_b_low: 0, sn_b_hp: 200, sn_r_type: 0 },
  vary: { sn_t_tune: [-6, 5], sn_t_round: [0.1, 0.35], sn_t_even: [0.2, 0.45], sn_t_pitch: [-3, 0.5], sn_t_pitch_ms: [15, 50], sn_t_att: [22, 45], sn_t_hold: [40, 250], sn_t_dec: [150, 700], sn_t_drive: [0, 6], sn_t_grit: [0.1, 0.35],
    sn_n_lvl: [-1, 5], sn_n_hp: [2500, 5000], sn_n_lp: [10000, 16000], sn_n_hold: [10, 35], sn_n_dec: [50, 120], sn_n_pk_f: [4000, 7000], sn_n_pk_g: [0, 5], sn_n_drive: [0, 6],
    sn_r_lvl: [-22, -12], sn_r_size: [300, 900], sn_r_hp: [600, 1500], sn_w_amt: [0.3, 0.7], sn_b_ott: [0.2, 0.5], sn_b_drive: [0, 6], sn_b_snap: [0, 2], sn_b_high: [0, 4] },
};
// small chances of extra layers on any flavor
const EXTRAS = [
  { prob: 0.15, vary: { sn_m_lvl: [-14, -6], sn_m_type: [0, 4], sn_m_oct: [5, 6], sn_m_dec: [80, 300], sn_m_delay: [2, 20] } },
  { prob: 0.25, vary: { sn_n_type: [2, 6], sn_n_rate: [200, 8000] } },
  { prob: 0.3, set: { sn_c_type: 1 }, vary: { sn_c_n: [2, 8], sn_c_spread: [0.5, 2.5], sn_c_len: [2, 10] } },
];

function drawRange(Q, id, [a, b], r) {
  const s = SPEC_BY_ID[id]; if (!s) return;
  const lo = toNorm(s, a), hi = toNorm(s, b); Q[id] = fromNorm(s, lo + r() * (hi - lo));
}
export function rollSnareFlavor(P, r = Math.random, locked = new Set(), flavor = null) {
  const name = flavor || pick(r, Object.keys(SNARE_FLAVORS)), F = name === 'flute' ? FLUTE : SNARE_FLAVORS[name];
  const Q = { ...P }, keep = (id) => locked.has(SPEC_BY_ID[id]?.g);
  for (const s of SPEC) if (s.eng === 'snare' && !keep(s.id)) Q[s.id] = s.def;
  for (const [k, v] of Object.entries(F.base)) if (!keep(k)) Q[k] = v;
  for (const [k, v] of Object.entries({ ...COMMON, ...F.vary })) if (!keep(k)) drawRange(Q, k, v, r);
  if (name !== 'flute') for (const ex of EXTRAS) if (r() < ex.prob) { for (const [k, v] of Object.entries(ex.set || {})) if (!keep(k)) Q[k] = v; for (const [k, v] of Object.entries(ex.vary)) if (!keep(k)) drawRange(Q, k, v, r); }
  if (!locked.has('global')) { Q.note = 24 + Math.floor(r() * 12); Q.seed = 1 + Math.floor(r() * 9999); }
  Q.engine = 1; Q.len = 700;
  return Q;
}
// jitter around a fitted snare; type switches re-pick now and then
export function rollSnareAnchored(P, anchors, r = Math.random, locked = new Set(), sd = 0.08) {
  const A = pick(r, anchors), Q = { ...P, ...A, engine: 1 };
  for (const s of SPEC) {
    if (!inEngine(s, 'snare') || s.g.startsWith('sample') || locked.has(s.g) || s.id === 'len' || s.id === 'note') continue;
    if (s.id === 'seed') { Q.seed = 1 + Math.floor(r() * 9999); continue; }
    if (s.id.endsWith('_lvl') && A[s.id] <= -59) continue; // off stays off
    let u = toNorm(s, A[s.id] ?? s.def) + gauss(r) * sd;
    if (s.scale === 'int' && s.unit?.includes('/') && r() < 0.2) u = r();
    Q[s.id] = fromNorm(s, u);
  }
  return Q;
}
