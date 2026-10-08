// Fit the snare engine to one snare one-shot: staged CMA-ES with grids over the switch params.
// loss = multi-res mel + LTAS + third-octave balance.
// usage: node fit/fit_snare.js <wav> <out.json> [evalScale]
import { writeFileSync } from 'fs';
import { render, SPEC, SPEC_BY_ID, toNorm, fromNorm, defaults, encodeWav, SR, fft } from '../dsp/engine.js';
import { readWav, melStack, melLoss, cmaes, ltas, ltasLoss, thirdOct, tobLoss } from './lib.js';

const [file, out, scaleArg] = process.argv.slice(2);
const ES = +(scaleArg || 1);
const w = readWav(file);
const to48 = (x, sr) => { if (sr === SR) return x; const r = sr / SR, n = Math.floor(x.length / r), y = new Float32Array(n); for (let i = 0; i < n; i++) { const t = i * r, j = Math.floor(t); y[i] = x[j] + (t - j) * ((x[j + 1] ?? x[j]) - x[j]); } return y; };
let tgt = to48(w.mono, w.sr);
// trim leading silence to the onset
{ let pk = 0; for (const v of tgt) pk = Math.max(pk, Math.abs(v)); let i = 0; while (i < tgt.length && Math.abs(tgt[i]) < pk * 0.03) i++; tgt = tgt.subarray(Math.max(0, i - 24)); }
// length: through -45 dB, 200..800 ms
const lenMs = (() => { const h = 96; let pk = -1e9; const e = []; for (let i = 0; i + h <= tgt.length; i += h) { let s = 0; for (let j = 0; j < h; j++) s += tgt[i + j] ** 2; e.push(10 * Math.log10(s / h + 1e-12)); pk = Math.max(pk, e.at(-1)); } let last = 0; e.forEach((v, i) => { if (v > pk - 45) last = i; }); return Math.min(800, Math.max(200, Math.round(((last + 1) * h * 1000) / SR) + 30)); })();
const n = Math.floor((lenMs * SR) / 1000);
const T0 = new Float32Array(n); T0.set(tgt.subarray(0, n));
const T = melStack(T0, SR), TL = ltas(T0, SR), TT = thirdOct(T0, SR);
let evals = 0;
const loss = (P) => { evals++; const o = render({ ...P, len: lenMs }); const m = Float32Array.from(o.L.subarray(0, n), (v, i) => 0.5 * (v + o.R[i])); return melLoss(melStack(m, SR), T).loss + 0.5 * ltasLoss(ltas(m, SR), TL) + 0.3 * tobLoss(m, TT, SR); };

// shell pitch: strongest peak 100–1000 Hz in the first 10–40 ms
function shellPitch(x) {
  const N = 8192, re = new Float64Array(N), im = new Float64Array(N), a = Math.floor(0.01 * SR), b = Math.min(x.length, Math.floor(0.04 * SR));
  for (let i = a; i < b; i++) re[i - a] = x[i] * (0.5 - 0.5 * Math.cos((2 * Math.PI * (i - a)) / (b - a)));
  fft(re, im, false);
  let best = 0, bk = 0; for (let k = Math.floor((100 * N) / SR); k < (1000 * N) / SR; k++) { const m = re[k] ** 2 + im[k] ** 2; if (m > best) { best = m; bk = k; } }
  return (bk * SR) / N;
}
const f0 = shellPitch(T0), midi = Math.round(69 + 12 * Math.log2(f0 / 440));
let cur = { ...defaults(), engine: 1, len: lenMs, seed: 1, note: 24 + (((midi % 12) + 12) % 12), sn_t_oct: Math.max(2, Math.min(5, Math.floor((midi - 4) / 12) - 1)), sn_t_tune: 0, sn_m_lvl: -60, sn_k_lvl: -60, sn_p_lvl: -60 };
const ids = (re, ex = []) => SPEC.filter((s) => s.eng === 'snare' && re.test(s.id) && !ex.includes(s.id) && s.scale !== 'int').map((s) => s.id);
let base = loss(cur);
const t0 = Date.now();
console.error(`${file.split('/').pop()} ${lenMs}ms f0 ${f0.toFixed(0)} start ${base.toFixed(3)}`);
function stage(name, idList, ev, from = cur, seed = 3) {
  const specs = idList.map((id) => SPEC_BY_ID[id]);
  const P = (u) => { const q = { ...from }; specs.forEach((s, i) => (q[s.id] = fromNorm(s, u[i]))); return q; };
  const b = cmaes((u) => loss(P(u)), specs.map((s) => toNorm(s, from[s.id])), { sigma: 0.2, maxEvals: Math.round(ev * ES), seed });
  return { P: P(b.x), f: b.f };
}
function accept(name, r) { if (r.f < base) { cur = r.P; base = r.f; } console.error(`  ${name}: ${base.toFixed(3)} (${((Date.now() - t0) / 1000).toFixed(0)}s, ${evals} evals)`); }
// try each value of a switch param with a short run, keep the best, then refine
function grid(name, id, values, idList, evEach, evRefine, extra = {}) {
  let best = { P: cur, f: base };
  for (const v of values) { const r = stage(`${name}=${v}`, idList, evEach, { ...cur, ...extra, [id]: v }); if (r.f < best.f) best = r; }
  accept(`${name} ${best.P[id]}`, best);
  if (evRefine) accept(`${name} refine`, stage(name, idList, evRefine));
}
const LVL = ['sn_t_lvl', 'sn_n_lvl', 'sn_c_lvl', 'sn_r_lvl', 'sn_b_drive', 'sn_b_snap', 'sn_b_high', 'sn_b_low', 'sn_b_hp'];
accept('levels', stage('levels', LVL, 500));
grid('octave', 'sn_t_oct', [2, 3, 4, 5].filter((o) => Math.abs(o - cur.sn_t_oct) <= 1), ['sn_t_lvl', 'sn_t_tune', 'sn_t_pitch', 'sn_t_pitch_ms', 'sn_t_dec'], 150, 0);
grid('wave', 'sn_t_wave', [0, 1, 2, 3], ids(/^sn_t_/), 220, 700);
grid('noise', 'sn_n_type', [0, 1, 2, 4, 5, 6], ids(/^sn_n_/, ['sn_n_width']), 180, 900);
grid('click', 'sn_c_type', [0, 1, 2, 3, 4], ids(/^sn_c_/), 120, 300);
{ // metal: on (each type) vs off
  const off = { P: cur, f: base }; let best = off;
  for (const t of [0, 1, 2, 4]) { const r = stage(`metal ${t}`, ids(/^sn_m_/, ['sn_m_width']), 260, { ...cur, sn_m_type: t, sn_m_lvl: -10 }); if (r.f < best.f * 0.985) best = r; }
  accept(`metal ${best === off ? 'off' : best.P.sn_m_type}`, best);
}
{ // clap: on vs off
  const r = stage('clap', ids(/^sn_k_/), 350, { ...cur, sn_k_lvl: -8 });
  accept(`clap ${r.f < base * 0.985 ? 'on' : 'off'}`, r.f < base * 0.985 ? r : { P: cur, f: base });
}
accept('room+bus', stage('room+bus', ids(/^(sn_r_|sn_b_)/, ['sn_b_width', 'sn_b_time']), 700));
const ACT = [...LVL, ...['sn_t_delay', 'sn_n_delay', 'sn_c_delay', 'sn_n_att'], ...(cur.sn_m_lvl > -59 ? ['sn_m_lvl', 'sn_m_delay'] : []), ...(cur.sn_k_lvl > -59 ? ['sn_k_lvl', 'sn_k_delay'] : [])];
accept('levels+delays', stage('levels+delays', ACT, 600));
accept('tone+noise', stage('tone+noise', [...ids(/^sn_t_/), ...ids(/^sn_n_/, ['sn_n_width'])], 900));
accept('body timing', stage('body timing', ['sn_t_lvl', 'sn_t_hold', 'sn_t_dec', 'sn_t_curve', 'sn_t_pitch', 'sn_t_pitch_ms', 'sn_t_drive', 'sn_t_drive_env', 'sn_b_ott', 'sn_b_drive', 'sn_r_lvl', 'sn_r_size', 'sn_n_lvl', 'sn_n_dec'], 500));
writeFileSync(out, JSON.stringify({ file, loss: base, lenMs, f0, evals, params: cur }, null, 1));
const o = render({ ...cur, len: lenMs });
writeFileSync(out.replace(/\.json$/, '.wav'), Buffer.from(encodeWav(o.L, o.R, o.sr)));
