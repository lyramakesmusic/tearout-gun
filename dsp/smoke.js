import { render, encodeWav, defaults } from './engine.js';
import { writeFileSync } from 'fs';
const cases = {
  default: {},
  burst: { b_shots: 8, b_rate: 70, b_accel: 0.4, b_drift: -0.5, b_jitter: 0.5 },
  laser: { body_drop: 30, body_drop_tau: 60, body_fm: -6, rev_type: 2 },
  metal: { body_metal: 1, body_q: 200, rev_type: 1, spice_lvl: -10, spice_type: 3 },
};
for (const [name, p] of Object.entries(cases)) {
  const t0 = performance.now();
  const out = render(p);
  const ms = performance.now() - t0;
  let pk = 0, ss = 0, nan = 0;
  for (const x of [out.L, out.R]) for (const v of x) { if (!Number.isFinite(v)) nan++; pk = Math.max(pk, Math.abs(v)); ss += v * v; }
  const rms = Math.sqrt(ss / (2 * out.L.length));
  const lay = Object.entries(out.layers).map(([k, v]) => v ? `${k}:${(20*Math.log10(Math.sqrt(v[0].reduce((a,b)=>a+b*b,0)/v[0].length)+1e-9)).toFixed(1)}` : `${k}:off`).join(' ');
  console.log(`${name.padEnd(8)} ${ms.toFixed(0)}ms len=${(out.L.length/out.sr).toFixed(2)}s peak=${(20*Math.log10(pk)).toFixed(1)}dB rms=${(20*Math.log10(rms)).toFixed(1)}dB nan=${nan} | ${lay}`);
  writeFileSync(`out/${name}.wav`, Buffer.from(encodeWav(out.L, out.R, out.sr)));
}
