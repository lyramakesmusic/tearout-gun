// One-line hover tips for the gun knobs, by spec id.
pub fn tip(id: &str) -> Option<&'static str> {
    Some(match id {
        // global
        "note" => "root note; every keyed layer follows it",
        "len" => "length of one shot in ms; long samples can extend it",
        "seed" => "random seed for the noise and per-shot variation; same seed, same hit",

        // shared envelope and timing (sub, transient, body, synth)
        "sub_shape" | "tr_shape" | "body_shape" | "syn_shape" => {
            "envelope: exp decays, spike drops fast then ramps out, dome rounds off over hold"
        }
        "sub_swell" | "tr_swell" | "body_swell" | "syn_swell" => {
            "fade-in in ms after the drive, for ^-shaped chug attacks; 0 is off"
        }
        "sub_delay" | "tr_delay" | "body_delay" | "syn_delay" => {
            "start this layer later by this many ms to stagger the layers"
        }
        "sub_att" | "tr_att" | "body_att" | "syn_att" => "attack time in ms; 0 starts at full level, raw and clicky",
        "sub_dec" | "tr_dec" | "body_dec" | "syn_dec" => {
            "exp decay time constant in ms; only used by the exp gate shape"
        }
        "sub_hold" | "tr_hold" | "body_hold" | "syn_hold" => {
            "ms from the start until the gate begins closing"
        }
        "sub_rel" | "tr_rel" | "body_rel" | "syn_rel" => {
            "gate fade-out length in ms after hold; short is a hard chop"
        }

        // sub
        "sub_lvl" => "sub sine level in dB; -60 is off",
        "sub_tune" => "sub pitch offset from the note in semitones",
        "sub_dive" => "semitones above the note the sub starts at before diving down",
        "sub_tau" => "dive time constant in ms; short is a punchy kick, long a slow drop",
        "sub_k" => "dive curve exponent; above 1 stays high then plunges, below 1 drops then drags",
        "sub_click" => "extra fast pitch dive in semitones for a click on top; 0 is off",
        "sub_click_ms" => "click dive time constant in ms",
        "sub_noise" => "highpassed noise mixed in before the sub drive, in dB; -60 is off",
        "sub_noise_hp" => "highpass on the sub noise in Hz",
        "sub_drive" => "saturation into the sub in dB; adds harmonics and squares it up",
        "sub_hard" => "sub clip shape: 0 smooth tanh, 1 hard clip",

        // transient
        "tr_lvl" => "transient noise burst level in dB; -60 is off",
        "tr_color" => "noise tilt: negative darker, positive brighter, 0 white",
        "tr_hp" => "steep highpass on the transient in Hz",
        "tr_lp" => "lowpass on the transient in Hz; also where the close sweep starts",
        "tr_pk_f" => "center of the transient's peak EQ in Hz",
        "tr_pk_g" => "peak EQ boost or cut in dB",
        "tr_pk_q" => "peak EQ width; higher is narrower and more whistly",
        "tr_click" => "sine chirp click from click freq down to 40 Hz, in dB; -60 is off",
        "tr_click_f0" => "where the click chirp starts in Hz",
        "tr_click_tau" => "click sweep time constant in ms; it fades over about 4x this",
        "tr_close" => "lowpass closes from the lowpass freq down to this, in Hz; 20k is off",
        "tr_close_ms" => "time constant of the lowpass close in ms",
        "tr_zap" => "saw sweep from zap-from to zap-to, in dB; -60 is off",
        "tr_zap_f0" => "zap sweep start freq in Hz",
        "tr_zap_f1" => "zap sweep end freq in Hz",
        "tr_zap_ms" => "zap sweep time constant in ms",
        "tr_stutter" => "number of envelope retriggers, each 20% quieter; 1 is a single hit",
        "tr_stut_ms" => "ms between stutter retriggers",
        "tr_drive" => "saturation on the transient in dB",

        // body
        "body_lvl" => "body layer level in dB; -60 is off",
        "body_noise" => "raw noise in the body mix, in dB; -60 is off",
        "body_color" => "noise tilt feeding the whole body: negative darker, positive brighter",
        "body_modes" => "resonator bank (metal partials) level in dB; -60 is off",
        "body_mode_f" => "base freq of the metal partials in Hz",
        "body_mode_set" => "partial ratio table: stiff string, membrane, bar, plate, cymbal, bell, 808",
        "body_key" => "free keeps metal and comb freqs as set; key snaps them to note harmonics",
        "body_vowel" => "vowel formant position: 0 a, 1 e, 2 i, 3 o, 4 u, morphing between",
        "body_vowel_mix" => "how much of the vowel formant filter is blended in; 0 is off",
        "body_metal" => "inharmonic stretch and random detune of the partials; 0 is clean",
        "body_q" => "resonator Q; higher rings longer and more pitched",
        "body_nmodes" => "how many partials the resonator bank uses",
        "body_mode_tilt" => "partial level slope in dB per octave; negative darkens upper partials",
        "body_comb" => "tuned comb resonator level in dB; -60 is off",
        "body_comb_f" => "comb pitch in Hz",
        "body_comb_fb" => "comb feedback; higher rings longer and more tonal",
        "body_comb_damp" => "lowpass inside the comb loop; higher makes the ring darker",
        "body_fm" => "FM tone level in dB; -60 is off",
        "body_fm_ratio" => "FM carrier pitch as a multiple of the note frequency",
        "body_fm_mod" => "modulator freq as a ratio of the carrier; non-integers clang",
        "body_fm_idx" => "FM depth at the start; higher is grittier and brighter",
        "body_fm_tau" => "time constant for the FM depth to die away, in ms",
        "body_drop" => "semitones the metal, comb and FM start above pitch and dive; 0 is off",
        "body_drop_tau" => "laser drop time constant in ms",
        "body_flt_type" => "sweep filter mode: lowpass, bandpass, highpass, notch",
        "body_flt_f0" => "filter freq at the start of the sweep, in Hz",
        "body_flt_f1" => "filter freq the sweep settles at, in Hz",
        "body_flt_tau" => "sweep time constant in ms from open to close",
        "body_flt_res" => "sweep filter resonance; near max it whistles",
        "body_flt_mix" => "wet/dry of the sweep filter; 0 bypasses it",
        "body_hp" => "steep highpass after the body drive in Hz",
        "body_drive" => "saturation on the body in dB",
        "body_fold" => "sine wavefolding blended into the drive; 0 is off",
        "body_width" => "stereo spread from a second noise pass; 0 is mono",

        // synth
        "syn_lvl" => "synth layer level in dB; -60 is off",
        "syn_wave" => "oscillator: saw, square, supersaw (3+ voices), thin pulse, sine",
        "syn_oct" => "octaves above the note",
        "syn_tune" => "fine pitch offset in semitones",
        "syn_voices" => "unison voice count",
        "syn_detune" => "unison spread; 1 is ±50 cents across the voices",
        "syn_int" => "second voice stack this many semitones from the root",
        "syn_int_mix" => "level of the interval stack; 0 is off",
        "syn_vowel" => "vowel formant position: 0 a, 1 e, 2 i, 3 o, 4 u, morphing between",
        "syn_vowel_mix" => "how much of the vowel formant filter is blended in; 0 is off",
        "syn_pitch" => "semitones the synth starts above pitch and dives down; 0 is off",
        "syn_pitch_ms" => "pitch dive time constant in ms",
        "syn_flt_f0" => "lowpass freq at the start of the sweep, in Hz",
        "syn_flt_f1" => "lowpass freq the sweep settles at, in Hz",
        "syn_flt_tau" => "lowpass sweep time constant in ms",
        "syn_flt_res" => "lowpass resonance",
        "syn_drive" => "saturation on the synth in dB",
        "syn_width" => "stereo spread from a mirrored-detune second pass; 0 is mono",

        // sample slots
        "s1_lvl" | "s2_lvl" | "s3_lvl" => "sample level in dB; -60 is off",
        "s1_pitch" | "s2_pitch" | "s3_pitch" => "repitch in semitones; speed changes with it",
        "s1_key" | "s2_key" | "s3_key" => "fixed plays as set; key follows the note, relative to C0",
        "s1_start" | "s2_start" | "s3_start" => "skip this many ms into the sample",
        "s1_len" | "s2_len" | "s3_len" => "ms of sample played before the gate fade",
        "s1_rel" | "s2_rel" | "s3_rel" => "fade-out length in ms after the sample length",
        "s1_hp" | "s2_hp" | "s3_hp" => "steep highpass in Hz; 20 is off",
        "s1_lp" | "s2_lp" | "s3_lp" => "lowpass in Hz; 20k is off",
        "s1_drive" | "s2_drive" | "s3_drive" => "saturation on the sample in dB; 0 is off",
        "s1_route" | "s2_route" | "s3_route" => {
            "mids goes through crunch and reverb; clean joins the sub, untouched"
        }

        // spice
        "spice_lvl" => "spice layer level in dB; -60 is off",
        "spice_type" => "comb ping, random sine bleeps, crackle, metal hit, or 808 cowbell-style metal",
        "spice_f" => "spice pitch or center freq in Hz",
        "spice_density" => "bleeps or crackles per second",
        "spice_len" => "window in ms the spice lives in; it fades out across it",
        "spice_hp" => "highpass on the spice in Hz",
        "spice_key" => "fixed keeps freq as set; key moves it with the note, relative to C0",
        "spice_delay" => "shift the spice in ms; negative crops its start",

        // reverb
        "rev_lvl" => "reverb return in dB relative to the tops; -60 is off",
        "rev_type" => "IR: plate, metal partials, whip chirp, spring, room",
        "rev_size" => "reverb decay time (to -60 dB) in ms",
        "rev_char" => "IR flavor: darkening, density, or chirp spacing depending on IR",
        "rev_tone" => "reverb brightness ceiling in Hz",
        "rev_pre" => "silence before the reverb starts, in ms",
        "rev_hp" => "steep highpass on the reverb return in Hz",

        // crunch
        "cr_mix" => "wet/dry of the whole crunch chain on the tops; 0 bypasses it",
        "cr_hp" => "crunch highpass in Hz; lows below it are cut from the crunched tops",
        "cr_mud" => "cut just above the crossover in dB, before the drive",
        "cr_pk_f" => "freq of the pre-drive boost that the distortion screams on, in Hz",
        "cr_pk_g" => "pre-drive boost at scream freq in dB; 0 is off",
        "cr_drive1" => "first distortion stage in dB",
        "cr_hard1" => "first stage clip shape: 0 smooth tanh, 1 hard clip",
        "cr_disp_n" => "allpass stages smearing phase into a laser chirp; 0 is off",
        "cr_disp_f" => "center freq of the dispersion in Hz",
        "cr_disp_q" => "allpass Q; higher pinches the chirp tighter around the freq",
        "cr_conv" => "blend of a short convolution between the drives; 0 is off",
        "cr_conv_type" => "convolution IR: plate, metal, whip, spring, room",
        "cr_conv_len" => "convolution IR length in ms",
        "cr_conv_char" => "convolution IR flavor, as reverb character",
        "cr_drive2" => "second distortion stage after dispersion and convolution, in dB",
        "cr_rack" => "number of OTT then clip passes for loudness; 0 is off",
        "cr_rack_depth" => "OTT depth inside each rack pass",
        "cr_pp" => "ping-pong delay level; 0 is off",
        "cr_pp_ms" => "ping-pong delay time in ms; right side is 3 ms later",
        "cr_pp_fb" => "ping-pong feedback; higher repeats longer",
        "cr_fs" => "frequency shift in Hz; positive up, negative down, inharmonic either way",
        "cr_fs_wet" => "wet/dry of the frequency shifter; 0 is off",

        // master
        "mst_sub_in" => "how much sub goes through the OTT and clip; 0 keeps the sub clean",
        "mst_swell" => "fade-in in ms on the mids after the master clip; 0 is off",
        "mst_fall" => "decay time constant in ms on the mids after the swell; 0 is off",
        "mst_snap" => "transient boost in dB on the mids; 0 is off",
        "mst_ott" => "3-band OTT compression depth; 0 is off",
        "mst_ott_up" => "OTT upward amount, lifting quiet tails and noise",
        "mst_ott_time" => "multiplier on OTT attack and release; higher is slower",
        "mst_drive" => "master clipper drive on the mids in dB",
        "mst_hard" => "master clip shape: 0 smooth tanh, 1 hard clip",
        "mst_mono" => "everything below this freq in Hz is summed to mono",
        "mst_low" => "low shelf at 90 Hz in dB",
        "mst_mid" => "peak EQ at 280 Hz in dB; negative scoops boxiness",
        "mst_high" => "high shelf at 3 kHz in dB",

        // burst
        "b_shots" => "number of repeated hits; 1 is a single shot",
        "b_rate" => "ms between the first two shots",
        "b_accel" => "positive speeds the repeats up, negative slows them down",
        "b_drift" => "pitch change per shot in semitones; sign sets the direction",
        "b_jitter" => "random per-shot nudges to dive, filter, metal and FM; 0 is identical",
        "b_gate" => "fraction of the interval each shot plays before it's cut",

        _ => return None,
    })
}
