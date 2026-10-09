// Training data v2 for the snare inverse model: prior mixed with jitter around library fits, plus a pitch spectrum input.
// usage: node fit/gen_data2.js <outPrefix> <count> <seed>
// writes <prefix>.params.f32, <prefix>.mel.u8 (64×48), <prefix>.pitch.u8 (PBINS log-frequency bins, 70 Hz–2240 Hz, first 10–200 ms)
import { writeFileSync, readFileSync, readdirSync } from 'fs';
import { render, defaults, fromNorm, SR, rng, fft } from '../dsp/engine.js';
import { rollSnareAnchored, rollSnareFlavor } from '../dsp/snare_roll.js';
import { trim, mono } from './lab.js';
import { melOf, vec, KNOBS, FRAMES, BANDS } from './gen_data.js';

export const PBINS = 180, PF0 = 70, PER_OCT = 36;
export function pitchOf(x) {
  const t = trim(x), N = 16384, re = new Float64Array(N), im = new Float64Array(N), a = Math.floor(0.01 * SR), b = Math.min(t.length, Math.floor(0.2 * SR));
  for (let i = a; i < b; i++) re[i - a] = t[i] * (0.5 - 0.5 * Math.cos((2 * Math.PI * (i - a)) / (b - a)));
  fft(re, im, false);
  const out = new Float32Array(PBINS); let mx = -1e9;
  for (let k = 0; k < PBINS; k++) { const f = PF0 * 2 ** (k / PER_OCT), j = (f * N) / SR, j0 = Math.floor(j), w = j - j0; const m = (1 - w) * Math.hypot(re[j0], im[j0]) + w * Math.hypot(re[j0 + 1], im[j0 + 1]); out[k] = 20 * Math.log10(m + 1e-9); mx = Math.max(mx, out[k]); }
  return Uint8Array.from(out, (v) => Math.round(Math.max(0, Math.min(255, ((v - mx + 60) / 60) * 255))));
}

if (process.argv[1].endsWith('gen_data2.js')) {
  const [prefix, count, seed] = process.argv.slice(2), N = +count, r = rng(+seed);
  const anchors = JSON.parse(readFileSync('web/family_fits.json', 'utf8')).snare_eng;
  const lib = readdirSync('out/gp/lib').filter((f) => (process.env.SRC === "fits" ? /^fits_\d+\.jsonl$/ : /^polish_\d+\.jsonl$/).test(f)).flatMap((f) => readFileSync(`out/gp/lib/${f}`, 'utf8').trim().split('\n').map(JSON.parse)).filter((x) => (x.v2 ?? x.loss) < 8).map((x) => x.params);
  console.error(lib.length, 'library fits as anchors');
  const D = KNOBS.length + 1, PV = new Float32Array(N * D), MEL = new Uint8Array(N * FRAMES * BANDS), PIT = new Uint8Array(N * PBINS);
  for (let i = 0; i < N; i++) {
    const u = r(); let P, keepNote = false;
    if (u < 0.3) { P = rollSnareAnchored(defaults(), lib, r, new Set(), 0.04); keepNote = r() < 0.5; }
    else if (u < 0.55) P = rollSnareAnchored(defaults(), lib, r, new Set(), 0.12);
    else if (u < 0.75) P = rollSnareAnchored(defaults(), anchors, r, new Set(), 0.1);
    else if (u < 0.92) P = rollSnareFlavor(defaults(), r);
    else { P = { ...defaults(), engine: 1 }; for (const s of KNOBS) P[s.id] = fromNorm(s, r()); }
    P.engine = 1; P.len = 700; P.seed = 1 + Math.floor(r() * 9999);
    if (!keepNote) { P.note = 24 + Math.floor(r() * 12); if (r() < 0.3) P.sn_t_oct = 2 + Math.floor(r() * 5); }
    const m = mono(render({ ...P, b_shots: 1 }));
    PV.set(vec(P), i * D); MEL.set(melOf(m), i * FRAMES * BANDS); PIT.set(pitchOf(m), i * PBINS);
    if (i % 2000 === 0) console.error(`${prefix} ${i}/${N}`);
  }
  writeFileSync(`${prefix}.params.f32`, Buffer.from(PV.buffer)); writeFileSync(`${prefix}.mel.u8`, Buffer.from(MEL.buffer)); writeFileSync(`${prefix}.pitch.u8`, Buffer.from(PIT.buffer));
  writeFileSync(`${prefix}.meta.json`, JSON.stringify({ N, D, knobs: [...KNOBS.map((s) => ({ id: s.id, scale: s.scale, min: s.min, max: s.max, int: s.scale === 'int' })), { id: 'note_pc', int: true, min: 0, max: 11 }], frames: FRAMES, bands: BANDS, pbins: PBINS }));
}
