# w6q4AQiRR1Y — "How to make SUBBY DUBSTEP GUN BASS in Serum" — PHENOMSOUND (14:23)

Serum (FL). Three layers from one patch: main (mono), noise (wide), mid-bass (wide).

## Body
- Osc A wavetable **"4088"**, octave **-2**, level LFO-automated.
- Osc B **"Dirty"**? (captions "Doty"), octave **-2**, semi **+7**, level automated ~**65**; warp **Sync 4.1%**.
- Osc A warp **FM from B** knob **30–31**, LFO amount **50**.
- Sub osc octave **-2**, level **25**, LFO-automated (sub is gated with the gun).
- Noise **"Inharmonic 91"**, pitch **87–90**, level automated.
- Filter: **Flanger -** (flange-minus), B osc routed, cutoff **3500** (Hz-ish), LFO amount **-32**; res automated **75**; drive **40**.
- Patch is **65% mono** (stereo width ~60%).

## Pattern / retrigger
- LFO1 pre-saved shape (on screen), rate **1/4**, **trigger mode**, → levels, FM, filter.
- LFO2 shape (on screen) → **master tune**, **unipolar**, amount **40**, trigger on (synced to LFO1) — per-hit pitch drop.
- Variation: duplicate the sound, **change the LFO shape a bit**, alternate the two in the pattern = groove.

## FX (Serum)
- Distortion drive **~90**. Flanger + chorus slight. Multiband comp gain **11 dB**, high **70**, mid **80**.
- FX filter **High Notch**?, cutoff down, LFO → res and drive **60** — "low-frequency boost".
- Reverb **Hall** size **5%**, decay **1 s**, mix **10%**.

## Group / post
- Main: OTT downward **47**, depth **76**, high **-4 dB**, low **-2 dB** → EQ → limiter. Master OTT downward 15/depth 55 → limiter.
- Noise layer: **Erosion** (noise distortion), OTT, **Blood Overdrive**, heavy low cut — wide.
- Mid-bass layer: OTT, EQ, **frequency shifter** (movement), **Fruity Stereo Shaper** (extreme width), saturator, EQ. Main bass stays mono.

## On-screen (frames/w6q4AQiRR1Y_*.jpg) — Serum, FL, 150 BPM
- Osc A **"4088"** oct -2; Osc B **"DirtySaw"** (captions "Doty") oct -2, **sem +7**. Osc filter MG Low 12. FX filter **HN 12** (captions "high notch").
- **LFO1 (TRIG, 1/4 ≈ 400 ms)**: a **convex "dome" fall** — starts at max, curves down slowly at first then faster (quarter-cosine-like), hitting **0 at ~50% of the cycle** and staying at 0. So each hit sounds ≈ 200 ms with a rounded, full-bodied decay rather than a spike. Distinctly different from the oddprophet/Pandorum spike-then-ramp shape.
- **LFO2 (TRIG, 1/4) → Mast.Tun (amount 1 = 100% of range shown)**: **exponential decay reaching ~0 at ~35% of the cycle (~140 ms)** — a longer pitch fall than the oddprophet blip.
- **Matrix (_342)**: LFO1 → **A Warp**, **Sub Osc Level**, **Noise Level**, **Fil Cutoff (negative)**, **Fil Reso**; LFO2 → **Mast.Tun**.
- FX order: Distortion → Flanger → Phaser → Chorus → Compressor (multiband) → Filter (HN 12). Mixer: **OTT → Pro-Q 3 → Ozone 10 Maximizer**.
