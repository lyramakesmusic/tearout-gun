// Assemble per-gun presets from layer + bus fits, and randomizer priors from their distribution.
// Writes web/presets.json and web/priors.json.
import { readdirSync, readFileSync, existsSync, writeFileSync } from 'fs';
import { SPEC, SPEC_BY_ID, defaults, toNorm } from '../dsp/engine.js';

const FITS = 'out/fits';
const load = (p) => (existsSync(p) ? JSON.parse(readFileSync(p, 'utf8')) : null);

export function assemble(dir) {
  const sub = load(`${dir}/sub.json`), tr = load(`${dir}/tr.json`), body = load(`${dir}/body.json`), bus = load(`${dir}/bus.json`);
  if (!sub || !tr || !body || !bus) return null;
  const p = { ...defaults(), ...sub.params, ...tr.params, ...body.params, ...bus.params, note: sub.note };
  // layer levels: fitted stem-vs-render offset plus the bus gain applied to that stem
  const lv = { sub_lvl: sub.gainDb + bus.gains.g_sub, tr_lvl: tr.gainDb + bus.gains.g_tops, body_lvl: body.gainDb + bus.gains.g_tops };
  const top = Math.max(...Object.values(lv));
  for (const [k, v] of Object.entries(lv)) p[k] = Math.max(-60, Math.min(6, v - top));
  // KFU stems already carry their own crunch; the reverb stem sits ~18 dB under the bounce
  p.cr_mix = 0; p.spice_lvl = -60;
  p.rev_lvl = Math.max(-40, Math.min(0, -18 + (bus.gains.g_rev - bus.gains.g_tops)));
  p.body_width = 0.5;
  const mix = load(`${dir}/mix.json`);
  if (mix) Object.assign(p, mix.params);
  return { mix: mix?.loss, params: p, loss: { sub: sub.loss, tr: tr.loss, body: body.loss, bus: bus.loss, bus_plain: bus.baseline_plain_sum } };
}

if (import.meta.url === `file://${process.argv[1]}`) {
  const guns = readdirSync(FITS).filter((g) => g.startsWith('KFU_GUN_')).sort();
  const NAMES = ['rivet', 'splinter', 'hacksaw', 'slag', 'crowbar', 'shrapnel', 'bolt', 'cinder', 'gristle', 'rebar', 'flint', 'scrap',
    'tinsnip', 'kickback', 'ratchet', 'molten', 'shard', 'grindstone', 'chainlink', 'burr', 'anvil', 'slug', 'piston', 'cleaver',
    'tarmac', 'skewer', 'bitumen', 'sprocket', 'ferrule', 'magnesium', 'blister', 'gravel', 'socket', 'cutter', 'ingot', 'pylon',
    'fracture', 'torque', 'sinter', 'carbide', 'lathe', 'solder', 'hex', 'rasp', 'filament', 'bracket', 'ballast', 'gimbal', 'quench', 'temper'];
  const NOTE = ['C', 'C#', 'D', 'D#', 'E', 'F', 'F#', 'G', 'G#', 'A', 'A#', 'B'];
  const map = {};
  const presets = [];
  for (const g of guns) {
    const a = assemble(`${FITS}/${g}`);
    if (!a) continue;
    const name = `${NAMES[presets.length % NAMES.length]} ${NOTE[Math.round(a.params.note) % 12]}`;
    map[name] = g; presets.push({ name, params: a.params, loss: a.loss });
  }
  // priors in normalized space over fitted params
  const fitted = SPEC.filter((s) => s.fit !== false && presets.length);
  const mean = {}, sd = {};
  for (const s of fitted) {
    const u = presets.map((pr) => toNorm(s, pr.params[s.id]));
    const m = u.reduce((a, b) => a + b, 0) / u.length;
    mean[s.id] = m; sd[s.id] = Math.max(0.03, Math.sqrt(u.reduce((a, b) => a + (b - m) ** 2, 0) / u.length));
  }
  writeFileSync('out/preset_map.json', JSON.stringify(map, null, 1));
  writeFileSync('web/presets.json', JSON.stringify(presets.map(({ name, params }) => ({ name, params }))));
  writeFileSync('web/priors.json', JSON.stringify({ mean, sd, guns: presets.map((p) => p.params) }));
  console.log(`${presets.length} presets`);
  const med = (k) => { const v = presets.map((p) => p.loss[k]).sort((a, b) => a - b); return v[v.length >> 1]?.toFixed(3); };
  console.log('median losses', ['sub', 'tr', 'body', 'bus', 'bus_plain'].map((k) => `${k} ${med(k)}`).join('  '));
  const show = ['sub_dive', 'sub_tau', 'sub_drive', 'sub_noise', 'tr_hold', 'tr_hp', 'body_hold', 'body_metal', 'body_mode_f', 'mst_ott', 'mst_drive'];
  for (const id of show) { const s = SPEC_BY_ID[id]; const v = presets.map((p) => p.params[id]).sort((a, b) => a - b); const q = (f) => v[Math.floor(f * (v.length - 1))]; console.log(id.padEnd(12), [q(0.1), q(0.5), q(0.9)].map((x) => (+x).toFixed(2)).join(' / ')); }
}
