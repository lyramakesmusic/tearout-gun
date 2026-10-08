# QtpT7PlkWCs — You Need To Know How To Make This Tearout Bass! [FREE DOWNLOAD]
XLNTSOUND · 15:42 · Serum (1)

**Relevance:** Medium-high. Resample-a-one-shot-into-wavetable workflow + Serum FX chain for metallic tearout. Few numbers.

## Chain, in order
1. Take a one-shot from a tearout track (prefer one containing kick/snare transient), consolidate, drag into Serum Osc A, **constant frame size**. WT editor: remove current index, **Morph → Crossfade** (256 frames), **normalize** all frames.
2. LFO1 slow ramp up (rate **1/2**, BPM) → WT position (scrub a "sweet spot" start).
3. FX: **Distortion** → **Compressor multiband** (threshold down, gain up — "shardy") → **Hyper/Dimension** (width; small-room slapback vibe) → **Delay, very short/fast** for metallic comb → **Filter High 12** to clean low end.
4. Osc filter **Band Notch 12**: LFO2 (up-down) on cutoff and inverse on freq/res, + drive → gunshot movement.
5. Sub: **triangle**, **direct out** (bypass FX and low cut).
6. Bonus "old VR trick": Osc B basic sine **FM from A** (complex WT into sine); find spot "right before it gets super metallic"; LFO on level; slow FM modulation; spread voices of the WT.
7. Optional noise osc.

## DSP translation
Short feedback delay = comb filter (metallic). Highpass after FX keeps LF for clean direct-out sub.

## Techniques tags
resampling, overdrive_stack, multiband_comp, chorus, comb, highcut/lowpass_after_dist, mono_low/sub_separate, sub_sidechain_or_split, filter_env, unison, layering

## Quotes worth keeping
- "i want more like metallic ... i usually like doing it with the delay just a really nice short fast delay"
- "you're going to want to go direct out because it's being affected by the effects on that low cut"
