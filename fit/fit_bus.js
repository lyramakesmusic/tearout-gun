// Fit the bus (master chain + stem gains) by pushing a KFU gun's own stems through it and matching PROCESSED.
// usage: node fit/fit_bus.js --dir "<gun folder>" [--evals 1500] [--out out/fits/<gun>/bus.json]
import { readdirSync, writeFileSync, mkdirSync } from 'fs';
import { join, dirname, basename } from 'path';
import { bus, SPEC_BY_ID, toNorm, fromNorm, defaults, encodeWav, SR } from '../dsp/engine.js';
import { readWav, melStack, melLoss, cmaes, ltas, ltasLoss } from './lib.js';

export const BUS_PARAMS = ['mst_ott', 'mst_ott_up', 'mst_ott_time', 'mst_drive', 'mst_hard', 'mst_mono', 'mst_low', 'mst_mid', 'mst_high'];
const GAINS = [['g_sub', -18, 12], ['g_tops', -18, 12], ['g_rev', -30, 12]];

export function findStems(dir) {
  const f = readdirSync(dir).filter((x) => x.endsWith('.wav'));
  const pick = (tag) => f.filter((x) => x.toUpperCase().includes(`_${tag}`));
  const one = (tag) => { const m = pick(tag); return m.length ? join(dir, m[0]) : null; };
  return { proc: one('PROCESSED'), sub: one('SUB'), tr: one('TR'), body: one('BODY') || one('FX'), rev: one('REV') };
}

export function fitBus(dir, evals = 1500, log = true) {
  const st = findStems(dir);
  const load = (p) => (p ? readWav(p).ch : null);
  const P = load(st.proc), S = load(st.sub), TRs = load(st.tr), B = load(st.body), RV = load(st.rev);
  const n = Math.min(P[0].length, Math.floor(0.9 * SR));
  const stereo = (c) => (c ? [c[0].subarray(0, n), (c[1] || c[0]).subarray(0, n)] : null);
  const sub = stereo(S), tr = stereo(TRs), body = stereo(B), rev = stereo(RV);
  const tops = [new Float32Array(n), new Float32Array(n)];
  for (const x of [tr, body]) if (x) for (let c = 0; c < 2; c++) for (let i = 0; i < n; i++) tops[c][i] += x[c][i];
  const pm = Float32Array.from(P[0].subarray(0, n), (v, i) => 0.5 * (v + (P[1] || P[0])[i]));
  const T = melStack(pm, SR), TL = ltas(pm, SR);
  const specs = BUS_PARAMS.map((i) => SPEC_BY_ID[i]);
  const scale = (x, g) => (x ? x.map((ch) => { const y = new Float32Array(n); const k = 10 ** (g / 20); for (let i = 0; i < n; i++) y[i] = ch[i] * k; return y; }) : null);
  const run = (u) => {
    const p = { ...defaults() }; specs.forEach((s, i) => (p[s.id] = fromNorm(s, u[i])));
    const g = GAINS.map(([, lo, hi], j) => lo + u[specs.length + j] * (hi - lo));
    const { L, R } = bus(scale(sub, g[0]), scale(tops, g[1]), scale(rev, g[2]), p, SR);
    return { p, g, L, R };
  };
  const loss = (u) => {
    const { L, R } = run(u); const m = Float32Array.from(L, (v, i) => 0.5 * (v + R[i]));
    return melLoss(melStack(m, SR), T).loss + 0.5 * ltasLoss(ltas(m, SR), TL);
  };
  const x0 = [...specs.map((s) => toNorm(s, s.def)), ...GAINS.map(([, lo, hi]) => (0 - lo) / (hi - lo))];
  const t0 = Date.now();
  let best = { f: Infinity };
  for (let k = 0; k < 2; k++) {
    const b = cmaes(loss, x0, { sigma: 0.3, maxEvals: evals / 2, seed: 11 + k });
    if (log) console.error(`[bus] ${basename(dir)} restart ${k}: ${b.f.toFixed(3)} (${((Date.now() - t0) / 1000).toFixed(0)}s)`);
    if (b.f < best.f) best = b;
  }
  const { p, g, L, R } = run(best.x);
  // baselines: plain sum of stems (no master) and default master
  const plain = bus(sub, tops, rev, p, SR, true), m0 = Float32Array.from(plain.L, (v, i) => 0.5 * (v + plain.R[i]));
  const def = bus(sub, tops, rev, defaults(), SR), m1 = Float32Array.from(def.L, (v, i) => 0.5 * (v + def.R[i]));
  const base = (m) => melLoss(melStack(m, SR), T).loss + 0.5 * ltasLoss(ltas(m, SR), TL);
  return {
    gun: basename(dir), loss: best.f, baseline_plain_sum: base(m0), baseline_default_master: base(m1),
    params: Object.fromEntries(BUS_PARAMS.map((i) => [i, p[i]])), gains: Object.fromEntries(GAINS.map(([k], j) => [k, g[j]])),
    L, R, secs: (Date.now() - t0) / 1000,
  };
}

if (import.meta.url === `file://${process.argv[1]}`) {
  const a = Object.fromEntries(process.argv.slice(2).reduce((acc, v, i, arr) => (v.startsWith('--') ? [...acc, [v.slice(2), arr[i + 1]]] : acc), []));
  const r = fitBus(a.dir, +(a.evals || 1500));
  const out = a.out || `out/fits/${r.gun}/bus.json`;
  mkdirSync(dirname(out), { recursive: true });
  const { L, R, ...rest } = r;
  writeFileSync(out, JSON.stringify(rest, null, 1));
  writeFileSync(out.replace(/\.json$/, '.wav'), Buffer.from(encodeWav(L, R, SR)));
  console.log(JSON.stringify({ gun: r.gun, loss: r.loss.toFixed(3), plain: r.baseline_plain_sum.toFixed(3), def: r.baseline_default_master.toFixed(3), secs: r.secs.toFixed(0) }));
}
