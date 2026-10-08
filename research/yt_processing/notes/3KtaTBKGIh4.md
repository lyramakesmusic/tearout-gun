# 3KtaTBKGIh4 — Gimmie 16 minutes and you'll make insane dubstep basses in SERUM 2 using this spectral manual mode (oddprophet, 16:04)

**Relevance:** Medium. Tearout gun built in Serum 2; post-chain described briefly (no numbers); main content is source generation.

## Chain (in order)
1. **Split low/high layers:** low = wavetable osc (Serum 2 default tables, e.g. "Oomph"), high = **Spectral oscillator** fed with *tonal percussion* (rimshots etc., not white noise).
2. Spectral osc in **manual mode**, position driven by LFO (inverted so bottom = quietest part) → loops the sample's spectrum like a wavetable wub.
3. Subtractive filter: band-pass on main osc.
4. Pitch bend / pitch envelope on the attack to accentuate gun transient.
5. **Sine-shaper** (wavetable warp / distortion "sine shaping") to emphasize harmonics.
6. Phaser layer (phaser IRs from previous video, i.e. phaser captured as convolver IR) [inferred].
7. **"The Big One" in-Serum post chain:** Overdrive (main) → EQ to remove mud → 2× Comb and Diffuser (mostly bypassed) → **Convolver** ("makes things really really fat") → **OTT**.
8. Spectral filter boost on brightest ridge to reinforce tonality; filter to sweep out low end.
9. Notch filter on shorter clones; layered "synth dirt" crash layer; Soothe2 on the bus.
- Variation: clone parent patch, one LFO for everything.

**Techniques tags:** overdrive_stack, convolver, OTT, comb, disperser/allpass [Diffuser], spectral/warp, pitch_env/gun_transient, layering, waveshaper, phaser/flanger, eq_after_dist

## Quotes worth keeping
- "it's mainly just overdrive ... a little bit of an EQ just to take out some of the muddy ... a convolver ... and then I just have a normal OTT"
- "tonal percussion. This is the secret."
- "manual mode and then have the position follow the LFO"
