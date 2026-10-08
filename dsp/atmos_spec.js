// Atmosphere engine parameters: sustained layers (bed, drone, noise bass, crunch, ship, shimmer) → movement fx → space → post.
const S = (g, id, label, min, max, def, scale = 'lin', unit = '') => ({ id, g, label, min, max, def, scale, unit, eng: 'atmos' });

export const ATMOS_SPEC = [
  // SHAPE — the whole sound
  S('at_shape', 'at_len', 'length', 1000, 10000, 4000, 'log', 'ms'),
  S('at_shape', 'at_att', 'fade in', 10, 4000, 600, 'log', 'ms'),
  S('at_shape', 'at_rel', 'fade out', 10, 4000, 900, 'log', 'ms'),
  S('at_shape', 'at_move', 'movement', 0, 1, 0.5),
  S('at_shape', 'at_speed', 'movement speed', 0.02, 4, 0.3, 'log', 'Hz'),

  // BED — noise through drifting resonant formants
  S('at_bed', 'at_b_lvl', 'level', -60, 6, -4, 'lin', 'dB'),
  S('at_bed', 'at_b_color', 'color', -1, 1, -0.3),
  S('at_bed', 'at_b_formants', 'formants', 1, 6, 3, 'int'),
  S('at_bed', 'at_b_freq', 'center', 100, 8000, 900, 'log', 'Hz'),
  S('at_bed', 'at_b_spread', 'spread', 0, 3, 1.5, 'lin', 'oct'),
  S('at_bed', 'at_b_q', 'resonance', 0.5, 30, 6, 'log'),

  // DRONE — detuned oscillators on the note, optional chord
  S('at_drone', 'at_d_lvl', 'level', -60, 6, -60, 'lin', 'dB'),
  S('at_drone', 'at_d_wave', 'wave', 0, 2, 0, 'int', 'saw/sine/square'),
  S('at_drone', 'at_d_oct', 'octave', -2, 3, 0, 'int'),
  S('at_drone', 'at_d_chord', 'chord', 0, 5, 1, 'int', 'unison/fifth/octave/minor/major/sus'),
  S('at_drone', 'at_d_voices', 'voices', 1, 7, 4, 'int'),
  S('at_drone', 'at_d_detune', 'detune', 0, 60, 18, 'lin', 'ct'),
  S('at_drone', 'at_d_cut', 'cutoff', 100, 12000, 1200, 'log', 'Hz'),
  S('at_drone', 'at_d_res', 'resonance', 0, 0.95, 0.3),

  // NOISE BASS — noise through a comb tuned to the note, plus sub
  S('at_bass', 'at_n_lvl', 'level', -60, 6, -60, 'lin', 'dB'),
  S('at_bass', 'at_n_oct', 'octave', -1, 2, 0, 'int'),
  S('at_bass', 'at_n_fb', 'tone', 0, 0.995, 0.93),
  S('at_bass', 'at_n_sub', 'sub', -60, 6, -12, 'lin', 'dB'),
  S('at_bass', 'at_n_drive', 'drive', 0, 30, 10, 'lin', 'dB'),
  S('at_bass', 'at_n_lp', 'lowpass', 80, 6000, 900, 'log', 'Hz'),

  // CRUNCH — decimated, bit-reduced, distorted, high-passed
  S('at_crunch', 'at_c_lvl', 'level', -60, 6, -60, 'lin', 'dB'),
  S('at_crunch', 'at_c_rate', 'sample rate', 500, 24000, 4000, 'log', 'Hz'),
  S('at_crunch', 'at_c_bits', 'bits', 2, 12, 5, 'lin'),
  S('at_crunch', 'at_c_drive', 'drive', 0, 36, 14, 'lin', 'dB'),
  S('at_crunch', 'at_c_hp', 'highpass', 500, 12000, 3000, 'log', 'Hz'),
  S('at_crunch', 'at_c_grit', 'sparseness', 0, 1, 0.3),

  // SHIP — engine hum, ring mod, sweeping comb, pitch wobble
  S('at_ship', 'at_s_lvl', 'level', -60, 6, -60, 'lin', 'dB'),
  S('at_ship', 'at_s_hum', 'hum', 20, 400, 55, 'log', 'Hz'),
  S('at_ship', 'at_s_ring', 'ring mod', 0, 2000, 0, 'lin', 'Hz'),
  S('at_ship', 'at_s_comb', 'comb', 100, 4000, 600, 'log', 'Hz'),
  S('at_ship', 'at_s_sweep', 'comb sweep', 0, 3, 1, 'lin', 'oct'),
  S('at_ship', 'at_s_wobble', 'wobble', 0, 2, 0.3, 'lin', 'st'),

  // SHIMMER — sparse high sine grains, blooming in the reverb
  S('at_shim', 'at_g_lvl', 'level', -60, 6, -60, 'lin', 'dB'),
  S('at_shim', 'at_g_density', 'density', 1, 80, 12, 'log', '/s'),
  S('at_shim', 'at_g_oct', 'octave', 3, 7, 5, 'int'),
  S('at_shim', 'at_g_spread', 'spread', 0, 3, 1.5, 'lin', 'oct'),
  S('at_shim', 'at_g_len', 'grain', 5, 400, 60, 'log', 'ms'),

  // FX — phaser and flanger on everything
  S('at_fx', 'at_x_phaser', 'phaser', 0, 1, 0.3),
  S('at_fx', 'at_x_flanger', 'flanger', 0, 1, 0),
  S('at_fx', 'at_x_fb', 'flanger feedback', 0, 0.95, 0.6),
  S('at_fx', 'at_x_width', 'width', 0, 1, 0.8),
  S('at_fx', 'at_x_rev', 'reverb', -60, 6, -6, 'lin', 'dB'),
  S('at_fx', 'at_x_size', 'size', 300, 8000, 3000, 'log', 'ms'),

  // POST
  S('at_post', 'at_p_hp', 'low cut', 20, 4000, 60, 'log', 'Hz'),
  S('at_post', 'at_p_lp', 'high cut', 1000, 20000, 16000, 'log', 'Hz'),
  S('at_post', 'at_p_drive', 'drive', 0, 24, 3, 'lin', 'dB'),
  S('at_post', 'at_p_ott', 'OTT', 0, 1, 0.2),
];
export const ATMOS_GROUPS = [['at_shape', 'shape'], ['at_bed', 'bed'], ['at_drone', 'drone'], ['at_bass', 'noise bass'], ['at_crunch', 'crunch'], ['at_ship', 'ship'], ['at_shim', 'shimmer'], ['at_fx', 'fx'], ['at_post', 'post']];
export const ATMOS_LEVEL = { at_bed: 'at_b_lvl', at_drone: 'at_d_lvl', at_bass: 'at_n_lvl', at_crunch: 'at_c_lvl', at_ship: 'at_s_lvl', at_shim: 'at_g_lvl' };
export const ATMOS_STEMS = [['bed', 'at_b_lvl'], ['drone', 'at_d_lvl'], ['noise bass', 'at_n_lvl'], ['crunch', 'at_c_lvl'], ['ship', 'at_s_lvl'], ['shimmer', 'at_g_lvl'], ['reverb', 'at_x_rev']];
