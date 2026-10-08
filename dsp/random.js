// Randomize / mutate, shared by the UI and offline evaluation.
import { SPEC, SPEC_BY_ID, toNorm, fromNorm, inEngine, engineOf } from './engine.js';
import { balance } from './archetypes.js';

function gauss(r) { let u = 0; while (!u) u = r(); return Math.sqrt(-2 * Math.log(u)) * Math.cos(2 * Math.PI * r()); }

// priors: { mean, sd, guns: [params...] } from fit/build_priors.js. Anchor on a fitted gun, jitter around it.
export function randomize(P, priors, locked = new Set(), r = Math.random) {
  const Q = { ...P, engine: 0 };
  const anchor = priors?.guns?.length ? priors.guns[Math.floor(r() * priors.guns.length)] : null;
  for (const s of SPEC) {
    if (!inEngine(s, 'gun') || locked.has(s.g) || s.g.startsWith('sample') || s.id === 'len' || (s.g === 'burst' && s.id !== 'b_jitter')) continue;
    if (s.id === 'seed') { Q.seed = 1 + Math.floor(r() * 9999); continue; }
    const fitted = anchor && s.id in anchor;
    const base = fitted ? toNorm(s, anchor[s.id]) : priors?.mean?.[s.id] ?? toNorm(s, s.def);
    const sd = fitted ? 0.08 : priors?.sd?.[s.id] ?? 0.22;
    let u = base + gauss(r) * sd;
    if (s.scale === 'int' && s.unit?.includes('/') && (!fitted || r() < 0.3)) u = r();
    Q[s.id] = fromNorm(s, u);
  }
  if (anchor) Q.note = anchor.note;
  if (!locked.has('spice') && !priors?.keepSpice) Q.spice_lvl = r() < 0.6 ? -60 : -28 + r() * 14;
  return balance(Q, locked);
}

// Tail generators move gently; anything sitting at its minimum ("off") stays there,
// so repeated presses can't ratchet a dormant feature on.
const TAIL = /(_hold|_dec|_rel|_len)$|^rev_|^cr_pp|_fb$/;
export function mutate(P, locked = new Set(), r = Math.random, amt = 0.05) {
  const Q = { ...P }, eng = engineOf(P);
  for (const s of SPEC) {
    if (!inEngine(s, eng) || locked.has(s.g) || s.g.startsWith('sample') || s.g === 'burst' || s.g === 'global' || s.scale === 'int') continue;
    const u = toNorm(s, Q[s.id]);
    if (u <= 0.002 || (s.id.endsWith('_lvl') && Q[s.id] <= -59)) continue;
    Q[s.id] = fromNorm(s, u + gauss(r) * amt * (TAIL.test(s.id) ? 0.4 : 1));
  }
  return Q;
}
