// Pitch gauge fix for snares: note, tone tune/octave, metal tune/octave and ring interval trade off exactly.
// canonSnare rewrites them so note's pitch class is the tone's sounding pitch class and tone tune is within ±0.5 st; the render is unchanged.
import { shellMidi } from '../dsp/snare.js';
export function canonSnare(P) {
  const pc = ((Math.round(P.note) % 12) + 12) % 12, m = shellMidi(P.note, Math.round(P.sn_t_oct)) + P.sn_t_tune, c = Math.round(m), pc2 = ((c % 12) + 12) % 12;
  const ot = (c - 4 - (((pc2 - 4) % 12) + 12) % 12) / 12 - 1;
  if (ot < 2 || ot > 6) return P;
  let tm = P.sn_m_tune + pc - pc2, om = Math.round(P.sn_m_oct);
  if (tm > 12) { tm -= 12; om += 1; } else if (tm < -12) { tm += 12; om -= 1; }
  if (om < 3 || om > 7) return P;
  const Q = { ...P, note: 24 + pc2, sn_t_oct: ot, sn_m_oct: om, sn_m_tune: tm, sn_n_ring_st: ((Math.round(P.sn_n_ring_st) + pc - pc2) % 12 + 12) % 12 };
  Q.sn_t_tune = m - shellMidi(Q.note, ot);
  return Q;
}
