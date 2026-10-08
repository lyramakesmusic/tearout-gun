import { SPEC, SPEC_BY_ID, defaults, toNorm, fromNorm, encodeWav, stemLayers, engineOf } from '../dsp/engine.js';
import { SNARE_GROUPS, SNARE_LEVEL } from '../dsp/snare_spec.js';
import { E808_GROUPS, E808_LEVEL } from '../dsp/e808_spec.js';
import { WHOOSH_GROUPS, WHOOSH_LEVEL } from '../dsp/whoosh_spec.js';
import { randomize, mutate } from '../dsp/random.js';
import { rollArchetype } from '../dsp/archetypes.js';
import { decodeCV } from '../dsp/cvae.js';

const GROUPS = [
  ['sub', 'sub'], ['tr', 'transient'], ['body', 'body'], ['synth', 'synth'], ['spice', 'spice'],
  ...SNARE_GROUPS, ...E808_GROUPS, ...WHOOSH_GROUPS,
  ['sample1', 'sample 1'], ['sample2', 'sample 2'], ['sample3', 'sample 3'], ['crunch', 'crunch'], ['rev', 'reverb'], ['master', 'master'], ['burst', 'burst'], ['global', 'global'],
];
const LEVEL = { sample1: 's1_lvl', sample2: 's2_lvl', sample3: 's3_lvl', sub: 'sub_lvl', tr: 'tr_lvl', body: 'body_lvl', synth: 'syn_lvl', spice: 'spice_lvl', rev: 'rev_lvl', crunch: 'cr_mix', ...SNARE_LEVEL, ...E808_LEVEL, ...WHOOSH_LEVEL };
// per-family target 1/3-octave balance (median of the references), dB re loudest band, 39 Hz .. 16 kHz
let TARGETS = { gun: [0, -3, -11, -13, -14, -18, -20, -24, -26, -24, -24, -23, -21, -20, -19, -18, -17, -16, -16, -17, -17, -17, -18, -19, -19, -20, -23] };
const targetTob = () => (document.getElementById('arch').value === 'whoosh' ? null : TARGETS[document.getElementById('arch').value] || TARGETS.gun);

let P = defaults();
const muted = new Set(), locked = new Set();
let priors = null, history = [], hpos = -1;

// ---------------------------------------------------------------- audio
const ac = new (window.AudioContext || window.webkitAudioContext)();
let buf = null, src = null, last = null, autoplay = false;
function play() {
  if (!buf) return;
  ac.resume();
  if (src) try { src.stop(); } catch {}
  src = ac.createBufferSource(); src.buffer = buf; src.connect(ac.destination); src.start();
  const b = document.getElementById('play'); b.textContent = '■';
  src.onended = () => { b.textContent = '▶'; };
}
document.getElementById('play').onclick = () => (src && document.getElementById('play').textContent === '■' ? (src.stop(), null) : play());

// ---------------------------------------------------------------- render worker
const worker = new Worker('worker.js', { type: 'module' });
let reqId = 0, pending = false, queued = false;
function effective() {
  const q = { ...P };
  for (const g of muted) if (LEVEL[g]) q[LEVEL[g]] = g === 'crunch' ? 0 : -60;
  return q;
}
function request(play = false) {
  if (play) autoplay = true;
  if (pending) { queued = true; return; }
  pending = true;
  worker.postMessage({ id: ++reqId, params: effective() });
}
worker.onmessage = (e) => {
  const d = e.data;
  if (d.ref) return showRef(d);
  if (d.stems) return showStems(d);
  if (d.steered) { P = { ...d.steered }; refreshAll(); commit(mutating); mutating = false; request(steerPlay); return; }
  if (d.rolled) return applyRolled(d.rolled, d.mode);
  pending = false;
  last = d;
  buf = ac.createBuffer(2, d.L.length, d.sr); buf.copyToChannel(d.L, 0); buf.copyToChannel(d.R, 1);
  drawSpec(document.getElementById('spec'), d.spec, d.sr);
  drawWave(d.L, d.R);
  drawTob(d.tob);
  document.getElementById('stat').textContent = `${(d.L.length / d.sr * 1000).toFixed(0)} ms · ${d.rms.toFixed(1)} dB rms`;
  updateWav();
  if (queued) { queued = false; request(); }
  else { if (autoplay) { autoplay = false; play(); } scheduleStems(); }
};

// ---------------------------------------------------------------- drawing
function magma(t) { // compact magma approximation, t in [0,1]
  const s = [[0, 0, 4], [40, 11, 84], [101, 21, 110], [159, 42, 99], [212, 72, 66], [245, 125, 21], [250, 193, 39], [252, 253, 191]];
  const x = Math.min(0.9999, Math.max(0, t)) * (s.length - 1), i = Math.floor(x), f = x - i;
  return s[i].map((v, k) => v + f * (s[i + 1][k] - v));
}
const LUT = Array.from({ length: 256 }, (_, i) => magma(i / 255));
function fitCanvas(c) {
  const r = devicePixelRatio || 1, w = Math.round(c.clientWidth * r), h = Math.round(c.clientHeight * r);
  if (c.width !== w || c.height !== h) { c.width = w; c.height = h; }
  return [w, h];
}
const SPEC_MS = 900;
function drawSpec(c, spec, sr) {
  const [w, h] = fitCanvas(c), ctx = c.getContext('2d');
  const img = ctx.createImageData(w, h), msPerFrame = (spec.hop / sr) * 1000;
  let mx = -Infinity; for (const v of spec.data) mx = Math.max(mx, v);
  const span = Math.max(SPEC_MS, spec.frames * msPerFrame);
  for (let x = 0; x < w; x++) {
    const f = Math.floor(((x / w) * span) / msPerFrame);
    for (let y = 0; y < h; y++) {
      const r = Math.floor(((h - 1 - y) / h) * spec.rows);
      const v = f < spec.frames ? spec.data[f * spec.rows + r] : -200;
      const col = LUT[Math.max(0, Math.min(255, Math.round(((v - mx + 90) / 90) * 255)))];
      const o = (y * w + x) * 4; img.data[o] = col[0]; img.data[o + 1] = col[1]; img.data[o + 2] = col[2]; img.data[o + 3] = 255;
    }
  }
  ctx.putImageData(img, 0, 0);
  // faint frequency ticks
  ctx.fillStyle = 'rgba(255,255,255,.35)'; ctx.font = `${10 * devicePixelRatio}px system-ui`;
  for (const fq of [100, 1000, 10000]) {
    const y = h - (Math.log(fq / 30) / Math.log(20000 / 30)) * h;
    ctx.fillText(fq >= 1000 ? `${fq / 1000}k` : fq, 4 * devicePixelRatio, y - 2);
  }
}
function drawWave(L, R) {
  const c = document.getElementById('wave'), [w, h] = fitCanvas(c), ctx = c.getContext('2d');
  ctx.clearRect(0, 0, w, h); ctx.fillStyle = '#6b6d72';
  const span = Math.max(SPEC_MS / 1000 * 48000, L.length);
  for (let x = 0; x < w; x++) {
    const a = Math.floor((x / w) * span), b = Math.floor(((x + 1) / w) * span);
    let mn = 0, mxv = 0; for (let i = a; i < b && i < L.length; i++) { const v = 0.5 * (L[i] + R[i]); mn = Math.min(mn, v); mxv = Math.max(mxv, v); }
    ctx.fillRect(x, h / 2 - mxv * h / 2, 1, Math.max(1, (mxv - mn) * h / 2));
  }
}
let refTob = null;
function drawTob(tob) {
  const c = document.getElementById('tob'), [w, h] = fitCanvas(c), ctx = c.getContext('2d');
  ctx.clearRect(0, 0, w, h);
  const r = devicePixelRatio, pad = 22 * r, X = (i) => pad + (i / 26) * (w - pad - 6 * r), Y = (db) => 6 * r + ((6 - db) / 54) * (h - 26 * r);
  ctx.strokeStyle = '#2a2b2e'; ctx.lineWidth = 1; ctx.fillStyle = '#5b5d61'; ctx.font = `${10 * r}px system-ui`;
  for (const db of [6, 0, -12, -24, -36, -48]) { ctx.beginPath(); ctx.moveTo(pad, Y(db)); ctx.lineTo(w, Y(db)); ctx.stroke(); ctx.fillText(db, 0, Y(db) + 3 * r); }
  [[1, '50'], [8, '250'], [14, '1k'], [20, '4k'], [26, '16k']].forEach(([i, t]) => ctx.fillText(t, X(i) - 8 * r, h - 4 * r));
  const line = (arr, col, wd) => { ctx.strokeStyle = col; ctx.lineWidth = wd * r; ctx.beginPath(); arr.forEach((v, i) => (i ? ctx.lineTo(X(i), Y(Math.max(-48, v))) : ctx.moveTo(X(i), Y(Math.max(-48, v))))); ctx.stroke(); };
  if (targetTob()) line(targetTob(), '#777', 1.5);
  if (refTob) line(refTob.db, getComputedStyle(document.body).getPropertyValue('--ref'), 1.5);
  if (tob) line(tob.db, getComputedStyle(document.body).getPropertyValue('--acc'), 2);
}

// ---------------------------------------------------------------- reference wav
let refBuf = null;
const drop = document.getElementById('drop'), dropLayer = document.getElementById('droplayer');
const overLayer = (e) => !!e.target.closest?.('#droplayer');
for (const ev of ['dragenter', 'dragover']) document.addEventListener(ev, (e) => {
  if (![...e.dataTransfer.types].includes('Files')) return;
  e.preventDefault(); const l = overLayer(e); dropLayer.classList.toggle('over', l); drop.classList.toggle('over', !l);
});
document.addEventListener('dragleave', (e) => { if (!e.relatedTarget) { drop.classList.remove('over'); dropLayer.classList.remove('over'); } });
document.addEventListener('drop', async (e) => {
  const f = [...(e.dataTransfer?.files || [])].find((x) => /\.(wav|aif+|mp3|flac|ogg)$/i.test(x.name));
  drop.classList.remove('over'); dropLayer.classList.remove('over');
  if (!f) return; e.preventDefault();
  if (overLayer(e)) return addSampleLayer(f);
  loadRef(f);
});
async function loadRef(f) {
  refBuf = await ac.decodeAudioData(await f.arrayBuffer());
  const a = refBuf.getChannelData(0), b = refBuf.numberOfChannels > 1 ? refBuf.getChannelData(1) : a;
  document.getElementById('refname').textContent = f.name.replace(/\.[^.]+$/, ''); document.getElementById('refctl').hidden = false;
  worker.postMessage({ id: -1, ref: { mono: Float32Array.from(a, (v, i) => 0.5 * (v + b[i])), sr: refBuf.sampleRate } });
}
function showRef(d) {
  const c = document.getElementById('refspec'); c.hidden = false;
  drawSpec(c, d.spec, refBuf.sampleRate);
  refTob = d.tob; document.getElementById('refleg').hidden = false;
  drawTob(last?.tob);
}
document.getElementById('spec').onclick = play;

// ---------------------------------------------------------------- wav export (download + drag-out)
let wavUrl = null;
function updateWav() {
  if (!last) return;
  if (wavUrl) URL.revokeObjectURL(wavUrl);
  wavUrl = URL.createObjectURL(new Blob([encodeWav(last.L, last.R, last.sr)], { type: 'audio/wav' }));
  const a = document.getElementById('wav'), nm = engineOf(P) === 'gun' ? 'gun' : document.getElementById('arch').value; a.href = wavUrl; a.download = `${nm}_${P.seed}_${hpos + 1}.wav`; a.textContent = `${nm}.wav`;
}
document.getElementById('wav').addEventListener('dragstart', (e) => {
  const a = e.currentTarget;
  e.dataTransfer.setData('DownloadURL', `audio/wav:${a.download}:${a.href}`);
});

// ---------------------------------------------------------------- knobs
const knobEls = {};
function fmt(s, v) {
  if (s.scale === 'int' && s.unit && s.unit.includes('/')) return s.unit.split('/')[Math.round(v)] ?? v;
  const a = Math.abs(v), n = a >= 1000 ? (v / 1000).toFixed(a >= 10000 ? 0 : 1) + 'k' : a >= 100 ? v.toFixed(0) : a >= 10 ? v.toFixed(1) : v.toFixed(2);
  const u = s.unit && !s.unit.includes('/') && s.unit !== 'midi' ? ` ${s.unit}` : '';
  if (s.unit === 'midi') { const nm = 'C C# D D# E F F# G G# A A# B'.split(' '); return nm[Math.round(v) % 12] + (Math.floor(Math.round(v) / 12) - 1); }
  return (s.scale === 'int' ? Math.round(v) : n) + u;
}
function knobSVG(u) {
  const a0 = -135, a1 = -135 + 270 * u, rad = (d) => ((d - 90) * Math.PI) / 180, R = 16;
  const p = (d) => `${20 + R * Math.cos(rad(d))} ${20 + R * Math.sin(rad(d))}`;
  const arc = (from, to, col, wd) => `<path d="M ${p(from)} A ${R} ${R} 0 ${to - from > 180 ? 1 : 0} 1 ${p(to)}" stroke="${col}" stroke-width="${wd}" fill="none" stroke-linecap="round"/>`;
  return `<svg viewBox="0 0 40 40" width="40" height="40">${arc(-135, 135, '#2c2d31', 4)}${u > 0.003 ? arc(a0, a1, 'var(--acc)', 4) : ''}<line x1="20" y1="20" x2="${20 + 11 * Math.cos(rad(a1))}" y2="${20 + 11 * Math.sin(rad(a1))}" stroke="#d9dadc" stroke-width="2" stroke-linecap="round"/></svg>`;
}
function setParam(id, v, render = true) {
  const s = SPEC_BY_ID[id];
  P[id] = s.scale === 'int' ? Math.round(v) : v;
  const el = knobEls[id];
  if (el) {
    if (el.knob) el.knob.innerHTML = knobSVG(toNorm(s, P[id]));
    else el.cycle.textContent = fmt(s, P[id]);
    if (el.k.classList.contains('active')) el.lbl.textContent = fmt(s, P[id]);
  }
  if (render) request();
}
function buildBank() {
  const bank = document.getElementById('bank');
  for (const [g, name] of GROUPS) {
    const specs = SPEC.filter((s) => s.g === g);
    const grp = document.createElement('section'); grp.className = specs.length > 16 ? 'grp wide' : 'grp';
    if (g !== 'global' && !g.startsWith('sample')) grp.dataset.eng = specs[0]?.eng || 'gun';
    if (g.startsWith('sample')) { grp.classList.add('empty'); grp.id = 'grp-' + g; }
    const h = document.createElement('h3');
    const title = document.createElement('span'); title.textContent = name;
    if (LEVEL[g]) title.onclick = () => { muted.has(g) ? muted.delete(g) : muted.add(g); grp.classList.toggle('muted', muted.has(g)); request(true); };
    const lock = document.createElement('span'); lock.className = 'lock'; lock.textContent = 'lock';
    lock.onclick = () => { locked.has(g) ? locked.delete(g) : locked.add(g); lock.classList.toggle('on', locked.has(g)); };
    const rollG = document.createElement('span'); rollG.className = 'lock'; rollG.textContent = 'roll';
    rollG.onclick = () => {
      const others = [...new Set(SPEC.map((s) => s.g))].filter((x) => x !== g);
      const mode = document.getElementById('arch').value;
      rollG.textContent = 'rolling…';
      worker.postMessage({ id: -2, rollReq: { mode, P, locked: others, group: g } });
      rollG._pending = true; pendingGroupRoll = rollG;
    };
    h.append(title, rollG, lock); grp.append(h);
    const wrap = document.createElement('div'); wrap.className = 'knobs';
    for (const s of specs) {
      const k = document.createElement('div'); k.className = 'k';
      const lbl = document.createElement('div'); lbl.className = 'lbl'; lbl.textContent = s.label;
      const isCycle = s.scale === 'int' && s.unit && s.unit.includes('/');
      if (isCycle) {
        const c = document.createElement('div'); c.className = 'cycle'; c.tabIndex = 0;
        c.textContent = fmt(s, P[s.id]);
        const step = (d) => { const n = s.max - s.min + 1; setParam(s.id, s.min + ((((P[s.id] - s.min + d) % n) + n) % n)); request(true); };
        c.onclick = (e) => step(e.shiftKey ? -1 : 1);
        c.oncontextmenu = (e) => { e.preventDefault(); step(-1); };
        k.append(c, lbl); knobEls[s.id] = { k, lbl, cycle: c };
      } else {
        const kn = document.createElement('div'); kn.className = 'knob'; kn.tabIndex = 0;
        kn.setAttribute('role', 'slider'); kn.setAttribute('aria-label', `${g} ${s.label}`);
        kn.innerHTML = knobSVG(toNorm(s, P[s.id]));
        let y0 = 0, u0 = 0, dragging = false;
        const show = (on) => { k.classList.toggle('active', on); lbl.textContent = on ? fmt(s, P[s.id]) : s.label; };
        kn.onpointerdown = (e) => { dragging = true; kn.setPointerCapture(e.pointerId); y0 = e.clientY; u0 = toNorm(s, P[s.id]); show(true); };
        kn.onpointermove = (e) => { if (!dragging) return; const du = (y0 - e.clientY) / (e.shiftKey ? 900 : 180); setParam(s.id, fromNorm(s, u0 + du)); lbl.textContent = fmt(s, P[s.id]); };
        kn.onpointerup = () => { dragging = false; show(false); commit(); request(true); };
        kn.onpointerenter = () => show(true);
        kn.onpointerleave = () => { if (!dragging) show(false); };
        kn.ondblclick = () => { setParam(s.id, s.def); commit(); request(true); };
        kn.onwheel = (e) => { e.preventDefault(); setParam(s.id, fromNorm(s, toNorm(s, P[s.id]) - Math.sign(e.deltaY) * (s.scale === 'int' ? 1 / (s.max - s.min) : 0.01))); show(true); clearTimeout(kn._t); kn._t = setTimeout(() => { show(false); commit(); request(true); }, 350); };
        kn.onkeydown = (e) => { const d = { ArrowUp: 1, ArrowRight: 1, ArrowDown: -1, ArrowLeft: -1 }[e.key]; if (d) { e.preventDefault(); setParam(s.id, fromNorm(s, toNorm(s, P[s.id]) + d * 0.02)); } };
        k.append(kn, lbl); knobEls[s.id] = { k, lbl, knob: kn };
      }
      wrap.append(k);
    }
    grp.append(wrap); bank.append(grp);
  }
}
function refreshAll() {
  for (const s of SPEC) setParam(s.id, P[s.id], false);
  const eng = engineOf(P), arch = document.getElementById('arch');
  document.querySelectorAll('.grp[data-eng]').forEach((g) => (g.hidden = g.dataset.eng !== eng));
  document.body.classList.toggle('eng-other', eng !== 'gun');
  // the family follows the loaded patch
  const famEng = { snare: 'snare', flute: 'snare', '808': '808', whoosh: 'whoosh' }[arch.value] || 'gun';
  if (famEng !== eng) { arch.value = eng === 'gun' ? 'fitted' : eng; drawTob(last?.tob); }
}

// ---------------------------------------------------------------- history
let pendingGroupRoll = null;
let anchor = null, anchorZ = [], anchorC = {}, mutating = false;
function commit(fromMutate = false) {
  if (!fromMutate) { anchor = { ...P }; anchorZ = [...Z]; anchorC = { ...C }; }
  history = history.slice(0, hpos + 1); history.push({ ...P }); hpos = history.length - 1;
  if (history.length > 200) { history.shift(); hpos--; }
}
function go(d) {
  const n = hpos + d; if (n < 0 || n >= history.length) return;
  hpos = n; P = { ...history[n] }; refreshAll(); request(true);
}
document.getElementById('back').onclick = () => go(-1);
document.getElementById('fwd').onclick = () => go(1);

// ---------------------------------------------------------------- randomize / mutate
function applyRolled(Q, mode) {
  if (pendingGroupRoll) { pendingGroupRoll.textContent = 'roll'; pendingGroupRoll = null; }
  const isMut = mode === 'mutate', b = document.getElementById(isMut ? 'mut' : 'rand');
  b.disabled = false; if (!isMut) b.textContent = 'randomize';
  P = { ...Q, ...sampleParams() }; refreshAll(); commit(isMut); request(true);
}
// best-of-K split across a small pool of workers: each scores two candidates, the best one wins
const POOL = Array.from({ length: Math.max(2, Math.min(4, (navigator.hardwareConcurrency || 4) - 2)) }, () => new Worker('worker.js', { type: 'module' }));
let rollSeq = 0;
function poolRoll(req) {
  const seq = ++rollSeq, got = [];
  POOL.forEach((w) => {
    w.onmessage = (e) => {
      if (seq !== rollSeq || !e.data.scored) return;
      got.push(e.data.scored);
      if (got.length === POOL.length) { const best = got.reduce((a, b) => (b.s < a.s ? b : a)); applyRolled(best.Q, req.mode); }
    };
    w.postMessage({ id: -2, rollReq: { ...req, k: 2, scored: true } });
  });
}
function doRandomize() {
  if (scope === 'space') { if (cv) { Z = Z.map(() => gaussZ()); for (const [k] of SPACE) C[k] = Math.max(-2, Math.min(2, 0.8 * gaussZ())); drawSpace(); applySteer(); } return; }
  const b = document.getElementById('rand'); b.disabled = true; b.textContent = 'rolling…';
  poolRoll({ mode: document.getElementById('arch').value, P, locked: [...locked] });
}
function doMutate() {
  if (scope === 'space') { if (cv) { Z = Z.map((z, i) => (anchorZ[i] ?? z) + 0.35 * gaussZ()); for (const [k] of SPACE) C[k] = Math.max(-2, Math.min(2, (anchorC[k] ?? C[k] ?? 0) + 0.3 * gaussZ())); drawSpace(); mutating = true; applySteer(); } return; }
  const b = document.getElementById('mut'); b.disabled = true;
  poolRoll({ mode: 'mutate', P: anchor || P, locked: [...locked] });
}
document.getElementById('rand').onclick = doRandomize;
document.getElementById('mut').onclick = doMutate;

// ---------------------------------------------------------------- presets
const presetSel = document.getElementById('preset');
let presets = [];
let initP = null;
presetSel.onchange = () => {
  const v = presetSel.value, saved = v.startsWith('saved:') ? loadSaved().find((x) => 'saved:' + x.name === v) : null;
  const pr = saved || presets.find((x) => x.name === v);
  P = { ...defaults(), ...(pr ? pr.params : initP || {}), ...sampleParams() };
  document.getElementById('del').hidden = !saved;
  refreshAll(); commit(); request(true);
};
async function loadJSON(path) { try { const r = await fetch(path); return r.ok ? r.json() : null; } catch { return null; } }

// ---------------------------------------------------------------- stems: click = solo audition, drag = file, "stems" = download all
let stemTimer = null, stemUrls = [];
function stemChips(ready) {
  const box = document.getElementById('stems'); box.textContent = '';
  const q = effective(), names = stemLayers(q).filter(([, k]) => q[k] > -59 && (!k.match(/^s\d_/) || sampleNames[+k[1]])).map(([n]) => n);
  for (const n of names) {
    const el = document.createElement('a'); el.textContent = n; el.className = ready?.[n] ? '' : 'wait';
    if (ready?.[n]) {
      el.href = ready[n].url; el.download = `${engineOf(P) === 'gun' ? 'gun' : document.getElementById('arch').value}_${P.seed}_${n}.wav`; el.draggable = true;
      el.onclick = (e) => { e.preventDefault(); playBuf(ready[n].buf, el); };
      el.ondragstart = (e) => e.dataTransfer.setData('DownloadURL', `audio/wav:${el.download}:${el.href}`);
    }
    box.append(el);
  }
  if (ready && names.length) {
    const all = document.createElement('a'); all.className = 'all'; all.textContent = 'stems';
    all.onclick = (e) => { e.preventDefault(); box.querySelectorAll('a[download]').forEach((x, i) => setTimeout(() => { const t = document.createElement('a'); t.href = x.href; t.download = x.download; t.click(); }, i * 250)); };
    box.append(all);
  }
}
function scheduleStems() {
  stemChips(null);
  clearTimeout(stemTimer);
  stemTimer = setTimeout(() => worker.postMessage({ id: -3, stemsReq: effective() }), 350);
}
function showStems(d) {
  stemUrls.forEach((u) => URL.revokeObjectURL(u)); stemUrls = [];
  const ready = {};
  for (const [n, [L, R]] of Object.entries(d.stems)) {
    const b = ac.createBuffer(2, L.length, d.sr); b.copyToChannel(L, 0); b.copyToChannel(R, 1);
    const url = URL.createObjectURL(new Blob([encodeWav(L, R, d.sr)], { type: 'audio/wav' })); stemUrls.push(url);
    ready[n] = { url, buf: b };
  }
  stemChips(ready);
}
let stemSrc = null;
function playBuf(b, el) {
  ac.resume(); if (stemSrc) try { stemSrc.stop(); } catch {}
  document.querySelectorAll('#stems a.playing').forEach((x) => x.classList.remove('playing'));
  stemSrc = ac.createBufferSource(); stemSrc.buffer = b; stemSrc.connect(ac.destination); stemSrc.start();
  el.classList.add('playing'); stemSrc.onended = () => el.classList.remove('playing');
}

// ---------------------------------------------------------------- gun space: verified feature knobs over a conditional VAE
// Each knob is a measured feature in σ from a typical gun. Drag = fast open-loop decode; release = closed-loop
// correction in the worker (render, measure, adjust), so the knob lands where it says.
const SPACE = [['noisy', 'noise'], ['move', 'motion'], ['crest', 'punch'], ['harsh', 'bite'], ['subMid', 'sub'], ['midLate', 'tail']];
let cv = null, Z = [0, 0, 0, 0], C = {};
const spaceEls = [];
function gaussZ() { let u = 0; while (!u) u = Math.random(); return Math.sqrt(-2 * Math.log(u)) * Math.cos(2 * Math.PI * Math.random()); }
function bipolarSVG(v) { // v in [-1, 1]
  const rad = (d) => ((d - 90) * Math.PI) / 180, R = 16, p = (d) => `${20 + R * Math.cos(rad(d))} ${20 + R * Math.sin(rad(d))}`, a1 = 135 * v;
  const arc = (from, to, col) => `<path d="M ${p(from)} A ${R} ${R} 0 ${Math.abs(to - from) > 180 ? 1 : 0} ${to > from ? 1 : 0} ${p(to)}" stroke="${col}" stroke-width="4" fill="none" stroke-linecap="round"/>`;
  return `<svg viewBox="0 0 40 40" width="40" height="40">${arc(-135, 135, '#2c2d31')}${Math.abs(v) > 0.01 ? arc(0, a1, 'var(--acc)') : ''}<line x1="20" y1="20" x2="${20 + 11 * Math.cos(rad(a1))}" y2="${20 + 11 * Math.sin(rad(a1))}" stroke="#d9dadc" stroke-width="2" stroke-linecap="round"/></svg>`;
}
const condVec = () => (cv ? cv.cond.map((k) => C[k] || 0) : []);
function spaceBase() { // what the space leaves alone
  const keep = {}; for (const s of SPEC) if (s.g === 'burst' || s.g.startsWith('sample') || locked.has(s.g) || ['len', 'seed', 'note'].includes(s.id)) keep[s.id] = P[s.id];
  return keep;
}
function applyOpen() { if (!cv) return; P = { ...P, ...decodeCV(cv, Z, condVec()), ...spaceBase() }; refreshAll(); request(); }
function applySteer(play = true) {
  if (!cv) return;
  worker.postMessage({ id: -4, steerReq: { z: Z, c: condVec(), base: { ...P, ...spaceBase() }, keep: spaceBase() } });
  steerPlay = play;
}
let steerPlay = false;
function drawSpace() { SPACE.forEach(([k], i) => (spaceEls[i].innerHTML = bipolarSVG((C[k] || 0) / 2))); }
function buildSpace() {
  const box = document.getElementById('spaceknobs');
  SPACE.forEach(([k, label]) => {
    const wrap = document.createElement('div'); wrap.className = 'k';
    const kn = document.createElement('div'); kn.className = 'knob'; kn.tabIndex = 0; kn.setAttribute('role', 'slider'); kn.setAttribute('aria-label', label);
    const lbl = document.createElement('div'); lbl.className = 'lbl'; lbl.textContent = label;
    let y0 = 0, v0 = 0, drag = false;
    kn.onpointerdown = (e) => { if (!cv) return; drag = true; kn.setPointerCapture(e.pointerId); y0 = e.clientY; v0 = C[k] || 0; };
    kn.onpointermove = (e) => { if (!drag) return; C[k] = Math.max(-2, Math.min(2, v0 + (y0 - e.clientY) / (e.shiftKey ? 400 : 90))); drawSpace(); applyOpen(); };
    kn.onpointerup = () => { if (!drag) return; drag = false; applySteer(); };
    kn.ondblclick = () => { C[k] = 0; drawSpace(); applySteer(); };
    spaceEls.push(kn); wrap.append(kn, lbl); box.append(wrap);
  });
  drawSpace();
}
// ---------------------------------------------------------------- scope: all settings vs gun-space knobs only
let scope = 'all';
try { scope = localStorage.getItem('scope') || 'all'; } catch {}
function setScope(s) {
  scope = s;
  document.body.classList.toggle('space-only', s === 'space');
  document.querySelectorAll('#scope button').forEach((b) => b.setAttribute('aria-selected', String(b.dataset.s === s)));
  try { localStorage.setItem('scope', s); } catch {}
}
document.querySelectorAll('#scope button').forEach((b) => (b.onclick = () => setScope(b.dataset.s)));

// ---------------------------------------------------------------- sample layers (dropped on "+ layer")
const sampleNames = {};
function sampleParams() { const o = {}; for (const s of SPEC) if (s.g.startsWith('sample')) o[s.id] = P[s.id]; return o; }
async function addSampleLayer(f) {
  const slot = [1, 2, 3].find((k) => !sampleNames[k]) ?? 3;
  const buf = await ac.decodeAudioData(await f.arrayBuffer());
  const L = buf.getChannelData(0).slice(), R = (buf.numberOfChannels > 1 ? buf.getChannelData(1) : buf.getChannelData(0)).slice();
  worker.postMessage({ sample: { slot, L, R, sr: buf.sampleRate } });
  sampleNames[slot] = f.name;
  setParam(`s${slot}_lvl`, 0, false); setParam(`s${slot}_len`, Math.min(2000, Math.max(10, (buf.length / buf.sampleRate) * 1000)), false);
  const grp = document.getElementById('grp-sample' + slot); grp.classList.remove('empty');
  const title = grp.querySelector('h3 span'); title.textContent = f.name.replace(/\.[^.]+$/, '');
  if (!grp.querySelector('.x')) {
    const x = document.createElement('span'); x.className = 'x'; x.textContent = 'remove';
    x.onclick = () => { worker.postMessage({ sample: { slot, L: null } }); delete sampleNames[slot]; setParam(`s${slot}_lvl`, -60, false); grp.classList.add('empty'); showLayers(); commit(); request(true); };
    grp.querySelector('h3').append(x);
  }
  commit(); request(true);
  showLayers();
  if (scope !== 'all') setScope('all');
  grp.scrollIntoView({ behavior: 'smooth', block: 'center' }); grp.classList.add('flash'); setTimeout(() => grp.classList.remove('flash'), 700);
}
function showLayers() {
  const n = Object.values(sampleNames);
  document.getElementById('droplayer').textContent = n.length ? `+ layer · ${n.map((x) => x.replace(/\.[^.]+$/, '')).join(', ')}` : '+ layer';
}

// ---------------------------------------------------------------- boot
// families that only show when running locally (works in progress)
const LOCAL = ['localhost', '127.0.0.1', '[::1]'].includes(location.hostname), LOCAL_ONLY = ['chug'];
if (!LOCAL) for (const o of [...document.querySelectorAll('#arch option')]) if (LOCAL_ONLY.includes(o.value)) o.remove();
buildBank(); renderSaved();
loadJSON('priors.json').then((j) => (priors = j));
loadJSON('targets.json').then((j) => { if (j) { TARGETS = j; TARGETS.fitted = j.gun; drawTob(last?.tob); } });
// first load opens on a fitted gun; "init" in the preset menu is the plain starting patch
const FIRST = 'filament B';
Promise.all([loadJSON('init.json'), loadJSON('your_presets.json'), loadJSON('presets.json')]).then(([ini, y, j]) => {
  initP = ini;
  const og2 = document.getElementById('og-yours');
  if (!LOCAL) y = (y || []).filter((pr) => !LOCAL_ONLY.some((f) => pr.name.startsWith(f)));
  for (const pr of y || []) { const o = document.createElement('option'); o.value = o.textContent = pr.name; og2.append(o); }
  og2.hidden = !(y || []).length;
  presets = [...(y || []), ...(j || [])];
  const og = document.getElementById('og-fitted');
  for (const pr of j || []) { const o = document.createElement('option'); o.value = o.textContent = pr.name; og.append(o); }
  if (hpos <= 0) {
    const first = presets.find((x) => x.name === FIRST);
    P = { ...defaults(), ...(first ? first.params : initP || {}) };
    if (first) presetSel.value = FIRST;
    refreshAll();
  }
  commit(); request();
});
addEventListener('resize', () => { if (last) { drawSpec(document.getElementById('spec'), last.spec, last.sr); drawWave(last.L, last.R); drawTob(last.tob); } });
document.getElementById('arch').onchange = () => { drawTob(last?.tob); doRandomize(); };

buildSpace();
loadJSON('cvae.json').then((j) => { cv = j; });
setScope(scope);

// ---------------------------------------------------------------- saved presets (this browser)
function loadSaved() { try { return JSON.parse(localStorage.getItem('savedPresets') || '[]'); } catch { return []; } }
function storeSaved(list) { try { localStorage.setItem('savedPresets', JSON.stringify(list)); return true; } catch { return false; } }
function renderSaved() {
  const og = document.getElementById('og-saved'); og.textContent = '';
  for (const s of loadSaved()) { const o = document.createElement('option'); o.value = 'saved:' + s.name; o.textContent = s.name; og.append(o); }
  og.hidden = !og.children.length;
}
function patchToSave() { const o = {}; for (const s of SPEC) if (!s.g.startsWith('sample')) o[s.id] = P[s.id]; return o; }
const saveSlot = document.getElementById('saveslot');
function showSaveButton() {
  saveSlot.innerHTML = '<button id="save">save</button>';
  document.getElementById('save').onclick = () => {
    saveSlot.innerHTML = '<input id="savename" placeholder="name" aria-label="preset name">';
    const inp = document.getElementById('savename'); inp.focus();
    inp.onkeydown = (e) => {
      if (e.key === 'Escape') return showSaveButton();
      if (e.key !== 'Enter' || !inp.value.trim()) return;
      const name = inp.value.trim(), list = loadSaved().filter((x) => x.name !== name);
      list.push({ name, params: patchToSave() });
      const ok = storeSaved(list); renderSaved(); showSaveButton();
      if (ok) { presetSel.value = 'saved:' + name; document.getElementById('del').hidden = false; }
      const b = document.getElementById('save'); b.textContent = ok ? 'saved' : 'storage blocked'; b.classList.add('flash');
      setTimeout(() => { b.textContent = 'save'; b.classList.remove('flash'); }, 1200);
    };
    inp.onblur = () => setTimeout(showSaveButton, 150);
  };
}
showSaveButton();
document.getElementById('del').onclick = () => {
  const v = presetSel.value; if (!v.startsWith('saved:')) return;
  storeSaved(loadSaved().filter((x) => 'saved:' + x.name !== v)); renderSaved();
  presetSel.value = ''; document.getElementById('del').hidden = true;
};

// ---------------------------------------------------------------- reference controls
let refMatch = false, refSrc = null;
function playRef() {
  if (!refBuf) return; ac.resume(); if (refSrc) try { refSrc.stop(); } catch {}
  const g = ac.createGain();
  if (refMatch && last) { // play the reference at the gun's loudness
    const rr = (() => { const a = refBuf.getChannelData(0); let s = 0; for (const v of a) s += v * v; return 10 * Math.log10(s / a.length + 1e-12); })();
    g.gain.value = Math.pow(10, (last.rms - rr) / 20);
  }
  refSrc = ac.createBufferSource(); refSrc.buffer = refBuf; refSrc.connect(g).connect(ac.destination); refSrc.start();
}
document.getElementById('refplay').onclick = playRef;
document.getElementById('refspec').onclick = playRef;
document.getElementById('refmatch').onclick = (e) => { refMatch = !refMatch; e.currentTarget.setAttribute('aria-pressed', String(refMatch)); };
// fit: a second worker runs the staged fit so the synth stays playable; pressing again stops it and keeps the best so far
let fitWorker = null;
const fitBtn = document.getElementById('reffit');
function stopFit(label = 'fit') { if (fitWorker) { fitWorker.terminate(); fitWorker = null; } fitBtn.textContent = label; }
fitBtn.onclick = () => {
  if (fitWorker) { stopFit(); commit(); return; }
  if (!refBuf) return;
  const a = refBuf.getChannelData(0), b = refBuf.numberOfChannels > 1 ? refBuf.getChannelData(1) : a;
  fitWorker = new Worker('worker.js', { type: 'module' });
  fitBtn.textContent = 'stop · starting';
  const keep = sampleParams();
  fitWorker.onmessage = (e) => {
    const d = e.data, pr = d.fitProgress;
    if (pr) { P = { ...P, ...pr.P, ...keep }; refreshAll(); request(); fitBtn.textContent = `stop · ${pr.stage} ${pr.i}/${pr.of}`; }
    if (d.fitDone) { P = { ...P, ...d.fitDone.P, ...keep }; refreshAll(); commit(); request(true); stopFit('fitted'); setTimeout(() => { if (!fitWorker) fitBtn.textContent = 'fit'; }, 1500); }
  };
  fitWorker.postMessage({ id: -5, fitReq: { mono: Float32Array.from(a, (v, i) => 0.5 * (v + b[i])), sr: refBuf.sampleRate, P: { ...P } } });
};
document.getElementById('refclear').onclick = () => {
  stopFit();
  refBuf = null; refTob = null; document.getElementById('refspec').hidden = true; document.getElementById('refleg').hidden = true;
  document.getElementById('refname').textContent = 'reference'; document.getElementById('refctl').hidden = true; drawTob(last?.tob);
};

// click to pick a file, as well as drag-and-drop
const pick = (inp, fn) => { inp.onchange = () => { if (inp.files[0]) fn(inp.files[0]); inp.value = ''; }; inp.click(); };
document.getElementById('droplayer').onclick = () => pick(document.getElementById('filelayer'), addSampleLayer);
document.getElementById('drop').onclick = (e) => { if (!e.target.closest('button')) pick(document.getElementById('fileref'), loadRef); };
for (const id of ['droplayer', 'refname']) document.getElementById(id).onkeydown = (e) => { if (e.key === 'Enter' || e.key === ' ') { e.preventDefault(); e.currentTarget.click(); } };
