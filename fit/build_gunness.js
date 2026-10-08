// Reference distribution of gun features from the fitted presets -> web/gunness.json. Then check separation.
import { readFileSync, writeFileSync } from 'fs';
import { render, rng, defaults } from '../dsp/engine.js';
import { features, gunScore, FEATURE_KEYS } from '../dsp/gunness.js';
import { rollArchetype } from '../dsp/archetypes.js';
const mono = (o) => Float32Array.from(o.L, (v, i) => 0.5 * (v + o.R[i]));
const presets = JSON.parse(readFileSync('web/presets.json', 'utf8'));
const F = presets.map((p) => features(mono(render({ ...p.params, len: 900 }))));
const med = (a) => { const s = [...a].sort((x, y) => x - y); return s[s.length >> 1]; };
const ref = { median: {}, mad: {} };
for (const k of FEATURE_KEYS) { const v = F.map((f) => f[k]); const m = med(v); ref.median[k] = m; ref.mad[k] = med(v.map((x) => Math.abs(x - m))); }
writeFileSync('web/gunness.json', JSON.stringify(ref));
console.log('reference medians:', FEATURE_KEYS.map((k) => `${k} ${ref.median[k].toFixed(2)}`).join('  '));
// separation: leave-one-out preset scores vs a "psy kick" (sub only, short) vs current archetype rolls
const kick = features(mono(render({ ...defaults(), tr_lvl: -14, body_lvl: -60, spice_lvl: -60, rev_lvl: -60, sub_lvl: 0, sub_dive: 48, sub_tau: 8, sub_drive: 24, sub_hold: 220, tr_click: 0 })));
const r = rng(4), rolls = {};
for (const a of ['gun', 'snare', 'ping', 'chug']) rolls[a] = Array.from({ length: 10 }, () => gunScore(features(mono(render(rollArchetype(a, r)))), ref));
const self = F.map((f) => gunScore(f, ref));
console.log(`score (lower = gunnier): presets median ${med(self).toFixed(2)} | psy kick ${gunScore(kick, ref).toFixed(2)} | ` + Object.entries(rolls).map(([a, v]) => `${a} ${med(v).toFixed(2)}`).join(' | '));
console.log('kick features:', FEATURE_KEYS.map((k) => `${k} ${kick[k].toFixed(2)}`).join('  '));
