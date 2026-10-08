// Best-of-K rolling: draw candidates from a family (or around a fitted preset), keep the most gun-like.
import { render } from './engine.js';
import { rollArchetype, balance, TARGETS } from './archetypes.js';
import { randomize, mutate } from './random.js';
import { features, gunScore } from './gunness.js';
import { engineOf, lr4 } from './engine.js';
import { rollSnareFlavor, rollSnareAnchored } from './snare_roll.js';

// energy after 500 ms relative to the whole hit, dB
function lateDb(o) {
  const n = o.L.length, cut = Math.floor(0.5 * 48000); let a = 0, b = 0;
  for (let i = 0; i < n; i++) { const v = 0.5 * (o.L[i] + o.R[i]), e = v * v; a += e; if (i >= cut) b += e; }
  return 10 * Math.log10(b / (a || 1) + 1e-9);
}
// Guns: the mids should be a real block over the sub, not the sub's own clipping buzz riding its peaks.
// am = correlation of the >300 Hz envelope with |sub| over 20–200 ms (KFU median −0.44); mos = mids peak re sub peak, dB (KFU −2.3)
export function subLock(x, sr = 48000) {
  const hi = Float32Array.from(x), lo = Float32Array.from(x); lr4(hi, 'hp', 300, sr); lr4(lo, 'lp', 120, sr);
  const a0 = Math.floor(0.02 * sr), a1 = Math.min(x.length, Math.floor(0.2 * sr)); let sx = 0, sy = 0, sxx = 0, syy = 0, sxy = 0, k = 0, sm = 0, ph = 0, pl = 0;
  for (let i = 0; i < x.length; i++) { ph = Math.max(ph, Math.abs(hi[i])); pl = Math.max(pl, Math.abs(lo[i])); }
  for (let i = a0; i < a1; i++) { sm = 0.98 * sm + 0.02 * Math.abs(hi[i]); const u = sm, v = Math.abs(lo[i]); sx += u; sy += v; sxx += u * u; syy += v * v; sxy += u * v; k++; }
  const am = k ? (sxy / k - (sx / k) * (sy / k)) / Math.sqrt((sxx / k - (sx / k) ** 2) * (syy / k - (sy / k) ** 2) + 1e-20) : 0;
  return { am, mos: 20 * Math.log10((ph + 1e-9) / (pl + 1e-9)) };
}
const lockPenalty = ({ am, mos }) => 2 * Math.max(0, am + 0.1) + 0.15 * Math.max(0, -6 - mos);
// chugs swell in: no click or snap at the onset; any transient sits inside the swell
export function chugify(Q, r = Math.random) {
  Q = { ...Q, mst_snap: 0, sub_click: 0, tr_click: -60, tr_zap: -60 };
  if (Q.tr_lvl > -59) { Q.tr_delay = Math.max(Q.tr_delay || 0, 18 + 25 * r()); Q.tr_att = Math.max(Q.tr_att || 0, 3); }
  Q.mst_swell = Math.max(Q.mst_swell || 0, 12 + 28 * r());
  Q.body_swell = Math.max(Q.body_swell || 0, 15); Q.syn_swell = Math.max(Q.syn_swell || 0, 15); Q.sub_swell = Math.max(Q.sub_swell || 0, 10);
  return Q;
}
// families with their own reference (from real samples) are scored against it; everything else against guns
export function roll(mode, P, { priors, ref, famRefs = null, famFits = null, locked = new Set(), k = 5, r = Math.random, withScore = false } = {}) {
  ref = famRefs?.[mode] || (mode === 'flute' ? null : ref);
  const anchors = famFits?.[mode];
  let best = null;
  const limit = mode === 'mutate' ? Math.max(-25, lateDb(render({ ...P, b_shots: 1 })) + 3) : Infinity;
  let tries = 0;
  for (let i = 0; i < k; i++) {
    let Q = mode === 'mutate' ? mutate(P, locked, r, 0.06)
      : mode === 'flute' ? rollSnareFlavor(P, r, locked, 'flute')
      : mode === 'snare' ? (famFits?.snare_eng?.length && r() < 0.75 ? rollSnareAnchored(P, famFits.snare_eng, r, locked) : rollSnareFlavor(P, r, locked))
      : mode === 'fitted' ? randomize(P, priors, locked, r)
      : anchors?.length && mode !== 'snare' ? balance(randomize(P, { guns: anchors, keepSpice: true }, locked, r), locked, TARGETS[mode])
      : rollArchetype(mode, r, locked, P);
    if (mode === 'chug') Q = chugify(Q, r);
    if (!ref && mode !== 'mutate') return withScore ? { Q, s: 0 } : Q;
    const o = render({ ...Q, b_shots: 1 });
    if (lateDb(o) > limit && ++tries < 3 * k) { i--; continue; } // too long: redraw
    const mono = Float32Array.from(o.L, (v, j) => 0.5 * (v + o.R[j]));
    let s = ref ? gunScore(features(mono), ref) : 0;
    if (engineOf(Q) === 'gun' && mode !== 'chug') s += lockPenalty(subLock(mono));
    if (!best || s < best.s) best = { Q, s };
  }
  return withScore ? best : best.Q;
}
