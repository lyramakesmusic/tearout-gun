# joYYP0ckXT4 — The Best Resampling Method For Metallic Basses (Sound Design Tutorial)
Channel: PostHumandubs · 9:10

**Relevance:** High for metallic/"swipy" texture design. A resampling chain built on Serum 2 allpasses, a big downward frequency shift and OTT, with concrete numbers.

## Chain
1. Source: **percussion one-shot with a metallic ringing tail**, on its own audio track. Pitching it (even 1 semitone) before the allpasses changes the result drastically.
2. Serum 2 FX: **3 × allpass filters** in series; macro 1 → all three cutoffs, macro 2 → all three resonances. **Resonance ≈30, cutoff ≈19 (macro units)**; adds smear + extra resonant harmonics. Higher resonance gets "too noisy."
3. Frequency shifter (Image-Line Frequency Shifter): range **20 kHz, high quality**, **shift DOWN 1–3 kHz (used ~2.2–2.5 kHz)**. "Taking advantage of artifacts from frequency shifting."
4. Filter automation (Kilohearts Filter, any LPF/EQ works) → "swishy" sweep.
5. Trim: shorten the tail, short fade-in.
6. **OTT ×1–2: depth ~80%, Time max, output +~10 dB.** More OTT gets "too noisy/muddy." Compresses hard to "bring those artifacts back up."
7. Reverb (Valhalla Supermassive / "ROM" [inferred: free Valhalla]) with automated mix, **low cut all the way up** (reverb HPF max).
8. Cleanup EQ: bring up the highs.
- Optional: **MSpectral Delay**: dry/wet 100%, feedback 0, sync off (ms), delay curve drawn swooshy, transform curve set straight; nudging Transform = spectral frequency shift; low time + high feedback = extra resonances. Alternatives: FL Multiband Delay, Ableton Spectral Time.

DSP: allpass cascade with resonance (2nd-order allpasses with high Q) = narrow group-delay peaks → ringing at cutoff; a downward SSB frequency shift by ~2 kHz folds content through 0 Hz (negative frequencies reflect) → inharmonic, aliased-sounding metallic partials; then heavy upward compression raises the low-level artifacts.

Techniques tags: resampling, disperser/allpass, freq_shifter, OTT, upward_comp, filter_env, reverb_on_bass, spectral/warp, eq_after_dist

## Quotes worth keeping
- "load up three allpass filters … one macro on the cutoff of all three … another macro on the resonance"
- "frequency shift this down quite a lot, and then compress it afterwards to bring those artifacts back up"
- "Depth at around 80%, time all the way up, and the out gain up maybe 10 dB"
