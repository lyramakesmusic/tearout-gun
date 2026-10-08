// Whoosh engine parameters: noise source → amp envelope → moving filter (+ body) → modulation → space → post.
const S = (g, id, label, min, max, def, scale = 'lin', unit = '') => ({ id, g, label, min, max, def, scale, unit, eng: 'whoosh' });

export const WHOOSH_SPEC = [
  // SOURCE — what's being shaped
  S('w_src', 'w_s_lvl', 'level', -60, 6, 0, 'lin', 'dB'),
  S('w_src', 'w_s_type', 'noise', 0, 4, 1, 'int', 'white/pink/brown/crackle/metal'),
  S('w_src', 'w_s_tilt', 'tilt', -1, 1, 0),
  S('w_src', 'w_s_density', 'density', 20, 20000, 2000, 'log', 'Hz'),
  S('w_src', 'w_s_metal', 'metal', -60, 6, -60, 'lin', 'dB'),
  S('w_src', 'w_s_metal_f', 'metal pitch', 200, 8000, 1800, 'log', 'Hz'),
  S('w_src', 'w_s_metal_spread', 'inharmonic', 0, 1, 0.6),

  // AMP — the shape over time; length is the whole sound
  S('w_amp', 'w_a_len', 'length', 100, 6000, 900, 'log', 'ms'),
  S('w_amp', 'w_a_att', 'attack', 0, 4000, 300, 'lin', 'ms'),
  S('w_amp', 'w_a_att_curve', 'attack curve', 0.3, 4, 2, 'log'),
  S('w_amp', 'w_a_hold', 'hold', 0, 3000, 0, 'lin', 'ms'),
  S('w_amp', 'w_a_dec', 'decay', 20, 6000, 600, 'log', 'ms'),
  S('w_amp', 'w_a_dec_curve', 'decay curve', 0.3, 4, 1, 'log'),

  // FILTER — cutoff moves start → peak → end; peak lands at the loudest point by default
  S('w_flt', 'w_f_type', 'filter', 0, 2, 1, 'int', 'lowpass/bandpass/highpass'),
  S('w_flt', 'w_f_start', 'start', 40, 20000, 300, 'log', 'Hz'),
  S('w_flt', 'w_f_peak', 'peak', 40, 20000, 4000, 'log', 'Hz'),
  S('w_flt', 'w_f_end', 'end', 40, 20000, 600, 'log', 'Hz'),
  S('w_flt', 'w_f_peak_at', 'peak at', 0, 1, 0.4, 'lin', '×len'),
  S('w_flt', 'w_f_q', 'resonance', 0.3, 12, 1.2, 'log'),
  S('w_flt', 'w_f_mix', 'filter mix', 0, 1, 1),

  // BODY — a low swell under the noise (the "weight" of a whoosh or impact)
  S('w_body', 'w_b_lvl', 'level', -60, 6, -60, 'lin', 'dB'),
  S('w_body', 'w_b_freq', 'freq', 30, 400, 70, 'log', 'Hz'),
  S('w_body', 'w_b_drop', 'pitch drop', 0, 36, 12, 'lin', 'st'),
  S('w_body', 'w_b_noise', 'noisy', 0, 1, 0.5),
  S('w_body', 'w_b_dec', 'decay', 30, 3000, 400, 'log', 'ms'),

  // MOVEMENT — wobble on the filter, rhythmic chop (hype loops)
  S('w_mod', 'w_m_lfo', 'wobble rate', 0.1, 30, 3, 'log', 'Hz'),
  S('w_mod', 'w_m_lfo_amt', 'wobble', 0, 3, 0, 'lin', 'oct'),
  S('w_mod', 'w_m_gate', 'chop rate', 1, 40, 8, 'log', 'Hz'),
  S('w_mod', 'w_m_gate_amt', 'chop', 0, 1, 0),
  S('w_mod', 'w_m_gate_duty', 'chop length', 0.1, 0.95, 0.5),
  S('w_mod', 'w_m_gate_accel', 'chop accel', -1, 1, 0),

  // SPACE — width is symmetric; pass-by sweeps left → right over the sound
  S('w_space', 'w_x_width', 'width', 0, 1, 0.6),
  S('w_space', 'w_x_pass', 'pass-by', 0, 1, 0),
  S('w_space', 'w_x_rev', 'reverb', -60, 6, -14, 'lin', 'dB'),
  S('w_space', 'w_x_size', 'size', 100, 5000, 1200, 'log', 'ms'),
  S('w_space', 'w_x_reverse', 'reverse', 0, 1, 0, 'int', 'off/on'),

  // POST
  S('w_post', 'w_p_drive', 'drive', 0, 30, 0, 'lin', 'dB'),
  S('w_post', 'w_p_hp', 'low cut', 20, 2000, 40, 'log', 'Hz'),
  S('w_post', 'w_p_lp', 'high cut', 1000, 20000, 20000, 'log', 'Hz'),
  S('w_post', 'w_p_ott', 'OTT', 0, 1, 0),
];
export const WHOOSH_GROUPS = [['w_src', 'source'], ['w_amp', 'amp'], ['w_flt', 'filter'], ['w_body', 'body'], ['w_mod', 'movement'], ['w_space', 'space'], ['w_post', 'post']];
export const WHOOSH_LEVEL = { w_src: 'w_s_lvl', w_body: 'w_b_lvl' };
export const WHOOSH_STEMS = [['noise', 'w_s_lvl'], ['body', 'w_b_lvl'], ['reverb', 'w_x_rev']];
