// Synthetic training data for the snare inverse model: patches near the real-sound prior + their mel spectrograms.
// usage: node fit/gen_data.js <outPrefix> <count> <seed>
// writes <prefix>.params.f32 (N × D normalized knobs), <prefix>.mel.u8 (N × 64 frames × 48 bands, 0..255 = −80..0 dB re peak)
import { writeFileSync, readFileSync } from 'fs';
import { render, defaults, SPEC, toNorm, fromNorm, SR, rng } from '../dsp/engine.js';
import { logMel } from '../dsp/fitcore.js';
import { rollSnareAnchored, rollSnareFlavor } from '../dsp/snare_roll.js';
import { trim, mono } from './lab.js';

export const KNOBS = SPEC.filter((s) => s.eng === 'snare');
export const FRAMES = 64, BANDS = 48, HOP = 480;
export function melOf(x) {
  const t = trim(x), y = new Float32Array(1024 + FRAMES * HOP); y.set(t.subarray(0, y.length));
  const M = logMel(y, SR, 1024, HOP, BANDS, 30, 16000), d = M.data.subarray(0, FRAMES * BANDS); let mx = -1e9; for (const v of d) mx = Math.max(mx, v);
  return Uint8Array.from(d, (v) => Math.round(Math.max(0, Math.min(255, ((v - mx + 80) / 80) * 255))));
}
export const vec = (P) => Float32Array.from([...KNOBS.map((s) => Math.max(0, Math.min(1, toNorm(s, P[s.id])))), (((Math.round(P.note) % 12) + 12) % 12) / 11]);

if (process.argv[1].endsWith('gen_data.js')) {
  const [prefix, count, seed] = process.argv.slice(2), N = +count, r = rng(+seed);
  const anchors = JSON.parse(readFileSync('web/family_fits.json', 'utf8')).snare_eng;
  const D = KNOBS.length + 1, PV = new Float32Array(N * D), MEL = new Uint8Array(N * FRAMES * BANDS);
  for (let i = 0; i < N; i++) {
    const u = r(); let P;
    if (u < 0.45) P = rollSnareAnchored(defaults(), anchors, r, new Set(), 0.08);
    else if (u < 0.7) P = rollSnareAnchored(defaults(), anchors, r, new Set(), 0.18);
    else if (u < 0.9) P = rollSnareFlavor(defaults(), r);
    else { P = { ...defaults(), engine: 1 }; for (const s of KNOBS) P[s.id] = fromNorm(s, r()); }
    P.engine = 1; P.len = 700; P.note = 24 + Math.floor(r() * 12); P.seed = 1 + Math.floor(r() * 9999);
    PV.set(vec(P), i * D); MEL.set(melOf(mono(render({ ...P, b_shots: 1 }))), i * FRAMES * BANDS);
    if (i % 2000 === 0) console.error(`${prefix} ${i}/${N}`);
  }
  writeFileSync(`${prefix}.params.f32`, Buffer.from(PV.buffer)); writeFileSync(`${prefix}.mel.u8`, Buffer.from(MEL.buffer));
  writeFileSync(`${prefix}.meta.json`, JSON.stringify({ N, D, knobs: [...KNOBS.map((s) => ({ id: s.id, scale: s.scale, min: s.min, max: s.max, int: s.scale === 'int' })), { id: 'note_pc', int: true, min: 0, max: 11 }], frames: FRAMES, bands: BANDS }));
}
