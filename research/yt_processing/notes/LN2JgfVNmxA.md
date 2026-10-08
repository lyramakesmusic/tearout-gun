# LN2JgfVNmxA — The SERUM 2 Bode Frequency Shifter Is INSANE For Heavy Dubstep
oddprophet · 15:39

**Relevance:** Directly on-topic: tearout gun in Serum 2 with split-band convolver + Bode frequency shifter on highs.

## Chain
1. Wavetable osc (Cymatics freebie tables); LFO1 (drawn shape) → wavetable position (main timbre driver).
2. Filter: gun = high transient then low, like a kick → envelope sweeps cutoff high→low.
3. Overdrive (Serum 2) — "been loving the overdrive."
4. Spectral oscillator loaded with a low tonal percussion one-shot; scan knob up to speed playback ("more growls"); noise osc loaded with a high transient one-shot, envelope mode.
5. FX Splitter (Serum 2 multiband splitter), default split at **210 Hz**; processing only on the HIGH band.
6. Convolver on highs: "the main component that makes it sound like a tearout gun". Pick a MEDIUM IR ("vocal hall" or a "mid" one; huge IR → just reverb). Raise IR gain; pull decay DOWN to lose the cave. IR size knob = stretch IR → changes tone. LFO on IR param tried.
7. Compressor, threshold only catching peaks (not maxed).
8. Sub sine osc, direct out (bypasses FX), level ~3; LFO in envelope mode on it.
9. More drive on overdrive for crunch (note: too much overdrive "deletes" spectral detail).
10. Pitch bend: LFO2 envelope → global main tuning, fast drop.
11. Alt warp on wavetable bend → thicker low end.
12. Bode frequency shifter (in high band of splitter): Range = max shift amount; Direction knob fully to one side = single-sided ordinary frequency shifter (centre = L up / R down stereo flutter, "sounds like ass"). Shift sweep gives many tones while sub unchanged. Blur = smeared reverb, softens transient (like another convolver). LFO fast on shift for sustains.
13. EQ: boost highs. If shifting removes too much low, raise split to ~**300 Hz**.
Ignore width/delay/speed for dubstep.

DSP: crossover 210–300 Hz; above it convolution with short-decay medium IR + single-sideband frequency shift (Hilbert) by modulated Hz; below it untouched sine sub.

**Techniques tags:** convolver, freq_shifter, multipass, overdrive_stack, mono_low/sub_separate, sub_sidechain_or_split, pitch_env/gun_transient, spectral/warp, filter_env, eq_after_dist

## Quotes worth keeping
- "with tear out guns, the main component that makes it sound like a tear out gun is the convolver"
- "If you put bode on the low frequencies, like the sub, your subbase is now frequency shifted out of key and it sounds trash"
- "I'm going to choose something from the medium section because if we have something too massive, it's just going to sound like a reverb"
- "you turn the range all the way ... but then the direction is where you want to go"
