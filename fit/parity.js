// JS vs Rust gun-engine parity: same params in, compare waveforms sample by sample.
// node fit/parity.js → out/parity/{jobs.jsonl, js/<i>.f32}; then run tg-cli on jobs.jsonl and `node fit/parity.js cmp`.
import { readFileSync, writeFileSync, mkdirSync, existsSync } from 'fs';
import { SPEC, inEngine, defaults, fromNorm, render, setSample, rng } from '../dsp/engine.js';
const D = 'out/parity';
const f32 = (p) => { const b = readFileSync(p); return new Float32Array(b.buffer, b.byteOffset, b.length / 4); };
if (process.argv[2] === 'cmp') {
  const jobs = readFileSync(`${D}/jobs.jsonl`, 'utf8').trim().split('\n').map(JSON.parse), rows = [];
  jobs.forEach((j, i) => {
    const a = f32(`${D}/js/${i}.f32`), b = f32(`${D}/rs/${i}.f32`);
    if (a.length !== b.length) { rows.push({ i, name: j.name, len: `${a.length} vs ${b.length}` }); return; }
    let mx = 0, se = 0, ss = 0, exact = 0;
    for (let k = 0; k < a.length; k++) { const d = Math.abs(a[k] - b[k]); mx = Math.max(mx, d); se += d * d; ss += a[k] * a[k]; if (d === 0) exact++; }
    rows.push({ i, name: j.name, max: mx, errDb: se ? 10 * Math.log10(se / ss) : -Infinity, exact: exact / a.length });
  });
  const bad = rows.filter((r) => r.len || r.errDb > -100);
  rows.sort((x, y) => (y.errDb ?? 99) - (x.errDb ?? 99));
  console.log('worst 12:'); for (const r of rows.slice(0, 12)) console.log(r.i, r.name.padEnd(28), r.len || `max ${r.max.toExponential(2)}  err ${r.errDb.toFixed(1)} dB  bit-exact ${(100 * r.exact).toFixed(1)}%`);
  const ex = rows.filter((r) => r.exact === 1).length;
  console.log(`\n${rows.length} jobs, ${ex} bit-identical, ${bad.length} above -100 dB error`);
  process.exit(0);
}
mkdirSync(`${D}/js`, { recursive: true });
const G = SPEC.filter((s) => inEngine(s, 'gun') && s.id !== 'engine');
const jobs = [];
for (const pr of JSON.parse(readFileSync('web/presets.json', 'utf8'))) if (!pr.params.engine) jobs.push({ name: pr.name, params: pr.params });
jobs.push({ name: 'defaults', params: {} });
const r = rng(4242);
for (let k = 0; k < 80; k++) {
  const P = {};
  for (const s of G) if (!/^s\d_/.test(s.id)) P[s.id] = fromNorm(s, r());
  P.b_shots = k % 4 === 0 ? 1 + Math.floor(r() * 6) : 1; // some bursts
  if (k % 3 === 0) for (const g of ['sub', 'tr', 'body', 'syn', 'spice', 'rev']) if (r() < 0.4) P[`${g}_lvl`] = -60;
  jobs.push({ name: `random ${k}${P.b_shots > 1 ? ` x${P.b_shots}` : ''}`, params: P });
}
// dropped samples: a synthetic stereo 44.1k hit, both routes, keyed and fixed
const n = 22050, L = new Float32Array(n), R = new Float32Array(n), sr0 = rng(7);
for (let i = 0; i < n; i++) { const e = Math.exp(-i / 4000); L[i] = e * (Math.sin(i * 0.05) + 0.3 * (sr0() * 2 - 1)); R[i] = e * (Math.sin(i * 0.051) + 0.3 * (sr0() * 2 - 1)); }
writeFileSync(`${D}/smpL.f32`, Buffer.from(L.buffer)); writeFileSync(`${D}/smpR.f32`, Buffer.from(R.buffer));
const smp = { l: `${D}/smpL.f32`, r: `${D}/smpR.f32`, rate: 44100 };
for (let k = 0; k < 8; k++) {
  const P = { ...jobs[k].params }, s = 1 + (k % 3);
  Object.assign(P, { [`s${s}_lvl`]: -6, [`s${s}_pitch`]: k - 4, [`s${s}_key`]: k % 2, [`s${s}_start`]: 10 * k, [`s${s}_len`]: 300, [`s${s}_rel`]: 20, [`s${s}_hp`]: k % 2 ? 150 : 20, [`s${s}_lp`]: k % 3 ? 20000 : 5000, [`s${s}_drive`]: k * 3, [`s${s}_route`]: k % 2 });
  if (k === 7) P.sub_lvl = -60;
  jobs.push({ name: `sample s${s} on ${jobs[k].name}`, params: P, samples: { [s]: smp } });
}
writeFileSync(`${D}/jobs.jsonl`, jobs.map((j) => JSON.stringify(j)).join('\n') + '\n');
jobs.forEach((j, i) => {
  for (const k of [1, 2, 3]) setSample(k, null);
  if (j.samples) for (const [k, s] of Object.entries(j.samples)) setSample(+k, f32(s.l), f32(s.r), s.rate);
  const t = performance.now(), o = render(j.params), ms = performance.now() - t;
  const out = new Float32Array(o.L.length * 2); out.set(o.L); out.set(o.R, o.L.length);
  writeFileSync(`${D}/js/${i}.f32`, Buffer.from(out.buffer));
  j.ms = ms;
});
writeFileSync(`${D}/js_ms.json`, JSON.stringify(jobs.map((j) => j.ms)));
console.log(jobs.length, 'jobs rendered in JS');
