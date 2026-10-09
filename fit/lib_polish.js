// Polish library fits under the transient-aware loss (v6), with the hit layer available: warm start from fits_k.jsonl, short coordinate descent.
// SHARD=k/N LOSS=v6 TW=0.3 node fit/lib_polish.js <budget>  → out/gp/lib/polish_k.jsonl
import { readFileSync, readdirSync, appendFileSync, existsSync } from 'fs';
import { target, refineCD } from "./lab.js";
import { defaults } from "../dsp/engine.js";
const [sk, sn] = (process.env.SHARD || '0/1').split('/').map(Number), B = +(process.argv[2] || 200), out = `out/gp/lib/polish_${sk}.jsonl`;
const lib = readdirSync('out/gp/lib').filter((f) => /^fits_\d+\.jsonl$/.test(f)).sort().flatMap((f) => readFileSync(`out/gp/lib/${f}`, 'utf8').trim().split('\n').map(JSON.parse)).sort((a, b) => (a.file < b.file ? -1 : 1));
const done = new Set(existsSync(out) ? readFileSync(out, 'utf8').trim().split('\n').filter(Boolean).map((l) => JSON.parse(l).file) : []);
lib.forEach((x, i) => {
  if (i % sn !== sk || done.has(x.file)) return;
  try { const t = target(x.file), off = { ...defaults(), ...x.params, sn_h_lvl: -60 }, on = { ...off, sn_h_lvl: 0, sn_h_len: 3, sn_h_drive: 12, sn_h_color: -0.5 };
    const r = refineCD(t, t.loss(on) < t.loss(off) ? on : off, B); appendFileSync(out, JSON.stringify({ file: x.file, v2: t.lossOf.v2(r.P), v2_before: x.loss, loss: r.f, lenMs: x.lenMs, params: r.P }) + '\n'); }
  catch (e) { appendFileSync('out/gp/lib/polish_errors.txt', `${x.file}\t${e.message}\n`); }
});
