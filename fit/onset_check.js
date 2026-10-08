// Onset punch: HF (>1.5 kHz) energy in the first 20 ms vs 20–120 ms (dB), and the transient layer's share of the onset HF.
import { readFileSync, readdirSync } from 'fs';
import { render, rng, renderStems, defaults } from '../dsp/engine.js';
import { roll } from '../dsp/roll.js';
import { mutate } from '../dsp/random.js';
import { readWav } from './lib.js';
import { findStems } from './fit_bus.js';
const SR = 48000;
function hp(x) { const y = Float32Array.from(x); for (let p = 0; p < 2; p++) { let s = 0, xp = 0; const a = Math.exp(-2 * Math.PI * 1500 / SR); for (let i = 0; i < y.length; i++) { s = a * (s + y[i] - xp); xp = y[i]; y[i] = s; } } return y; }
function onset(x) {
  let i0 = 0, pk = 0; for (const v of x) pk = Math.max(pk, Math.abs(v)); while (i0 < x.length && Math.abs(x[i0]) < 0.05 * pk) i0++;
  const h = hp(x.subarray(i0)), e = (a, b) => { let s = 0; for (let i = Math.floor(a * SR); i < Math.min(h.length, b * SR); i++) s += h[i] * h[i]; return s / ((b - a) * SR); };
  return 10 * Math.log10(e(0, 0.02) / (e(0.02, 0.12) + 1e-12));
}
const mono = (L, R) => Float32Array.from(L, (v, i) => 0.5 * (v + R[i]));
const med = (a) => { const s = [...a].sort((x, y) => x - y); return s[s.length >> 1]; };
const K = '/Users/lyra/Music/samples/KFU GUN KIT VOL 2.1/KFU GUN KIT VOL 2.1 GUN SORT';
const ref = readdirSync(K).filter((d) => d.startsWith('KFU_GUN_')).map((d) => findStems(`${K}/${d}`).proc).filter(Boolean).map((p) => { const w = readWav(p); return onset(w.mono); });
const presets = JSON.parse(readFileSync('web/presets.json', 'utf8')), g = JSON.parse(readFileSync('web/gunness.json', 'utf8')), r = rng(4);
const share = (P) => { const st = renderStems(P); const tr = st.transient ? onsetE(mono(...st.transient)) : 0, all = Object.values(st).reduce((a, s) => a + onsetE(mono(...s)), 0); return tr / (all || 1); };
function onsetE(x) { const h = hp(x); let s = 0; for (let i = 0; i < 0.02 * SR; i++) s += h[i] * h[i]; return s; }
const P = presets.map((p) => onset(mono(...Object.values(render(p.params)).slice(0, 2))));
const rolls = Array.from({ length: 12 }, () => roll('gun', defaults(), { ref: g, r, k: 5 }));
const RO = rolls.map((q) => { const o = render(q); return onset(mono(o.L, o.R)); });
const chains = presets.slice(0, 12).map((p) => { let q = { ...p.params }; for (let i = 0; i < 10; i++) q = mutate(q, new Set(), r, 0.05); const o = render(q); return onset(mono(o.L, o.R)); });
console.log('onset punch (dB, first 20 ms vs next 100 ms, >1.5 kHz) median:');
console.log(`  reference guns ${med(ref).toFixed(1)} | presets ${med(P).toFixed(1)} | gun rolls ${med(RO).toFixed(1)} | presets after 10 mutates ${med(chains).toFixed(1)}`);
console.log(`transient layer's share of onset HF: presets ${(med(presets.slice(0, 12).map((p) => share(p.params))) * 100).toFixed(0)}% | gun rolls ${(med(rolls.map(share)) * 100).toFixed(0)}%`);
const lv = presets.map((p) => p.params.tr_lvl - Math.max(p.params.body_lvl, p.params.sub_lvl)); console.log('presets tr_lvl minus loudest other layer: median', med(lv).toFixed(1), 'dB');
