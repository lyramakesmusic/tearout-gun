// Recover known params: render a target with perturbed params, fit from defaults, compare.
import { renderShot, SPEC_BY_ID, defaults, toNorm } from '../dsp/engine.js';
import { fitLayer, layerOnly, LAYER_PARAMS } from './fit_layer.js';
const layer = process.argv[2] || 'sub', evals = +(process.argv[3] || 1500);
const truth = { sub: { sub_dive: 40, sub_tau: 6, sub_k: 1.4, sub_hold: 250, sub_drive: 28, sub_tune: 7 },
  tr: { tr_hp: 1500, tr_pk_f: 6000, tr_pk_g: 14, tr_hold: 35, tr_drive: 25, tr_color: 0.5 },
  body: { body_mode_f: 900, body_metal: 0.9, body_q: 150, body_hold: 180, body_flt_f1: 3000, body_drive: 25, body_comb: -40 } }[layer];
const p = { ...layerOnly(layer, { note: 28 }), ...truth, seed: 42 };
const n = 43200, target = renderShot(p, n)[layer][0];
const res = fitLayer({ layer, target, note: 28, evals, seed: 3 });
console.log('loss', res.loss.toFixed(3), 'secs', res.secs.toFixed(0));
for (const id of LAYER_PARAMS[layer]) {
  const s = SPEC_BY_ID[id], t = p[id], f = res.params[id];
  const mark = id in truth ? '*' : ' ';
  console.log(mark, id.padEnd(16), 'truth', (+t).toFixed(2).padStart(9), 'fit', (+f).toFixed(2).padStart(9), 'dnorm', (toNorm(s, f) - toNorm(s, t)).toFixed(2));
}
