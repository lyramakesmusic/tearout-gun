import { render, fft, renderStems, setSample } from '../dsp/engine.js';
import { roll } from '../dsp/roll.js';
import { steer } from '../dsp/cvae.js';
import { fitTo, fitLoss } from '../dsp/fitstages.js';
import { makePool } from '../dsp/pool.js';
import { melOf, pitchOf, decodeInv } from '../dsp/invfeat.js';
let cv = null;
// inverse model: onnxruntime-web, loaded on the first snare fit
let inv = null;
async function invModel() {
  if (inv) return inv;
  const ort = await import('https://cdn.jsdelivr.net/npm/onnxruntime-web@1.20.1/dist/ort.wasm.min.mjs');
  ort.env.wasm.wasmPaths = 'https://cdn.jsdelivr.net/npm/onnxruntime-web@1.20.1/dist/';
  const [sess, meta] = await Promise.all([ort.InferenceSession.create('inv2.onnx'), fetch('inv2.json').then((r) => r.json())]);
  return (inv = { ort, sess, meta });
}
async function invGuess(x, base) {
  const { ort, sess, meta } = await invModel(), mel = melOf(x), pit = pitchOf(x);
  const r = await sess.run({ mel: new ort.Tensor('float32', Float32Array.from(mel, (v) => v / 255), [1, 1, meta.frames, meta.bands]), pitch: new ort.Tensor('float32', Float32Array.from(pit, (v) => v / 255), [1, 1, meta.pbins]) });
  return decodeInv(meta, r.out.data, base);
}
let priors = null, ref = null, famRefs = null, famFits = null;
if (typeof WorkerGlobalScope !== 'undefined') {
  fetch('priors.json').then((r) => (r.ok ? r.json() : null)).then((j) => (priors = j)).catch(() => {});
  fetch('cvae.json').then((r) => (r.ok ? r.json() : null)).then((j) => (cv = j)).catch(() => {});
  fetch('family_fits.json').then((r) => (r.ok ? r.json() : null)).then((j) => { famFits = { ...j, snare_lib: famFits?.snare_lib }; }).catch(() => {});
  fetch('snare_lib.json').then((r) => (r.ok ? r.json() : null)).then((L) => { if (L) (famFits ||= {}).snare_lib = L.rows.map((row) => Object.fromEntries(L.keys.map((k, i) => [k, row[i]]))); }).catch(() => {});
  fetch('family_refs.json').then((r) => (r.ok ? r.json() : null)).then((j) => (famRefs = j)).catch(() => {});
  fetch('gunness.json').then((r) => (r.ok ? r.json() : null)).then((j) => (ref = j)).catch(() => {});
}

const NFFT = 1024, HOP = 128, ROWS = 220, FMIN = 30, FMAX = 20000;
const win = Float64Array.from({ length: NFFT }, (_, i) => 0.5 - 0.5 * Math.cos((2 * Math.PI * i) / NFFT));

// log-frequency spectrogram, dB, rows bottom->top
export function spectro(x, sr) {
  const frames = Math.max(1, Math.floor((x.length - NFFT) / HOP) + 1);
  const out = new Float32Array(frames * ROWS);
  const re = new Float64Array(NFFT), im = new Float64Array(NFFT);
  const rowBins = Array.from({ length: ROWS + 1 }, (_, r) => (FMIN * Math.pow(FMAX / FMIN, r / ROWS) * NFFT) / sr);
  for (let f = 0; f < frames; f++) {
    for (let i = 0; i < NFFT; i++) { re[i] = x[f * HOP + i] * win[i]; im[i] = 0; }
    fft(re, im, false);
    for (let r = 0; r < ROWS; r++) {
      const a = rowBins[r], b = Math.max(a + 1, rowBins[r + 1]);
      let s = 0, c = 0;
      for (let k = Math.floor(a); k < Math.ceil(b); k++) { s += re[k] * re[k] + im[k] * im[k]; c++; }
      out[f * ROWS + r] = 10 * Math.log10(s / Math.max(1, c) + 1e-14);
    }
  }
  return { frames, rows: ROWS, data: out, hop: HOP };
}

// 1/3-octave long-term spectrum, dB re max
export function thirdOct(x, sr) {
  const n = 8192, re = new Float64Array(n), im = new Float64Array(n), acc = new Float64Array(n / 2);
  const w = Float64Array.from({ length: n }, (_, i) => 0.5 - 0.5 * Math.cos((2 * Math.PI * i) / n));
  let fr = 0;
  for (let o = 0; o + n <= Math.max(n, x.length); o += 2048) {
    for (let i = 0; i < n; i++) { re[i] = (x[o + i] || 0) * w[i]; im[i] = 0; }
    fft(re, im, false); for (let k = 0; k < n / 2; k++) acc[k] += re[k] ** 2 + im[k] ** 2; fr++;
  }
  const centers = Array.from({ length: 27 }, (_, i) => 1000 * 2 ** ((i - 14) / 3));
  const db = centers.map((c) => {
    const lo = Math.floor((c / 2 ** (1 / 6) * n) / sr), hi = Math.ceil((c * 2 ** (1 / 6) * n) / sr);
    let s = 0; for (let k = Math.max(1, lo); k <= Math.min(n / 2 - 1, hi); k++) s += acc[k];
    return 10 * Math.log10(s / fr + 1e-14);
  });
  const mx = Math.max(...db);
  return { centers, db: db.map((v) => v - mx) };
}

if (typeof WorkerGlobalScope !== "undefined") self.onmessage = async (e) => {
  const { id, params, ref: refFile, rollReq, stemsReq, sample, steerReq, fitReq } = e.data;
  if (fitReq) { // staged fit of the current engine to the reference; progress after every stage
    const { mono, sr } = fitReq, r = sr / 48000, n = Math.floor(mono.length / r), x = new Float32Array(n);
    for (let i = 0; i < n; i++) { const t = i * r, j = Math.floor(t); x[i] = mono[j] + (t - j) * ((mono[j + 1] ?? mono[j]) - mono[j]); }
    let guess = null;
    if (fitReq.P.engine === 1 && !fitReq.noModel) try { self.postMessage({ id, fitProgress: { stage: 'model', i: 0, of: 1 } }); guess = await invGuess(x, fitReq.P); self.postMessage({ id, fitProgress: { stage: 'model', i: 1, of: 1, P: guess } }); } catch (err) { console.warn('inverse model unavailable', err); }
    const pool = fitReq.threads === 1 ? null : await makePool(x, fitReq.P, fitReq.threads || Math.max(2, Math.min(10, (navigator.hardwareConcurrency || 4) - 2)), new URL('evalworker.js', import.meta.url), fitLoss(x, fitReq.P).loss);
    const out = fitTo(x, fitReq.P, { scale: fitReq.scale || 0.6, guess, batch: pool?.batch, onProgress: (p) => self.postMessage({ id, fitProgress: p }) });
    pool?.close();
    self.postMessage({ id, fitDone: out });
    return;
  }
  if (steerReq) {
    if (!cv || !ref) return;
    const b = steer(cv, steerReq.z, steerReq.c, ref, steerReq.base);
    self.postMessage({ id, steered: { ...steerReq.base, ...b.P, ...steerReq.keep } });
    return;
  }
  if (sample) { setSample(sample.slot, sample.L, sample.R, sample.sr); return; }
  if (stemsReq) {
    const st = renderStems(stemsReq), bufs = [];
    for (const v of Object.values(st)) bufs.push(v[0].buffer, v[1].buffer);
    self.postMessage({ id, stems: st, sr: 48000 }, bufs);
    return;
  }
  if (rollReq) { // best-of-K roll
    const k = rollReq.k || (rollReq.mode === 'mutate' ? 4 : 5);
    if (rollReq.scored) { const b = roll(rollReq.mode, rollReq.P, { priors, ref, famRefs, famFits, locked: new Set(rollReq.locked), k, withScore: true }); self.postMessage({ id, scored: b, mode: rollReq.mode }); return; }
    const Q = roll(rollReq.mode, rollReq.P, { priors, ref, famRefs, famFits, locked: new Set(rollReq.locked), k });
    self.postMessage({ id, rolled: Q, mode: rollReq.mode });
    return;
  }
  if (refFile) { // analyze a reference file
    const mono = refFile.mono;
    self.postMessage({ id, ref: true, spec: spectro(mono, refFile.sr), tob: thirdOct(mono, refFile.sr) });
    return;
  }
  const t0 = performance.now();
  const out = render(params);
  const mono = Float32Array.from(out.L, (v, i) => 0.5 * (v + out.R[i]));
  const ms = performance.now() - t0;
  let pk = 0, ss = 0; for (const v of mono) { pk = Math.max(pk, Math.abs(v)); ss += v * v; }
  self.postMessage({
    id, L: out.L, R: out.R, sr: out.sr, ms,
    rms: 10 * Math.log10(ss / mono.length + 1e-12),
    spec: spectro(mono, out.sr), tob: thirdOct(mono, out.sr),
  }, [out.L.buffer, out.R.buffer]);
};
