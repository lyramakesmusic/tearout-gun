# HuO05jyAVUI — I'VE BEEN SINE COMPRESSING ALL WRONG! (Bricksquash Jade Cicada Vital Dubstep Sound Design Tutorial)
Channel: Bunting · 23:53

**Relevance:** Moderate. Not tearout per se (Jade Cicada / Bricksquash "sine compression" dubstep), but it's raw-patch-into-distortion technique with an important claim about **notch position relative to distortion**. No numeric values.

## Core recipe ("harmonic sine compression")
1. Osc1: sine. Osc2: "harmonic series" wavetable, scrubbing the frame position to select one precise harmonic (often automated/LFO'd so the frame jumps between harmonics; octave jump for glitch).
2. Distortion: **hard clip** (slammed) or **soft clip** with lower drive for subtler; Ableton-style "oct saturation" afterwards optional.
3. Movement: LFO on harmonic frame; LFO on distortion **drive** (more slammed over time → wub); LFO not retriggered (free-running) for rhythmic variation.
4. Filter: low-pass 24 dB with plucky envelope, cutoff "around a quarter way" for clean fat pluck; or none for "full-spectrum beef".
5. Notches (Ableton Auto Filter in notch mode, stacked): rule = **low-mid notch BEFORE distortion → pronounced movement** (distortion exaggerates the low partials it creates/removes), **high notch AFTER distortion → "whooshy, sweepy"**.
6. His own bass (Operator): sine + harmonic sine (via Operator "chorus"/harmonic control, automated) → slight LPF movement → Glue Compressor ("slams it, beefs it up") → notches in low-mid → distortion → filter → distortion → EQs → small reverb → Multiband Dynamics pushing the highs → multiband comp "rounding it out".
7. Extra layers: duplicated patch keeping only the saw, band-passed into its own lane for high-end texture; white noise layer (HPF'd) at low level.
8. Reverb: low mix, low time ("little room"), low end cut from reverb (reverb HPF).
9. Sync-mode saw for harmonic feel; envelope attack longer = wub, short = pluck.

DSP translation: "sine compression" = hard-clipping a sine (+ a single harmonic partial) so its spectrum becomes a square-ish odd-harmonic series; notch filters before the clipper reshape which intermods appear (tonal motion); after the clipper they act as simple spectral sweeps.

Techniques tags: hardclip, softclip, overdrive_stack, filter_env, eq_before_dist, eq_after_dist, multiband_comp, OTT, reverb_on_bass, layering, chorus, lowmid_scoop

## Quotes worth keeping
- "if you have a lower mid range notch before the distortion that's going to give you much more pronounced movement versus a higher range notch at the end right gives you more of that sweepiness"
- "it's a sine wave harmonic series … with distortion and this is the basis to this entire sound design"
- "messing with those lower harmonics to be further distorted then filter it again then distorted"
