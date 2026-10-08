// Lyra's labeled exports -> family references (her examples take precedence over pack samples).
import { readFileSync, writeFileSync } from 'fs';
import { features, FEATURE_KEYS } from '../dsp/gunness.js';
import { readWav } from './lib.js';
import { thirdOct } from '../web/worker.js';
const E = '/Users/lyra/Music/Export/';
const SR = 48000;
const to48 = (x, sr) => { if (sr === SR) return x; const r = sr / SR, n = Math.floor(x.length / r), y = new Float32Array(n); for (let i = 0; i < n; i++) { const t = i * r, j = Math.floor(t); y[i] = x[j] + (t - j) * ((x[j + 1] ?? x[j]) - x[j]); } return y; };
const load = (f) => { const w = readWav(E + f); return to48(w.mono, w.sr); };
const seg = (x, a, b) => x.subarray(Math.floor(a * SR / 1000), Math.floor(b * SR / 1000));
const combo = load('this is a snare and then 3 chugs.wav');
const sets = {
  snare: [1, 2, 3, 4, 5, 6].map((i) => load(`snare ${i}.wav`)).concat([seg(combo, 0, 265)]),
  chug: [load('this is borderline between chug and gun.wav'), seg(combo, 265, 475), seg(combo, 475, 685), seg(combo, 685, 857)],
  ping: [load('this is a ping.wav'), load('this is also a ping.wav')],
};
const prev = JSON.parse(readFileSync('web/family_refs.json', 'utf8'));
const med = (a) => { const s = [...a].sort((x, y) => x - y); return s[s.length >> 1]; };
const out = { ...prev };
for (const [fam, xs] of Object.entries(sets)) {
  const F = xs.map((x) => features(x));
  const hb = xs.map((x) => { const t = thirdOct(x, SR); const v = t.db.filter((_, i) => t.centers[i] >= 1000 && t.centers[i] < 8100); return v.reduce((a, b) => a + b, 0) / v.length; });
  const ref = { median: {}, mad: {}, n: F.length, highBand: med(hb), source: 'lyra' };
  for (const k of FEATURE_KEYS) {
    const v = F.map((x) => x[k]), m = med(v);
    // few examples: floor the spread at half the pack reference's spread so one-off values don't become hard walls
    const packMad = prev[fam]?.mad?.[k] ?? prev.snare.mad[k];
    ref.median[k] = m; ref.mad[k] = Math.max(med(v.map((x) => Math.abs(x - m))), 0.5 * packMad, 1e-3);
  }
  out[fam] = ref;
  console.log(fam.padEnd(6), `n ${F.length}  1–8k ${ref.highBand.toFixed(1)} |`, FEATURE_KEYS.map((k) => `${k} ${ref.median[k].toFixed(2)}`).join('  '), ` | was: ${prev[fam] ? FEATURE_KEYS.map((k) => prev[fam].median[k].toFixed(1)).join('/') : '—'}`);
}
writeFileSync('web/family_refs.json', JSON.stringify(out));
