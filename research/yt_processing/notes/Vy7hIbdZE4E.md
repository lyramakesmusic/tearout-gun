# Vy7hIbdZE4E — FL Studio, NIMDA Tearout gun - Tutorial
Channel: Silvyr | 10:36

**Relevance:** High. Tearout gun (NIMDA-style) built from three layers (gun tail / sub / transient) with long stacked FX chains.

## Layer 1: Gun tail (Serum)
- Wavetable 2 octaves down; LFO1 → volume + filter (cutoff/res) + FX params; LFO2 → wavetable position; LFO3 → white-noise level ("adds tone, brighter, more aggressive").
- Serum FX: Distortion with **LFO on drive** ("cleaner" than drive maxed) → Compressor (multiband) → Filter: Comb ("comb filter always works for tearout") → stereo effect.
- Mixer insert chain 1: EQ (low cut) → OTT → distortion → small convolver/reverb (free plugin) → EQ → Multipass distortion → Disperser → reverb (low end removed, highs kept, size small) → EQ → OTT → EQ.
- Insert chain 2 ("remove resonant frequencies", [inferred: resonance suppressor like Soothe]) → Multipass distortion → EQ → waveshaper → EQ.

## Layer 2: Sub (Vital)
- Pitch envelope: **+15 semitones** down-sweep for the "kick" ("go too high and it sounds goofy").
- Small white noise; band/"crunch" control.
- Distortion → EQ remove low-mids → (linear phase EQ "when dealing with subs") → distortion with **bias 30%** (asymmetric → crunch) → EQ remove low-mids → gentle high cut (gun already has highs; "stack high end on high end = messy") → Clipper.
- Optional grit layer: duplicate sub patch, noise up, no pitch env, distortion bias fully down, low cut, low level.

## Layer 3: Transient
- White noise, LFO→volume (one-shot burst), timed to gun hit. FX: EQ "make super loud" → Vocodex/"Vocalex" [inferred: Fruity Vocodex or a vocal-zap effect] for "zappy" → EQ remove sub only. Keep below gun level.

## Group bus
- Waveshaper → FabFilter Saturn: multiband — distortion on sub band, **50% on mid, 25% on high**, **mix 50%**, linear phase.
- Master/whole song: FabFilter (Saturn) **30%** linear-phase distortion.

DSP: three-layer split (sub with pitch env + asymmetric saturation; mid/high gun with alternating EQ→OTT→dist→allpass; noise transient). Asymmetric bias = even harmonics.

**Techniques tags:** layering, overdrive_stack, waveshaper, OTT, multiband_comp, multipass, convolver, reverb_on_bass, disperser/allpass, comb, pitch_env/gun_transient, eq_before_dist, eq_after_dist, highcut/lowpass_after_dist, hardclip, mono_low/sub_separate, lowmid_scoop, vocoder

## Quotes worth keeping
- "make sure to put this LFO on the drive so it sounds cleaner"
- "comb filter always works for tearout"
- "putting the bias 30% to make it crunchy"
- "if you stack high end on high end, it's going to sound messy"
