# sk--QbE_TM0 — MACHINE GUN BASS DESIGN – Heavy Chain guns like Code: Pandorum, Nimda
Code: Pandorum · 16:14 · Serum 2 + Massive

**Relevance:** High (gun architecture). Three-layer gun (mid + click + pitch-falling sub); convolver placed BEFORE distortion; LF cut before vs after distortion choice.

## Architecture
1. **Mid/foundation layer (Serum 2):** two wavetables (one "dinosaur"), LFO retrigger, **triplet** rate (staccato 3/4 triplet machine gun), noise osc for click, FM from B, volume automation, comb filter on same movement.
   FX order: **Delay** (mix in, tonality) → **steep lowpass + drive** → **Convolver (before distortion, "to give it a bit more of a tone")**, random → 0, try different IRs → **low cut** (he chooses after distortion so "some crackliness of the low end distortion" remains; option to put before) → **Distortion** → **Compressor ("extremely")**. Macros: "tone", "length".
2. **Sub layer (Massive):** sine with **fast pitch fall** per hit — "crucial for the sound of a machine gun"; boost distortion slightly. Without pitch fall it "does not connect".
3. **Click/high layer (separate instance):** noise with downward sweep → compression → distortion → delay → EQ, pitched down, sub boosted a bit. Optional real gunshot sample, short and low-cut.
Keeps layers in separate instances so transient can be edited independently.

## DSP translation
Gun = short burst with exponential pitch envelope (downward chirp) on the sub; mid layer convolved then clipped (convolution pre-dist spreads energy so the clipper generates denser inharmonics).

## Techniques tags
layering, pitch_env/gun_transient, convolver, overdrive_stack, comb, highcut/lowpass_after_dist, eq_before_dist, eq_after_dist, multiband_comp, mono_low/sub_separate, filter_env

## Quotes worth keeping
- "one fundamental mid-range layer, one high layer that gives it a nice click, and one low layer, one punchy subbase layer"
- "before the distortion, I actually want to add a convolver"
- "this pitch fall is crucial for the sound of a machine gun"
