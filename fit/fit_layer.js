// Fit one layer's params to a target stem with CMA-ES on multi-res log-mel loss.
// usage: node fit/fit_layer.js --layer sub|tr|body --target path.wav --note 26 [--evals 2500] [--out out/fits/x.json] [--seed 1]
import { writeFileSync, mkdirSync } from 'fs';
import { dirname } from 'path';
import { renderShot, SPEC_BY_ID, defaults, toNorm, fromNorm, encodeWav, SR } from '../dsp/engine.js';
import { readWav, melStack, melLoss, cmaes, pitchTrackRobust, pitchLoss, ltas, ltasLoss } from './lib.js';

export const LAYER_PARAMS = {
  sub: ['sub_tune', 'sub_dive', 'sub_tau', 'sub_k', 'sub_att', 'sub_dec', 'sub_hold', 'sub_rel', 'sub_noise', 'sub_noise_hp', 'sub_drive', 'sub_hard'],
  tr: ['tr_color', 'tr_hp', 'tr_lp', 'tr_pk_f', 'tr_pk_g', 'tr_pk_q', 'tr_click', 'tr_click_f0', 'tr_click_tau', 'tr_att', 'tr_dec', 'tr_hold', 'tr_rel', 'tr_drive'],
  body: ['body_noise', 'body_color', 'body_modes', 'body_mode_f', 'body_metal', 'body_q', 'body_nmodes', 'body_mode_tilt',
    'body_comb', 'body_comb_f', 'body_comb_fb', 'body_comb_damp', 'body_fm', 'body_fm_ratio', 'body_fm_mod', 'body_fm_idx', 'body_fm_tau',
    'body_drop', 'body_drop_tau', 'body_flt_type', 'body_flt_f0', 'body_flt_f1', 'body_flt_tau', 'body_flt_res', 'body_flt_mix',
    'body_hp', 'body_att', 'body_dec', 'body_hold', 'body_rel', 'body_drive', 'body_fold'],
};
const LVL = { sub: 'sub_lvl', tr: 'tr_lvl', body: 'body_lvl' };

export function layerOnly(layer, base) {
  const p = { ...defaults(), ...base, sub_lvl: -60, tr_lvl: -60, body_lvl: -60, spice_lvl: -60, rev_lvl: -60, body_width: 0 };
  p[LVL[layer]] = 0;
  return p;
}

// Fit st(t) = fe + D*exp(-(t/tau)^k) directly to a pitch-track grid (0.5 ms steps). No rendering needed.
export function analyticDive(TP, note) {
  const pts = []; for (let g = 0; g < TP.length; g++) if (Number.isFinite(TP[g])) pts.push([g / 2000, TP[g]]);
  if (pts.length < 6) return {};
  const tailSt = pts.slice(-Math.max(3, pts.length >> 3)).map((p) => p[1]).sort((a, b) => a - b)[0];
  const model = ([fe, D, lt, k]) => pts.reduce((s, [t, st]) => s + Math.abs(fe + D * Math.exp(-Math.pow(t / Math.exp(lt), k)) - st), 0) / pts.length;
  const box = [[tailSt - 6, tailSt + 3], [0, 100], [Math.log(0.0004), Math.log(0.15)], [0.4, 2.5]];
  const toX = (u) => u.map((v, i) => box[i][0] + Math.min(1, Math.max(0, v)) * (box[i][1] - box[i][0]));
  const res = cmaes((u) => model(toX(u)), [0.66, 0.5, 0.5, 0.3], { sigma: 0.3, maxEvals: 3000, seed: 7 });
  const [fe, D, lt, k] = toX(res.x);
  return { sub_tune: fe - (12 * Math.log2(440) + (note - 69)), sub_dive: D, sub_tau: 1000 * Math.exp(lt), sub_k: k };
}

// Staged fit. Each stage optimizes a subset of params (others frozen at the current best),
// with its own loss and budget. Sub: pitch-only on clean renders -> envelope/drive on mel -> joint polish.
const ENV = { sub: ['sub_att', 'sub_dec', 'sub_hold', 'sub_rel'], tr: ['tr_att', 'tr_dec', 'tr_hold', 'tr_rel'], body: ['body_att', 'body_dec', 'body_hold', 'body_rel'] };
// Mechanism priors for the body: each restart starts with one generator dominant.
const BODY_PRIORS = [
  { body_modes: 0, body_comb: -50, body_fm: -50, body_noise: -20 },
  { body_modes: -50, body_comb: 0, body_fm: -50, body_noise: -20 },
  { body_modes: -50, body_comb: -50, body_fm: 0, body_noise: -20 },
  { body_modes: -50, body_comb: -50, body_fm: -50, body_noise: 0 },
];
// Coarse grid over metal freq x metallic x ring with the modal bank dominant; returns the best cell.
function gridModes(cur, pri, layer, nn, f, specs) {
  let best = { f: Infinity };
  for (let i = 0; i < 40; i++) for (const metal of [0, 0.35, 0.65, 0.9]) for (const q of [20, 120]) {
    const cand = { ...pri, body_mode_f: 60 * Math.pow(4000 / 60, i / 39), body_metal: metal, body_q: q };
    const u = specs.map((s) => toNorm(s, s.id in cand ? cand[s.id] : cur[s.id]));
    const v = f(u);
    if (v < best.f) best = { f: v, cand };
  }
  return best.cand;
}
function stagesFor(layer) {
  const all = LAYER_PARAMS[layer];
  if (layer === 'sub') return [
    { name: 'pitch', ids: ['sub_tune', 'sub_dive', 'sub_tau', 'sub_k'], loss: 'pitch', frac: 0.2, sigma: 0.15, override: { sub_att: 0, sub_hold: 1400, sub_dec: 4000, sub_drive: 0, sub_noise: -60 } },
    { name: 'env+drive', ids: [...ENV.sub, 'sub_noise', 'sub_noise_hp', 'sub_drive', 'sub_hard'], loss: 'mel', frac: 0.5, sigma: 0.3, inits: [{}, { sub_noise: -12, sub_drive: 24 }] },
    { name: 'polish', ids: all, loss: 'both', frac: 0.3, sigma: 0.06 },
  ];
  const env = ENV[layer], rest = all.filter((i) => !env.includes(i));
  return [
    { name: 'env', ids: env, loss: 'mel', frac: 0.15, sigma: 0.3, restarts: 2 },
    { name: 'timbre', ids: rest, loss: 'mel', frac: 0.6, sigma: 0.25, inits: layer === 'body' ? BODY_PRIORS : [{}, {}] },
    { name: 'polish', ids: all, loss: 'mel', frac: 0.3, sigma: 0.08 },
  ];
}

export function fitLayer({ layer, target, note, evals = 2500, seed = 1, log = true }) {
  const ids = LAYER_PARAMS[layer];
  const n = Math.min(target.length, Math.floor(0.9 * SR));
  const FMAX = 16000;
  const T = melStack(target.subarray(0, n), SR, FMAX);
  const TP = layer === "sub" ? pitchTrackRobust(target, SR, 300) : null;
  const TL = layer === 'sub' ? null : ltas(target.subarray(0, n), SR);
  const LW = 0.5; // weight of the LTAS term (non-sub layers)
  const lossMel = (x) => melLoss(melStack(x, SR, FMAX), T).loss + (TL ? LW * ltasLoss(ltas(x, SR), TL) : 0);
  const PW = 0.05; // loss units per semitone in the joint polish
  let cur = { ...layerOnly(layer, { note }) };
  if (TP) Object.assign(cur, analyticDive(TP, note));
  const t0 = Date.now(), history = [];
  for (const st of stagesFor(layer)) {
    const specs = st.ids.map((i) => SPEC_BY_ID[i]);
    const nn = st.loss === "pitch" ? Math.floor(0.35 * SR) : n;
    const toParams = (u) => { const p = { ...cur, ...(st.override || {}) }; specs.forEach((s, i) => (p[s.id] = fromNorm(s, u[i]))); return p; };
    const f = (u) => {
      const x = renderShot(toParams(u), nn, SR)[layer][0];
      if (st.loss === 'pitch') return pitchLoss(pitchTrackRobust(x, SR, 300), TP);
      const m = lossMel(x);
      return st.loss === 'both' && TP ? m + PW * pitchLoss(pitchTrackRobust(x, SR, 300), TP) : m;
    };
    const x0 = specs.map((s) => toNorm(s, cur[s.id]));
    const inits = st.inits || Array.from({ length: st.restarts || 1 }, () => ({}));
    const budget = Math.floor((evals * st.frac) / inits.length);
    let best = { x: x0, f: f(x0) };
    inits.forEach((pri0, k) => {
      let pri = pri0;
      if (pri.body_modes === 0) pri = { ...pri, ...gridModes(cur, pri, layer, nn, f, specs) };
      const start = specs.map((s) => toNorm(s, s.id in pri ? pri[s.id] : cur[s.id]));
      const b = cmaes(f, start, { sigma: st.sigma, maxEvals: budget, seed: seed + 101 * k + history.length * 7 });
      if (log && inits.length > 1) console.error(`  [${layer}] ${st.name} init ${k}: ${b.f.toFixed(3)}`);
      if (b.f < best.f) best = b;
    });
    specs.forEach((s, i) => (cur[s.id] = fromNorm(s, best.x[i])));
    history.push({ stage: st.name, f: best.f, secs: (Date.now() - t0) / 1000 });
    if (log) console.error(`[${layer}] stage ${st.name}: ${best.f.toFixed(3)} (${((Date.now() - t0) / 1000).toFixed(0)}s)`);
  }
  const lay = renderShot(cur, n, SR)[layer][0];
  const { loss, gainDb } = melLoss(melStack(lay, SR, FMAX), T);
  const pitchErr = TP ? pitchLoss(pitchTrackRobust(lay, SR, 300), TP) : null;
  const params = Object.fromEntries(ids.map((i) => [i, cur[i]]));
  return { layer, note, loss, pitchErr, gainDb, params, history, render: lay, secs: (Date.now() - t0) / 1000 };
}

// CLI
if (import.meta.url === `file://${process.argv[1]}`) {
  const a = Object.fromEntries(process.argv.slice(2).reduce((acc, v, i, arr) => (v.startsWith('--') ? [...acc, [v.slice(2), arr[i + 1]]] : acc), []));
  const tw = readWav(a.target);
  if (tw.sr !== SR) throw new Error(`target sr ${tw.sr} != ${SR}`);
  const res = fitLayer({ layer: a.layer, target: tw.mono, note: +a.note, evals: +(a.evals || 2500), seed: +(a.seed || 1) });
  const out = a.out || `out/fits/${a.layer}.json`;
  mkdirSync(dirname(out), { recursive: true });
  const { render, ...rest } = res;
  writeFileSync(out, JSON.stringify({ ...rest, target: a.target }, null, 1));
  writeFileSync(out.replace(/\.json$/, '.wav'), Buffer.from(encodeWav(render, render, SR)));
  console.log(JSON.stringify({ out, loss: res.loss, secs: res.secs }));
}
