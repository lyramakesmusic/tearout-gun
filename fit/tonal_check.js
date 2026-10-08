// Tonal balance of presets (vs their own PROCESSED) and of randomized guns (vs KFU median), 1/3-oct.
import { readFileSync, readdirSync } from 'fs';
import { render, rng, defaults } from '../dsp/engine.js';
import { randomize } from '../dsp/random.js';
import { readWav } from './lib.js';
import { thirdOct } from '../web/worker.js';
import { findStems } from './fit_bus.js';
const K = '/Users/lyra/Music/samples/KFU GUN KIT VOL 2.1/KFU GUN KIT VOL 2.1 GUN SORT';
const presets = JSON.parse(readFileSync('web/presets.json', 'utf8')), priors = JSON.parse(readFileSync('web/priors.json', 'utf8'));
const dirs = readdirSync(K).filter((d) => d.startsWith('KFU_GUN_'));
const MAP = JSON.parse(readFileSync('out/preset_map.json', 'utf8'));
const mono = (o) => Float32Array.from(o.L, (v, i) => 0.5 * (v + o.R[i]));
const bands = (tob) => { const c = tob.centers, d = tob.db; const avg = (lo, hi) => { const v = d.filter((_, i) => c[i] >= lo && c[i] < hi); return v.reduce((a, b) => a + b, 0) / v.length; }; return { sub: avg(35, 90), lowmid: avg(150, 400), mid: avg(500, 2000), high: avg(2000, 8000), air: avg(8000, 17000) }; };
const rows = { kfu: [], preset: [], rand: [] };
for (const pr of presets) {
  const d = dirs.find((x) => x === MAP[pr.name]); if (!d) continue;
  const t = readWav(findStems(`${K}/${d}`).proc); const tm = t.ch.length > 1 ? Float32Array.from(t.ch[0], (v, i) => 0.5 * (v + t.ch[1][i])) : t.ch[0];
  rows.kfu.push(bands(thirdOct(tm, t.sr))); rows.preset.push(bands(thirdOct(mono(render({ ...pr.params, len: 900 })), 48000)));
}
const r = rng(99);
for (let i = 0; i < 40; i++) rows.rand.push(bands(thirdOct(mono(render(randomize(defaults(), priors, new Set(), r))), 48000)));
const med = (a) => { const s = [...a].sort((x, y) => x - y); return s[s.length >> 1]; };
console.log('median band level, dB re loudest 1/3-oct band:');
for (const [k, v] of Object.entries(rows)) console.log(k.padEnd(7), Object.keys(v[0]).map((b) => `${b} ${med(v.map((x) => x[b])).toFixed(1)}`).join('  '));
const deficit = rows.preset.map((p, i) => p.high - rows.kfu[i].high);
const worst = rows.preset.map((p, i) => [presets[i]?.name, p.high - rows.kfu[i].high, p.sub - rows.kfu[i].sub]).sort((a, b) => a[1] - b[1]).slice(0, 13);
console.log("darkest presets (high deficit, sub excess):", worst.map(([n, h, s]) => `${n} ${h.toFixed(0)}/${s >= 0 ? "+" : ""}${s.toFixed(0)}`).join("  "));
console.log('preset high-band deficit vs own gun: median', med(deficit).toFixed(1), 'dB; worse than -6 dB:', deficit.filter((x) => x < -6).length, '/', deficit.length);
