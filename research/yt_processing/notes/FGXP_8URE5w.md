# FGXP_8URE5w — The Last TEAROUT Tutorial You Need (2026 Step-By-Step)
Channel: BassTi · 14:04

**Relevance:** High. Tearout gun built from 4 layers (sub / transient(kick) / body / noise) with heavy per-layer Kilohearts Snap Heap chains and a group "glue" chain. Few numeric values, but a detailed chain order.

## Per-layer chains
**Sub layer** (Serum built-in sub-bass sample, triplet grid):
1. EQ: remove upper content
2. Disperser: "disperse the low end a bit and delay the really low frequencies," adds movement [DSP: allpass cascade at low center freq → group delay on lows]
3. EQ
4. Ableton Corpus, preset "Kick Tight", **tuned to the note of the track** (tuning knob swept to taste) [DSP: tuned resonator / modal body]

**Transient layer (kick):** HPF (cut low end) → Snap Heap: frequency shifter UP → Saturator (heavy) → Shaper (more distortion) → transient shaper: attack very high, speed max, sustain down ("make it plucky again").

**Body layer (gun loop sample):** Snap Heap: Filter Table whose table position is driven by an Audio Follower of the sample → Shaper → EQ (HPF).

**Noise layer (short sample):** HPF → Snap Heap: freq shifter DOWN → filter (cutoff driven by audio follower) → freq shifter UP → Convolver with the "Disperser" preset IR, **feedback cranked** (adds resonances) → second Convolver → Shaper table (also audio-follower driven) "high-end sizzle" → Disperser → filter (different Q) → safety EQ → Nasko Spec Comp (very intense) → AC1 (saturation) → EQ → Serum FX for width.
[DSP: freq-shift down → filter → shift back up = moves the filter's effective cutoff/inharmonic offset; convolution-with-feedback = comb-like resonances.]

**Group chain on the whole gun ("Tearout group"):**
1. EQ (inspection), Filter (adds "glassy" texture), String filter (resonator), Minimal Audio Rift (intense distortion), Smack/Spec-comp "contrast heavy curvature compression", EQ (HPF), reverb (room) — on a sub-group
2. **Master gun group:**
   a. Disperser — "higher frequencies come first, lower frequencies slightly delayed… that nice pulling character"
   b. EQ: reduce sub (too much sub → muddy when distorted) — eq_before_dist
   c. Kilohearts Distortion: lots of **bias**, some drive, some spread, **mix < 100%** (parallel texture)
   d. Parallel split (2 lanes):
      - Sub lane: LPF → Ambassador (low-frequency enhancer ≤100 Hz), wet low → Corpus "Kick Tight" (tuned)
      - High lane: HPF
   e. Snap Heap: Bitcrusher (texture) → Slice EQ cuts highest freqs (LPF) → parallel lane: **ring mod on itself** + Slice EQ keeping only high freqs (ring-modded highs only) → Disperser → "fake-out chain" → Kilohearts Distortion (saturate mode, some drive, little spread) → Kilohearts Filter **notch** to cut muddy freqs → EQ boost "in this area" [unspecified] → delay 100% wet, few ms to the right (Haas)
   [DSP: ring-mod on self = squaring → even harmonics/octave-up + DC; kept on highs only.]

**Fake-out chain:** Convolver → Spec Comp (old) → EQ round-off.

**Other main sound:** HPF → Snap Heap (Nasko filter, saturator, freq shifter, audio-follower filter, Shaper table) → AC1 → Smack/stack comp. Another body layer: Fusion (resize) → notch filters with drive → Specter contrast → Snap Heap Disperser → Filter Table → Shaper → **comb filter** → AC1 → EQ → Serum FX width.

Techniques tags: layering, disperser/allpass, freq_shifter, convolver, comb, overdrive_stack, waveshaper, bitcrush/downsample, multiband_comp, sub_sidechain_or_split, mono_low/sub_separate, eq_before_dist, eq_after_dist, highcut/lowpass_after_dist, pitch_env/gun_transient, spectral/warp, reverb_on_bass, filter_env

## Quotes worth keeping
- "The disperser is responsible for delaying all the frequency content down below. So the higher frequencies are coming first and the lower frequencies are slightly being delayed. So it gets that nice pulling character."
- "get rid of some of the sub frequencies… because now we are distorting it. And when we are distorting it too much, everything sounds muddy"
- "a ring mod on self and another Slice EQ just with the high frequencies… adds this really polished, clean, and nasty high end"
- "convolver with the disperser preset and cranked up the feedback because the feedback adds some cool resonances"
