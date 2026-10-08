// Per-family 1/3-octave target curves (median dB re loudest band) -> web/targets.json
import { readFileSync, writeFileSync } from 'fs';
import { readWav } from './lib.js';
import { thirdOct } from '../web/worker.js';
import { globSync } from 'node:fs';
const SR = 48000, E = '/Users/lyra/Music/Export/';
const to48 = (x, sr) => { if (sr === SR) return x; const r = sr / SR, n = Math.floor(x.length / r), y = new Float32Array(n); for (let i = 0; i < n; i++) { const t = i * r, j = Math.floor(t); y[i] = x[j] + (t - j) * ((x[j + 1] ?? x[j]) - x[j]); } return y; };
const load = (f) => { const w = readWav(f); return to48(w.mono, w.sr); };
const seg = (x, a, b) => x.subarray(Math.floor(a * SR / 1000), Math.floor(b * SR / 1000));
const combo = load(E + 'this is a snare and then 3 chugs.wav');
const camacon = globSync('/Users/lyra/Music/samples/CamaCon Snaretober 202*/*.wav');
const sets = {
  snare: [...camacon.map(load), ...[1, 2, 3, 4, 5, 6].map((i) => load(E + `snare ${i}.wav`)), seg(combo, 0, 265)],
  chug: [load(E + 'this is borderline between chug and gun.wav'), seg(combo, 265, 475), seg(combo, 475, 685), seg(combo, 685, 857)],
  ping: [load(E + 'this is a ping.wav'), load(E + 'this is also a ping.wav')],
};
const med = (a) => { const s = [...a].sort((x, y) => x - y); return s.length % 2 ? s[s.length >> 1] : 0.5 * (s[s.length / 2 - 1] + s[s.length / 2]); };
const out = { gun: [0, -3, -11, -13, -14, -18, -20, -24, -26, -24, -24, -23, -21, -20, -19, -18, -17, -16, -16, -17, -17, -17, -18, -19, -19, -20, -23] };
for (const [k, xs] of Object.entries(sets)) {
  const T = xs.map((x) => thirdOct(x.subarray(0, Math.min(x.length, SR)), SR).db);
  const m = T[0].map((_, i) => med(T.map((t) => t[i]))), mx = Math.max(...m);
  out[k] = m.map((v) => Math.round((v - mx) * 10) / 10);
  console.log(k, xs.length, out[k].join(' '));
}
writeFileSync('web/targets.json', JSON.stringify(out));
// flute: Lyra's trap flutes + CamaCon's flute smack
{
  const fl = [load(E + 'trap flute 1.wav'), load(E + 'trap flute 2.wav'), load('/Users/lyra/Music/samples/CamaCon Drum Kit Vol. 1/Snares/camaSnareExtra - flute smack - Bb.wav')];
  const T = fl.map((x) => thirdOct(x.subarray(0, Math.min(x.length, SR)), SR).db), m = T[0].map((_, i) => med(T.map((t) => t[i]))), mx = Math.max(...m);
  const cur = JSON.parse(readFileSync('web/targets.json', 'utf8')); cur.flute = m.map((v) => Math.round((v - mx) * 10) / 10); writeFileSync('web/targets.json', JSON.stringify(cur));
  console.log('flute', cur.flute.join(' '));
}
