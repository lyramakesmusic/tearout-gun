// Fit all layers + bus for one KFU gun folder. usage: node fit/batch_gun.js "<gun dir>"
import { writeFileSync, mkdirSync, existsSync } from 'fs';
import { basename } from 'path';
import { encodeWav, SR } from '../dsp/engine.js';
import { readWav } from './lib.js';
import { fitLayer } from './fit_layer.js';
import { fitBus, findStems } from './fit_bus.js';
const dir = process.argv[2], gun = basename(dir), out = `out/fits/${gun}`;
mkdirSync(out, { recursive: true });
const PC = { C: 0, 'C#': 1, D: 2, 'D#': 3, E: 4, F: 5, 'F#': 6, G: 7, 'G#': 8, A: 9, 'A#': 10, B: 11 };
const m = gun.match(/_([A-G]#?)$/); const note = 24 + (m ? PC[m[1]] : 0);
const st = findStems(dir);
const EV = { sub: 3000, tr: 3000, body: 6000 };
for (const layer of ['sub', 'tr', 'body']) {
  const path = st[layer]; if (!path || existsSync(`${out}/${layer}.json`)) continue;
  const r = fitLayer({ layer, target: readWav(path).mono, note, evals: EV[layer], log: false });
  const { render, ...rest } = r;
  writeFileSync(`${out}/${layer}.json`, JSON.stringify({ ...rest, target: path }, null, 1));
  writeFileSync(`${out}/${layer}.wav`, Buffer.from(encodeWav(render, render, SR)));
  console.error(`${gun} ${layer} loss ${r.loss.toFixed(3)} ${r.secs.toFixed(0)}s`);
}
if (!existsSync(`${out}/bus.json`) && st.proc) {
  const r = fitBus(dir, 2000, false); const { L, R, ...rest } = r;
  writeFileSync(`${out}/bus.json`, JSON.stringify(rest, null, 1));
  writeFileSync(`${out}/bus.wav`, Buffer.from(encodeWav(L, R, SR)));
  console.error(`${gun} bus loss ${r.loss.toFixed(3)} (plain ${r.baseline_plain_sum.toFixed(3)})`);
}
