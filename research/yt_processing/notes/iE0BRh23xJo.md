# iE0BRh23xJo — How to Make FREQUENCY SHIFTER Basses like ZEDS DEAD, TAPE B and LEVITY!!
Channel: Dripment · 8:32

**Relevance:** Moderate-high. A clear, numeric freq-shifter "phase interference" recipe on a Serum 2 bass. The style is wiggly dubstep (Tape B/Levity) rather than tearout, but the technique transfers directly.

## Chain
**Serum 2 patch (key F minor, notes F2 / Ab / G / Eb):**
1. Osc A: saw, octave −1, level 75%, PD (phase distortion) from B = **46%**
2. Osc B: default saw, octave −2, level 75%, **hard sync on, sync = 1.39**
3. Filter: default LPF, base cutoff **75 Hz**, LFO2 (ramp shape, square-ish top, half rate) on cutoff at full depth → opens slowly
4. FX: Compressor **+10 dB gain** ("pump up the jam") → Phaser mix **68%**, freq **33** (lower pitch) → **Convolve: mix 28%, IR "Growlish"** (metallic reverb) → Hyper: mix at 0 with LFO pushing it up to 27 (width)

**Post:**
5. OTT (specific settings, low band turned off) [inferred: low-band depth/gain disabled so sub isn't pumped]
6. **Ableton Shifter, frequency mode, Fine knob just off zero, Dry/Wet 50%**, automated small up/down moves. "If you go too high it gets out of control, stay close to the middle."
   - Alternatives: Melda MFreqShifter at **50% wet, shift 3.7 Hz**, with feedback/delay optional; Echobode-style "Echo mode" shifter at 50% mix, fine mode, optional tempo sync (1/4 triplet), smear, delay, feedback.

**DSP:** dry + frequency-shifted copy (shift Δf of a few Hz) at 50/50 → time-varying comb/notch pattern: partial n at f_n and f_n+Δf beat at Δf, so every partial amplitude-modulates at the same rate. For inharmonic shift the notches sweep across the spectrum (barber-pole phaser). Δf ≈ 1–10 Hz = slow phasing; larger = inharmonic detune.

Techniques tags: freq_shifter, phaser/flanger, convolver, OTT, upward_comp, filter_env, unison

## Quotes worth keeping
- "the most important thing is dry and wet set 50%"
- "we're blending the original signal with the shifted signal, and that creates phase interference"
- "if we go too high, it gets out of control … you got to stay close to the middle"
