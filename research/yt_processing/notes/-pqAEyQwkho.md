# -pqAEyQwkho — My Ultimate Dubstep Bass Processing Rack for Free!? (Wheysted Music, 11:42)

**Relevance:** Medium. Heavy-dubstep producer's Ableton 11 post-processing rack; mostly parallel texture/stereo + selectable compression stage. Few numbers.

## Chain (in order)
1. **Input:** any bass one-shot, checked mono in SPAN ("100% in mono").
2. **Stage A, parallel "voicing/stereo" chains** (Ableton Instrument Rack, each chain has its own dry/wet macro, 0 = off, 100 = full; summed *in parallel* with dry, so total level rises):
   - Chorus-Ensemble (warm preset, tweaked feedback/warmth), low end pre-cut on all stereo chains.
   - Erosion "wide noise" for crisp highs.
   - Phaser (Phaser-Flanger) [inferred from "phase plan"].
   - Overdrive in parallel on mids only: "mids" band ≈ 600 Hz–4 kHz, peak ≈ 1 kHz; also a highs version. Used *instead of layering* when mids are missing.
   - Spectral Resonator (space/texture).
   - Hybrid Reverb with a user sample loaded as IR (convolution), size/decay tweaked.
   - Corpus (physical-model resonator) tuned to the bass note (example F#, F1/F2).
   - Haas width chain.
3. **Stage B, compression (after the stereo rack), choose one:**
   - Multiband Dynamics (Ableton) "all the way up".
   - Glue Compressor (macro = threshold) followed by grit.
   - OTT ×3 in series ("if you squash it you're going to want to saturate it"); "unless you're trying to Barely Alive it".
   - "Enhance" (very aggressive compression preset).
4. Optional low-cut chain ("cut out your low").
5. **Saturator at end with soft clip always on** → never exceeds 0 dBFS.

DSP: parallel sends with low-cut per send; post-sum compression; final static soft-clip ceiling.

**Techniques tags:** OTT, multiband_comp, softclip, overdrive_stack, chorus, phaser/flanger, convolver, reverb_on_bass, comb [Corpus resonator ~ tuned resonator; inferred], mid_boost, mono_low/sub_separate, layering

## Quotes worth keeping
- "if you squash it you're going to want to saturate it"
- "the saturate is automatically soft clipping at all times so ... you'll never be peaking"
- mids "around peak of 1k so it goes down to about 600 to about 4k"
