// Staged CMA-ES fit of the current engine to a target sound. Shared by the browser worker and offline scripts.
// onProgress({ stage, i, of, loss, P }) is called after every stage.
import { render, SPEC, SPEC_BY_ID, toNorm, fromNorm, SR, fft, engineOf } from './engine.js';
import { melStack, melLoss, cmaes, ltas, ltasLoss, thirdOct, tobLoss, tenv2, tenv2Loss, fspec, fspecLoss } from "./fitcore.js";

export function prepTarget(x) {
  let pk = 0; for (const v of x) pk = Math.max(pk, Math.abs(v));
  let i = 0; while (i < x.length && Math.abs(x[i]) < pk * 0.03) i++;
  const t = x.subarray(Math.max(0, i - 24)), h = 96, e = []; let top = -1e9;
  for (let k = 0; k + h <= t.length; k += h) { let s = 0; for (let j = 0; j < h; j++) s += t[k + j] ** 2; e.push(10 * Math.log10(s / h + 1e-12)); top = Math.max(top, e.at(-1)); }
  let last = 0; e.forEach((v, k) => { if (v > top - 45) last = k; });
  return { x: t, lenMs: Math.round(((last + 1) * h * 1000) / SR) + 30 };
}
export function shellPitch(x) {
  const N = 8192, re = new Float64Array(N), im = new Float64Array(N), a = Math.floor(0.01 * SR), b = Math.min(x.length, Math.floor(0.04 * SR));
  for (let i = a; i < b; i++) re[i - a] = x[i] * (0.5 - 0.5 * Math.cos((2 * Math.PI * (i - a)) / (b - a)));
  fft(re, im, false);
  let best = 0, bk = 0; for (let k = Math.floor((100 * N) / SR); k < (1000 * N) / SR; k++) { const m = re[k] ** 2 + im[k] ** 2; if (m > best) { best = m; bk = k; } }
  return (bk * SR) / N;
}

// the fit objective for a target: mel + spectrum + third-octave, and for snares the front envelope and front spectrum
export function fitLoss(target, P0) {
  const eng = engineOf(P0), prep = prepTarget(target);
  const lenMs = eng === 'snare' ? Math.min(800, Math.max(200, prep.lenMs)) : Math.min(1500, Math.max(200, prep.lenMs));
  const n = Math.floor((lenMs * SR) / 1000), T0 = new Float32Array(n); T0.set(prep.x.subarray(0, n));
  const T = melStack(T0, SR), TL = ltas(T0, SR), TT = thirdOct(T0, SR);
  // snares also match their front: the first 40 ms as a two-band envelope from the onset
  const TE = eng === 'snare' ? tenv2(T0) : null, TF = eng === "snare" ? fspec(T0) : null, onset = (m) => { let pk = 0; for (const v of m) pk = Math.max(pk, Math.abs(v)); let i = 0; while (i < m.length && Math.abs(m[i]) < pk * 0.03) i++; return m.subarray(Math.max(0, i - 24)); };
  const loss = (P) => { const o = render({ ...P, len: lenMs, b_shots: 1 }); const m = new Float32Array(n); for (let i = 0; i < n; i++) m[i] = 0.5 * (o.L[i] + o.R[i]); return melLoss(melStack(m, SR), T).loss + 0.5 * ltasLoss(ltas(m, SR), TL) + 0.3 * tobLoss(m, TT, SR) + (TE ? 0.3 * tenv2Loss(tenv2(onset(m)), TE) + 0.1 * fspecLoss(fspec(onset(m)), TF) : 0); };
  return { loss, lenMs, n, T0 };
}

export function fitTo(target, P0, { scale = 1, onProgress = () => {}, guess = null, batch = null } = {}) {
  const eng = engineOf(P0), { loss, lenMs, n, T0 } = fitLoss(target, P0);
  let cur = { ...P0, len: lenMs, b_shots: 1 }, base = loss(cur);
  const stage = (ids, ev, from = cur, seed = 3) => {
    const specs = ids.map((id) => SPEC_BY_ID[id]).filter(Boolean);
    const P = (u) => { const q = { ...from }; specs.forEach((s, i) => (q[s.id] = fromNorm(s, u[i]))); return q; };
    const b = cmaes((u) => loss(P(u)), specs.map((s) => toNorm(s, from[s.id])), { sigma: 0.2, maxEvals: Math.max(30, Math.round(ev * scale)), seed, fBatch: batch && ((us) => batch(us.map(P))) });
    return { P: P(b.x), f: b.f };
  };
  const plan = [];
  const accept = (r) => { if (r.f < base) { cur = r.P; base = r.f; } };
  const grid = (id, values, ids, evEach, evRefine, extra = {}) => () => {
    let best = { P: cur, f: base };
    for (const v of values) { const r = stage(ids, evEach, { ...cur, ...extra, [id]: v }); if (r.f < best.f) best = r; }
    accept(best); if (evRefine) accept(stage(ids, evRefine));
  };
  if (eng === 'snare') {
    const ids = (re, ex = []) => SPEC.filter((s) => s.eng === 'snare' && re.test(s.id) && !ex.includes(s.id) && s.scale !== 'int').map((s) => s.id);
    if (!P0._keepPitch) {
      const midi = Math.round(69 + 12 * Math.log2(shellPitch(T0) / 440));
      cur = { ...cur, note: 24 + (((midi % 12) + 12) % 12), sn_t_oct: Math.max(2, Math.min(5, Math.floor((midi - 4) / 12) - 1)), sn_t_tune: 0 };
      base = loss(cur);
    }
    if (guess) { const g = { ...cur, ...guess, len: lenMs, b_shots: 1 }, f = loss(g); if (f < base) { cur = g; base = f; } }
    const LVL = ['sn_t_lvl', 'sn_n_lvl', 'sn_c_lvl', 'sn_r_lvl', 'sn_b_drive', 'sn_b_snap', 'sn_b_high', 'sn_b_low', 'sn_b_hp'];
    plan.push(['levels', () => accept(stage(LVL, 500))]);
    plan.push(['octave', grid('sn_t_oct', [2, 3, 4, 5].filter((o) => Math.abs(o - cur.sn_t_oct) <= 1), ['sn_t_lvl', 'sn_t_tune', 'sn_t_pitch', 'sn_t_pitch_ms', 'sn_t_dec'], 150, 0)]);
    plan.push(['tone', grid('sn_t_wave', [0, 1, 2, 3], ids(/^sn_t_/), 220, 700)]);
    plan.push(['noise', grid('sn_n_type', [0, 1, 2, 4, 5, 6], ids(/^sn_n_/, ['sn_n_width']), 180, 900)]);
    plan.push(['click', grid('sn_c_type', [0, 1, 2, 3, 4], ids(/^sn_c_/), 120, 300)]);
    plan.push(['hit', () => { const r = stage([...ids(/^sn_h_/), 'sn_c_lvl', 'sn_b_drive'], 300, { ...cur, sn_h_lvl: cur.sn_h_lvl > -59 ? cur.sn_h_lvl : 0, sn_h_color: cur.sn_h_lvl > -59 ? cur.sn_h_color : -0.5 }); if (r.f < base) accept(r); }]);
    plan.push(['metal', () => {
      let best = { P: cur, f: base };
      for (const t of [0, 1, 2, 4]) { const r = stage(ids(/^sn_m_/, ['sn_m_width']), 260, { ...cur, sn_m_type: t, sn_m_lvl: cur.sn_m_lvl > -59 ? cur.sn_m_lvl : -10 }); if (r.f < best.f * 0.985) best = r; }
      accept(best);
    }]);
    plan.push(['clap', () => { const r = stage(ids(/^sn_k_/), 350, { ...cur, sn_k_lvl: cur.sn_k_lvl > -59 ? cur.sn_k_lvl : -8 }); if (r.f < base * 0.985) accept(r); }]);
    plan.push(['room+bus', () => accept(stage(ids(/^(sn_r_|sn_b_)/, ['sn_b_width', 'sn_b_time']), 700))]);
    plan.push(['levels+delays', () => accept(stage([...LVL, ...(cur.sn_h_lvl > -59 ? ['sn_h_lvl'] : []), 'sn_t_delay', 'sn_n_delay', 'sn_c_delay', 'sn_n_att', ...(cur.sn_m_lvl > -59 ? ['sn_m_lvl', 'sn_m_delay'] : []), ...(cur.sn_k_lvl > -59 ? ['sn_k_lvl', 'sn_k_delay'] : [])], 600))]);
    plan.push(['tone+noise', () => accept(stage([...ids(/^sn_t_/), ...ids(/^sn_n_/, ['sn_n_width'])], 900))]);
    plan.push(['body timing', () => accept(stage(['sn_t_lvl', 'sn_t_hold', 'sn_t_dec', 'sn_t_curve', 'sn_t_pitch', 'sn_t_pitch_ms', 'sn_t_drive', 'sn_t_drive_env', 'sn_b_ott', 'sn_b_drive', 'sn_r_lvl', 'sn_r_size', 'sn_n_lvl', 'sn_n_dec'], 500))]);
  } else {
    const fit = (re) => SPEC.filter((s) => (s.eng || 'gun') === 'gun' && re.test(s.id) && s.fit !== false && !s.g.startsWith('sample') && s.g !== 'meta').map((s) => s.id);
    const LV = fit(/^(sub_lvl|tr_lvl|body_lvl|syn_lvl|rev_lvl|mst_|cr_mix)/);
    plan.push(['levels', () => accept(stage(LV, 900))]);
    plan.push(['stagger', () => accept(stage(['tr_delay', 'sub_delay', 'body_delay', 'tr_lvl', 'body_lvl', 'sub_lvl', 'sub_swell', 'body_swell', 'mst_swell', 'mst_fall'], 600))]);
    plan.push(['body', () => accept(stage(fit(/^body_/), 2400))]);
    plan.push(['transient', () => accept(stage(fit(/^tr_/), 1400))]);
    plan.push(['sub', () => accept(stage([...fit(/^sub_/), 'note'], 1200))]);
    if (cur.syn_lvl > -59) plan.push(['synth', () => accept(stage(fit(/^syn_/), 1200))]);
    plan.push(['crunch+reverb', () => accept(stage(fit(/^(cr_|rev_)/), 1200))]);
    plan.push(['levels again', () => accept(stage(LV, 700))]);
  }
  plan.forEach(([name, fn], i) => { fn(); onProgress({ stage: name, i: i + 1, of: plan.length, loss: base, P: { ...cur } }); });
  return { P: cur, loss: base, lenMs };
}
