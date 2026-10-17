// Knob impact for params whose section is switched off in every preset: open the gate, then randomize as in knob_impact.js.
// node fit/knob_impact_gated.js → out/knobs/impact_gated.jsonl
import { readFileSync, appendFileSync, existsSync } from 'fs';
import { SPEC, inEngine, fromNorm, render, rng } from '../dsp/engine.js';
import { features, gunScore } from '../dsp/gunness.js';
import { embed, edist, mono } from './lab.js';
const out = 'out/knobs/impact_gated.jsonl', R = 3;
const ref = JSON.parse(readFileSync('web/gunness.json', 'utf8'));
const gun = SPEC.filter((s) => inEngine(s, 'gun'));
const G = [
  [{ syn_lvl: -12 }, (s) => s.g === 'synth' && s.id !== 'syn_lvl'],
  [{ spice_lvl: -12 }, (s) => s.g === 'spice' && s.id !== 'spice_lvl'],
  [{ cr_mix: 0.6 }, (s) => s.g === 'crunch' && s.id !== 'cr_mix'],
  [{ tr_zap: -12 }, (s) => /^tr_zap_/.test(s.id)],
  [{ tr_stutter: 3 }, (s) => s.id === 'tr_stut_ms'],
  [{ tr_close: 2000 }, (s) => s.id === 'tr_close_ms'],
  [{ sub_click: 24 }, (s) => s.id === 'sub_click_ms'],
  [{ body_vowel_mix: 0.6 }, (s) => s.id === 'body_vowel'],
];
const done = new Set(existsSync(out) ? readFileSync(out, 'utf8').trim().split('\n').filter(Boolean).map((l) => { const j = JSON.parse(l); return j.preset + '|' + j.id; }) : []);
const presets = JSON.parse(readFileSync('web/presets.json', 'utf8')).filter((p) => !p.params.engine).filter((_, i) => i % 2 === 0);
const r = rng(4242);
for (const pr of presets) {
  for (const [gate, pick] of G) {
    const P = { ...pr.params, ...gate, b_shots: 1 }, x0 = mono(render(P)), e0 = embed(x0), g0 = gunScore(features(x0), ref);
    const ds0 = []; for (let k = 0; k < R; k++) ds0.push(edist(e0, embed(mono(render({ ...P, seed: 1 + Math.floor(r() * 9998) })))));
    for (const s of gun.filter(pick)) {
      if (done.has(pr.name + '|' + s.id)) continue;
      const ds = [], gs = [];
      for (let k = 0; k < R; k++) { const x = mono(render({ ...P, [s.id]: fromNorm(s, r()) })); ds.push(edist(e0, embed(x))); gs.push(gunScore(features(x), ref)); }
      appendFileSync(out, JSON.stringify({ preset: pr.name, id: s.id, g: s.g, gate, g0, seed: ds0.map((v) => +v.toFixed(3)), d: ds.map((v) => +v.toFixed(3)), gun: gs.map((v) => +v.toFixed(3)) }) + '\n');
    }
  }
  console.error(pr.name);
}
