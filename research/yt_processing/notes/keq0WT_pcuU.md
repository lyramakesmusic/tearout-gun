# keq0WT_pcuU — Ableton Live Dubstep Tutorial - Filthy & Nasty Bass Processing / Resampling (Part 1)
AscianUK · 14:50

**Relevance:** Heavy-dubstep bass processing/resampling (older brostep era, Massive source). On-topic for multiband-split processing; not tearout-specific.

## Chain (bass 1, frequency-split method)
1. Source: Massive preset, 8-bar F1 note; tracks frozen, then resampled to audio.
2. Split into 3 bands with Auto Filter presets: low (< ~300–315 Hz), mid ("power"), high ("fizz/detail"). [crossover ~315 Hz stated; mid/high split not given]
3. Rule stated: distortion must go BEFORE the band filters, since saturation creates new partials below the crossover (filters cleanup what distortion generates).
4. Low band (< 300 Hz): compressor only, threshold lowered to thicken. Nothing else.
5. Mid band:
   - Saturator, "a few dB" drive.
   - CamelCrusher → CamelFat (Camel Audio), distortion + notch filter; notch cutoff ~750 Hz, slow sine LFO on cutoff, plus envelope + faster sine on filter envelope ("rise and fall").
   - Phaser, slow LFO, dry/wet ~30% (phaser loses power).
   - Saturator again ("saturation and distortion is the main effect").
   - Overdrive on upper mid, reduced dry/wet.
   - Flanger, very low in mix (slight 3D).
   - Second CamelFat notch, ~1.14 kHz, some resonance, sine LFO at 2 cycles/bar, LFO also mapped to resonance.
   - Auto Wah ("wow filter") with envelope follower, band-reject mode → snarl/growl.
6. High band: continues in part 2 (not in this video).

DSP: 3-band LR-style split; per-band waveshaping; moving notch filters (~750 Hz, ~1.14 kHz) LFO'd for movement; envelope-follower band-reject.

**Techniques tags:** multipass, resampling, multiband_comp, overdrive_stack, waveshaper, phaser/flanger, filter_env, mono_low/sub_separate, mid_boost [inferred: mid band is where all the processing goes]

## Quotes worth keeping
- "pretty much all processing needs to be done before these filters ... as soon as you boost it you're distorting the signal and creating frequencies which weren't there before"
- "saturation and distortion is going to be the main effect I'm using because ... you just want to beef up the mid"
- "the phaser is going to make it lose a lot of power cuz it's a time based effect ... dry wet to about 30%"
