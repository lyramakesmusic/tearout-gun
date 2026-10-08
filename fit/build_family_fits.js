// Collect fits of Lyra's examples -> web/family_fits.json { snare: [params...], chug: [...], ping: [...] } + named presets
import { readdirSync, readFileSync, writeFileSync } from 'fs';
const out = {}, named = [];
for (const f of readdirSync('out/lyra').filter((x) => x.endsWith('.json') && !x.startsWith('jobs')).sort()) {
  const j = JSON.parse(readFileSync(`out/lyra/${f}`, 'utf8'));
  if (j.family === 'ping') continue; // modal partial positions don't fit well; the ping family covers these
  (out[j.family] = out[j.family] || []).push(j.params);
  named.push({ name: f.replace('.json', '').replace(/(\d)$/, ' $1'), params: j.params, loss: j.loss });
}
writeFileSync('web/family_fits.json', JSON.stringify(out));
writeFileSync('web/your_presets.json', JSON.stringify(named.map(({ name, params }) => ({ name, params }))));
console.log(Object.entries(out).map(([k, v]) => `${k}: ${v.length}`).join('  '), '|', named.map((n) => `${n.name} ${n.loss.toFixed(2)}`).join('  '));
