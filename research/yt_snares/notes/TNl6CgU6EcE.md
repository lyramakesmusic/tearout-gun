# TNl6CgU6EcE — Au5, "1 Minute Snare With Operator"
Archetype: acoustic-ish thick snare from pure FM noise in Operator. All values said (exact).

- Algorithm parallel. Osc A: level 0 (off as source?) — Global: amp env decay **~111 ms**, sustain 0, loop trigger.
- **Filter: hard-shaper high-pass, cutoff ~666 Hz, keytrack 100%, resonance ~44%.**
- **Osc B: Noise Looped**, level **-11 dB**, peak -inf, initial max, attack **~6 ms**, decay **~33 ms**, release **~333 ms**, sustain **-11 dB**, coarse **22** (higher), **feedback 33** ("noisier").
- **Osc C** (copy of B): coarse **1**, decay **2x B (~66 ms)**, feedback **11** ("darker noise"), sustain **-22 dB** ("softer"). Phase restart off, spread 1% for stereo.
- Velocity -> time 33%, velocity -> volume 66%; global time down for tighter.
- Glue compressor: attack 30 (ms?), threshold ~half, makeup, **soft clip**.
## Takeaways
- Two noise layers at different "pitch" (brightness) with different decays: bright short (6/33 ms) + dark long (66 ms) — the dark/low noise lasts longer here.
- Resonant HPF at ~666 Hz keytracked: the resonance supplies the tonal "body".
- Velocity scales envelope time as well as level.
