// Render N patches from a sampling method into a directory: node sample.mjs <method> <n> <outdir>
import * as m from '../../dsp/engine.js';
import { roll } from '../../dsp/roll.js';
import { rollSnareFlavor, rollSnareAnchored } from '../../dsp/snare_roll.js';
import { readFileSync, writeFileSync, mkdirSync } from 'fs';
const [method, N, out] = process.argv.slice(2); mkdirSync(out, { recursive: true });
const famFits = JSON.parse(readFileSync('web/family_fits.json')), famRefs = JSON.parse(readFileSync('web/family_refs.json'));
const r = m.rng(123), SN = m.SPEC.filter((s) => s.eng === 'snare');
const gen = {
  uniform: () => { const P = { ...m.defaults(), engine: 1, len: 700, note: 24 + Math.floor(r() * 12) }; for (const s of SN) P[s.id] = m.fromNorm(s, r()); return P; },
  flavor: () => rollSnareFlavor(m.defaults(), r),
  anchored: () => rollSnareAnchored(m.defaults(), famFits.snare_eng, r),
  anchors: () => ({ ...m.defaults(), ...famFits.snare_eng[Math.floor(r() * famFits.snare_eng.length)] }),
  current: () => roll('snare', m.defaults(), { famFits, famRefs, r, k: 5 }),
  model: () => { const s = JSON.parse(readFileSync(process.env.SAMPLES)); return s[Math.floor(r() * s.length)]; },
};
const list = method === 'model' ? JSON.parse(readFileSync(process.env.SAMPLES)) : null;
for (let i = 0; i < +N; i++) {
  const P = list ? { ...m.defaults(), ...list[i % list.length] } : gen[method]();
  const o = m.render({ ...P, b_shots: 1 }); writeFileSync(`${out}/${String(i).padStart(4, '0')}.wav`, Buffer.from(m.encodeWav(o.L, o.R, o.sr)));
}
