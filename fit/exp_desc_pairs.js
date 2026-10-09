// Library fit vs its own source, per exact descriptor: signed median error and median |error|.
import { readFileSync, readdirSync } from 'fs';
import { render } from '../dsp/engine.js';
import { readWav } from './lib.js';
import { mono } from './lab.js';
import { describe, DESC } from './descriptors.js';
const to48 = (x, sr) => { if (sr === 48000) return x; const q = sr / 48000, n = Math.floor(x.length / q), y = new Float32Array(n); for (let i = 0; i < n; i++) { const t = i * q, j = Math.floor(t); y[i] = x[j] + (t - j) * ((x[j + 1] ?? x[j]) - x[j]); } return y; };
const lib = readdirSync('out/gp/lib').filter((f) => /^fits_\d+\.jsonl$/.test(f)).flatMap((f) => readFileSync(`out/gp/lib/${f}`, 'utf8').trim().split('\n').map(JSON.parse)).filter((x) => x.loss < 7);
const S = lib.filter((_, i) => i % 8 === 0), err = Object.fromEntries(DESC.map((k) => [k, []]));
const rows = [];
for (const x of S) { const w = readWav(x.file), T = describe(to48(w.mono, w.sr)), F = describe(mono(render({ ...x.params, len: 800, b_shots: 1 }))); rows.push({ f: x.file, T, F }); for (const k of DESC) err[k].push(F[k] - T[k]); }
const q = (a, p) => [...a].sort((x, y) => x - y)[Math.floor(p * (a.length - 1))], f = (v) => (Math.abs(v) >= 100 ? v.toFixed(0) : v.toFixed(2));
console.log(S.length, 'fit/source pairs\ndescriptor       fit−real: p10 / median / p90     median |err|');
for (const k of DESC) console.log(`${k.padEnd(16)} ${[0.1, 0.5, 0.9].map((p) => f(q(err[k], p))).join(' / ').padEnd(30)} ${f(q(err[k].map(Math.abs), 0.5))}`);
const A = rows.map((r) => [r.F.attack_ms - r.T.attack_ms, r]).sort((a, b) => b[0] - a[0]);
console.log('\nlatest-peaking fits (fit attack vs real attack, ms):'); A.slice(0, 6).forEach(([d, r]) => console.log(`  ${f(r.F.attack_ms)} vs ${f(r.T.attack_ms)}  ${r.f.split('/').slice(-2).join('/')}`));
