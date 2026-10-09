// Fitted library snares → web/snare_lib.json (knob values only), the anchor set for snare randomize.
import { readFileSync, readdirSync, writeFileSync } from 'fs';
import { SPEC } from '../dsp/engine.js';
const KEYS = ['note', ...SPEC.filter((s) => s.eng === 'snare').map((s) => s.id)];
const all = readdirSync('out/gp/lib').filter((f) => (process.env.SRC === "fits" ? /^fits_\d+\.jsonl$/ : /^polish_\d+\.jsonl$/).test(f)).flatMap((f) => readFileSync(`out/gp/lib/${f}`, 'utf8').trim().split('\n').map(JSON.parse));
const thr = +(process.argv[2] || 7), keep = all.filter((x) => (x.v2 ?? x.loss) < thr && !/ping|ploink/i.test(x.file));
const q = (v) => (Number.isInteger(v) ? v : +v.toPrecision(4));
writeFileSync('web/snare_lib.json', JSON.stringify({ keys: KEYS, rows: keep.map((x) => KEYS.map((k) => q(x.params[k]))) }));
console.log(keep.length, 'of', all.length, 'fits kept (loss <', thr + ')');
