// Sample candidate patches, keep the gun-like ones. usage: node fit/gen_manifold.js <seed> <count> -> out/vae/cand_<seed>.jsonl
import { readFileSync, appendFileSync, writeFileSync } from 'fs';
import { render, rng, defaults, SPEC, toNorm } from '../dsp/engine.js';
import { rollArchetype } from '../dsp/archetypes.js';
import { randomize } from '../dsp/random.js';
import { features, gunScore } from '../dsp/gunness.js';
export { VAE_IDS } from '../dsp/vae.js';
import { VAE_IDS } from '../dsp/vae.js';
if (import.meta.url === `file://${process.argv[1]}`) {
  const seed = +process.argv[2], count = +process.argv[3], r = rng(seed * 7919 + 1);
  const priors = JSON.parse(readFileSync('web/priors.json', 'utf8')), ref = JSON.parse(readFileSync('web/gunness.json', 'utf8'));
  const modes = [['fitted', 0.4], ['gun', 0.35], ['snare', 0.1], ['ping', 0.1], ['chug', 0.05]];
  const out = `out/vae/cand_${seed}.jsonl`;
  writeFileSync('out/vae/ids.json', JSON.stringify(VAE_IDS));
  for (let i = 0; i < count; i++) {
    let u = r(), mode = modes[0][0]; for (const [m, w] of modes) { if (u < w) { mode = m; break; } u -= w; }
    const Q = mode === 'fitted' ? randomize(defaults(), priors, new Set(), r) : rollArchetype(mode, r);
    const o = render({ ...Q, b_shots: 1 });
    const f = features(Float32Array.from(o.L, (v, j) => 0.5 * (v + o.R[j])));
    const score = gunScore(f, ref);
    appendFileSync(out, JSON.stringify({ mode, score, f, x: VAE_IDS.map((id) => +toNorm(SPEC.find((s) => s.id === id), Q[id]).toFixed(5)) }) + '\n');
  }
}
