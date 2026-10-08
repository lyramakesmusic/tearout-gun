# fIZHjnAlKW8 — Art1fact, "Designing a KILLER SNARE in Phase Plant"
Archetype: neuro/DnB synth snare, Phase Plant. Said; see frames for LFO shapes.

- Sources: **sine + white noise only** ("punchy low end + crispy rattle"). "It's all about the minutest details in the timing and envelopes" + **heavy drive gluing sine and noise**.
- Three LFO-envelopes (hand-drawn): (1) sine volume, (2) **sine pitch: +12 st dropping "really really quickly"**, (3) noise volume.
- **Noise env: leaves a small gap after the sine punch, ramps up slightly later, then a "double peak / flutter"** (two bumps) then decays — "more splashy"; without the double peak "not as dynamic".
- Noise HPF'd (low end removed), slightly stereo; filter movement for life.
- **Macro "tightness" scales the rate of all LFO-envelopes** (global time).
- **Main distortion ~15 dB drive (saturate mode)** — "glues them together"; nonlinear filter adds character and removes low rumble.
- Transient shapers used only to **cut sustain**, never boost attack ("attack should be designed with pitch and envelopes").
- "Ring" macro: a resonant clap sample with ringing harmonic (HPF'd) layered.
- Low cut below fundamental (**fundamental ~200 Hz**), but leave space; not too close.
- **~4% reverb** mix: "from a digital void to a room".
- Played on **E** rather than C (pitch matters).

## On-screen (frames/fIZHjnAlKW8) — LFO editor shows ms axis (best timing data in corpus)
- LFO rates 1.953 / 1.554 / 1.587 Hz (one-shot "Ramp Down" envelope mode; ~515-645 ms per cycle), all scaled by "tightness" macro.
- **Pitch LFO (sine semi +12)**: 100% at 0 ms -> ~50% at ~20 ms -> ~25% at ~40 ms -> ~5% at ~80 ms -> 0 by ~160 ms. I.e. **+12 st with a ~20 ms half-life** (fast exponential).
- **Noise level LFO**: 0 at 0 ms, **rises to ~95% at ~20 ms, dips to ~75% at ~30 ms, second peak 100% at ~45 ms**, falls to ~25% at ~80 ms, ~5% at ~120 ms, ~0 at ~160 ms. (Noise onset delayed, double-peak "flutter".)
- Sine volume LFO: concave decay similar to pitch but slower (visible thumbnails).
- Sine group: Env attack 0, decay 100 ms, sustain 100%, release 7.7 ms (gate envelope; LFO does the shaping). Noise group: attack 0, decay 85 ms, sustain 91%, release 5 ms; noise filter HP.
- Lane FX: **Nonlinear Filter (Warm mode)**, 2x Transient Shaper (sustain down), **Distortion (Saturate)**, Slice EQ x2. Third generator: sample "Resident Clap F" (ring).
