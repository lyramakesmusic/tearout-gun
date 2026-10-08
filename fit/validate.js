// Kill test: does each assembled preset match its own gun's PROCESSED better than other guns' (and than defaults)?
import { readFileSync, readdirSync } from 'fs';
import { render, defaults, SR } from '../dsp/engine.js';
import { readWav, melStack, melLoss, ltas, ltasLoss } from './lib.js';
import { findStems } from './fit_bus.js';
const K = '/Users/lyra/Music/samples/KFU GUN KIT VOL 2.1/KFU GUN KIT VOL 2.1 GUN SORT';
const presets = JSON.parse(readFileSync('web/presets.json', 'utf8'));
const dirs = readdirSync(K).filter((d) => d.startsWith('KFU_GUN_'));
const MAP = JSON.parse(readFileSync('out/preset_map.json', 'utf8'));
const items = presets.map((p) => ({ ...p, dir: dirs.find((d) => d === MAP[p.name]) })).filter((x) => x.dir);
const n = Math.floor(0.9 * SR);
const tgt = items.map((it) => { const m = readWav(findStems(`${K}/${it.dir}`).proc).mono.subarray(0, n); return { M: melStack(m, SR), L: ltas(m, SR) }; });
const score = (x, t) => melLoss(melStack(x, SR), t.M).loss + 0.5 * ltasLoss(ltas(x, SR), t.L);
const mono = (o) => Float32Array.from(o.L.subarray(0, n), (v, i) => 0.5 * (v + o.R[i]));
const rend = items.map((it) => mono(render({ ...it.params, len: 900 })));
const def = mono(render({ ...defaults(), len: 900 }));
let own = [], other = [], dflt = [], rank = [];
items.forEach((it, i) => {
  const s = items.map((_, j) => score(rend[j], tgt[i]));   // all presets scored against gun i's target
  own.push(s[i]); other.push(s.filter((_, j) => j !== i).reduce((a, b) => a + b, 0) / (s.length - 1)); dflt.push(score(def, tgt[i]));
  rank.push(1 + s.filter((v) => v < s[i]).length);
});
const med = (a) => [...a].sort((x, y) => x - y)[a.length >> 1];
console.log(`N=${items.length}  median loss: own preset ${med(own).toFixed(3)} | other guns' presets ${med(other).toFixed(3)} | default engine ${med(dflt).toFixed(3)}`);
console.log(`own preset rank among ${items.length} (1 = best match): median ${med(rank)}, rank-1 count ${rank.filter((r) => r === 1).length}`);
console.log(items.map((it, i) => `${it.name}:${rank[i]}`).join('  '));
