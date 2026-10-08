// Tearout gun engine. Pure JS, deterministic, offline. Runs in browser and Node.
// render(params) -> { L, R, sr, layers: {sub, tr, body, spice, rev} }

import { SNARE_SPEC, SNARE_STEMS } from './snare_spec.js';
import { renderSnare } from './snare.js';

export const SR = 48000;

// ---------------------------------------------------------------- param spec
// scale: 'lin' | 'log' | 'int'. Units in labels. `fit: false` keeps a param out of the optimizer.
export const SPEC = [
  // global
  { id: 'note', g: 'global', label: 'note', min: 21, max: 47, def: 26, scale: 'int', unit: 'midi' },
  { id: 'len', g: 'global', label: 'length', min: 200, max: 1500, def: 900, scale: 'lin', unit: 'ms', fit: false },
  { id: 'seed', g: 'global', label: 'seed', min: 1, max: 9999, def: 1, scale: 'int', fit: false },

  // SUB — sine with exponential-in-pitch dive (reference median: ~60–90 st, tau ~20 ms)
  { id: 'sub_lvl', g: 'sub', label: 'level', min: -60, max: 6, def: 0, unit: 'dB' },
  { id: 'sub_tune', g: 'sub', label: 'tune', min: -12, max: 24, def: 0, unit: 'st' },
  { id: 'sub_dive', g: 'sub', label: 'dive', min: 0, max: 120, def: 72, unit: 'st' },
  { id: 'sub_tau', g: 'sub', label: 'dive time', min: 0.5, max: 120, def: 17, scale: 'log', unit: 'ms' },
  { id: 'sub_k', g: 'sub', label: 'dive curve', min: 0.4, max: 2.5, def: 1 },
  { id: 'sub_click', g: 'sub', label: 'click dive', min: 0, max: 48, def: 0, unit: 'st' },
  { id: 'sub_click_ms', g: 'sub', label: 'click time', min: 0.3, max: 10, def: 2, scale: 'log', unit: 'ms' },
  { id: 'sub_shape', g: 'sub', label: 'gate shape', min: 0, max: 2, def: 0, scale: 'int', unit: 'exp/spike/dome' },
  { id: 'sub_swell', g: 'sub', label: 'swell', min: 0, max: 150, def: 0, unit: 'ms' },
  { id: 'sub_delay', g: 'sub', label: 'delay', min: 0, max: 60, def: 0, unit: 'ms' },
  { id: 'sub_att', g: 'sub', label: 'attack', min: 0, max: 10, def: 0.2, unit: 'ms' },
  { id: 'sub_dec', g: 'sub', label: 'decay', min: 20, max: 4000, def: 900, scale: 'log', unit: 'ms' },
  { id: 'sub_hold', g: 'sub', label: 'hold', min: 20, max: 1400, def: 420, scale: 'log', unit: 'ms' },
  { id: 'sub_rel', g: 'sub', label: 'gate', min: 1, max: 300, def: 15, scale: 'log', unit: 'ms' },
  { id: 'sub_noise', g: 'sub', label: 'noise', min: -60, max: 6, def: -60, unit: 'dB' },
  { id: 'sub_noise_hp', g: 'sub', label: 'noise highpass', min: 100, max: 8000, def: 1500, scale: 'log', unit: 'Hz' },
  { id: 'sub_drive', g: 'sub', label: 'drive', min: 0, max: 40, def: 14, unit: 'dB' },
  { id: 'sub_hard', g: 'sub', label: 'hardness', min: 0, max: 1, def: 0.5 },

  // TRANSIENT — colored noise burst + optional pitch-swept click
  { id: 'tr_lvl', g: 'tr', label: 'level', min: -60, max: 6, def: -8, unit: 'dB' },
  { id: 'tr_color', g: 'tr', label: 'color', min: -1, max: 1, def: 0 },
  { id: 'tr_hp', g: 'tr', label: 'highpass', min: 20, max: 8000, def: 400, scale: 'log', unit: 'Hz' },
  { id: 'tr_lp', g: 'tr', label: 'lowpass', min: 500, max: 20000, def: 16000, scale: 'log', unit: 'Hz' },
  { id: 'tr_pk_f', g: 'tr', label: 'peak freq', min: 200, max: 12000, def: 2500, scale: 'log', unit: 'Hz' },
  { id: 'tr_pk_g', g: 'tr', label: 'peak gain', min: -12, max: 24, def: 6, unit: 'dB' },
  { id: 'tr_pk_q', g: 'tr', label: 'peak Q', min: 0.3, max: 12, def: 1.5, scale: 'log' },
  { id: 'tr_click', g: 'tr', label: 'click', min: -60, max: 6, def: -12, unit: 'dB' },
  { id: 'tr_click_f0', g: 'tr', label: 'click freq', min: 800, max: 16000, def: 8000, scale: 'log', unit: 'Hz' },
  { id: 'tr_click_tau', g: 'tr', label: 'click time', min: 0.2, max: 30, def: 3, scale: 'log', unit: 'ms' },
  { id: 'tr_close', g: 'tr', label: 'close to', min: 200, max: 20000, def: 20000, scale: 'log', unit: 'Hz' },
  { id: 'tr_close_ms', g: 'tr', label: 'close time', min: 2, max: 200, def: 30, scale: 'log', unit: 'ms' },
  { id: 'tr_zap', g: 'tr', label: 'zap', min: -60, max: 6, def: -60, unit: 'dB' },
  { id: 'tr_zap_f0', g: 'tr', label: 'zap from', min: 1000, max: 18000, def: 10000, scale: 'log', unit: 'Hz' },
  { id: 'tr_zap_f1', g: 'tr', label: 'zap to', min: 60, max: 3000, def: 300, scale: 'log', unit: 'Hz' },
  { id: 'tr_zap_ms', g: 'tr', label: 'zap time', min: 2, max: 150, def: 25, scale: 'log', unit: 'ms' },
  { id: 'tr_stutter', g: 'tr', label: 'stutter', min: 1, max: 4, def: 1, scale: 'int' },
  { id: 'tr_stut_ms', g: 'tr', label: 'stutter gap', min: 5, max: 60, def: 18, scale: 'log', unit: 'ms' },
  { id: 'tr_shape', g: 'tr', label: 'gate shape', min: 0, max: 2, def: 0, scale: 'int', unit: 'exp/spike/dome' },
  { id: 'tr_swell', g: 'tr', label: 'swell', min: 0, max: 150, def: 0, unit: 'ms' },
  { id: 'tr_delay', g: 'tr', label: 'delay', min: 0, max: 60, def: 0, unit: 'ms' },
  { id: 'tr_att', g: 'tr', label: 'attack', min: 0, max: 5, def: 0.1, unit: 'ms' },
  { id: 'tr_dec', g: 'tr', label: 'decay', min: 2, max: 600, def: 60, scale: 'log', unit: 'ms' },
  { id: 'tr_hold', g: 'tr', label: 'hold', min: 5, max: 400, def: 70, scale: 'log', unit: 'ms' },
  { id: 'tr_rel', g: 'tr', label: 'gate', min: 0.5, max: 80, def: 3, scale: 'log', unit: 'ms' },
  { id: 'tr_drive', g: 'tr', label: 'drive', min: 0, max: 40, def: 12, unit: 'dB' },

  // BODY — noise through modal bank / comb / FM tone, open-close filter, laser drop
  { id: 'body_lvl', g: 'body', label: 'level', min: -60, max: 6, def: -9, unit: 'dB' },
  { id: 'body_noise', g: 'body', label: 'noise', min: -60, max: 6, def: -6, unit: 'dB' },
  { id: 'body_color', g: 'body', label: 'noise color', min: -1, max: 1, def: 0 },
  { id: 'body_modes', g: 'body', label: 'metal mix', min: -60, max: 6, def: 0, unit: 'dB' },
  { id: 'body_mode_f', g: 'body', label: 'metal freq', min: 60, max: 4000, def: 420, scale: 'log', unit: 'Hz' },
  { id: 'body_mode_set', g: 'body', label: 'partials', min: 0, max: 6, def: 0, scale: 'int', unit: 'stiff/membrane/bar/plate/cymbal/bell/808' },
  { id: 'body_key', g: 'body', label: 'tuning', min: 0, max: 1, def: 0, scale: 'int', unit: 'free/key' },
  { id: 'body_vowel', g: 'body', label: 'vowel', min: 0, max: 4, def: 0 },
  { id: 'body_vowel_mix', g: 'body', label: 'vowel mix', min: 0, max: 1, def: 0 },
  { id: 'body_metal', g: 'body', label: 'metallic', min: 0, max: 1, def: 0.65 },
  { id: 'body_q', g: 'body', label: 'ring', min: 2, max: 400, def: 60, scale: 'log' },
  { id: 'body_nmodes', g: 'body', label: 'partial count', min: 1, max: 24, def: 12, scale: 'int' },
  { id: 'body_mode_tilt', g: 'body', label: 'partial tilt', min: -12, max: 6, def: -2, unit: 'dB/oct' },
  { id: 'body_comb', g: 'body', label: 'comb mix', min: -60, max: 6, def: -12, unit: 'dB' },
  { id: 'body_comb_f', g: 'body', label: 'comb freq', min: 40, max: 4000, def: 180, scale: 'log', unit: 'Hz' },
  { id: 'body_comb_fb', g: 'body', label: 'comb feedback', min: 0, max: 0.99, def: 0.85 },
  { id: 'body_comb_damp', g: 'body', label: 'comb damp', min: 0, max: 1, def: 0.3 },
  { id: 'body_fm', g: 'body', label: 'FM mix', min: -60, max: 6, def: -60, unit: 'dB' },
  { id: 'body_fm_ratio', g: 'body', label: 'FM carrier', min: 0.5, max: 16, def: 4, scale: 'log', unit: '×note' },
  { id: 'body_fm_mod', g: 'body', label: 'FM ratio', min: 0.25, max: 8, def: 1.41, scale: 'log' },
  { id: 'body_fm_idx', g: 'body', label: 'FM index', min: 0, max: 30, def: 8 },
  { id: 'body_fm_tau', g: 'body', label: 'FM index time', min: 1, max: 400, def: 40, scale: 'log', unit: 'ms' },
  { id: 'body_drop', g: 'body', label: 'laser drop', min: 0, max: 48, def: 0, unit: 'st' },
  { id: 'body_drop_tau', g: 'body', label: 'laser time', min: 1, max: 400, def: 40, scale: 'log', unit: 'ms' },
  { id: 'body_flt_type', g: 'body', label: 'filter', min: 0, max: 3, def: 1, scale: 'int', unit: 'LP/BP/HP/notch' },
  { id: 'body_flt_f0', g: 'body', label: 'filter open', min: 100, max: 20000, def: 6000, scale: 'log', unit: 'Hz' },
  { id: 'body_flt_f1', g: 'body', label: 'filter close', min: 100, max: 20000, def: 1200, scale: 'log', unit: 'Hz' },
  { id: 'body_flt_tau', g: 'body', label: 'filter time', min: 1, max: 600, def: 80, scale: 'log', unit: 'ms' },
  { id: 'body_flt_res', g: 'body', label: 'filter res', min: 0, max: 0.97, def: 0.3 },
  { id: 'body_flt_mix', g: 'body', label: 'filter mix', min: 0, max: 1, def: 0.6 },
  { id: 'body_hp', g: 'body', label: 'highpass', min: 20, max: 2000, def: 280, scale: 'log', unit: 'Hz' },
  { id: 'body_shape', g: 'body', label: 'gate shape', min: 0, max: 2, def: 0, scale: 'int', unit: 'exp/spike/dome' },
  { id: 'body_swell', g: 'body', label: 'swell', min: 0, max: 150, def: 0, unit: 'ms' },
  { id: 'body_delay', g: 'body', label: 'delay', min: 0, max: 60, def: 0, unit: 'ms' },
  { id: 'body_att', g: 'body', label: 'attack', min: 0, max: 20, def: 0.5, unit: 'ms' },
  { id: 'body_dec', g: 'body', label: 'decay', min: 10, max: 4000, def: 400, scale: 'log', unit: 'ms' },
  { id: 'body_hold', g: 'body', label: 'hold', min: 20, max: 1000, def: 280, scale: 'log', unit: 'ms' },
  { id: 'body_rel', g: 'body', label: 'gate', min: 1, max: 300, def: 8, scale: 'log', unit: 'ms' },
  { id: 'body_drive', g: 'body', label: 'drive', min: 0, max: 40, def: 14, unit: 'dB' },
  { id: 'body_fold', g: 'body', label: 'fold', min: 0, max: 1, def: 0.15 },
  { id: 'body_width', g: 'body', label: 'width', min: 0, max: 1, def: 0.6 },

  // SYNTH — tonal layer: polyBLEP oscillators with unison, pitch dive, filter sweep
  { id: 'syn_lvl', g: 'synth', label: 'level', min: -60, max: 6, def: -60, unit: 'dB' },
  { id: 'syn_wave', g: 'synth', label: 'wave', min: 0, max: 3, def: 2, scale: 'int', unit: 'saw/square/supersaw/pulse' },
  { id: 'syn_oct', g: 'synth', label: 'octave', min: -1, max: 4, def: 2, scale: 'int' },
  { id: 'syn_tune', g: 'synth', label: 'tune', min: -12, max: 12, def: 0, unit: 'st' },
  { id: 'syn_voices', g: 'synth', label: 'voices', min: 1, max: 7, def: 5, scale: 'int' },
  { id: 'syn_detune', g: 'synth', label: 'detune', min: 0, max: 1, def: 0.35 },
  { id: 'syn_int', g: 'synth', label: 'interval', min: -12, max: 24, def: 7, scale: 'int', unit: 'st' },
  { id: 'syn_int_mix', g: 'synth', label: 'interval mix', min: 0, max: 1, def: 0 },
  { id: 'syn_vowel', g: 'synth', label: 'vowel', min: 0, max: 4, def: 0 },
  { id: 'syn_vowel_mix', g: 'synth', label: 'vowel mix', min: 0, max: 1, def: 0 },
  { id: 'syn_pitch', g: 'synth', label: 'pitch dive', min: 0, max: 48, def: 12, unit: 'st' },
  { id: 'syn_pitch_ms', g: 'synth', label: 'dive time', min: 1, max: 300, def: 30, scale: 'log', unit: 'ms' },
  { id: 'syn_flt_f0', g: 'synth', label: 'filter open', min: 200, max: 20000, def: 9000, scale: 'log', unit: 'Hz' },
  { id: 'syn_flt_f1', g: 'synth', label: 'filter close', min: 100, max: 20000, def: 1800, scale: 'log', unit: 'Hz' },
  { id: 'syn_flt_tau', g: 'synth', label: 'filter time', min: 5, max: 600, def: 90, scale: 'log', unit: 'ms' },
  { id: 'syn_flt_res', g: 'synth', label: 'filter res', min: 0, max: 0.95, def: 0.3 },
  { id: 'syn_shape', g: 'synth', label: 'gate shape', min: 0, max: 2, def: 1, scale: 'int', unit: 'exp/spike/dome' },
  { id: 'syn_swell', g: 'synth', label: 'swell', min: 0, max: 150, def: 0, unit: 'ms' },
  { id: 'syn_delay', g: 'synth', label: 'delay', min: 0, max: 60, def: 0, unit: 'ms' },
  { id: 'syn_att', g: 'synth', label: 'attack', min: 0, max: 20, def: 0.5, unit: 'ms' },
  { id: 'syn_dec', g: 'synth', label: 'decay', min: 10, max: 4000, def: 400, scale: 'log', unit: 'ms' },
  { id: 'syn_hold', g: 'synth', label: 'hold', min: 20, max: 1000, def: 220, scale: 'log', unit: 'ms' },
  { id: 'syn_rel', g: 'synth', label: 'gate', min: 1, max: 300, def: 8, scale: 'log', unit: 'ms' },
  { id: 'syn_drive', g: 'synth', label: 'drive', min: 0, max: 40, def: 8, unit: 'dB' },
  { id: 'syn_width', g: 'synth', label: 'width', min: 0, max: 1, def: 0.5 },

  // SAMPLES — dropped audio, stacked as extra layers
  { id: 's1_lvl', g: 'sample1', label: 'level', min: -60, max: 6, def: -60, unit: 'dB' },
  { id: 's1_pitch', g: 'sample1', label: 'pitch', min: -24, max: 24, def: 0, unit: 'st' },
  { id: 's1_key', g: 'sample1', label: 'tracking', min: 0, max: 1, def: 0, scale: 'int', unit: 'fixed/key' },
  { id: 's1_start', g: 'sample1', label: 'start', min: 0, max: 500, def: 0, unit: 'ms' },
  { id: 's1_len', g: 'sample1', label: 'length', min: 10, max: 2000, def: 900, scale: 'log', unit: 'ms' },
  { id: 's1_rel', g: 'sample1', label: 'gate', min: 1, max: 300, def: 10, scale: 'log', unit: 'ms' },
  { id: 's1_hp', g: 'sample1', label: 'highpass', min: 20, max: 8000, def: 20, scale: 'log', unit: 'Hz' },
  { id: 's1_lp', g: 'sample1', label: 'lowpass', min: 300, max: 20000, def: 20000, scale: 'log', unit: 'Hz' },
  { id: 's1_drive', g: 'sample1', label: 'drive', min: 0, max: 40, def: 0, unit: 'dB' },
  { id: 's1_route', g: 'sample1', label: 'route', min: 0, max: 1, def: 0, scale: 'int', unit: 'mids/clean' },
  { id: 's2_lvl', g: 'sample2', label: 'level', min: -60, max: 6, def: -60, unit: 'dB' },
  { id: 's2_pitch', g: 'sample2', label: 'pitch', min: -24, max: 24, def: 0, unit: 'st' },
  { id: 's2_key', g: 'sample2', label: 'tracking', min: 0, max: 1, def: 0, scale: 'int', unit: 'fixed/key' },
  { id: 's2_start', g: 'sample2', label: 'start', min: 0, max: 500, def: 0, unit: 'ms' },
  { id: 's2_len', g: 'sample2', label: 'length', min: 10, max: 2000, def: 900, scale: 'log', unit: 'ms' },
  { id: 's2_rel', g: 'sample2', label: 'gate', min: 1, max: 300, def: 10, scale: 'log', unit: 'ms' },
  { id: 's2_hp', g: 'sample2', label: 'highpass', min: 20, max: 8000, def: 20, scale: 'log', unit: 'Hz' },
  { id: 's2_lp', g: 'sample2', label: 'lowpass', min: 300, max: 20000, def: 20000, scale: 'log', unit: 'Hz' },
  { id: 's2_drive', g: 'sample2', label: 'drive', min: 0, max: 40, def: 0, unit: 'dB' },
  { id: 's2_route', g: 'sample2', label: 'route', min: 0, max: 1, def: 0, scale: 'int', unit: 'mids/clean' },
  { id: 's3_lvl', g: 'sample3', label: 'level', min: -60, max: 6, def: -60, unit: 'dB' },
  { id: 's3_pitch', g: 'sample3', label: 'pitch', min: -24, max: 24, def: 0, unit: 'st' },
  { id: 's3_key', g: 'sample3', label: 'tracking', min: 0, max: 1, def: 0, scale: 'int', unit: 'fixed/key' },
  { id: 's3_start', g: 'sample3', label: 'start', min: 0, max: 500, def: 0, unit: 'ms' },
  { id: 's3_len', g: 'sample3', label: 'length', min: 10, max: 2000, def: 900, scale: 'log', unit: 'ms' },
  { id: 's3_rel', g: 'sample3', label: 'gate', min: 1, max: 300, def: 10, scale: 'log', unit: 'ms' },
  { id: 's3_hp', g: 'sample3', label: 'highpass', min: 20, max: 8000, def: 20, scale: 'log', unit: 'Hz' },
  { id: 's3_lp', g: 'sample3', label: 'lowpass', min: 300, max: 20000, def: 20000, scale: 'log', unit: 'Hz' },
  { id: 's3_drive', g: 'sample3', label: 'drive', min: 0, max: 40, def: 0, unit: 'dB' },
  { id: 's3_route', g: 'sample3', label: 'route', min: 0, max: 1, def: 0, scale: 'int', unit: 'mids/clean' },

  // SPICE — comb ping / sine bleeps / crackle / metal hit
  { id: 'spice_lvl', g: 'spice', label: 'level', min: -60, max: 6, def: -60, unit: 'dB' },
  { id: 'spice_type', g: 'spice', label: 'type', min: 0, max: 4, def: 1, scale: 'int', unit: 'comb/bleeps/crackle/metal/808' },
  { id: 'spice_f', g: 'spice', label: 'freq', min: 100, max: 14000, def: 3000, scale: 'log', unit: 'Hz' },
  { id: 'spice_density', g: 'spice', label: 'density', min: 1, max: 300, def: 30, scale: 'log', unit: '/s' },
  { id: 'spice_len', g: 'spice', label: 'length', min: 5, max: 800, def: 200, scale: 'log', unit: 'ms' },
  { id: 'spice_hp', g: 'spice', label: 'highpass', min: 20, max: 4000, def: 300, scale: 'log', unit: 'Hz' },

  // REVERB — convolver with synthesized IRs
  { id: 'rev_lvl', g: 'rev', label: 'level', min: -60, max: 6, def: -18, unit: 'dB' },
  { id: 'rev_type', g: 'rev', label: 'IR', min: 0, max: 4, def: 0, scale: 'int', unit: 'plate/metal/whip/spring/room' },
  { id: 'rev_size', g: 'rev', label: 'size', min: 20, max: 2500, def: 350, scale: 'log', unit: 'ms' },
  { id: 'rev_char', g: 'rev', label: 'character', min: 0, max: 1, def: 0.5 },
  { id: 'rev_tone', g: 'rev', label: 'tone', min: 800, max: 20000, def: 9000, scale: 'log', unit: 'Hz' },
  { id: 'rev_pre', g: 'rev', label: 'predelay', min: 0, max: 60, def: 4, unit: 'ms' },
  { id: 'rev_hp', g: 'rev', label: 'highpass', min: 20, max: 3000, def: 350, scale: 'log', unit: 'Hz' },

  // CRUNCH — tearout chain on the tops (TR+BODY+SPICE): pre-EQ -> clip -> disperser -> short convolver -> clip -> [OTT -> clip] x N -> freq shift
  { id: 'cr_mix', g: 'crunch', label: 'mix', min: 0, max: 1, def: 1 },
  { id: 'cr_hp', g: 'crunch', label: 'crossover', min: 80, max: 600, def: 250, scale: 'log', unit: 'Hz' },
  { id: 'cr_mud', g: 'crunch', label: 'mud cut', min: -18, max: 0, def: -6, unit: 'dB' },
  { id: 'cr_pk_f', g: 'crunch', label: 'scream freq', min: 300, max: 4000, def: 800, scale: 'log', unit: 'Hz' },
  { id: 'cr_pk_g', g: 'crunch', label: 'scream boost', min: 0, max: 18, def: 6, unit: 'dB' },
  { id: 'cr_drive1', g: 'crunch', label: 'drive 1', min: 0, max: 40, def: 18, unit: 'dB' },
  { id: 'cr_hard1', g: 'crunch', label: 'hardness 1', min: 0, max: 1, def: 0.4 },
  { id: 'cr_disp_n', g: 'crunch', label: 'disperse', min: 0, max: 32, def: 8, scale: 'int', unit: 'stages' },
  { id: 'cr_disp_f', g: 'crunch', label: 'disperse freq', min: 80, max: 8000, def: 1200, scale: 'log', unit: 'Hz' },
  { id: 'cr_disp_q', g: 'crunch', label: 'disperse pinch', min: 0.3, max: 4, def: 0.8, scale: 'log' },
  { id: 'cr_conv', g: 'crunch', label: 'convolve', min: 0, max: 1, def: 0.5 },
  { id: 'cr_conv_type', g: 'crunch', label: 'conv IR', min: 0, max: 4, def: 0, scale: 'int', unit: 'plate/metal/whip/spring/room' },
  { id: 'cr_conv_len', g: 'crunch', label: 'conv size', min: 4, max: 200, def: 30, scale: 'log', unit: 'ms' },
  { id: 'cr_conv_char', g: 'crunch', label: 'conv character', min: 0, max: 1, def: 0.5 },
  { id: 'cr_drive2', g: 'crunch', label: 'drive 2', min: 0, max: 40, def: 10, unit: 'dB' },
  { id: 'cr_rack', g: 'crunch', label: 'loud rack', min: 0, max: 3, def: 2, scale: 'int', unit: 'OTT→clip passes' },
  { id: 'cr_rack_depth', g: 'crunch', label: 'rack depth', min: 0, max: 1, def: 0.5 },
  { id: 'cr_pp', g: 'crunch', label: 'ping-pong', min: 0, max: 1, def: 0 },
  { id: 'cr_pp_ms', g: 'crunch', label: 'pp time', min: 6, max: 40, def: 18, scale: 'log', unit: 'ms' },
  { id: 'cr_pp_fb', g: 'crunch', label: 'pp feedback', min: 0, max: 0.95, def: 0.6 },
  { id: 'cr_fs', g: 'crunch', label: 'freq shift', min: -400, max: 400, def: 0, unit: 'Hz' },
  { id: 'cr_fs_wet', g: 'crunch', label: 'shift wet', min: 0, max: 1, def: 0 },

  // MASTER
  { id: 'mst_sub_in', g: 'master', label: 'sub into chain', min: 0, max: 1, def: 0 },
  { id: 'mst_swell', g: 'master', label: 'post swell', min: 0, max: 150, def: 0, unit: 'ms' },
  { id: 'mst_fall', g: 'master', label: 'post fall', min: 0, max: 1500, def: 0, unit: 'ms' },
  { id: 'mst_snap', g: 'master', label: 'snap', min: 0, max: 12, def: 0, unit: 'dB' },
  { id: 'mst_ott', g: 'master', label: 'OTT', min: 0, max: 1, def: 0.5 },
  { id: 'mst_ott_up', g: 'master', label: 'OTT upward', min: 0, max: 1, def: 0.6 },
  { id: 'mst_ott_time', g: 'master', label: 'OTT time', min: 0.1, max: 4, def: 1, scale: 'log' },
  { id: 'mst_drive', g: 'master', label: 'drive', min: 0, max: 30, def: 6, unit: 'dB' },
  { id: 'mst_hard', g: 'master', label: 'clip hardness', min: 0, max: 1, def: 0.6 },
  { id: 'mst_mono', g: 'master', label: 'mono below', min: 40, max: 300, def: 130, scale: 'log', unit: 'Hz' },
  { id: 'mst_low', g: 'master', label: 'low shelf', min: -12, max: 12, def: 0, unit: 'dB' },
  { id: 'mst_mid', g: 'master', label: 'low-mid scoop', min: -18, max: 6, def: -4, unit: 'dB' },
  { id: 'mst_high', g: 'master', label: 'high shelf', min: -12, max: 12, def: 0, unit: 'dB' },

  // BURST
  { id: 'b_shots', g: 'burst', label: 'shots', min: 1, max: 32, def: 1, scale: 'int', fit: false },
  { id: 'b_rate', g: 'burst', label: 'interval', min: 12, max: 500, def: 90, scale: 'log', unit: 'ms', fit: false },
  { id: 'b_accel', g: 'burst', label: 'accel', min: -1, max: 1, def: 0, fit: false },
  { id: 'b_drift', g: 'burst', label: 'pitch drift', min: -4, max: 4, def: 0, unit: 'st/shot', fit: false },
  { id: 'b_jitter', g: 'burst', label: 'variation', min: 0, max: 1, def: 0.3, fit: false },
  { id: 'b_gate', g: 'burst', label: 'chop', min: 0.2, max: 1, def: 0.9, fit: false },
  // which engine renders the patch: 0 gun, 1 snare
  { id: 'engine', g: 'meta', label: 'engine', min: 0, max: 1, def: 0, scale: 'int', fit: false },
  ...SNARE_SPEC,
];
// params that belong to the patch's engine (plus the shared globals)
export const engineOf = (p) => (Math.round(p?.engine || 0) === 1 ? 'snare' : 'gun');
export const inEngine = (s, eng) => s.g === 'global' || s.g.startsWith('sample') || (s.eng || 'gun') === eng;

export const SPEC_BY_ID = Object.fromEntries(SPEC.map((s) => [s.id, s]));
export const defaults = () => Object.fromEntries(SPEC.map((s) => [s.id, s.def]));

// normalized [0,1] <-> value
export function toNorm(s, v) {
  if (s.scale === 'log') return Math.log(v / s.min) / Math.log(s.max / s.min);
  return (v - s.min) / (s.max - s.min);
}
export function fromNorm(s, u) {
  u = Math.min(1, Math.max(0, u));
  if (s.scale === 'log') return s.min * Math.pow(s.max / s.min, u);
  const v = s.min + u * (s.max - s.min);
  return s.scale === 'int' ? Math.round(v) : v;
}

// ---------------------------------------------------------------- utilities
export function rng(seed) {
  let a = (seed >>> 0) || 1;
  return () => {
    a |= 0; a = (a + 0x6d2b79f5) | 0;
    let t = Math.imul(a ^ (a >>> 15), 1 | a);
    t = (t + Math.imul(t ^ (t >>> 7), 61 | t)) ^ t;
    return ((t ^ (t >>> 14)) >>> 0) / 4294967296;
  };
}
export const dbToLin = (d) => Math.pow(10, d / 20);
export const midiHz = (m) => 440 * Math.pow(2, (m - 69) / 12);
export const TAU = Math.PI * 2;

// attack (linear) -> exp decay while holding -> raised-cosine gate. Times in ms.
// shape 1 (spike): drops to 35% over the first 10% of hold, then a straight ramp to 0 at hold end.
// shape 2 (dome): quarter-cosine from 1 to 0 across the hold.
export function envelope(n, sr, att, dec, hold, rel, shape = 0) {
  const e = new Float32Array(n);
  const a = att * sr / 1000, h = hold * sr / 1000, r = Math.max(1, rel * sr / 1000);
  const k = 1000 / (dec * sr), span = Math.max(1, h - a);
  for (let i = 0; i < n; i++) {
    let v = i < a ? i / a : 1;
    if (shape === 0) v *= Math.exp(-(i - Math.min(i, a)) * k);
    else if (i >= a) {
      const x = Math.min(1, (i - a) / span);
      v = shape === 1 ? (x < 0.1 ? 1 - 6.5 * x : 0.35 * (1 - (x - 0.1) / 0.9)) : Math.cos(0.5 * Math.PI * x);
    }
    if (i > h) { const x = (i - h) / r; v *= x >= 1 ? 0 : 0.5 + 0.5 * Math.cos(Math.PI * x); }
    e[i] = v;
  }
  return e;
}
const activeLen = (sr, att, hold, rel) => Math.ceil(((att + hold + rel) * sr) / 1000);

// RBJ biquad, processes in place
export function biquad(x, type, f, q, gainDb, sr) {
  const w = TAU * Math.min(f, sr * 0.49) / sr, cw = Math.cos(w), sw = Math.sin(w);
  const al = sw / (2 * q), A = Math.pow(10, gainDb / 40);
  let b0, b1, b2, a0, a1, a2;
  if (type === 'lp') { b0 = (1 - cw) / 2; b1 = 1 - cw; b2 = b0; a0 = 1 + al; a1 = -2 * cw; a2 = 1 - al; }
  else if (type === 'hp') { b0 = (1 + cw) / 2; b1 = -(1 + cw); b2 = b0; a0 = 1 + al; a1 = -2 * cw; a2 = 1 - al; }
  else if (type === 'bp') { b0 = al; b1 = 0; b2 = -al; a0 = 1 + al; a1 = -2 * cw; a2 = 1 - al; }
  else if (type === 'peak') { b0 = 1 + al * A; b1 = -2 * cw; b2 = 1 - al * A; a0 = 1 + al / A; a1 = -2 * cw; a2 = 1 - al / A; }
  else if (type === 'ls' || type === 'hs') {
    const s = type === 'ls' ? 1 : -1, sq = 2 * Math.sqrt(A) * al;
    b0 = A * ((A + 1) - s * (A - 1) * cw + sq); b1 = s * 2 * A * ((A - 1) - s * (A + 1) * cw); b2 = A * ((A + 1) - s * (A - 1) * cw - sq);
    a0 = (A + 1) + s * (A - 1) * cw + sq; a1 = -s * 2 * ((A - 1) + s * (A + 1) * cw); a2 = (A + 1) + s * (A - 1) * cw - sq;
  } else if (type === 'ap') { b0 = 1 - al; b1 = -2 * cw; b2 = 1 + al; a0 = 1 + al; a1 = -2 * cw; a2 = 1 - al; }
  b0 /= a0; b1 /= a0; b2 /= a0; a1 /= a0; a2 /= a0;
  let x1 = 0, x2 = 0, y1 = 0, y2 = 0;
  for (let i = 0; i < x.length; i++) {
    const xi = x[i], y = b0 * xi + b1 * x1 + b2 * x2 - a1 * y1 - a2 * y2;
    x2 = x1; x1 = xi; y2 = y1; y1 = y; x[i] = y;
  }
  return x;
}
export const lr4 = (x, type, f, sr) => biquad(biquad(x, type, f, Math.SQRT1_2, 0, sr), type, f, Math.SQRT1_2, 0, sr);

// one-pole tilt: color<0 darkens (LP 20k->250 Hz), color>0 brightens (HP 20->5k Hz)
export function colorNoise(x, color, sr) {
  if (Math.abs(color) < 1e-3) return x;
  if (color < 0) {
    const fc = 20000 * Math.pow(250 / 20000, -color), a = Math.exp(-TAU * fc / sr);
    let y = 0; for (let i = 0; i < x.length; i++) { y = (1 - a) * x[i] + a * y; x[i] = y; }
  } else {
    const fc = 20 * Math.pow(5000 / 20, color), a = Math.exp(-TAU * fc / sr);
    let y = 0, xp = 0; for (let i = 0; i < x.length; i++) { y = a * (y + x[i] - xp); xp = x[i]; x[i] = y; }
  }
  return x;
}

// ADAA1 waveshaper: blend of tanh and hard clip, plus sine fold. In place.
const logcosh = (x) => { const a = Math.abs(x); return a + Math.log1p(Math.exp(-2 * a)) - Math.LN2; };
const hcF = (x) => (Math.abs(x) <= 1 ? 0.5 * x * x : Math.abs(x) - 0.5);
const hc = (x) => (x > 1 ? 1 : x < -1 ? -1 : x);
export function shape(x, driveDb, hard, fold = 0) {
  if (driveDb <= 0.01 && fold <= 0.001) return x;
  const g = dbToLin(driveDb);
  let xp = 0;
  for (let i = 0; i < x.length; i++) {
    let v = x[i] * g;
    if (fold > 0) {
      // sine fold blended in: sin(pi/2 * v * (1+3 fold)) has gain ~1 near 0
      const k = 1 + 3 * fold;
      v = (1 - fold) * v + fold * Math.sin(0.5 * Math.PI * v * k) * 1.2;
    }
    const d = v - xp;
    let yt, yh;
    if (Math.abs(d) > 1e-5) { yt = (logcosh(v) - logcosh(xp)) / d; yh = (hcF(v) - hcF(xp)) / d; }
    else { const m = 0.5 * (v + xp); yt = Math.tanh(m); yh = hc(m); }
    xp = v;
    x[i] = (1 - hard) * yt + hard * yh;
  }
  return x;
}

export function rmsOf(x, n = x.length) { let s = 0; n = Math.min(n, x.length); for (let i = 0; i < n; i++) s += x[i] * x[i]; return Math.sqrt(s / Math.max(1, n)); }
export function normRms(x, target, n) { const r = rmsOf(x, n); if (r > 1e-9) { const g = target / r; for (let i = 0; i < x.length; i++) x[i] *= g; } return x; }
export function addScaled(dst, src, g) { for (let i = 0; i < dst.length; i++) dst[i] += src[i] * g; }
export function mulEnv(x, e) { for (let i = 0; i < x.length; i++) x[i] *= e[i]; return x; }
export function whiteNoise(n, r) { const x = new Float32Array(n); for (let i = 0; i < n; i++) x[i] = r() * 2 - 1; return x; }

// ---------------------------------------------------------------- FFT / convolution
function fft(re, im, inv) {
  const n = re.length;
  for (let i = 1, j = 0; i < n; i++) {
    let bit = n >> 1; for (; j & bit; bit >>= 1) j ^= bit; j ^= bit;
    if (i < j) { let t = re[i]; re[i] = re[j]; re[j] = t; t = im[i]; im[i] = im[j]; im[j] = t; }
  }
  for (let len = 2; len <= n; len <<= 1) {
    const ang = (inv ? 2 : -2) * Math.PI / len, wr = Math.cos(ang), wi = Math.sin(ang);
    for (let i = 0; i < n; i += len) {
      let cr = 1, ci = 0;
      for (let j = 0; j < len / 2; j++) {
        const a = i + j, b = a + len / 2;
        const tr = re[b] * cr - im[b] * ci, ti = re[b] * ci + im[b] * cr;
        re[b] = re[a] - tr; im[b] = im[a] - ti; re[a] += tr; im[a] += ti;
        const t = cr * wr - ci * wi; ci = cr * wi + ci * wr; cr = t;
      }
    }
  }
  if (inv) for (let i = 0; i < n; i++) { re[i] /= n; im[i] /= n; }
}
export { fft };
// mono x convolved with stereo IR (hL, hR) via one complex FFT (IR packed as re/im)
export function convolveStereo(x, hL, hR, outLen) {
  let n = 1; while (n < x.length + hL.length) n <<= 1;
  const xr = new Float64Array(n), xi = new Float64Array(n), hr = new Float64Array(n), hi = new Float64Array(n);
  xr.set(x); hr.set(hL); hi.set(hR);
  fft(xr, xi, false); fft(hr, hi, false);
  for (let k = 0; k < n; k++) { const a = xr[k], b = xi[k], c = hr[k], d = hi[k]; xr[k] = a * c - b * d; xi[k] = a * d + b * c; }
  fft(xr, xi, true);
  return [Float32Array.from(xr.subarray(0, outLen)), Float32Array.from(xi.subarray(0, outLen))];
}

// ---------------------------------------------------------------- IR synthesis
const irCache = new Map();
export function makeIR(p, sr = SR) {
  const key = ['rev_type', 'rev_size', 'rev_char', 'rev_tone', 'rev_pre', 'seed'].map((k) => (+p[k]).toFixed(3)).join('|') + '|' + sr;
  if (irCache.has(key)) return irCache.get(key);
  const type = Math.round(p.rev_type), size = p.rev_size, ch = p.rev_char;
  const n = Math.ceil((size * 1.3 + p.rev_pre + 5) * sr / 1000), pre = Math.floor(p.rev_pre * sr / 1000);
  const out = [new Float32Array(n), new Float32Array(n)];
  for (let c = 0; c < 2; c++) {
    const r = rng((p.seed | 0) * 7919 + 17 + c * 104729), h = out[c];
    const T60 = size / 1000, dk = 6.9078 / (T60 * sr);
    if (type === 0 || type === 4) {
      // plate / room: decaying noise whose lowpass closes over time
      for (let i = pre; i < n; i++) h[i] = (r() * 2 - 1) * Math.exp(-(i - pre) * dk);
      let y = 0;
      for (let i = pre; i < n; i++) {
        const t = (i - pre) / (n - pre), fc = p.rev_tone * Math.pow(0.12 + 0.88 * (1 - ch), t * 3);
        const a = Math.exp(-TAU * Math.min(fc, sr * 0.45) / sr); y = (1 - a) * h[i] + a * y; h[i] = y;
      }
      if (type === 4) { // early reflections
        const taps = 6 + Math.floor(ch * 18);
        for (let k = 0; k < taps; k++) { const at = pre + Math.floor(r() * 0.035 * sr); if (at < n) h[at] += (r() < 0.5 ? -1 : 1) * (0.6 + 0.6 * r()) * (1 - k / taps); }
      }
    } else if (type === 1) {
      // metal: dense inharmonic decaying partials
      const np = 24 + Math.floor(ch * 60), lo = 250, hi = Math.min(p.rev_tone, 16000);
      for (let k = 0; k < np; k++) {
        const f = lo * Math.pow(hi / lo, Math.pow(r(), 0.8)), ph = r() * TAU, dd = dk * (0.4 + 1.6 * r()) * (f / 2000) ** 0.3;
        const amp = 1 / Math.sqrt(np), w = TAU * f / sr;
        for (let i = pre; i < n; i++) h[i] += amp * Math.sin(w * (i - pre) + ph) * Math.exp(-(i - pre) * dd);
      }
      for (let i = pre; i < n; i++) h[i] += 0.15 * (r() * 2 - 1) * Math.exp(-(i - pre) * dk * 3);
    } else if (type === 2) {
      // whip: downward exponential chirp (dispersion) + faint tail
      const fhi = Math.min(p.rev_tone * 1.2, 18000), flo = 60 + 400 * (1 - ch), Tc = Math.max(0.01, T60 * (0.25 + 0.6 * ch));
      const nc = Math.floor(Tc * sr); let ph = c * 0.7;
      for (let i = 0; i < nc && pre + i < n; i++) {
        const t = i / sr, f = fhi * Math.pow(flo / fhi, t / Tc); ph += TAU * f / sr;
        h[pre + i] += Math.sin(ph) * Math.pow(1 - i / nc, 1.5) * Math.sqrt(f / fhi + 0.05);
      }
      for (let i = pre; i < n; i++) h[i] += 0.08 * (r() * 2 - 1) * Math.exp(-(i - pre) * dk);
    } else if (type === 3) {
      // spring: repeating dispersive chirps
      const sp = Math.max(0.004, (0.012 + 0.05 * ch) * (1 + 0.03 * c)), nc = Math.floor(0.018 * sr);
      for (let e = 0; pre + e * sp * sr < n; e++) {
        const st = pre + Math.floor(e * sp * sr), g = Math.exp(-(st - pre) * dk) * (e % 2 ? -1 : 1);
        let ph = 0;
        for (let i = 0; i < nc && st + i < n; i++) { const f = 5000 * Math.pow(150 / 5000, i / nc); ph += TAU * f / sr; h[st + i] += g * Math.sin(ph) * (1 - i / nc); }
      }
      for (let i = pre; i < n; i++) h[i] += 0.1 * (r() * 2 - 1) * Math.exp(-(i - pre) * dk);
    }
  }
  // unit energy (per channel average)
  const e = Math.sqrt((rmsOf(out[0]) ** 2 + rmsOf(out[1]) ** 2) / 2 * n) || 1;
  for (const h of out) for (let i = 0; i < n; i++) h[i] /= e;
  if (irCache.size > 64) irCache.delete(irCache.keys().next().value);
  irCache.set(key, out);
  return out;
}

// ---------------------------------------------------------------- layers
function renderSub(p, n, sr, r) {
  const fe = midiHz(p.note + p.sub_tune), D = p.sub_dive / 12, tau = p.sub_tau / 1000, k = p.sub_k, C = p.sub_click / 12, tc = p.sub_click_ms / 1000;
  const x = new Float32Array(n); let ph = 0;
  for (let i = 0; i < n; i++) {
    const t = i / sr, f = fe * Math.pow(2, D * Math.exp(-Math.pow(t / tau, k)) + C * Math.exp(-t / tc));
    ph += TAU * Math.min(f, sr * 0.45) / sr; x[i] = Math.sin(ph);
  }
  if (p.sub_noise > -59) {
    // noise crunched together with the sine through the same drive
    const nz = whiteNoise(n, r); biquad(nz, 'hp', p.sub_noise_hp, 0.707, 0, sr); normRms(nz, 0.7071);
    const g = dbToLin(p.sub_noise); for (let i = 0; i < n; i++) x[i] += g * nz[i];
  }
  mulEnv(x, envelope(n, sr, p.sub_att, p.sub_dec, p.sub_hold, p.sub_rel, Math.round(p.sub_shape)));
  shape(x, p.sub_drive, p.sub_hard);
  normRms(x, 0.25, activeLen(sr, p.sub_att, p.sub_hold, p.sub_rel));
  return x;
}

// polyBLEP residual for a discontinuity at phase 0 (t in [0,1), dt = phase increment)
const blep = (t, dt) => (t < dt ? ((t /= dt), t + t - t * t - 1) : t > 1 - dt ? ((t = (t - 1) / dt), t * t + t + t + 1) : 0);
function renderTr(p, n, sr, r) {
  const x = colorNoise(whiteNoise(n, r), p.tr_color, sr);
  normRms(x, 0.25);
  if (p.tr_click > -59) {
    const g = dbToLin(p.tr_click), tau = p.tr_click_tau / 1000; let ph = 0;
    for (let i = 0; i < n; i++) {
      const t = i / sr, f = 40 + (p.tr_click_f0 - 40) * Math.exp(-t / tau);
      ph += TAU * f / sr; x[i] += g * 0.35 * Math.sin(ph) * Math.exp(-t / (tau * 4));
    }
  }
  if (p.tr_zap > -59) { // harmonic (saw) sweep, exponential from zap-from to zap-to
    const g = dbToLin(p.tr_zap) * 0.5, l0 = Math.log(p.tr_zap_f0), l1 = Math.log(p.tr_zap_f1), zt = p.tr_zap_ms / 1000; let ph = 0;
    for (let i = 0; i < n; i++) {
      const f = Math.exp(l1 + (l0 - l1) * Math.exp(-(i / sr) / zt)), dt = Math.min(f, sr * 0.45) / sr;
      ph += dt; if (ph >= 1) ph -= 1;
      x[i] += g * (2 * ph - 1 - blep(ph, dt));
    }
  }
  biquad(x, 'hp', p.tr_hp, 0.707, 0, sr); biquad(x, 'hp', p.tr_hp, 0.707, 0, sr);
  biquad(x, 'lp', p.tr_lp, 0.707, 0, sr);
  if (p.tr_close < 19900) { // lowpass closing from the lowpass setting to close-to: highs end first
    const l0 = Math.log(p.tr_lp), l1 = Math.log(Math.min(p.tr_close, p.tr_lp)), ct = p.tr_close_ms / 1000, k = 1.2;
    let ic1 = 0, ic2 = 0, a1 = 0, a2 = 0, a3 = 0;
    for (let i = 0; i < n; i++) {
      if ((i & 15) === 0) { const f = Math.exp(l1 + (l0 - l1) * Math.exp(-(i / sr) / ct)); const gg = Math.tan(Math.PI * Math.min(f, sr * 0.45) / sr); a1 = 1 / (1 + gg * (gg + k)); a2 = gg * a1; a3 = gg * a2; }
      const v3 = x[i] - ic2, v1 = a1 * ic1 + a2 * v3, v2 = ic2 + a2 * ic1 + a3 * v3; ic1 = 2 * v1 - ic1; ic2 = 2 * v2 - ic2; x[i] = v2;
    }
  }
  biquad(x, 'peak', p.tr_pk_f, p.tr_pk_q, p.tr_pk_g, sr);
  { // envelope, optionally retriggered (stutter) with each repeat a little quieter
    const e = envelope(n, sr, p.tr_att, p.tr_dec, p.tr_hold, p.tr_rel, Math.round(p.tr_shape));
    const S = Math.round(p.tr_stutter);
    if (S > 1) {
      const gap = Math.floor(p.tr_stut_ms * sr / 1000), sum = new Float32Array(n);
      for (let k = 0; k < S; k++) { const g = Math.pow(0.8, k), o = k * gap; for (let i = 0; i + o < n; i++) sum[i + o] = Math.max(sum[i + o], g * e[i] * (k < S - 1 && i > gap ? 0 : 1)); }
      mulEnv(x, sum);
    } else mulEnv(x, e);
  }
  shape(x, p.tr_drive, 0.4);
  biquad(x, 'hp', p.tr_hp * 0.7, 0.707, 0, sr);
  normRms(x, 0.25, activeLen(sr, p.tr_att, p.tr_hold, p.tr_rel));
  return x;
}

// time-varying multiplier for laser drop
function dropMul(p, t) { return p.body_drop > 0.01 ? Math.pow(2, (p.body_drop / 12) * Math.exp(-t / (p.body_drop_tau / 1000))) : 1; }

// key tracking: snap a frequency to the nearest harmonic of the note
function keyed(p, f) { if (!Math.round(p.body_key || 0)) return f; const h = midiHz(p.note); return h * Math.max(1, Math.round(f / h)); }
// vowel formants (F1, F2, F3), a e i o u
const VOWELS = [[800, 1150, 2900], [400, 1600, 2700], [300, 2250, 3000], [450, 800, 2830], [325, 700, 2530]];
function formant(x, pos, mix, sr) {
  if (mix < 0.001) return x;
  const i = Math.min(3, Math.floor(pos)), t = Math.min(1, pos - i), fs = VOWELS[i].map((f, k) => f * Math.pow(VOWELS[i + 1][k] / f, t));
  const y = new Float32Array(x.length);
  fs.forEach((f, k) => { const b = biquad(x.slice(), 'bp', f, 7, 0, sr); const g = [1, 0.7, 0.45][k]; for (let j = 0; j < y.length; j++) y[j] += g * b[j]; });
  const rx = rmsOf(x), ry = rmsOf(y) || 1;
  for (let j = 0; j < x.length; j++) x[j] = (1 - mix) * x[j] + mix * y[j] * (rx / ry);
  return x;
}
function renderBodyChannel(p, n, sr, r, modeRatios) {
  const combF = keyed(p, p.body_comb_f);
  const exc = colorNoise(whiteNoise(n, r), p.body_color, sr);
  normRms(exc, 0.25);
  const out = new Float32Array(n);
  if (p.body_noise > -59) addScaled(out, exc, dbToLin(p.body_noise));
  // modal bank (2-pole resonators with time-varying frequency)
  if (p.body_modes > -59) {
    const m = new Float32Array(n), Q = p.body_q;
    for (const [ratio, amp] of modeRatios) {
      let y1 = 0, y2 = 0;
      const f0 = keyed(p, p.body_mode_f) * ratio;
      let c1 = 0, c2 = 0, gn = 0;
      for (let i = 0; i < n; i++) {
        if ((i & 15) === 0) {
          const f = Math.min(f0 * dropMul(p, i / sr), sr * 0.45), R = Math.exp(-Math.PI * (f / Q) / sr);
          c1 = 2 * R * Math.cos(TAU * f / sr); c2 = -R * R; gn = (1 - R * R) * 0.5;
        }
        const y = gn * exc[i] + c1 * y1 + c2 * y2; y2 = y1; y1 = y; m[i] += amp * y;
      }
    }
    normRms(m, 0.25); addScaled(out, m, dbToLin(p.body_modes));
  }
  // comb (feedback delay, cubic interp, damped, soft-limited loop)
  if (p.body_comb > -59) {
    const c = new Float32Array(n), N = 1 << 14, buf = new Float32Array(N), damp = p.body_comb_damp * 0.9;
    let w = 0, lp = 0;
    for (let i = 0; i < n; i++) {
      const f = Math.min(combF * dropMul(p, i / sr), sr * 0.4), d = Math.min(N - 4, Math.max(2, sr / f));
      const rp = w - d + N * 2, i0 = Math.floor(rp), fr = rp - i0;
      const xm1 = buf[(i0 - 1) & (N - 1)], x0 = buf[i0 & (N - 1)], x1 = buf[(i0 + 1) & (N - 1)], x2 = buf[(i0 + 2) & (N - 1)];
      const del = x0 + 0.5 * fr * (x1 - xm1 + fr * (2 * xm1 - 5 * x0 + 4 * x1 - x2 + fr * (3 * (x0 - x1) + x2 - xm1)));
      lp = (1 - damp) * del + damp * lp;
      const y = exc[i] * 0.3 + p.body_comb_fb * Math.tanh(lp);
      buf[w & (N - 1)] = y; w++; c[i] = y;
    }
    normRms(c, 0.25); addScaled(out, c, dbToLin(p.body_comb));
  }
  // FM tone
  if (p.body_fm > -59) {
    const fm = new Float32Array(n), base = midiHz(p.note) * p.body_fm_ratio; let pc = 0, pm = 0;
    for (let i = 0; i < n; i++) {
      const t = i / sr, fc = Math.min(base * dropMul(p, t), sr * 0.45);
      pc += TAU * fc / sr; pm += TAU * fc * p.body_fm_mod / sr;
      fm[i] = Math.sin(pc + p.body_fm_idx * Math.exp(-t / (p.body_fm_tau / 1000)) * Math.sin(pm));
    }
    normRms(fm, 0.25); addScaled(out, fm, dbToLin(p.body_fm));
  }
  // open-close SVF (Cytomic TPT), blended with dry
  if (p.body_flt_mix > 0.001) {
    const k = 2 - 2 * p.body_flt_res, type = Math.round(p.body_flt_type), mix = p.body_flt_mix;
    let ic1 = 0, ic2 = 0, a1 = 0, a2 = 0, a3 = 0;
    const lf0 = Math.log(p.body_flt_f0), lf1 = Math.log(p.body_flt_f1), ft = p.body_flt_tau / 1000;
    for (let i = 0; i < n; i++) {
      if ((i & 15) === 0) {
        const f = Math.exp(lf1 + (lf0 - lf1) * Math.exp(-(i / sr) / ft));
        const g = Math.tan(Math.PI * Math.min(f, sr * 0.45) / sr); a1 = 1 / (1 + g * (g + k)); a2 = g * a1; a3 = g * a2;
      }
      const v0 = out[i], v3 = v0 - ic2, v1 = a1 * ic1 + a2 * v3, v2 = ic2 + a2 * ic1 + a3 * v3;
      ic1 = 2 * v1 - ic1; ic2 = 2 * v2 - ic2;
      const hp = v0 - k * v1 - v2;
      const y = type === 0 ? v2 : type === 1 ? v1 * k : type === 2 ? hp : v2 + hp;
      out[i] = (1 - mix) * v0 + mix * y;
    }
  }
  formant(out, p.body_vowel || 0, p.body_vowel_mix || 0, sr);
  mulEnv(out, envelope(n, sr, p.body_att, p.body_dec, p.body_hold, p.body_rel, Math.round(p.body_shape)));
  normRms(out, 0.25, activeLen(sr, p.body_att, p.body_hold, p.body_rel));
  shape(out, p.body_drive, 0.3, p.body_fold);
  biquad(out, 'hp', p.body_hp, 0.707, 0, sr); biquad(out, 'hp', p.body_hp, 0.707, 0, sr);
  return out;
}

// Partial ratio tables: membrane / free bar / clamped plate / cymbal / bell (re prime) / TR-808 metal oscillators.
const MODE_SETS = [null,
  [1, 1.593, 2.136, 2.295, 2.653, 2.917, 3.155, 3.501, 3.598, 3.647, 4.06, 4.23],
  [1, 2.757, 5.404, 8.933, 13.344, 18.64],
  [1, 2.081, 3.414, 3.893, 5.2, 6.1],
  [1, 3.3, 8.9, 12.6],
  [0.5, 1, 1.183, 1.506, 2, 2.514, 3.011, 4.166],
  [1, 1.483, 1.8, 2.546, 2.63, 3.897],
];
function modeTable(p, r) {
  const set = MODE_SETS[Math.round(p.body_mode_set || 0)];
  const B = 0.6 * p.body_metal * p.body_metal, jit = 0.35 * p.body_metal, out = [];
  for (let i = 1; i <= p.body_nmodes; i++) {
    const base = !set ? i * Math.sqrt(1 + B * i * i) : i <= set.length ? set[i - 1] : set[set.length - 1] * Math.pow(i / set.length, 1.5);
    const ratio = base * (1 + (set ? 0.5 : 1) * jit * (r() - 0.5) * (i > 1 ? 1 : 0));
    out.push([ratio, Math.pow(10, (p.body_mode_tilt * Math.log2(ratio)) / 20)]);
  }
  return out;
}

function renderBody(p, n, sr, seed) {
  const modes = modeTable(p, rng(12345));
  const mid = renderBodyChannel(p, n, sr, rng(seed * 101 + 1), modes);
  if (p.body_width < 0.01) return [mid, mid.slice()];
  const side = renderBodyChannel(p, n, sr, rng(seed * 101 + 2), modes);
  const L = new Float32Array(n), R = new Float32Array(n), w = p.body_width;
  const gm = 1 / Math.sqrt(1 + w * w);
  for (let i = 0; i < n; i++) { L[i] = gm * (mid[i] + w * side[i]); R[i] = gm * (mid[i] - w * side[i]); }
  return [L, R];
}

function synthVoices(p, n, sr, r, side) {
  const wave = Math.round(p.syn_wave), V = wave === 2 ? Math.max(3, Math.round(p.syn_voices)) : Math.round(p.syn_voices);
  const root = midiHz(p.note + 12 * Math.round(p.syn_oct) + p.syn_tune), D = p.syn_pitch / 12, pt = p.syn_pitch_ms / 1000;
  const x = new Float32Array(n);
  const stacks = [[root, 1]]; if ((p.syn_int_mix || 0) > 0.001) stacks.push([root * Math.pow(2, Math.round(p.syn_int) / 12), p.syn_int_mix]);
  for (const [base, sg] of stacks) for (let v = 0; v < V; v++) {
    const spread = V > 1 ? (v / (V - 1)) * 2 - 1 : 0, cents = spread * p.syn_detune * 50 * (side ? -1 : 1);
    const mul = Math.pow(2, cents / 1200), amp = sg * (V > 1 && wave === 2 ? (v === (V >> 1) ? 1 : 0.7) : 1) / Math.sqrt(V);
    let ph = r(), ph2 = 0;
    for (let i = 0; i < n; i++) {
      const f = Math.min(base * mul * Math.pow(2, D * Math.exp(-i / sr / pt)), sr * 0.45), dt = f / sr;
      ph += dt; if (ph >= 1) ph -= 1;
      let y;
      if (wave === 0 || wave === 2) y = 2 * ph - 1 - blep(ph, dt);
      else { const pw = wave === 1 ? 0.5 : 0.18; y = (ph < pw ? 1 : -1) + blep(ph, dt) - blep((ph2 = ph - pw + (ph < pw ? 1 : 0)), dt); }
      x[i] += amp * y;
    }
  }
  // filter sweep (SVF lowpass)
  const k = 2 - 2 * p.syn_flt_res, l0 = Math.log(p.syn_flt_f0), l1 = Math.log(p.syn_flt_f1), ft = p.syn_flt_tau / 1000;
  let ic1 = 0, ic2 = 0, a1 = 0, a2 = 0, a3 = 0;
  for (let i = 0; i < n; i++) {
    if ((i & 15) === 0) { const f = Math.exp(l1 + (l0 - l1) * Math.exp(-(i / sr) / ft)); const g = Math.tan(Math.PI * Math.min(f, sr * 0.45) / sr); a1 = 1 / (1 + g * (g + k)); a2 = g * a1; a3 = g * a2; }
    const v3 = x[i] - ic2, v1 = a1 * ic1 + a2 * v3, v2 = ic2 + a2 * ic1 + a3 * v3; ic1 = 2 * v1 - ic1; ic2 = 2 * v2 - ic2; x[i] = v2;
  }
  formant(x, p.syn_vowel || 0, p.syn_vowel_mix || 0, sr);
  mulEnv(x, envelope(n, sr, p.syn_att, p.syn_dec, p.syn_hold, p.syn_rel, Math.round(p.syn_shape)));
  normRms(x, 0.25, activeLen(sr, p.syn_att, p.syn_hold, p.syn_rel));
  shape(x, p.syn_drive, 0.4);
  biquad(x, 'hp', 120, 0.707, 0, sr);
  return x;
}
function renderSynth(p, n, sr, seed) {
  const a = synthVoices(p, n, sr, rng(seed * 23 + 1), false);
  if (p.syn_width < 0.01) return [a, a.slice()];
  const b = synthVoices(p, n, sr, rng(seed * 23 + 2), true), w = p.syn_width, L = new Float32Array(n), R = new Float32Array(n);
  for (let i = 0; i < n; i++) { const m = 0.5 * (a[i] + b[i]), d = 0.5 * (a[i] - b[i]); L[i] = m + w * d; R[i] = m - w * d; }
  return [L, R];
}

function renderSpice(p, n, sr, r) {
  const x = new Float32Array(n), type = Math.round(p.spice_type), len = Math.floor(p.spice_len * sr / 1000);
  if (type === 0) { // comb ping
    const d = Math.max(2, sr / p.spice_f), fb = Math.pow(0.001, d / Math.max(1, len));
    const N = 1 << 14, buf = new Float32Array(N); let w = 0;
    for (let i = 0; i < n; i++) {
      const rp = w - d + N * 2, i0 = Math.floor(rp), fr = rp - i0;
      const del = buf[i0 & (N - 1)] * (1 - fr) + buf[(i0 + 1) & (N - 1)] * fr;
      const y = (i < 48 ? r() * 2 - 1 : 0) + fb * del; buf[w & (N - 1)] = y; w++; x[i] = y;
    }
  } else if (type === 1) { // random sine bleeps
    const count = Math.max(1, Math.round(p.spice_density * p.spice_len / 1000));
    for (let b = 0; b < count; b++) {
      const st = Math.floor(Math.pow(r(), 1.8) * len * 0.6), bl = Math.floor((0.004 + 0.03 * r()) * sr), f = p.spice_f * Math.pow(2, (r() - 0.5) * 3);
      const amp = 0.4 + 0.6 * r();
      for (let i = 0; i < bl && st + i < n; i++) x[st + i] += amp * Math.sin(TAU * f * i / sr) * Math.sin(Math.PI * i / bl);
    }
  } else if (type === 2) { // crackle
    const prob = p.spice_density * 4 / sr;
    for (let i = 0; i < len && i < n; i++) if (r() < prob) x[i] = (r() * 2 - 1) * (1 - i / len);
    biquad(x, 'bp', p.spice_f, 0.8, 0, sr);
  } else if (type === 4) { // TR-808 metal: six squares -> two bandpasses -> highpass, scaled by freq
    const sc = p.spice_f / 3440, fr = [205.3, 304.4, 369.6, 522.7, 540, 800].map((f) => f * sc);
    for (let i = 0; i < n; i++) { let s = 0; for (const f of fr) s += ((i * f / sr) % 1) < 0.5 ? 1 : -1; x[i] = s / 6; }
    const a = x.slice(), b = x.slice();
    biquad(a, 'bp', 3440 * sc, 3, 0, sr); biquad(b, 'bp', 7100 * sc, 3, 0, sr);
    const dk = 6.9 / Math.max(1, len);
    for (let i = 0; i < n; i++) x[i] = (a[i] + 0.7 * b[i]) * Math.exp(-i * dk);
  } else { // metal hit
    const ex = new Float32Array(n); for (let i = 0; i < 96; i++) ex[i] = r() * 2 - 1;
    for (let k = 1; k <= 8; k++) {
      const f = Math.min(p.spice_f * k * Math.sqrt(1 + 0.3 * k * k) * (1 + 0.2 * (r() - 0.5)), sr * 0.45);
      const R = Math.pow(0.001, 1 / Math.max(1, len * (1.2 - k / 10)));
      const c1 = 2 * R * Math.cos(TAU * f / sr), c2 = -R * R; let y1 = 0, y2 = 0;
      for (let i = 0; i < n; i++) { const y = (1 - R) * ex[i] + c1 * y1 + c2 * y2; y2 = y1; y1 = y; x[i] += y / k; }
    }
  }
  // every spice type lives inside its window and fades across it
  for (let i = 0; i < n; i++) x[i] *= i < len ? Math.pow(1 - i / len, 1.5) : 0;
  biquad(x, 'hp', p.spice_hp, 0.707, 0, sr);
  normRms(x, 0.25, Math.max(len, 480));
  return x;
}

// ---------------------------------------------------------------- crunch chain
function disperse(x, n, f, q, sr) { for (let k = 0; k < n; k++) biquad(x, 'ap', f, q, 0, sr); return x; }
// single-sideband frequency shift via FFT analytic signal
function freqShift(x, df, sr) {
  let n = 1; while (n < x.length) n <<= 1;
  const re = new Float64Array(n), im = new Float64Array(n); re.set(x);
  fft(re, im, false);
  for (let k = 1; k < n / 2; k++) { re[k] *= 2; im[k] *= 2; }
  for (let k = n / 2 + 1; k < n; k++) { re[k] = 0; im[k] = 0; }
  fft(re, im, true);
  const out = new Float32Array(x.length), w = TAU * df / sr;
  for (let i = 0; i < x.length; i++) out[i] = re[i] * Math.cos(w * i) - im[i] * Math.sin(w * i);
  return out;
}
function peakNorm(L, R, target) {
  let pk = 0; for (let i = 0; i < L.length; i++) pk = Math.max(pk, Math.abs(L[i]), Math.abs(R[i]));
  if (pk > 1e-9) { const g = target / pk; for (let i = 0; i < L.length; i++) { L[i] *= g; R[i] *= g; } }
}
export function crunch(L, R, p, sr) {
  if (p.cr_mix < 0.001) return;
  const dL = L.slice(), dR = R.slice();
  const rmsIn = Math.sqrt((rmsOf(L) ** 2 + rmsOf(R) ** 2) / 2);
  for (const x of [L, R]) {
    lr4(x, 'hp', p.cr_hp, sr);
    if (p.cr_mud < -0.05) biquad(x, 'peak', p.cr_hp * 1.2, 1.0, p.cr_mud, sr);
    if (p.cr_pk_g > 0.05) biquad(x, 'peak', p.cr_pk_f, 1.4, p.cr_pk_g, sr);
  }
  peakNorm(L, R, 0.7);
  shape(L, p.cr_drive1, p.cr_hard1); shape(R, p.cr_drive1, p.cr_hard1);
  if (p.cr_disp_n > 0) { disperse(L, p.cr_disp_n, p.cr_disp_f, p.cr_disp_q, sr); disperse(R, p.cr_disp_n, p.cr_disp_f, p.cr_disp_q, sr); }
  if (p.cr_conv > 0.001) {
    const [hL, hR] = makeIR({ rev_type: p.cr_conv_type, rev_size: p.cr_conv_len, rev_char: p.cr_conv_char, rev_tone: 16000, rev_pre: 0, seed: 3 }, sr);
    const mono = new Float32Array(L.length); for (let i = 0; i < L.length; i++) mono[i] = 0.5 * (L[i] + R[i]);
    const [cL, cR] = convolveStereo(mono, hL, hR, L.length);
    const gc = Math.sqrt((rmsOf(L) ** 2 + rmsOf(R) ** 2) / 2) / (Math.sqrt((rmsOf(cL) ** 2 + rmsOf(cR) ** 2) / 2) || 1);
    for (let i = 0; i < L.length; i++) { L[i] = (1 - p.cr_conv) * L[i] + p.cr_conv * gc * cL[i]; R[i] = (1 - p.cr_conv) * R[i] + p.cr_conv * gc * cR[i]; }
  }
  peakNorm(L, R, 0.7);
  shape(L, p.cr_drive2, 0.5); shape(R, p.cr_drive2, 0.5);
  for (let k = 0; k < p.cr_rack; k++) {
    ottCore(L, R, p.cr_rack_depth, 0.8, 2.5, sr);
    peakNorm(L, R, 1.4); shape(L, 0, 0.3); shape(R, 0, 0.3);
    for (const x of [L, R]) for (let i = 0; i < x.length; i++) x[i] = Math.tanh(x[i]);
  }
  if (p.cr_pp > 0.001) {
    const dl = Math.floor(p.cr_pp_ms * sr / 1000), dr = dl + Math.floor(0.003 * sr), N = 1 << 13, bl = new Float32Array(N), br = new Float32Array(N);
    for (let i = 0; i < L.length; i++) {
      const yl = bl[(i - dl + N * 4) & (N - 1)], yr = br[(i - dr + N * 4) & (N - 1)];
      bl[i & (N - 1)] = L[i] + p.cr_pp_fb * Math.tanh(yr); br[i & (N - 1)] = R[i] + p.cr_pp_fb * Math.tanh(yl);
      L[i] += p.cr_pp * yl; R[i] += p.cr_pp * yr;
    }
  }
  if (p.cr_fs_wet > 0.001 && Math.abs(p.cr_fs) > 0.05) {
    const sL = freqShift(L, p.cr_fs, sr), sR = freqShift(R, p.cr_fs, sr);
    for (let i = 0; i < L.length; i++) { L[i] = (1 - p.cr_fs_wet) * L[i] + p.cr_fs_wet * sL[i]; R[i] = (1 - p.cr_fs_wet) * R[i] + p.cr_fs_wet * sR[i]; }
  }
  for (const x of [L, R]) lr4(x, 'hp', p.cr_hp * 0.8, sr);
  // keep the tops' loudness relationship with the sub: output RMS = input RMS
  const rmsOut = Math.sqrt((rmsOf(L) ** 2 + rmsOf(R) ** 2) / 2), g = rmsOut > 1e-9 ? rmsIn / rmsOut : 0;
  for (let i = 0; i < L.length; i++) { L[i] = (1 - p.cr_mix) * dL[i] + p.cr_mix * g * L[i]; R[i] = (1 - p.cr_mix) * dR[i] + p.cr_mix * g * R[i]; }
}

// ---------------------------------------------------------------- samples
const SAMPLES = {};
// store a dropped file (any rate) resampled to the engine rate
export function setSample(slot, L, R, rate, sr = SR) {
  if (!L) { delete SAMPLES[slot]; return; }
  const ratio = rate / sr, n = Math.floor(L.length / ratio), a = new Float32Array(n), b = new Float32Array(n);
  for (let i = 0; i < n; i++) { const t = i * ratio, j = Math.floor(t), f = t - j; a[i] = L[j] + f * ((L[j + 1] ?? L[j]) - L[j]); b[i] = R[j] + f * ((R[j + 1] ?? R[j]) - R[j]); }
  SAMPLES[slot] = [a, b];
}
export const hasSample = (slot) => !!SAMPLES[slot];
export function renderSample(p, k, n, sr) {
  const src = SAMPLES[k]; if (!src) return null;
  const st = p[`s${k}_pitch`] + (Math.round(p[`s${k}_key`]) ? p.note - 24 : 0), rate = Math.pow(2, st / 12);
  const o0 = p[`s${k}_start`] * sr / 1000, len = Math.floor(p[`s${k}_len`] * sr / 1000), rel = Math.max(1, p[`s${k}_rel`] * sr / 1000);
  const out = [new Float32Array(n), new Float32Array(n)];
  for (let c = 0; c < 2; c++) {
    const s = src[c], y = out[c];
    for (let i = 0; i < n; i++) {
      const t = o0 + i * rate, j = Math.floor(t); if (j + 1 >= s.length) break;
      const g = i < len ? 1 : i < len + rel ? 0.5 + 0.5 * Math.cos(Math.PI * (i - len) / rel) : 0; if (!g) break;
      y[i] = g * (s[j] + (t - j) * (s[j + 1] - s[j]));
    }
    if (p[`s${k}_hp`] > 21) { biquad(y, 'hp', p[`s${k}_hp`], 0.707, 0, sr); biquad(y, 'hp', p[`s${k}_hp`], 0.707, 0, sr); }
    if (p[`s${k}_lp`] < 19900) biquad(y, 'lp', p[`s${k}_lp`], 0.707, 0, sr);
    if (p[`s${k}_drive`] > 0.05) shape(y, p[`s${k}_drive`], 0.4);
  }
  const g = dbToLin(p[`s${k}_lvl`]);
  for (const y of out) for (let i = 0; i < n; i++) y[i] *= g;
  return out;
}

// ---------------------------------------------------------------- master
export function envFollow(x, sr, attMs, relMs) {
  const a = Math.exp(-1 / (attMs * sr / 1000)), r = Math.exp(-1 / (relMs * sr / 1000)), e = new Float32Array(x.length);
  let v = 0; for (let i = 0; i < x.length; i++) { const s = Math.abs(x[i]); v = s > v ? a * v + (1 - a) * s : r * v + (1 - r) * s; e[i] = v; }
  return e;
}
function ott(L, R, p, sr) { ottCore(L, R, p.mst_ott, p.mst_ott_up, p.mst_ott_time, sr); }
export function ottCore(L, R, depth, up, tm, sr) {
  if (depth < 0.001) return;
  const n = L.length, split = (x) => {
    const lo = lr4(x.slice(), 'lp', 120, sr), rest = lr4(x.slice(), 'hp', 120, sr);
    // allpass-compensate the low band for the 2.5k crossover
    const loA = lr4(lo.slice(), 'lp', 2500, sr), loB = lr4(lo, 'hp', 2500, sr); for (let i = 0; i < n; i++) loA[i] += loB[i];
    const mid = lr4(rest.slice(), 'lp', 2500, sr), hi = lr4(rest, 'hp', 2500, sr);
    return [loA, mid, hi];
  };
  const bl = split(L), br = split(R);
  const cfg = [[0, 12 * tm, 120 * tm], [1, 4 * tm, 70 * tm], [2, 2 * tm, 40 * tm]];
  L.fill(0); R.fill(0);
  for (const [b, at, rl] of cfg) {
    const sum = new Float32Array(n); for (let i = 0; i < n; i++) sum[i] = Math.max(Math.abs(bl[b][i]), Math.abs(br[b][i]));
    const e = envFollow(sum, sr, at, rl);
    for (let i = 0; i < n; i++) {
      const lv = 20 * Math.log10(e[i] + 1e-9);
      let gdb = 0;
      if (lv > -18) gdb -= (lv + 18) * (1 - 1 / 8);
      if (lv < -30 && lv > -75) gdb += Math.min(24, (-30 - lv) * (1 - 1 / 3)) * up;
      const g = Math.pow(10, (depth * gdb) / 20);
      L[i] += bl[b][i] * g; R[i] += br[b][i] * g;
    }
  }
}
const rms2 = (L, R) => Math.sqrt((rmsOf(L) ** 2 + rmsOf(R) ** 2) / 2);
// Mids chain: OTT -> drive/clip. Output loudness matches input, so layer levels keep their meaning.
// transient shaper: boost where a fast envelope runs ahead of a slow one
export function snap(L, R, amtDb, sr) {
  if (amtDb < 0.05) return;
  const n = L.length, m = new Float32Array(n); for (let i = 0; i < n; i++) m[i] = 0.5 * (Math.abs(L[i]) + Math.abs(R[i]));
  const fast = envFollow(m, sr, 0.3, 25), slow = envFollow(m, sr, 12, 90);
  for (let i = 0; i < n; i++) {
    const d = 20 * Math.log10((fast[i] + 1e-9) / (slow[i] + 1e-9));
    const g = Math.pow(10, (Math.max(0, Math.min(1, d / 12)) * amtDb) / 20);
    L[i] *= g; R[i] *= g;
  }
}
function midsChain(L, R, p, sr, fixed) {
  const r0 = rms2(L, R); if (r0 < 1e-9) return fixed || { pre: 1, g: 1 };
  ott(L, R, p, sr);
  snap(L, R, p.mst_snap || 0, sr);
  let pk = 0; for (let i = 0; i < L.length; i++) pk = Math.max(pk, Math.abs(L[i]), Math.abs(R[i]));
  const pre = fixed ? fixed.pre : pk > 0 ? 0.7 / pk : 1; for (let i = 0; i < L.length; i++) { L[i] *= pre; R[i] *= pre; }
  shape(L, p.mst_drive, p.mst_hard); shape(R, p.mst_drive, p.mst_hard);
  const g = fixed ? fixed.g : r0 / (rms2(L, R) || 1); for (let i = 0; i < L.length; i++) { L[i] *= g; R[i] *= g; }
  return { pre, g };
}
// Final: tone shelves, mono lows, light soft ceiling, peak at -0.3 dBFS.
export function finalStage(L, R, p, sr, fixed) {
  for (const x of [L, R]) {
    if (Math.abs(p.mst_low) > 0.05) biquad(x, 'ls', 90, 0.7, p.mst_low, sr);
    if (Math.abs(p.mst_mid) > 0.05) biquad(x, 'peak', 280, 0.9, p.mst_mid, sr);
    if (Math.abs(p.mst_high) > 0.05) biquad(x, 'hs', 3000, 0.7, p.mst_high, sr);
  }
  const lowL = lr4(L.slice(), 'lp', p.mst_mono, sr), lowR = lr4(R.slice(), 'lp', p.mst_mono, sr);
  const hiL = lr4(L, 'hp', p.mst_mono, sr), hiR = lr4(R, 'hp', p.mst_mono, sr);
  for (let i = 0; i < L.length; i++) { const m = 0.5 * (lowL[i] + lowR[i]); L[i] = hiL[i] + m; R[i] = hiR[i] + m; }
  let pk = 0; for (let i = 0; i < L.length; i++) pk = Math.max(pk, Math.abs(L[i]), Math.abs(R[i]));
  const pre = fixed ? fixed.pre : pk > 0 ? 1 / pk : 1; for (let i = 0; i < L.length; i++) { L[i] *= pre; R[i] *= pre; }
  shape(L, 3, 0.2); shape(R, 3, 0.2);
  pk = 0; for (let i = 0; i < L.length; i++) pk = Math.max(pk, Math.abs(L[i]), Math.abs(R[i]));
  const g = fixed ? fixed.g : pk > 0 ? 0.966 / pk : 1; for (let i = 0; i < L.length; i++) { L[i] *= g; R[i] *= g; }
  return { pre, g };
}

// ---------------------------------------------------------------- shot / burst
// Returns per-layer stereo buffers (post layer level, pre master).
export function renderShot(p, n, sr = SR) {
  const seed = p.seed | 0, layers = {};
  const sub = p.sub_lvl > -59 ? renderSub(p, n, sr, rng(seed * 19 + 11)) : null;
  layers.sub = sub ? [sub, sub] : null;
  const tr = p.tr_lvl > -59 ? renderTr(p, n, sr, rng(seed * 13 + 3)) : null;
  layers.tr = tr ? [tr, tr] : null;
  layers.body = p.body_lvl > -59 ? renderBody(p, n, sr, seed) : null;
  const sp = p.spice_lvl > -59 ? renderSpice(p, n, sr, rng(seed * 17 + 9)) : null;
  layers.spice = sp ? [sp, sp] : null;
  layers.synth = p.syn_lvl > -59 ? renderSynth(p, n, sr, seed) : null;
  for (const k of [1, 2, 3]) layers['smp' + k] = p[`s${k}_lvl`] > -59 ? renderSample(p, k, n, sr) : null;
  // per-layer swell: a raised-sine ramp-in after the layer's drive, for ^-shaped (chug) attacks
  const SWL = { sub: p.sub_swell, tr: p.tr_swell, body: p.body_swell, synth: p.syn_swell };
  for (const [k, ms] of Object.entries(SWL)) {
    const S = Math.floor(((ms || 0) * sr) / 1000);
    if (S > 1 && layers[k]) layers[k] = layers[k].map((x) => { const y = x.slice(); for (let i = 0; i < Math.min(S, n); i++) y[i] *= 0.5 - 0.5 * Math.cos((Math.PI * i) / S); return y; });
  }
  // per-layer onset delay (staggered layers: transient, then shell, then wash)
  const DLY = { sub: p.sub_delay, tr: p.tr_delay, body: p.body_delay, synth: p.syn_delay };
  for (const [k, ms] of Object.entries(DLY)) {
    const d = Math.floor(((ms || 0) * sr) / 1000);
    if (d > 0 && layers[k]) layers[k] = layers[k].map((x) => { const y = new Float32Array(n); y.set(x.subarray(0, Math.max(0, n - d)), d); return y; });
  }
  const gains = { sub: p.sub_lvl, tr: p.tr_lvl, body: p.body_lvl, spice: p.spice_lvl, synth: p.syn_lvl };
  for (const k in gains) if (layers[k]) { const g = dbToLin(gains[k]); layers[k] = layers[k].map((x) => { const y = x.slice(); for (let i = 0; i < n; i++) y[i] *= g; return y; }); }
  return layers;
}

function addReverb(layers, p, n, sr) {
  if (p.rev_lvl <= -59) { layers.rev = null; return; }
  const send = new Float32Array(n);
  if (layers.tops) for (let i = 0; i < n; i++) send[i] = 0.5 * (layers.tops[0][i] + layers.tops[1][i]);
  const [hL, hR] = makeIR(p, sr);
  const [L, R] = convolveStereo(send, hL, hR, n);
  for (const x of [L, R]) { biquad(x, 'hp', p.rev_hp, 0.707, 0, sr); biquad(x, 'hp', p.rev_hp, 0.707, 0, sr); }
  // scale so that rev_lvl is relative to the send's RMS
  const rs = rmsOf(send), rr = Math.sqrt((rmsOf(L) ** 2 + rmsOf(R) ** 2) / 2), g = rr > 1e-9 ? (rs / rr) * dbToLin(p.rev_lvl) : 0;
  for (let i = 0; i < n; i++) { L[i] *= g; R[i] *= g; }
  layers.rev = [L, R];
}

// Bus: sub + tops + reverb -> master. Exported so stems from elsewhere can be pushed through it.
export function bus(sub, tops, rev, params, sr = SR, raw = false, fixed = null) {
  const p = { ...defaults(), ...params };
  const n = Math.max(...[sub, tops, rev].filter(Boolean).map((x) => x[0].length));
  const L = new Float32Array(n), R = new Float32Array(n);
  const si = raw ? 1 : p.mst_sub_in;
  for (const x of [tops, rev]) if (x) { addScaled(L, x[0], 1); addScaled(R, x[1], 1); }
  if (sub && si > 0) { addScaled(L, sub[0], si); addScaled(R, sub[1], si); }
  if (raw) return { L, R };
  const mid = midsChain(L, R, p, sr, fixed?.mid);
  // post-crush envelope on the mids: shape survives the distortion upstream (^ for chugs)
  if ((p.mst_swell || 0) > 0.5 || (p.mst_fall || 0) > 0.5) {
    let on = 0; const thr = 1e-4; while (on < n && Math.abs(L[on]) + Math.abs(R[on]) < thr) on++;
    const S = Math.max(1, (p.mst_swell * sr) / 1000), F = (p.mst_fall * sr) / 1000;
    for (let i = on; i < n; i++) {
      const t = i - on, up = t < S ? 0.5 - 0.5 * Math.cos((Math.PI * t) / S) : 1, dn = F > 0.5 && t > S ? Math.exp(-(t - S) / F) : 1;
      L[i] *= up * dn; R[i] *= up * dn;
    }
  }
  if (sub && si < 1) { addScaled(L, sub[0], 1 - si); addScaled(R, sub[1], 1 - si); }
  const fin = finalStage(L, R, p, sr, fixed?.fin);
  return { L, R, gains: { mid, fin } };
}

// Per-shot variation for bursts: round-robin seed, drift, jitter on timbral params.
function shotParams(p, s, r) {
  const q = { ...p, seed: (p.seed | 0) * 1000 + s + 1 };
  q.note = p.note + p.b_drift * s;
  const j = p.b_jitter;
  if (j > 0) {
    const nudge = (id, amt) => { const sp = SPEC_BY_ID[id]; q[id] = fromNorm(sp, toNorm(sp, q[id]) + (r() - 0.5) * amt * j); };
    nudge('sub_dive', 0.15); nudge('sub_tau', 0.15); nudge('body_flt_f0', 0.2); nudge('body_flt_f1', 0.2);
    nudge('body_mode_f', 0.08); nudge('body_comb_f', 0.08); nudge('tr_pk_f', 0.2); nudge('body_fm_idx', 0.2);
    q.note = Math.round(q.note);
  }
  return q;
}

export function render(params, opts = {}) {
  const p = { ...defaults(), ...params };
  const sr = opts.sr || SR;
  if (engineOf(p) === 'snare') {
    const n = Math.floor((p.len * sr) / 1000), extra = {};
    for (const k of [1, 2, 3]) if (p[`s${k}_lvl`] > -59 && SAMPLES[k]) extra['smp' + k] = renderSample(p, k, n, sr);
    return renderSnare(p, { ...opts, sr, extra });
  }
  const shots = Math.max(1, Math.round(p.b_shots));
  const shotLen = Math.floor(p.len * sr / 1000);
  // shot onsets
  const onsets = [0];
  for (let s = 1; s < shots; s++) {
    const frac = (s - 1) / Math.max(1, shots - 1);
    const iv = p.b_rate * Math.pow(2, -p.b_accel * frac * 1.5);
    onsets.push(onsets[s - 1] + Math.max(4, iv) * sr / 1000);
  }
  const total = Math.floor(onsets[shots - 1]) + shotLen;
  const names = ['sub', 'tr', 'body', 'synth', 'spice', 'smp1', 'smp2', 'smp3'];
  const acc = Object.fromEntries(names.map((k) => [k, [new Float32Array(total), new Float32Array(total)]]));
  const r = rng((p.seed | 0) * 7 + 77);
  for (let s = 0; s < shots; s++) {
    const q = shots > 1 ? shotParams(p, s, r) : p;
    const L = shots > 1 && s < shots - 1 ? Math.floor((onsets[s + 1] - onsets[s]) * p.b_gate + 0.003 * sr) : shotLen;
    const n = Math.min(shotLen, Math.max(64, L));
    const lay = renderShot(q, n, sr);
    const fade = Math.min(n, Math.floor(0.002 * sr)), o = Math.floor(onsets[s]);
    for (const k of names) {
      if (!lay[k]) continue;
      for (let c = 0; c < 2; c++) {
        const src = lay[k][c], dst = acc[k][c];
        for (let i = 0; i < n; i++) { const g = i > n - fade ? (n - i) / fade : 1; dst[o + i] += src[i] * g; }
      }
    }
  }
  const layers = Object.fromEntries(names.map((k) => [k, acc[k]]));
  // tops = tr + body + spice, through the crunch chain
  const tL = new Float32Array(total), tR = new Float32Array(total);
  let anyTop = false;
  const LVL = { tr: 'tr_lvl', body: 'body_lvl', synth: 'syn_lvl', spice: 'spice_lvl' };
  for (const k of ['tr', 'body', 'synth', 'spice']) if (layers[k] && p[LVL[k]] > -59) { addScaled(tL, layers[k][0], 1); addScaled(tR, layers[k][1], 1); anyTop = true; }
  for (const k of [1, 2, 3]) {
    const sm = layers['smp' + k]; if (!sm || p[`s${k}_lvl`] <= -59 || !SAMPLES[k]) continue;
    if (Math.round(p[`s${k}_route`]) === 1) { // clean: alongside the sub
      if (!layers.sub) layers.sub = [new Float32Array(total), new Float32Array(total)];
      addScaled(layers.sub[0], sm[0], 1); addScaled(layers.sub[1], sm[1], 1);
    } else { addScaled(tL, sm[0], 1); addScaled(tR, sm[1], 1); anyTop = true; }
  }
  if (anyTop) crunch(tL, tR, p, sr);
  layers.tops = anyTop ? [tL, tR] : null;
  addReverb(layers, p, total, sr);
  const { L, R, gains } = bus(opts.noSub ? null : layers.sub, opts.dropDry ? null : layers.tops, layers.rev, p, sr, opts.raw, opts.fixed);
  return { L, R, sr, layers, gains };
}

// ---------------------------------------------------------------- WAV
export function encodeWav(L, R, sr) {
  const n = L.length, buf = new ArrayBuffer(44 + n * 8), v = new DataView(buf);
  const w = (o, s) => { for (let i = 0; i < s.length; i++) v.setUint8(o + i, s.charCodeAt(i)); };
  w(0, 'RIFF'); v.setUint32(4, 36 + n * 8, true); w(8, 'WAVE'); w(12, 'fmt ');
  v.setUint32(16, 16, true); v.setUint16(20, 3, true); v.setUint16(22, 2, true); v.setUint32(24, sr, true);
  v.setUint32(28, sr * 8, true); v.setUint16(32, 8, true); v.setUint16(34, 32, true); w(36, 'data'); v.setUint32(40, n * 8, true);
  for (let i = 0; i < n; i++) { v.setFloat32(44 + i * 8, L[i], true); v.setFloat32(48 + i * 8, R[i], true); }
  return buf;
}

// Stems: each layer soloed through the chain at the full mix's gains, so relative levels hold.
export const STEM_LAYERS = [['sub', 'sub_lvl'], ['transient', 'tr_lvl'], ['body', 'body_lvl'], ['synth', 'syn_lvl'], ['spice', 'spice_lvl'], ['sample 1', 's1_lvl'], ['sample 2', 's2_lvl'], ['sample 3', 's3_lvl'], ['reverb', 'rev_lvl']];
export const stemLayers = (p) => (engineOf(p) === 'snare' ? [...SNARE_STEMS, ['sample 1', 's1_lvl'], ['sample 2', 's2_lvl'], ['sample 3', 's3_lvl']] : STEM_LAYERS);
export function renderStems(params, full = null) {
  const p = { ...defaults(), ...params };
  full = full || render(p);
  if (engineOf(p) === 'snare') {
    const out = {};
    for (const [name, k] of stemLayers(p)) {
      if (p[k] <= -59 || (name.startsWith('sample') && !SAMPLES[+name.slice(-1)])) continue;
      const solo = name.startsWith('sample') ? 'smp' + name.slice(-1) : name;
      const o = render(p, { fixed: full.gains, solo }); out[name] = [o.L, o.R];
    }
    return out;
  }
  const out = {}, off = Object.fromEntries(STEM_LAYERS.map(([, k]) => [k, -60]));
  for (const [name, k] of STEM_LAYERS) {
    if (p[k] <= -59 || (name.startsWith('sample') && !SAMPLES[+name.slice(-1)])) continue;
    if (name === 'reverb') { const o = render(p, { fixed: full.gains, dropDry: true, noSub: true }); out[name] = [o.L, o.R]; continue; }
    const o = render({ ...p, ...off, [k]: p[k] }, { fixed: full.gains });
    out[name] = [o.L, o.R];
  }
  return out;
}
