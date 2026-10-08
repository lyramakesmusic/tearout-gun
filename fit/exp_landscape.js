// Loss landscape around a known answer: per-knob sensitivity (dead knobs) and 1-D slices (ruggedness).
// usage: node fit/exp_landscape.js [targetIndex]
import { readFileSync } from 'fs';
import { defaults, SPEC_BY_ID, toNorm, fromNorm } from '../dsp/engine.js';
import { target, liveIds } from './lab.js';
const k = +(process.argv[2] || 0), truth = { ...defaults(), ...JSON.parse(readFileSync(`out/recov/t${k}.truth.json`, 'utf8')) };
const t = target(`out/recov/t${k}.wav`), f0 = t.loss(truth), ids = liveIds(truth);
console.log(`target t${k}: loss at truth ${f0.toFixed(3)} (floor from onset trim / length), ${ids.length} live knobs`);
const at = (id, u) => ({ ...truth, [id]: fromNorm(SPEC_BY_ID[id], u) });
const sens = [];
for (const id of ids) {
  const u0 = toNorm(SPEC_BY_ID[id], truth[id]), d = 0.1;
  const up = t.loss(at(id, Math.min(1, u0 + d))), dn = t.loss(at(id, Math.max(0, u0 - d)));
  sens.push([id, Math.max(up, dn) - f0]);
}
sens.sort((a, b) => b[1] - a[1]);
console.log('loss rise for a 10% knob move, most → least sensitive:');
console.log(sens.map(([id, s]) => `${id.replace('sn_', '')} ${s.toFixed(2)}`).join('  '));
const dead = sens.filter(([, s]) => s < 0.05).length;
console.log(`${dead}/${ids.length} knobs move the loss < 0.05 for a 10% move`);
// ruggedness: 1-D slices through the truth along the 6 most sensitive knobs, 21 points; count local minima
for (const [id] of sens.slice(0, 6)) {
  const v = Array.from({ length: 21 }, (_, i) => t.loss(at(id, i / 20)));
  let mins = 0; for (let i = 1; i < 20; i++) if (v[i] < v[i - 1] && v[i] < v[i + 1]) mins++;
  console.log(`${id.replace('sn_', '').padEnd(14)} minima ${mins}  ${v.map((x) => x.toFixed(1)).join(' ')}`);
}
