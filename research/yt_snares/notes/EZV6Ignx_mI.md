# EZV6Ignx_mI — KelSounds, "Phase Plant Drum Synthesis Series (The Snare)"
Archetype: snappy synth snare, Phase Plant. Said; see frames for shapes.

- Sine body (MIDI note **G2** ~98 Hz root; pitch mod brings it higher), amp via LFO-as-envelope (rate 7) on gain: hand-drawn shape with "a little room for the transient" (body dips/starts after transient).
- **Noise group** with HPF ~**2 kHz**; one LFO-envelope covers both transient spike and tail ("transient and tail all in one LFO").
- **Pitch**: LFO-shape on master pitch (downward drop).
- FX: distortion, **clipper**, frequency shifter, OTT.
- **Master volume shaped by LFO (rate 5) with a small dip right after the transient** — "makes it more snappy".
- Dynamics, slice EQ, shaper, limiter pushed (in gain).
## Takeaways
- Post-transient amplitude dip (~few ms notch) as a snappiness device — contrasts with Au5 "no gaps". Both agree the transient must be separated from the body.

## On-screen (frames/EZV6Ignx_mI)
- Generators: **Analog sine** (body) and **Noise** (slope/stereo, seed random), each with gate env (A 1 ms, D 100 ms, S 100%, R 5 ms); shaping by LFO "Pyramid" one-shots.
- Body LFO (gain) **~6.2-7 Hz cycle (~145-160 ms)**: flat-ish top briefly, concave fall to ~0 by ~80-90% of cycle (**~130 ms body**).
- Tail (noise) LFO **1 Hz (1 s cycle)**: spike at start then near-linear ramp down -> transient+tail in one shape, tail ~0.8-1 s (heard much shorter due to level).
- Pitch LFO on master pitch (downward).
- Lane FX: Distortion, Clipper, Frequency Shifter, **multipass with OTT**, then Post Processing gain.
- **Volume-shaper LFO (5 Hz, editor shows ms axis)**: 100% from 0 to ~4 ms, **dips to ~45-50% at ~6 ms, back to 100% by ~12 ms**, then full. **A ~8 ms wide, -6 dB notch right after the transient** = his "snappier" trick.
