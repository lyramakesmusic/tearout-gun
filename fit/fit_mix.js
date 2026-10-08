// Fit layer levels + master on our own rendered layers against a KFU gun's PROCESSED.
// usage: node fit/fit_mix.js "<gun dir>"  -> out/fits/<gun>/mix.json
import { writeFileSync, existsSync } from 'fs';
import { basename } from 'path';
import { render, bus, SPEC_BY_ID, toNorm, fromNorm, defaults, SR } from '../dsp/engine.js';
import { readWav, melStack, melLoss, cmaes, ltas, ltasLoss, thirdOct, tobLoss } from './lib.js';
import { assemble } from './build_priors.js';
import { findStems, BUS_PARAMS } from './fit_bus.js';

const dir = process.argv[2], gun = basename(dir), out = `out/fits/${gun}`;
const base = assemble(out)?.params;
if (!base) { console.error(`${gun}: missing layer fits`); process.exit(0); }
const n = Math.floor(0.9 * SR);
const P = readWav(findStems(dir).proc).ch, pm = Float32Array.from(P[0].subarray(0, n), (v, i) => 0.5 * (v + (P[1] || P[0])[i]));
const T = melStack(pm, SR), TL = ltas(pm, SR), TT = thirdOct(pm, SR);
const TW = 0.3; // tonal-balance weight

// each layer rendered alone at 0 dB, raw (no master); reverb from the tops of the base mix
const only = (k) => ({ ...base, len: 900, sub_lvl: -60, tr_lvl: -60, body_lvl: -60, spice_lvl: -60, rev_lvl: -60, cr_mix: 0, [k]: 0 });
const lay = {};
for (const k of ['sub_lvl', 'tr_lvl', 'body_lvl']) { const o = render(only(k), { raw: true }); lay[k] = [o.L.subarray(0, n), o.R.subarray(0, n)]; }
// reverb is fed by the tops, so it is rendered per top layer and scales with that layer's level
const revOf = (k) => { const rv = render({ ...only(k), rev_lvl: 0 }, { raw: true }).layers.rev; return rv ? [rv[0].subarray(0, n), rv[1].subarray(0, n)] : null; };
const revTr = revOf('tr_lvl'), revBody = revOf('body_lvl');

const LV = [['sub_lvl', -24, 6], ['tr_lvl', -40, 6], ['body_lvl', -40, 6], ['rev_lvl', -40, 0]];
const MIX_PARAMS = [...BUS_PARAMS, 'mst_sub_in'];
const specs = MIX_PARAMS.map((i) => SPEC_BY_ID[i]);
const mix = (u) => {
  const g = LV.map(([, lo, hi], j) => lo + u[j] * (hi - lo));
  const acc = (j) => { const k = LV[j][0]; if (!lay[k]) return null; const s = 10 ** (g[j] / 20); return lay[k].map((ch) => Float32Array.from(ch, (v) => v * s)); };
  const tops = [acc(1), acc(2)].filter(Boolean).reduce((a, b) => (a ? a.map((ch, c) => ch.map((v, i) => v + b[c][i])) : b), null);
  const gr = 10 ** (g[3] / 20), gt = 10 ** (g[1] / 20) * gr, gb = 10 ** (g[2] / 20) * gr;
  const rev = revTr || revBody ? [0, 1].map((c) => Float32Array.from({ length: n }, (_, i) => (revTr ? gt * revTr[c][i] : 0) + (revBody ? gb * revBody[c][i] : 0))) : null;
  const p = { ...defaults() }; specs.forEach((s, i) => (p[s.id] = fromNorm(s, u[LV.length + i])));
  return { g, p, ...bus(acc(0), tops, rev, p, SR) };
};
const loss = (u) => { const { L, R } = mix(u); const m = Float32Array.from(L, (v, i) => 0.5 * (v + R[i])); return melLoss(melStack(m, SR), T).loss + 0.5 * ltasLoss(ltas(m, SR), TL) + TW * tobLoss(m, TT, SR); };
const x0 = [...LV.map(([k, lo, hi]) => (Math.max(lo, Math.min(hi, base[k])) - lo) / (hi - lo)), ...specs.map((s) => toNorm(s, base[s.id]))];
const before = loss(x0);
let best = { f: Infinity };
for (let k = 0; k < 2; k++) { const b = cmaes(loss, x0, { sigma: 0.25, maxEvals: 900, seed: 5 + k }); if (b.f < best.f) best = b; }
const { g, p } = mix(best.x);
const params = { ...Object.fromEntries(LV.map(([k], j) => [k, g[j]])), ...Object.fromEntries(MIX_PARAMS.map((k) => [k, p[k]])) };
const top = Math.max(params.sub_lvl, params.tr_lvl, params.body_lvl);
for (const k of ['sub_lvl', 'tr_lvl', 'body_lvl']) params[k] -= top;
writeFileSync(`${out}/mix.json`, JSON.stringify({ gun, before, loss: best.f, params }, null, 1));
console.error(`${gun} mix ${before.toFixed(3)} -> ${best.f.toFixed(3)}`);
