import { readdirSync } from 'fs';
import { readWav, pitchTrack, pitchTrackRobust } from './lib.js';
const K = '/Users/lyra/Music/samples/KFU GUN KIT VOL 2.1/KFU GUN KIT VOL 2.1 GUN SORT/';
const hz = (st) => (Number.isFinite(st) ? (2 ** (st / 12)).toFixed(0) : '-');
for (const g of process.argv.slice(2)) {
  const f = readdirSync(K + g).find((x) => x.includes('SUB') && x.endsWith('.wav'));
  const x = readWav(K + g + '/' + f).mono;
  const show = (tr, lab) => console.log(lab.padEnd(8), [2, 6, 10, 20, 40, 80, 160, 300, 500].map((i) => hz(tr[i])).join(' '));
  console.log(g, '(Hz at 1,3,5,10,20,40,80,150,250 ms)');
  show(pitchTrack(x, 48000, 300), "raw"); show(pitchTrackRobust(x, 48000, 300), "robust");
  const y = Float32Array.from(x), a = Math.exp(-2 * Math.PI * 1500 / 48000);
  for (let p = 0; p < 4; p++) { let s = 0; for (let i = 0; i < y.length; i++) { s = (1 - a) * y[i] + a * s; y[i] = s; } s = 0; for (let i = y.length - 1; i >= 0; i--) { s = (1 - a) * y[i] + a * s; y[i] = s; } }
  show(pitchTrack(y, 48000, 300), 'lp1.5k');
}
