// Snare-engine fits -> randomize anchors (family_fits.snare_eng) + "your sounds" presets for Lyra's own snares
// + a "snares" preset group (web/snare_presets.json) named after the source file.
import { readdirSync, readFileSync, writeFileSync } from 'fs';
import { SPEC, render, defaults } from '../dsp/engine.js';
const keep = (p) => { const o = { engine: 1, note: p.note, len: p.len, seed: p.seed }; for (const s of SPEC) if (s.eng === 'snare') o[s.id] = p[s.id]; return o; };
const fits = readdirSync('out/sfits').filter((f) => /^s\d+\.json$/.test(f)).map((f) => JSON.parse(readFileSync(`out/sfits/${f}`, 'utf8')));
const L = fits.map((f) => f.loss).sort((a, b) => a - b), cut = L[Math.floor(L.length * 0.85)];
// one-shots decay: at 150–250 ms after the peak the level is at least 5 dB under the peak (rolls and buzzes stay up)
const decays = (P) => {
  const o = render({ ...defaults(), ...P }), h = 240, e = [];
  for (let i = 0; i + h <= o.L.length; i += h) { let s = 0; for (let j = 0; j < h; j++) s += (0.5 * (o.L[i + j] + o.R[i + j])) ** 2; e.push(10 * Math.log10(s / h + 1e-12)); }
  const pk = Math.max(...e), ip = e.indexOf(pk), w = e.slice(ip + 30, ip + 50);
  return !w.length || Math.max(...w) < pk - 5;
};
const roll = /roll|buzz|fill|loop/i;
const good = fits.filter((f) => f.loss <= cut && !roll.test(f.file) && decays(f.params));
console.log('rejected as non-decaying:', fits.filter((f) => f.loss <= cut && !good.includes(f)).map((f) => f.file.split('/').pop()).join(' | '));
const fam = JSON.parse(readFileSync('web/family_fits.json', 'utf8'));
fam.snare_eng = good.map((f) => ({ ...keep(f.params), len: Math.max(400, Math.min(800, f.lenMs + 100)) }));
writeFileSync('web/family_fits.json', JSON.stringify(fam));
const lyra = fits.filter((f) => f.file.includes('/Music/Export/')).map((f) => ({ name: f.file.split('/').pop().replace(/\.wav$/, '').replace(/^lyraaaa /, ''), params: keep(f.params) }));
const yours = JSON.parse(readFileSync('web/your_presets.json', 'utf8')).filter((p) => !/^snare \d$/.test(p.name) || p.params.engine === 1);
const names = new Set(lyra.map((p) => p.name));
// snare fits stay out of the preset menu until they're close enough to their sources to carry the name
writeFileSync('out/your_snare_presets.json', JSON.stringify(lyra));
const tidy = (s) => s.replace(/\.(wav|aif+|flac)$/i, '').replace(/^(camaSnare( -)? ?ST\d\d -? ?)/, '').replace(/\s+/g, ' ').trim().toLowerCase().slice(0, 40);
const lib = good.filter((f) => !f.file.includes('/Music/Export/')).map((f) => ({ name: tidy(f.file.split('/').pop()), params: keep(f.params) }));
writeFileSync('web/snare_presets.json', JSON.stringify(lib));
console.log(`${fits.length} fits, loss p50 ${L[L.length >> 1].toFixed(2)} cut ${cut.toFixed(2)} -> ${good.length} anchors; ${lyra.length} of yours; ${lib.length} library presets`);
