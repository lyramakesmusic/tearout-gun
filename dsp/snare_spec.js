// Snare engine parameters. Layers: click, tone (pitched shell), noise (wires/wash), metal (ping/ring),
// clap, room; then width and the bus. Every layer has its own onset delay.
const S = (g, id, label, min, max, def, scale = 'lin', unit = '', extra = {}) => ({ id, g, label, min, max, def, scale, unit, eng: 'snare', ...extra });
const NOISE_TYPES = 'white/pink/geiger/crackle/crushed/digital/ring';

export const SNARE_SPEC = [
  // TONE — the pitched shell, tuned to the note
  S('sn_tone', 'sn_t_lvl', 'level', -60, 6, 0, 'lin', 'dB'),
  S('sn_tone', 'sn_t_oct', 'octave', 2, 6, 3, 'int'),
  S('sn_tone', 'sn_t_tune', 'tune', -12, 12, 0, 'lin', 'st'),
  S('sn_tone', 'sn_t_wave', 'wave', 0, 3, 0, 'int', 'sine/tri/saw/square'),
  S('sn_tone', 'sn_t_round', 'square-ish', 0, 1, 0.3),
  S('sn_tone', 'sn_t_even', 'even', 0, 1, 0),
  S('sn_tone', 'sn_t_pitch', 'pitch drop', -12, 48, 12, 'lin', 'st'),
  S('sn_tone', 'sn_t_pitch_ms', 'drop time', 1, 120, 15, 'log', 'ms'),
  S('sn_tone', 'sn_t_click', 'click pitch', 0, 48, 24, 'lin', 'st'),
  S('sn_tone', 'sn_t_click_ms', 'click time', 0.3, 10, 2.5, 'log', 'ms'),
  S('sn_tone', 'sn_t_att', 'attack', 0, 60, 0, 'lin', 'ms'),
  S('sn_tone', 'sn_t_hold', 'hold', 0, 120, 30, 'lin', 'ms'),
  S('sn_tone', 'sn_t_dec', 'decay', 10, 800, 120, 'log', 'ms'),
  S('sn_tone', 'sn_t_curve', 'gate', 0, 1, 0.2),
  S('sn_tone', 'sn_t_partials', 'shell modes', -60, 0, -18, 'lin', 'dB'),
  S('sn_tone', 'sn_t_fm', 'FM', 0, 8, 0),
  S('sn_tone', 'sn_t_fm_ratio', 'FM ratio', 0.25, 8, 1.41, 'log', '×'),
  S('sn_tone', 'sn_t_grit', 'noise mod', 0, 1, 0),
  S('sn_tone', 'sn_t_drive', 'drive', 0, 36, 10, 'lin', 'dB'),
  S('sn_tone', 'sn_t_drive_env', 'drive falls', 0, 1, 0.5),
  S('sn_tone', 'sn_t_fold', 'fold', 0, 1, 0),
  S('sn_tone', 'sn_t_delay', 'delay', 0, 60, 0, 'lin', 'ms'),

  // HIT — the front edge, locked to the onset
  S('sn_hit', 'sn_h_lvl', 'level', -60, 12, -60, 'lin', 'dB'),
  S('sn_hit', 'sn_h_color', 'color', -1, 1, 0.3),
  S('sn_hit', 'sn_h_len', 'length', 0.5, 10, 3, 'log', 'ms'),
  S('sn_hit', 'sn_h_drive', 'drive', 0, 30, 12, 'lin', 'dB'),

  // CLICK — the first few ms
  S('sn_click', 'sn_c_lvl', 'level', -60, 6, -6, 'lin', 'dB'),
  S('sn_click', 'sn_c_type', 'type', 0, 4, 3, 'int', 'click/bleeps/zap/burst/snap'),
  S('sn_click', 'sn_c_f', 'freq', 300, 16000, 4000, 'log', 'Hz'),
  S('sn_click', 'sn_c_len', 'length', 1, 60, 4, 'log', 'ms'),
  S('sn_click', 'sn_c_n', 'bleeps', 1, 12, 4, 'int'),
  S('sn_click', 'sn_c_spread', 'spread', 0, 3, 1.5, 'lin', 'oct'),
  S('sn_click', 'sn_c_drive', 'drive', 0, 36, 6, 'lin', 'dB'),
  S('sn_click', 'sn_c_delay', 'delay', 0, 40, 0, 'lin', 'ms'),

  // NOISE — wires / wash, with its own distortion
  S('sn_noise', 'sn_n_lvl', 'level', -60, 6, 0, 'lin', 'dB'),
  S('sn_noise', 'sn_n_type', 'type', 0, 6, 0, 'int', NOISE_TYPES),
  S('sn_noise', 'sn_n_rate', 'density', 30, 24000, 3000, 'log', 'Hz'),
  S('sn_noise', 'sn_n_hp', 'highpass', 100, 8000, 450, 'log', 'Hz'),
  S('sn_noise', 'sn_n_lp', 'lowpass', 1500, 20000, 15000, 'log', 'Hz'),
  S('sn_noise', 'sn_n_close', 'lp close', 0, 1, 0.3),
  S('sn_noise', 'sn_n_close_ms', 'close time', 5, 400, 60, 'log', 'ms'),
  S('sn_noise', 'sn_n_pk_f', 'peak', 400, 12000, 2400, 'log', 'Hz'),
  S('sn_noise', 'sn_n_pk_g', 'peak gain', 0, 18, 4, 'lin', 'dB'),
  S('sn_noise', 'sn_n_att', 'fade in', 0, 60, 10, 'lin', 'ms'),
  S('sn_noise', 'sn_n_flutter', 'flutter', 0, 1, 0),
  S('sn_noise', 'sn_n_flutter_ms', 'flutter gap', 8, 60, 25, 'log', 'ms'),
  S('sn_noise', 'sn_n_ring', 'ring', 0, 30, 0, 'lin', 'dB'),
  S('sn_noise', 'sn_n_ring_st', 'ring interval', 0, 12, 0, 'int', 'st'),
  S('sn_noise', 'sn_n_ring_q', 'ring Q', 4, 120, 30, 'log'),
  S('sn_noise', 'sn_n_hold', 'hold', 0, 200, 15, 'lin', 'ms'),
  S('sn_noise', 'sn_n_dec', 'decay', 20, 1500, 300, 'log', 'ms'),
  S('sn_noise', 'sn_n_curve', 'gate', 0, 1, 0.15),
  S('sn_noise', 'sn_n_rattle', 'rattle', 0, 1, 0),
  S('sn_noise', 'sn_n_rattle_f', 'rattle rate', 15, 400, 70, 'log', 'Hz'),
  S('sn_noise', 'sn_n_drive', 'drive', 0, 40, 10, 'lin', 'dB'),
  S('sn_noise', 'sn_n_hard', 'clip hardness', 0, 1, 0.4),
  S('sn_noise', 'sn_n_fold', 'fold', 0, 1, 0),
  S('sn_noise', 'sn_n_width', 'width', 0, 1, 0),
  S('sn_noise', 'sn_n_delay', 'delay', 0, 60, 0, 'lin', 'ms'),

  // METAL — tuned ring / ping
  S('sn_metal', 'sn_m_lvl', 'level', -60, 6, -60, 'lin', 'dB'),
  S('sn_metal', 'sn_m_type', 'type', 0, 4, 0, 'int', 'modal/comb/unison/808/bell'),
  S('sn_metal', 'sn_m_oct', 'octave', 3, 7, 5, 'int'),
  S('sn_metal', 'sn_m_tune', 'tune', -12, 12, 0, 'lin', 'st'),
  S('sn_metal', 'sn_m_spread', 'inharmonic', 0, 1, 0.5),
  S('sn_metal', 'sn_m_att', 'swell', 0, 80, 0, 'lin', 'ms'),
  S('sn_metal', 'sn_m_dec', 'decay', 20, 1500, 220, 'log', 'ms'),
  S('sn_metal', 'sn_m_bright', 'brightness', 0, 1, 0.5),
  S('sn_metal', 'sn_m_drive', 'drive', 0, 36, 4, 'lin', 'dB'),
  S('sn_metal', 'sn_m_width', 'width', 0, 1, 0),
  S('sn_metal', 'sn_m_delay', 'delay', 0, 80, 4, 'lin', 'ms'),

  // CLAP — a few quick bandpassed bursts, then a tail
  S('sn_clap', 'sn_k_lvl', 'level', -60, 6, -60, 'lin', 'dB'),
  S('sn_clap', 'sn_k_n', 'bursts', 1, 6, 3, 'int'),
  S('sn_clap', 'sn_k_gap', 'gap', 2, 25, 9, 'log', 'ms'),
  S('sn_clap', 'sn_k_f', 'freq', 500, 6000, 1400, 'log', 'Hz'),
  S('sn_clap', 'sn_k_q', 'Q', 0.4, 6, 1.2, 'log'),
  S('sn_clap', 'sn_k_dec', 'tail', 20, 600, 140, 'log', 'ms'),
  S('sn_clap', 'sn_k_drive', 'drive', 0, 30, 6, 'lin', 'dB'),
  S('sn_clap', 'sn_k_delay', 'delay', 0, 60, 6, 'lin', 'ms'),

  // WIDTH — symmetric stereo tail; the hit and the lows stay mono
  S('sn_width', 'sn_w_amt', 'width', 0, 1, 0.5),
  S('sn_width', 'sn_w_fade', 'fade in', 10, 300, 110, 'log', 'ms'),
  S('sn_width', 'sn_w_hp', 'mono below', 100, 3000, 350, 'log', 'Hz'),

  // ROOM — short convolution
  S('sn_room', 'sn_r_lvl', 'level', -60, 6, -18, 'lin', 'dB'),
  S('sn_room', 'sn_r_type', 'IR', 0, 4, 4, 'int', 'plate/metal/whip/spring/room'),
  S('sn_room', 'sn_r_size', 'size', 20, 2000, 220, 'log', 'ms'),
  S('sn_room', 'sn_r_char', 'character', 0, 1, 0.5),
  S('sn_room', 'sn_r_tone', 'tone', 800, 20000, 9000, 'log', 'Hz'),
  S('sn_room', 'sn_r_hp', 'highpass', 100, 4000, 500, 'log', 'Hz'),

  // BUS
  S('sn_bus', 'sn_b_time', 'time', 0.4, 2.5, 1, 'log', '×'),
  S('sn_bus', 'sn_b_hp', 'low cut', 20, 400, 90, 'log', 'Hz'),
  S('sn_bus', 'sn_b_low', 'body', -12, 12, 0, 'lin', 'dB'),
  S('sn_bus', 'sn_b_mid_f', 'mid freq', 300, 6000, 500, 'log', 'Hz'),
  S('sn_bus', 'sn_b_mid', 'mid', -12, 12, -2, 'lin', 'dB'),
  S('sn_bus', 'sn_b_whip', 'whip', 0, 15, 0, 'lin', 'dB'),
  S('sn_bus', 'sn_b_whip_f0', 'whip from', 150, 2000, 300, 'log', 'Hz'),
  S('sn_bus', 'sn_b_whip_f1', 'whip to', 400, 5000, 1000, 'log', 'Hz'),
  S('sn_bus', 'sn_b_whip_ms', 'whip time', 10, 300, 60, 'log', 'ms'),
  S('sn_bus', 'sn_b_dip', 'post-click dip', 0, 1, 0),
  S('sn_bus', 'sn_b_high', 'air', -12, 12, 2, 'lin', 'dB'),
  S('sn_bus', 'sn_b_ott', 'OTT', 0, 1, 0.3),
  S('sn_bus', 'sn_b_drive', 'drive', 0, 30, 6, 'lin', 'dB'),
  S('sn_bus', 'sn_b_hard', 'clip hardness', 0, 1, 0.6),
  S('sn_bus', 'sn_b_snap', 'snap', 0, 12, 3, 'lin', 'dB'),
];

export const SNARE_GROUPS = [['sn_hit', 'hit'], ['sn_click', 'click'], ['sn_tone', 'tone'], ['sn_noise', 'noise'], ['sn_metal', 'metal'], ['sn_clap', 'clap'], ['sn_room', 'room'], ['sn_width', 'width'], ['sn_bus', 'bus']];
export const SNARE_LEVEL = { sn_hit: 'sn_h_lvl', sn_click: 'sn_c_lvl', sn_tone: 'sn_t_lvl', sn_noise: 'sn_n_lvl', sn_metal: 'sn_m_lvl', sn_clap: 'sn_k_lvl', sn_room: 'sn_r_lvl' };
export const SNARE_STEMS = [['hit', 'sn_h_lvl'], ['click', 'sn_c_lvl'], ['tone', 'sn_t_lvl'], ['noise', 'sn_n_lvl'], ['metal', 'sn_m_lvl'], ['clap', 'sn_k_lvl'], ['room', 'sn_r_lvl']];
