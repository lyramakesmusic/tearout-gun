# _yvW7iH_ufk — "HOW TO MAKE A TEAROUT GUN WITH ABLETON STOCK EFFECTS ONLY" — BassTi (21:28)

Ableton stock. **Four layers: sub (most important), tonal, noise, transient** (+ density layer). Very detailed; transient kept outside group processing.

## Pattern
- MIDI notes **D#, F#, F, D#, D** (root D). Short, no-release notes.

## Sub (Operator)
- Sine + **one added harmonic** for fullness.
- **Pitch envelope, 75% pitch-bend amount**, shape on screen (fast drop). Amp envelope **no release** (notes don't bleed).
- EQ cuts harmonics → **Disperser rack** (stock) slight → **Corpus "Kick Tight"** preset tuned to root **D** (warmer tonality) → EQ mid cut.
- "**Sub must be the loudest part in a tearout gun.**"

## Noise layer
- Resampled white noise in Sampler, **+3 octaves** (Pitch device), **one-shot mode with fade-out** = plucky.
- EQ (cut lows/mids) → Spectral Time (slight, metallic/width) → EQ → **Drum Buss/transient shaping → "rim shot"** → small reverb → EQ → gate → OTT → EQ → Chorus-Ensemble (width).
- Tip: print noise hits to audio so each transient enters at equal level.

## Tonal layer (Collision)
- Collision (Corpus-as-instrument) with membrane, decay tweaked, **tuned +40 st**. EQ → **OTT heavy** → gate.
- Parallel chains: (a) EQ → Saturator → EQ low cut → Dynamic Tube → EQ; (b) **Hybrid Reverb convolution "Small Ambience" IR**, -8 dB, stereo max → OTT → EQ → gate; (c) dry **+6 dB** (most audible); (d) chorus-ensemble → Utility max width → reverb → Roar texture (~0.5%). → Saturator → EQ. Channel **+2.6 dB**.
- Support tonal: Collision **+16 st** (24 lower), mid-focused, saturator bass shaper, OTT **32%**, reverb, hybrid convolution. ~15% more mids.

## Group processing (key "make 4 sounds one sound")
- Dynamic Tube (sub fizz) → **Roar multiband: low = diode shaper with bias, mid = soft sat, high; mix 78%** → Roar **22%** (low diode, mid digital, high bitcrush) → OTT **28%** → EQ cut muddy mids → **Split high/low**: low gets Corpus Kick Tight → **Redux** a bit → parallel: LP ~**8 kHz** vs high band **Shifter ring-mod with lots of drive** (high-end crunch) → Disperser rack → Hybrid reverb **21%**, mids cut (highs only wet) → Roar serial noise + poly modules **50%** → Roar fractal shaper **50%** → notch → Saturator waveshaper **30%** → Saturator bass shaper **17%** → OTT **12%**. High shelf cut for harshness.

## Transient (excluded from group processing so it keeps power)
- Collision **beam** module, high tune, short decay = click. **OTT heavy (×2)** → EQ cut mids/lows → Glue comp → EQ. Channel -10 dB.

## Context
- **Delay 100% wet, 0 feedback, 17 ms** on the gun to sit with drums (latency offset).
- Would print to audio, clean OTT artifacts, add half-time background hits on 1 and 3.

## On-screen (frames/_yvW7iH_ufk_*.jpg) — Ableton, **145 BPM**
- **_161 Sub (Operator) pitch envelope**: **Initial +12 st, Peak +36 st, Sustain 0 st**, A.Slope −100%, D.Slope +100%, R.Slope 47%; Pitch Env amount **75%** (→ effective peak ≈ +27 st). Envelope graph: instant peak then **drop to sustain within the first ~5% of the display** (a few tens of ms), then flat. Operator filter LP 12 kHz, res 28%; LFO sine 100 Hz rate, 10% amount (on the sub). Following EQ Eight shelving lows.
- **_161 MIDI pattern (zoomed crop)**: two bars of the gun. Note blocks are ≈1/6 beat wide → **16th-note-triplet (1/24) runs**, mostly on the root with **short climbs/falls of 1–5 semitones** (step pattern like root×5 → +2, root×4…, then +7 → +5 → +2 → root as a descending staircase). Groupings of 5, 4, 3 hits separated by single-step rests; some notes 1/8-triplet length. Second bar repeats with variation. Concrete evidence of 16th-triplet machine-gun bursts.
- **_738 Group processing**: Dynamic Tube (dry/wet 28.6%, drive 5.48 dB, tone −0.35) → **Roar multiband (crossovers 180 Hz / 2.00 kHz), low band Diode shaper, amount 17%, bias −0.25, feedback mode Time 18.2 ms, compress 20%, dry/wet 78%** → second Roar (low band amount 57%, bias −0.14, Bit Crush shaper on another band).
- **_1086 Transient (Collision)**: Mallet stiffness 50%, noise 50%, color 50%; Resonator 1 **Beam**, decay **139 ms**, material 95.31, **tune +36 st**; noise filter LP 949 Hz, A 0 ms / D 16 ms / S 0 / R 16 ms. → OTT (split 88.3 Hz / 2.5 kHz). Channel −10.6 dB.
