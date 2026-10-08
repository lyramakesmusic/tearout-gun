// 808 engine parameters: sub (pitch dive + slow drift), FM, noise/click, then post-processing.
const S = (g, id, label, min, max, def, scale = 'lin', unit = '', extra = {}) => ({ id, g, label, min, max, def, scale, unit, eng: '808', ...extra });

export const E808_SPEC = [
  // SUB — sine on the note; pitch lands on it in two stages: a fast dive, then a slow tail settling onto the root
  S('e8_sub', 'e8_s_lvl', 'level', -60, 6, 0, 'lin', 'dB'),
  S('e8_sub', 'e8_s_oct', 'octave', -1, 1, 0, 'int'),
  S('e8_sub', 'e8_s_tune', 'tune', -12, 12, 0, 'lin', 'st'),
  S('e8_sub', 'e8_s_dive', 'pitch dive', 0, 36, 16, 'lin', 'st'),
  S('e8_sub', 'e8_s_dive_ms', 'dive time', 2, 120, 14, 'log', 'ms'),
  S('e8_sub', 'e8_s_drift', 'tail', 0, 6, 1.6, 'lin', 'st'),
  S('e8_sub', 'e8_s_drift_ms', 'tail time', 30, 1500, 180, 'log', 'ms'),
  S('e8_sub', 'e8_s_shape', 'triangle', 0, 1, 0),
  S('e8_sub', 'e8_s_att', 'attack', 0, 20, 0, 'lin', 'ms'),
  S('e8_sub', 'e8_s_hold', 'hold', 1, 1500, 120, 'log', 'ms'),
  S('e8_sub', 'e8_s_dec', 'decay', 50, 6000, 2600, 'log', 'ms'),
  S('e8_sub', 'e8_s_gate', 'length', 100, 3000, 1000, 'log', 'ms'),
  S('e8_sub', 'e8_s_rel', 'release', 2, 200, 40, 'log', 'ms'),

  // FM — a modulator on the sub's own pitch; index has its own decay
  S('e8_fm', 'e8_f_amt', 'amount', 0, 6, 0),
  S('e8_fm', 'e8_f_ratio', 'ratio', 0.5, 8, 2, 'log', '×'),
  S('e8_fm', 'e8_f_dec', 'decay', 5, 2000, 120, 'log', 'ms'),
  S('e8_fm', 'e8_f_sustain', 'sustain', 0, 1, 0),
  S('e8_fm', 'e8_f_fb', 'feedback', 0, 1, 0),

  // NOISE — click / texture on the front
  S('e8_noise', 'e8_n_lvl', 'level', -60, 6, -60, 'lin', 'dB'),
  S('e8_noise', 'e8_n_hp', 'highpass', 100, 12000, 1500, 'log', 'Hz'),
  S('e8_noise', 'e8_n_dec', 'decay', 1, 400, 12, 'log', 'ms'),
  S('e8_noise', 'e8_n_click', 'click pitch', 0, 48, 0, 'lin', 'st'),
  S('e8_noise', 'e8_n_click_ms', 'click time', 0.3, 10, 2, 'log', 'ms'),

  // POST — saturation, filter, OTT, clip, EQ; the sub below the crossover can stay clean
  S('e8_post', 'e8_p_drive', 'drive', 0, 40, 8, 'lin', 'dB'),
  S('e8_post', 'e8_p_drive_fall', 'drive falls', 0, 1, 0.5),
  S('e8_post', 'e8_p_type', 'type', 0, 3, 3, 'int', 'tanh/hard/fold/tube'),
  S('e8_post', 'e8_p_tone', 'tone', 200, 20000, 3500, 'log', 'Hz'),
  S('e8_post', 'e8_p_clean', 'clean sub', 0, 1, 0),
  S('e8_post', 'e8_p_xover', 'crossover', 40, 200, 90, 'log', 'Hz'),
  S('e8_post', 'e8_p_ott', 'OTT', 0, 1, 0),
  S('e8_post', 'e8_p_low', 'low', -12, 12, 0, 'lin', 'dB'),
  S('e8_post', 'e8_p_mid', 'mid', -12, 12, 0, 'lin', 'dB'),
  S('e8_post', 'e8_p_mid_f', 'mid freq', 150, 3000, 500, 'log', 'Hz'),
  S('e8_post', 'e8_p_clip', 'clip', 0, 18, 1.5, 'lin', 'dB'),
];
export const E808_GROUPS = [['e8_sub', 'sub'], ['e8_fm', 'FM'], ['e8_noise', 'noise'], ['e8_post', 'post']];
export const E808_LEVEL = { e8_sub: 'e8_s_lvl', e8_noise: 'e8_n_lvl' };
export const E808_STEMS = [['sub', 'e8_s_lvl'], ['noise', 'e8_n_lvl']];
