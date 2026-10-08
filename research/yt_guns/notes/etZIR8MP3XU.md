# etZIR8MP3XU — "TEAROUT GUNS: MULTIPASS MAKES THEM FAT" — oddprophet (11:28)

Serum (shape only) → Kilohearts Multipass (where the sound is made). "Guns of the Unholy 4" pack. Highly influential (credited by Code: Pandorum as multipass master).

## Serum stage (shape only, no Serum FX)
- Any wavetable (Cymatics WTs), osc octave **-3**.
- **LFO1 in envelope mode**, drawn gun shape (on screen) → amp/level [inferred → level] .
- Filter: **"band pass, low pass, high pass shoved together"** (Serum multi-mode e.g. LBH filter) with **LFO1 → cutoff** → creates the transient: **"high frequencies come first, then it comes down into the low"** (descending filter sweep per hit). Drive cranked.
- **Pitch bend is "really important with guns — emphasizes the transient"**: LFO2 (envelope mode) → **Global master tune**, bends one way (down) [on screen].

## Multipass stage (band-split)
- Purpose: different processing per band — protect the sub, **process transients differently**.
- Distortion → **Phase Distortion** ("feeding the sound into itself, almost like FM feedback") → **Wave shaper** → **Convolver** = "most important ingredient — where you get most of the gun sound". IR from **"Spaces – Real" reverbs** made **way shorter**, fade-out shaped, **tone up**. Then more distortion.
- Convolver adds stereo width → keep it off the low bands (move band split / mid-side EQ).
- Optional: **frequency shifter before phase distortion** for variations; second convolver using **"Phases"/"Phaser 1"** IRs (acts like disperser → "whip", adds to transient); IR **stretch** param to set length; **small convolver on the transient band** with IR start positioned **halfway into the sample** (initial attack of IR is dry, later is wetter).
- EQ: boost mids/presence, cut excess.
- Result "has sub in it — don't even need a separate sub".

## Variation
- Longer LFO1 → turns gun into stab → sustain.
- Mix-and-match Serum patches and Multipass presets for variety.

## On-screen (frames/etZIR8MP3XU_*.jpg)
- **_53**: Serum init build. Osc A WT **"Get It Noisy"** (Cymatics), oct **-3**. Filter **MG Low 12**. Env1 A **0.5 ms**, H 0, D 1.00 s, S 0.0 dB, R **15 ms** (default-ish). **LFO1 shape**: starts at max at x=0, **near-vertical drop to ~20% within the first ~5–8% of the cycle**, then a long, nearly linear slow fall to 0 at the end of the cycle (a "kick-like" fast-decay with a long low tail). Rate **1/4** (BPM on), grid 8. Being drawn in Off/Env mode.
- **_115**: Matrix: **LFO1 → A WTPos** (positive, moderate), **LFO1 → Fil Cutoff** (positive, larger), **LFO2 → Mast.Tun** (small). LFO2 still default triangle at that moment (he then redraws a one-way bend).
- **_203/_346/_390/_444**: Multipass layout: **3 bands: <250 Hz | 250 Hz–3 kHz | >3 kHz** (later moved to **<424 Hz | 424 Hz–3 kHz | >3 kHz**). Low band empty (sub untouched). **Mid band: Frequency Shifter (165 Hz) → Phase Distortion → Convolver (IR "Garage Ramp…", fade-out 100%) → Distortion**. High band: **Distortion (Overdrive)**, later + convolver ("Empty Apartment Bedroom", Spaces Real). **Pre-FX convolver** with IR **"Phaser 1"** (Phase folder), stretch **40%**. **Post-FX: Distortion Overdrive** on the whole sum. Fruity WaveShaper after Multipass in mixer.
- **_582** (pack preset "Basic and Useable"): Osc A **"Basic Mini"** oct -3 with **FM (from B)** warp; Osc B **saw**, oct -2 (modulator); Filter **BN 12** (band/notch type) with a visible notch+peak response. LFO1 in **ENV mode, 1/4**: starts at max, **concave fast drop to ~45% by ~20% of the cycle**, near-flat plateau ~45–55% to ~85%, then a **steep drop to 0 at the end** (a "hold then cut" envelope — the gun's body gate).
