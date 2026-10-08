# 98Ky4Qq1MBk — Fragmental, "How to Make Dubstep Snares With Vital" (Vital, FL Studio)
Supplementary (not CamaCon). Frames: frames/98Ky4Qq1MBk_{75,250,430}.png

## Layers
1. **Tonal body**: Osc1 Basic Shapes **triangle**, 1 voice, phase randomization 0 [said+screen]. Played note **G4 (~392 Hz)** [screen: "Note on: G4"].
2. **Noise body/tail**: Osc2 = **white-noise wavetable**, note-track OFF, pitch **-36 st** [said+screen], unison **2 voices, detune 100%** for L/R decorrelation [said+screen]. Routed to Filter1 only.
3. **Click transient (optional)**: Sampler "White Noise", gated by the same short env as the pitch env with a remap curve [said].

## Envelopes
- Body amp (Env2): attack 0, decay ~0.06 s ("0.6"→ means 0.06), decay power ~3 (exponential) [said].
- Pitch env (Env3): power fully down, decay **0.05 s** [screen: "Envelope 3 Decay: 0.05 secs"], "only a little longer than body env" [said]. Depth: "turn up until the top is no longer audible" — amount not shown [said].
- Noise amp (Env4): **attack = 0.06 s (= body decay)**, sustain 0, short decay, power down. i.e. noise fades IN as the body fades OUT [said]. Env4 also modulates noise wavetable position.

## Filter
- Filter1 Analog **24 dB**, blend ~1.5 (between BP and HP), res 0, cutoff ~"24"(semitone-ish knob; low-cut of noise) [said+screen 430].

## FX (Vital, in order on screen 250): EQ → Distortion
- **EQ band peak with gain driven by a Saw-Up LFO in envelope mode, 0.5 s, smooth 0**: band gain starts fully down and rises over the hit — a resonant formant that "blooms" late [said+screen].
- EQ high band: resonance 30–40%, cutoff 120–130 st (high-shelf/cut of extreme top) [said].
- Distortion: **Soft Clip** [screen]. Then FL **Fruity Limiter** to clip peaks [said].

## Balance
- Body vs noise by ear; body shortened at end ("make the triangle envelope a bit shorter") [said].
