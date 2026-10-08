# IWUVd0fiyH4 — Negativist, "How To Sound Design Punchy Snare Drums (using Vital)" (4:26)
Source class: supplementary (not CamaCon). Pure synthesis, single Vital patch. Frames: frames/IWUVd0fiyH4_{65,88,145,165}.png

## Layers
1. Body: OSC1 sine (Basic Shapes, 1v unison, phase 100% randomization visible) [screen]
2. Noise: SMP = Vital "White Noise" sample [said+screen]
No separate transient layer: the "click" is noise-on-attack + fast pitch env.

## Envelopes (Vital LFOs in Envelope mode, seconds) [screen]
- ENV1 (amp, whole patch): square gate — A 0, flat hold, cut to 0 at ~25% of view [screen]. So shape comes from LFOs, ENV1 is just a gate.
- LFO1 → OSC1 pitch: exponential decay, cycle = **0.035 s** (35 ms), steep convex curve, ~90% of the drop done by ~12 ms [screen]. Depth not shown (no matrix view). [said] "shape like a drum-synth pitch env".
- LFO2 → OSC1 level: cycle **0.456 s**; holds ~full for ~35 ms, slight dip, then curved decay to 0 at ~32% of cycle ≈ **~145 ms** body length [screen, mid-edit at 0:88].
- LFO3 → noise level: cycle **0.167 s**. Final shape (0:145): spike at t=0 (noise click), dips to 0 at ~8% (~13 ms), rises back to peak at ~15% (~25 ms), then linear decay to 0 at 167 ms [screen]. Earlier draft (0:110): attack ramp 0→peak in ~15% (~25 ms), linear decay [screen]. [said] "what usually changes the sound quite a bit is if we also put some white noise on the attack".
  -> Noise envelope = click spike + delayed swell (peak ~25 ms) + ~140 ms linear tail.

## Filters / FX
- Noise routed to FILTER1 Analog 12 dB, high-pass-ish shape (low cut, cutoff ~ lower mids), with DRIVE turned up [said+screen].
- FX: Distortion **Hard Clip**, drive ~+ moderate, mix 100% [screen]; then Equalizer (low band shelf/peak, band & high) [screen]. [said] "distortion always works, might even be using hard clip".
- EQ band gain modulated by the noise-level LFO ("mimic this movement") [said]. Boost body fundamental with EQ [said].
- Reverb: left dry, added later per context [said].

## Balance
Not quantified; snare "too loud" vs kick when layered as hit [said].
