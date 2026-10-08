import { readFileSync, readdirSync } from 'fs';
import { render, SR } from '../dsp/engine.js';
import { readWav, melStack, melLoss, ltas, ltasLoss, RES } from './lib.js';
import { thirdOct } from '../web/worker.js';
import { findStems } from './fit_bus.js';
const K = '/Users/lyra/Music/samples/KFU GUN KIT VOL 2.1/KFU GUN KIT VOL 2.1 GUN SORT';
const name = process.argv[2], dir = readdirSync(K).find((d) => d.toLowerCase().includes(name));
const pr = JSON.parse(readFileSync('web/presets.json', 'utf8')).find((p) => dir.toLowerCase().includes(p.name.split(' ')[0]));
const n = Math.floor(0.9 * SR), t = readWav(findStems(`${K}/${dir}`).proc), tm = Float32Array.from(t.ch[0].subarray(0, n), (v, i) => 0.5 * (v + (t.ch[1] || t.ch[0])[i]));
const T = melStack(tm, SR), TL = ltas(tm, SR), tt = thirdOct(tm, SR);
const hi = (tob) => tob.db.filter((_, i) => tob.centers[i] >= 2000 && tob.centers[i] < 8000).reduce((a, b) => a + b, 0) / 6;
for (const up of [0, 6, 12, 18]) {
  const o = render({ ...pr.params, len: 900, tr_lvl: pr.params.tr_lvl + up, body_lvl: pr.params.body_lvl + up });
  const m = Float32Array.from(o.L.subarray(0, n), (v, i) => 0.5 * (v + o.R[i]));
  const mel = melLoss(melStack(m, SR), T).loss, lt = ltasLoss(ltas(m, SR), TL);
  console.log(`tops +${up} dB: mel ${mel.toFixed(3)} ltas ${lt.toFixed(3)} total ${(mel + 0.5 * lt).toFixed(3)} | high band ${hi(thirdOct(m, SR)).toFixed(1)} (target ${hi(tt).toFixed(1)})`);
}
{
  const o = render({ ...pr.params, len: 900, tr_lvl: pr.params.tr_lvl + 18, body_lvl: pr.params.body_lvl + 18 });
  const m = Float32Array.from(o.L.subarray(0, n), (v, i) => 0.5 * (v + o.R[i]));
  const a = thirdOct(m, SR);
  console.log('Hz      ', tt.centers.map((c) => (c < 1000 ? c.toFixed(0) : (c / 1000).toFixed(1) + 'k').padStart(5)).join(''));
  console.log('target  ', tt.db.map((v) => v.toFixed(0).padStart(5)).join(''));
  console.log('ours+18 ', a.db.map((v) => v.toFixed(0).padStart(5)).join(''));
  for (const k of ['tr', 'body']) { const q = { ...pr.params, len: 900, sub_lvl: -60, rev_lvl: -60, tr_lvl: k === 'tr' ? 0 : -60, body_lvl: k === 'body' ? 0 : -60 }; const oo = render(q, { raw: true }); const mm = Float32Array.from(oo.L.subarray(0, n), (v, i) => 0.5 * (v + oo.R[i])); console.log((k + ' alone').padEnd(9), thirdOct(mm, SR).db.map((v) => v.toFixed(0).padStart(5)).join('')); }
  const st = findStems(`${K}/${dir}`);
  for (const k of ['tr', 'body']) { const w = readWav(st[k]).mono.subarray(0, n); console.log(('KFU ' + k).padEnd(9), thirdOct(w, SR).db.map((v) => v.toFixed(0).padStart(5)).join('')); }
}
