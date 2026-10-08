// Fitting utilities: WAV I/O, multi-resolution log-mel, gain-invariant loss, CMA-ES.
import { readFileSync } from 'fs';

// ---------------------------------------------------------------- WAV read (PCM16/24/32, float32), mixdown optional
export function readWav(path) {
  const b = readFileSync(path), v = new DataView(b.buffer, b.byteOffset, b.byteLength);
  let o = 12, fmt = null, data = null;
  while (o + 8 <= b.length) {
    const id = b.toString('ascii', o, o + 4), sz = v.getUint32(o + 4, true);
    if (id === 'fmt ') fmt = { tag: v.getUint16(o + 8, true), ch: v.getUint16(o + 10, true), sr: v.getUint32(o + 12, true), bits: v.getUint16(o + 22, true) };
    if (id === 'data') data = { off: o + 8, sz: Math.min(sz, b.length - o - 8) };
    o += 8 + sz + (sz & 1);
  }
  const { ch, sr, bits } = fmt, tag = fmt.tag === 0xfffe ? (bits === 32 ? 3 : 1) : fmt.tag;
  const bps = bits / 8, n = Math.floor(data.sz / (bps * ch));
  const out = Array.from({ length: ch }, () => new Float32Array(n));
  for (let i = 0; i < n; i++) for (let c = 0; c < ch; c++) {
    const p = data.off + (i * ch + c) * bps;
    let s;
    if (tag === 3) s = bits === 32 ? v.getFloat32(p, true) : v.getFloat64(p, true);
    else if (bits === 16) s = v.getInt16(p, true) / 32768;
    else if (bits === 24) { let x = v.getUint8(p) | (v.getUint8(p + 1) << 8) | (v.getInt8(p + 2) << 16); s = x / 8388608; }
    else s = v.getInt32(p, true) / 2147483648;
    out[c][i] = s;
  }
  return { sr, ch: out, mono: out.length === 1 ? out[0] : Float32Array.from(out[0], (x, i) => 0.5 * (x + out[1][i])) };
}

export * from '../dsp/fitcore.js';
