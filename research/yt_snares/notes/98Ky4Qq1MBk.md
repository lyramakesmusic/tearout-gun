# 98Ky4Qq1MBk — Fragmental, "How to Make Dubstep Snares With Vital"
Archetype: dubstep synth snare, Vital. Exact values said.

- **Osc1 triangle**, 1 voice, phase randomization 0. Amp env2: attack 0, **decay 0.06 s (60 ms)** ("somewhere up to 0.1"), decay power ~3 (exp).
- **Pitch env3**: power down (fast/convex), decay **~0.05 s** ("try not to have it longer than env2"); amount: raise until the top is no longer audible as pitch, then stop.
- **Osc2 white-noise wavetable**, note track off, **pitched -36 st**, 2 unison voices with full detune => stereo (decorrelated L/R).
- Noise amp env4: **attack = env2 decay (0.06 s)** — noise fades *in* as the body ends; sustain 0, decay tweaked, decay power low. Env4 also modulates **wavetable position** (noise character evolves).
- Filter on osc2 only: **24 dB HPF**, blend ~1.5, res 0, cutoff ~MIDI 24 (?) — "don't want low freqs in the noise".
- Distortion.
- **EQ band gain enveloped**: gain min, env (seconds, sine mode) ramps a band *up* over the hit — dubstep-snare "effect" (band rises as the snare decays; cf. VR 1 kHz sweep).
- Free limiter to clip unneeded transient.
- Optional click: sample osc white noise with the **pitch env reused, remapped square-ish** (short gate) = click.
- EQ high band res 30-40 at MIDI 120-130 st to cut extreme highs.
## Takeaways
- Body ~60 ms, pitch env ~50 ms; noise starts *after* body (attack = body decay). Noise layer delayed rather than simultaneous.

## On-screen (frames/98Ky4Qq1MBk, 140 BPM)
- Osc1 triangle, phase 180°(?), 1 voice; Osc2 White Noise pitch **-36**, 2 unison voices, 100% detune spread.
- ENV2 (body amp) display: instant attack, very steep exponential decay (power ~-20 on env4 noted in hint bar "Envelope 4 Decay Power: -20").
- Distortion **Soft Clip**, drive ~60%, mix ~85%. EQ: band mode as a deep narrow cut at ~mid-high (gain driven by LFO1 "Saw Up" envelope 1/2 -> seconds mode: cut swells back up = band rises over the hit).
