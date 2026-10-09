// Renders of the library randomize built from the first-pass fits and from the polished fits (training half only), for CLAP and descriptor scoring.
import { readFileSync, readdirSync, writeFileSync, mkdirSync } from 'fs';
import { render, defaults, encodeWav, rng } from '../dsp/engine.js';
import { rollSnareAnchored, rollSnareFlavor, rollSnareLibrary } from '../dsp/snare_roll.js';
const D = 'out/gp/rollab'; mkdirSync(D, { recursive: true });
const r = rng(5150), hash = (s) => { let h = 0; for (const c of s) h = (h * 31 + c.charCodeAt(0)) | 0; return h; };
const load = (src) => readdirSync('out/gp/lib').filter((f) => new RegExp(`^${src}_\\d+\\.jsonl$`).test(f)).flatMap((f) => readFileSync(`out/gp/lib/${f}`, 'utf8').trim().split('\n').map(JSON.parse)).filter((x) => (x.v2 ?? x.loss) < 8);
const F = load('fits'), Pz = load('polish');
const train = (L) => L.filter((x) => (hash(x.file) & 1) === 0).map((x) => x.params), held = new Set(F.filter((x) => (hash(x.file) & 1) === 1).map((x) => x.file));
const fam = JSON.parse(readFileSync('web/family_fits.json', 'utf8')).snare_eng, corpus = JSON.parse(readFileSync('analysis/snares/corpus.json', 'utf8'));
const lr = (A) => () => (r() < 0.8 ? rollSnareLibrary(defaults(), A, r, new Set(), 0.05) : r() < 0.5 ? rollSnareAnchored(defaults(), fam, r) : rollSnareFlavor(defaults(), r));
const S = { fits: lr(train(F)), polish: lr(train(Pz)), old: () => (r() < 0.5 ? rollSnareAnchored(defaults(), fam, r) : rollSnareFlavor(defaults(), r)) };
const out = [];
for (const [m, f] of Object.entries(S)) for (let i = 0; i < 200; i++) { const o = render({ ...f(), engine: 1, len: 700, b_shots: 1, note: 24 + Math.floor(r() * 12), seed: 1 + Math.floor(r() * 9999) }), w = `${D}/${m}${i}.wav`; writeFileSync(w, Buffer.from(encodeWav(o.L, o.R, o.sr))); out.push({ method: m, wav: w }); }
writeFileSync(`${D}/list.json`, JSON.stringify({ items: out, held: corpus.map((c, i) => (held.has(c.path) ? i : -1)).filter((i) => i >= 0) }));
console.log(out.length, 'renders,', held.size, 'held-out real');
