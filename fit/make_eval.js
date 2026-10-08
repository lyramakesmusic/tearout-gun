// Evaluation set for inverse-model experiments: synthetic targets with known knobs + unseen real snares.
// writes out/gp/eval/{synthetic,real}.json (list of {wav, truth?}) and eval.mel.u8 (mel of each target, same recipe as training)
import { readFileSync, writeFileSync, mkdirSync } from 'fs';
import { render, defaults, encodeWav, fromNorm, rng, SR } from '../dsp/engine.js';
import { rollSnareAnchored, rollSnareFlavor } from '../dsp/snare_roll.js';
import { readWav } from './lib.js';
import { melOf, KNOBS, FRAMES, BANDS } from './gen_data.js';
import { mono } from './lab.js';
const D = 'out/gp/eval'; mkdirSync(D, { recursive: true });
const r = rng(424242), anchors = JSON.parse(readFileSync('web/family_fits.json', 'utf8')).snare_eng;
const synth = [];
for (let i = 0; i < 40; i++) {
  const u = r(); let P = u < 0.5 ? rollSnareAnchored(defaults(), anchors, r, new Set(), 0.12) : rollSnareFlavor(defaults(), r);
  P = { ...P, engine: 1, len: 700, note: 24 + Math.floor(r() * 12), seed: 1 + Math.floor(r() * 9999) };
  const o = render(P), wav = `${D}/syn${i}.wav`; writeFileSync(wav, Buffer.from(encodeWav(o.L, o.R, o.sr))); synth.push({ wav, truth: P });
}
// real: corpus snares never fitted (not in the 212), roll/loop names excluded, spread across the list
const fitted = new Set(readFileSync('out/sfits/jobs.tsv', 'utf8').trim().split('\n').map((l) => l.split('\t')[0]));
const C = JSON.parse(readFileSync('analysis/snares/corpus.json', 'utf8')).filter((c) => !fitted.has(c.path) && !/roll|buzz|fill|loop|reverse/i.test(c.path));
const real = C.filter((_, i) => i % Math.floor(C.length / 30) === 5).slice(0, 30).map((c) => ({ wav: c.path }));
const to48 = (x, sr) => { if (sr === SR) return x; const q = sr / SR, n = Math.floor(x.length / q), y = new Float32Array(n); for (let i = 0; i < n; i++) { const t = i * q, j = Math.floor(t); y[i] = x[j] + (t - j) * ((x[j + 1] ?? x[j]) - x[j]); } return y; };
const all = [...synth, ...real], MEL = new Uint8Array(all.length * FRAMES * BANDS);
all.forEach((t, i) => { const w = readWav(t.wav); MEL.set(melOf(to48(w.mono, w.sr)), i * FRAMES * BANDS); });
writeFileSync(`${D}/synthetic.json`, JSON.stringify(synth)); writeFileSync(`${D}/real.json`, JSON.stringify(real)); writeFileSync(`${D}/eval.mel.u8`, Buffer.from(MEL.buffer));
console.log(synth.length, 'synthetic,', real.length, 'real');
