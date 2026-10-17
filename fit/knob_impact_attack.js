// Attack-window knob impact: fine-hop log-mel over the first 64 ms after onset, for transient + attack-shaping params.
// node fit/knob_impact_attack.js → out/knobs/impact_attack.jsonl
import { readFileSync, appendFileSync } from 'fs';
import { SPEC, inEngine, fromNorm, render, rng } from '../dsp/engine.js';
import { edist, mono } from './lab.js';
import { logMel } from '../dsp/fitcore.js';
const SR = 48000, out = 'out/knobs/impact_attack.jsonl', R = 3;
function aembed(x) {
  let pk = 0; for (const v of x) pk = Math.max(pk, Math.abs(v)); let on = 0; while (on < x.length && Math.abs(x[on]) < pk * 0.05) on++;
  const y = new Float32Array(512 + 48 * 64); y.set(x.subarray(Math.max(0, on - 48), Math.max(0, on - 48) + y.length));
  const M = logMel(y, SR, 512, 64, 40, 60, 16000), d = M.data.subarray(0, 48 * 40); let mx = -1e9; for (const v of d) mx = Math.max(mx, v);
  return Float32Array.from(d, (v) => Math.max(v - mx, -60));
}
const ids = new Set(['seed', 'note', 'sub_lvl', 'body_lvl', 'body_att', 'sub_att', 'mst_snap', 'mst_ott', 'mst_drive', 'mst_hard', 'body_drop', 'body_drop_tau', 'sub_click', 'sub_dive', 'sub_tau']);
const params = SPEC.filter((s) => inEngine(s, 'gun') && (s.g === 'tr' || ids.has(s.id)));
const presets = JSON.parse(readFileSync('web/presets.json', 'utf8')).filter((p) => !p.params.engine).filter((_, i) => i % 3 === 1);
const r = rng(777);
for (const pr of presets) {
  const P = { ...pr.params, b_shots: 1 }, e0 = aembed(mono(render(P)));
  for (const s of params) {
    const ds = [];
    for (let k = 0; k < R; k++) { const v = s.id === 'seed' ? 1 + Math.floor(r() * 9998) : fromNorm(s, r()); ds.push(edist(e0, aembed(mono(render({ ...P, [s.id]: v }))))); }
    appendFileSync(out, JSON.stringify({ preset: pr.name, id: s.id, g: s.g, d: ds.map((v) => +v.toFixed(3)) }) + '\n');
  }
  console.error(pr.name);
}
