// Which gun knobs matter? One-at-a-time randomization over the shipped presets.
// For each preset and param: R uniform draws over the param's range; record how far the sound moves (edist of
// log-mel fingerprints) and the gun score of the result. "seed" is the control: re-rolling noise alone.
// node fit/knob_impact.js → out/knobs/impact.jsonl (one line per preset × param)
import { readFileSync, appendFileSync, existsSync } from 'fs';
import { SPEC, inEngine, fromNorm, render, rng } from '../dsp/engine.js';
import { features, gunScore } from '../dsp/gunness.js';
import { embed, edist, mono } from './lab.js';
const out = 'out/knobs/impact.jsonl', R = +(process.env.R || 3);
const ref = JSON.parse(readFileSync('web/gunness.json', 'utf8'));
const done = new Set(existsSync(out) ? readFileSync(out, 'utf8').trim().split('\n').filter(Boolean).map((l) => { const j = JSON.parse(l); return j.preset + '|' + j.id; }) : []);
const params = SPEC.filter((s) => inEngine(s, 'gun') && s.id !== 'engine' && !/^s\d_/.test(s.id) && !/^b_/.test(s.id) && s.id !== 'len');
const presets = JSON.parse(readFileSync('web/presets.json', 'utf8')).filter((p) => !p.params.engine);
const r = rng(31337);
for (const pr of presets) {
  const P = { ...pr.params, b_shots: 1 }, x0 = mono(render(P)), e0 = embed(x0), g0 = gunScore(features(x0), ref);
  for (const s of params) {
    if (done.has(pr.name + '|' + s.id)) continue;
    const ds = [], gs = [];
    for (let k = 0; k < R; k++) {
      const v = s.id === 'seed' ? 1 + Math.floor(r() * 9998) : fromNorm(s, r());
      const x = mono(render({ ...P, [s.id]: v }));
      ds.push(edist(e0, embed(x))); gs.push(gunScore(features(x), ref));
    }
    appendFileSync(out, JSON.stringify({ preset: pr.name, id: s.id, g: s.g, g0, d: ds.map((v) => +v.toFixed(3)), gun: gs.map((v) => +v.toFixed(3)) }) + '\n');
  }
  console.error(pr.name);
}
