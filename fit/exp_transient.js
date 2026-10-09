// Does a transient-envelope loss term fix the soft fronts? Warm-start from library fits with the worst snap error,
// refine 300 evals under LOSS (v2 control: same budget, no term), then measure exact descriptors and the v2 loss.
import { readFileSync, readdirSync, writeFileSync, existsSync } from 'fs';
import { render, defaults } from "../dsp/engine.js";
import { readWav } from './lib.js';
import { target, refineCD, mono, tenv, tenvLoss, trim } from './lab.js';
import { describe } from './descriptors.js';
const to48 = (x, sr) => { if (sr === 48000) return x; const q = sr / 48000, n = Math.floor(x.length / q), y = new Float32Array(n); for (let i = 0; i < n; i++) { const t = i * q, j = Math.floor(t); y[i] = x[j] + (t - j) * ((x[j + 1] ?? x[j]) - x[j]); } return y; };
const SEL = 'out/gp/transient_sel.json';
if (!existsSync(SEL)) {
  const lib = readdirSync('out/gp/lib').filter((f) => /^fits_\d+\.jsonl$/.test(f)).flatMap((f) => readFileSync(`out/gp/lib/${f}`, 'utf8').trim().split('\n').map(JSON.parse)).filter((x) => x.loss < 7);
  const S = lib.filter((_, i) => i % 8 === 4).map((x) => { const w = readWav(x.file); return { x, e: describe(mono(render({ ...x.params, len: 800, b_shots: 1 }))).snap - describe(to48(w.mono, w.sr)).snap }; });
  writeFileSync(SEL, JSON.stringify(S.sort((a, b) => a.e - b.e).slice(0, 30).map((s) => s.x)));
}
if (process.env.ARM === "sel") process.exit(0);
const sel = JSON.parse(readFileSync(SEL, 'utf8')), arm = process.env.ARM, out = [];
for (const x of sel) {
  const t = target(x.file), off = { ...defaults(), ...x.params, sn_h_lvl: -60 }, on = { ...off, sn_h_lvl: 0, sn_h_len: 3, sn_h_drive: 12, sn_h_color: +(process.env.HC ?? 0.3) };
  const start = process.env.HIT ? (t.loss(on) < t.loss(off) ? on : off) : off, r = refineCD(t, start, 300), v2 = t.lossOf.v2(r.P);
  const m = mono(render({ ...r.P, len: 800, b_shots: 1 })), w = readWav(x.file), real = to48(w.mono, w.sr);
  out.push({ file: x.file, v2, v2_before: x.loss, tenv: tenvLoss(tenv(trim(m)), tenv(trim(real))), F: describe(m), T: describe(real), P: r.P });
}
writeFileSync(`out/gp/transient_${arm}.json`, JSON.stringify(out)); console.log(arm, 'done');
