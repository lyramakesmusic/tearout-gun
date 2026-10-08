# OwQDpCtbNTM — I Made A 1-2-3 Method For Dubstep Sound Design
oddprophet · 16:41 · Serum 2

**Relevance:** High (workflow). Order-of-operations claim: get the recipe, then "make it fat" (distort → split → convolve → multiband comp) BEFORE any creative modulation. Few numbers.

## Method
1. **Recipe** (e.g. growl): growl wavetable in bass octave; LFO1 "open-close" shape → WT position; **band-pass/notch "multi" filter** on same LFO1; LFO2 → global main tuning (pitch bend). Gun = open at start then close (fast decay); wobble = close→open→close.
2. **Make it fat (stage 2), in order:**
   - Distortion
   - **Split** (low band kept dry: "no reverb on the sub")
   - **Convolver** (reverb IR, medium): **decay down, gain up, mix up** — "convolving also creates combing and phasing effects" → growl texture.
   - **Multiband compressor** (Serum OTT-like) to flatten spectrum, then make-up gain.
3. **Make it unique (stage 3):** flick wavetables (claims "50% of the way"), pitch bend, **Disperser** (Serum "diffuser"), **notch filters on LFO1**, **comb filter with resonance turned up** and small moves, Serum **Spectral** filter in manual mode/"tonal percussion", Multipass, square-4 recipe (square at −3, band-pass on LFO1, level LFO).

## DSP translation
Short-decay convolution reverb on a bass = short dense FIR whose spectrum is a random comb → adds inharmonic resonances/phasing ("metal"), mixed high. Sub split before so LF stays clean.

## Techniques tags
overdrive_stack, convolver, reverb_on_bass, multiband_comp, OTT, mono_low/sub_separate, sub_sidechain_or_split, disperser/allpass, comb, spectral/warp, filter_env, pitch_env/gun_transient, multipass

## Quotes worth keeping
- "Over here, I'm going to distort it. Then, I'm going to split it up and I'm going to put a convolver in."
- "bring down the decay. Bring up the gain. And bring up the mix."
- "convoluting also creates combing and phasing effects on the sound so you get even more texture. It's the absolute goat way of making growls."
- "No reverb on the sub. That's what they teach you in dubstep school day one."
