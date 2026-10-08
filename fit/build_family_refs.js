// Per-family feature references from real samples (snares, chugs) -> web/family_refs.json
import { readdirSync, writeFileSync, statSync } from 'fs';
import { join } from 'path';
import { features, FEATURE_KEYS } from '../dsp/gunness.js';
import { readWav } from './lib.js';
const S = '/Users/lyra/Music/samples';
const walk = (d, out = []) => { for (const f of readdirSync(d)) { const p = join(d, f); try { if (statSync(p).isDirectory()) walk(p, out); else out.push(p); } catch {} } return out; };
const all = walk(S).filter((p) => p.toLowerCase().endsWith('.wav'));
const heavy = /tearout|riddim|dubstep|kfu|kaifu|heavy|deathstep|lyra sample pack|camacon|xlnt|avant|cult|disciple/i;
const pick = {
  snare: all.filter((p) => /snare/i.test(p.split('/').pop()) && heavy.test(p) && !/loop|roll|fill|build/i.test(p)).slice(0, 40),
  chug: all.filter((p) => /chug/i.test(p.split('/').pop()) && !/loop|convos|arp|growls r/i.test(p)),
};
const out = {};
for (const [fam, files] of Object.entries(pick)) {
  const F = [];
  for (const f of files) { try { const w = readWav(f); if (w.sr !== 48000 && w.sr !== 44100) continue; F.push(features(w.mono)); } catch {} }
  const med = (a) => { const s = [...a].sort((x, y) => x - y); return s[s.length >> 1]; };
  const ref = { median: {}, mad: {}, n: F.length };
  for (const k of FEATURE_KEYS) { const v = F.map((x) => x[k]); const m = med(v); ref.median[k] = m; ref.mad[k] = Math.max(med(v.map((x) => Math.abs(x - m))), 1e-3); }
  out[fam] = ref;
  console.log(fam, 'n', F.length, FEATURE_KEYS.map((k) => `${k} ${ref.median[k].toFixed(2)}`).join('  '));
}
writeFileSync('web/family_refs.json', JSON.stringify(out));
