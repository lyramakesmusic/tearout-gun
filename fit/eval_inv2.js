// Inverse model v1 vs v2 as fit starts on the held-out eval set, refined by coordinate descent under the v6 loss.
// Judged by spectral loss (v2), front loss (two-band envelope) and exact descriptors.
// prep:  node fit/eval_inv2.js prep   → out/gp/eval/eval.pitch.u8
// run:   SHARD=k/N LOSS=v6 TW=0.3 node fit/eval_inv2.js <budget> <v2pred.json>
import { readFileSync, writeFileSync } from 'fs';
import { render, defaults, fromNorm, SPEC_BY_ID, SR } from '../dsp/engine.js';
import { readWav } from './lib.js';
import { target, targetFrom, mono, refineCD, trim, tenv2, tenv2Loss } from './lab.js';
import { shellPitch } from '../dsp/fitstages.js';
import { pitchOf, PBINS } from './gen_data2.js';
import { describe } from './descriptors.js';
const to48 = (x, sr) => { if (sr === SR) return x; const q = sr / SR, n = Math.floor(x.length / q), y = new Float32Array(n); for (let i = 0; i < n; i++) { const t = i * q, j = Math.floor(t); y[i] = x[j] + (t - j) * ((x[j + 1] ?? x[j]) - x[j]); } return y; };
const synth = JSON.parse(readFileSync('out/gp/eval/synthetic.json', 'utf8')), real = JSON.parse(readFileSync('out/gp/eval/real.json', 'utf8')), all = [...synth.map((x) => ({ ...x, kind: 'synthetic' })), ...real.map((x) => ({ ...x, kind: 'real' }))];
const mode = process.argv[2], [budget, v2path] = mode === 'prep' ? [] : process.argv.slice(2);
if (mode === 'prep') {
  const H = new Uint8Array(all.length * PBINS); all.forEach((x, i) => { const w = readWav(x.wav); H.set(pitchOf(to48(w.mono, w.sr)), i * PBINS); });
  writeFileSync('out/gp/eval/eval.pitch.u8', Buffer.from(H.buffer)); console.log(all.length, 'pitch rows'); process.exit(0);
}
const v1meta = JSON.parse(readFileSync('out/gp/c0.meta.json', 'utf8')).knobs.map((k) => k.id), V1 = JSON.parse(readFileSync('out/gp/eval/pred.json', 'utf8')), V2 = JSON.parse(readFileSync(v2path, 'utf8'));
const fromVec = (v, ids) => { const P = { ...defaults(), engine: 1, len: 700, seed: 1 }; ids.forEach((id, i) => { if (id === 'note_pc') P.note = 24 + Math.round(v[i] * 11); else P[id] = fromNorm(SPEC_BY_ID[id], v[i]); }); return P; };
const B = +(budget || 800), [sk, sn] = (process.env.SHARD || '0/1').split('/').map(Number), rows = [];
all.forEach((x, i) => {
  if (i % sn !== sk) return;
  const t = target(x.wav), jx = x.truth ? mono(render({ ...x.truth, seed: 1 })) : to48(readWav(x.wav).mono, readWav(x.wav).sr), judge = x.truth ? targetFrom(jx) : t, TE = tenv2(trim(jx)), TD = describe(jx);
  const midi = Math.round(69 + 12 * Math.log2(shellPitch(t.T0) / 440)), pitch = (P) => ({ ...P, note: 24 + (((midi % 12) + 12) % 12), sn_t_oct: Math.max(2, Math.min(6, Math.floor((midi - 4) / 12) - 1)) });
  const a = pitch(fromVec(V1[i], v1meta)), b = fromVec(V2.pred[i], V2.knobs), c = pitch(b);
  const runs = { 'v1+pitch only': () => a, 'v2 only': () => b, 'v2+pitch only': () => c, 'v1+pitch + CD': () => refineCD(t, a, B).P, 'v2 + CD': () => refineCD(t, b, B).P, 'v2+pitch + CD': () => refineCD(t, c, B).P };
  for (const [name, fn] of Object.entries(runs)) {
    const P = { ...fn(), seed: 1 }, m = mono(render({ ...P, len: 800, b_shots: 1 })), D = describe(m);
    rows.push({ kind: x.kind, name, spec: judge.lossOf.v2(P), front: tenv2Loss(tenv2(trim(m)), TE), snap: Math.abs(D.snap - TD.snap), attack: Math.abs(D.attack_ms - TD.attack_ms), hf: D.click_hf - TD.click_hf, pitch: Math.abs(1200 * Math.log2(D.pitch_hz / TD.pitch_hz)) });
    console.error(`${x.kind} ${i} ${name}`);
  }
});
writeFileSync(`out/gp/eval/rows2_${sk}.json`, JSON.stringify(rows));
