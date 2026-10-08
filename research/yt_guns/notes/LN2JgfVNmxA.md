# LN2JgfVNmxA — "The SERUM 2 Bode Frequency Shifter Is INSANE For Heavy Dubstep" — oddprophet (15:39)

Serum 2. Builds a gun from scratch, then the Bode trick. Reconfirms his core recipe.

## Body / envelope (core oddprophet recipe, restated)
- Wavetable (Cymatics freebies). **LFO1 = "main characteristic timbre maker"** → WT position and **filter cutoff** (+ other things). Draw different LFO1 shapes to get variations.
- Filter "really important for guns, focuses an area": **"guns start with a high transient and end up low, kind of like a kick"** → cutoff sweep high→low.
- Overdrive.
- **Spectral oscillator** loaded with a low "tonal pop" one-shot from his toolkit; spectral **scan** up "makes sample go faster". High one-shot in the **noise oscillator**, envelope mode.
- Sub osc **direct out**, octave **-3**, LFO in **envelope mode**.
- **LFO2 (envelope) → Global Main Tuning** = pitch bend ("we need some pitchbend").
- Alt warp: **Bend** on WT → "really nice thick low end".

## Splitter (Serum 2) + convolver
- "**The main component that makes it sound like a tearout gun is the convolver.**"
- Serum 2 **Splitter** at default **210 Hz**; convolver on **highs only**. IR from the **medium** category ("too massive = just a reverb"), e.g. **"Vocal Hall"**; raise IR gain; **decay down** so it isn't "in a cave"; size = stretch/tone of IR; can LFO the convolver.
- Compressor lightly on top. Raise split to ~**300 Hz** to keep more low end unshifted.

## Bode frequency shifter (spice/variation)
- Inside the splitter's high band only (sub stays in key).
- Set **Range**, then **Direction fully to one side** → behaves as normal single frequency shifter (default is up-L/down-R flutter). Shift amount sweeps = many gun tones from one patch. LFO2 → shift for variation; fast LFO on shift for sustains.
- **Blur** = blurry verb, "like another convolver", **softens the transient**.
- Width/delay/speed: not useful for bass.

## On-screen (frames/LN2JgfVNmxA_*.jpg)
- 140 BPM, Serum 2. Osc A WT **"Saw Phasery Warp II"** (later **"Rounded Saw Phase"**), oct **-3**, warp **Bend +/-**. Phase 180°, rand 100.
- **_261/_501 LFO1** (ENVELOPE mode, rate **1/4** = one quarter note ≈ 430 ms at 140 BPM): starts at max, **straight steep drop to ~40% by ~15% of the cycle (~65 ms)**, then a near-linear slow fall (slight knee at ~60% of cycle, ~17%) reaching 0 at the end. Same family as his 2024 shape.
- **_547 Matrix**: LFO1 → **A WT Pos**, LFO1 → **Filter 1 Freq** (bipolar), LFO1 → **B Level**.
- **_586 LFO2 (pitch, ENVELOPE, 1/4)**: **exponential plunge from max to ~0 within ~10% of the cycle (~40 ms)**, then flat zero — a very short downward pitch blip, kick-style. Osc B = **Spectral, "Oddprophet Toolkit – Mid (41)"**, oct 0, **sem -3**, **one-shot** mode. Sub osc on (sine, oct -3). Filter 1 **BN 12**.
- **_501 FX**: Distortion (stack 3, pre-filter freq **425**, Q **1.9**) → **Splitter L/H at 210 Hz** → (Highs) **Convolve "L90 Hall – Vocal Hall"** → Compressor (**-11.6 dB, 4:1**).
- **_836**: Splitter moved to **546 Hz**; Highs chain = **Bode** → Convolve (Vocal Hall); then Compressor → Distortion (**Overdrive**). Noise osc loaded with "Oddprophet Toolkit – High (7)".
