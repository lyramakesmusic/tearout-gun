# vhciw1HgExE — "How to make machine gun Dubstep bass in Serum" — PHENOMSOUND (10:15)

Serum; "choppy, punchy, metallic" machine-gun bass. Very numeric.

## Body
- Osc A: wavetable **"Red"** (Digital category), octave **-4**, level down (LFO-driven).
- Osc B: **Basic MG**, octave **+2**, level down; warp **Sync (wrap mode)** ~**1.6%** + LFO1; **FM from B** on osc A full, modulated by LFO1 (dragged down = negative/decaying amount).
- Noise: **"JP106"**, pitch **75%**, level automated.
- Filter: **comb**, cutoff **93** with LFO1 **bipolar 32**; res down + LFO1; drive **15%** + LFO1 ~30. "Very crucial — any slight change changes the sound dramatically" (comb cutoff = pitch of the metallic ring).
- LFO2: custom shape, rate **1.5 (bars?)** → osc B sync = slow variation across hits.

## Pattern / retrigger
- **LFO1** custom pre-saved shape [on screen], **1/8 triplet**, → osc A level, WT position, osc B sync, FM amount, comb cutoff/res/drive, hyper mix. The whole gun is one LFO hitting every parameter at 1/8T.

## FX (Serum)
- Hyper/Dimension: size and mix down, LFO1 → mix 35–40%; dimension mix 20% + LFO1 50.
- Distortion **Soft Clip**, drive **70–80%**.
- Flanger default, mix **30%**, (second instance/phaser?) 15%.
- Delay unlinked: both channels ~**10 ms**, filter ~**6000 Hz** / 500 [ambiguous], mix **40%** — "nice trick to add a metallic rhythmic flavor".
- Compressor multiband, gain **~10 dB**.
- Reverb **Plate** size **5%**, predelay 0, high cut and damp down, width full, mix **10%**.
- Matrix: LFO2 → master tune, amount **24** (pitch movement).
- Serum master volume maxed.

## Group
- OTT → EQ (low removal) → OTT → EQ → small reverb ("Charm Reverb", free). Layered with separate fat sub.

## On-screen (frames/vhciw1HgExE_*.jpg) — Serum, FL, 140 BPM
- Osc A WT is **"Wraith"** (captions said "Red"), oct **-4**, warp FM (from B). Osc B **"Basic Mg"** oct +2, warp **Sync**. Noise **"J106"**. Filter **Combs** (drive, damp, mix used).
- **LFO1 (TRIG, 1/8 triplet ≈ 143 ms at 140 BPM; 10 destinations)**: brief flat at max for ~5% of the cycle, then **concave exponential drop to ~25% by ~35%**, a gentler tail to ~15% at ~60%, reaching ~0 at the end.
- **LFO2 (TRIG, 1/4)**: plain **rising linear ramp 0 → max** across the cycle (→ osc B sync), so each hit sweeps sync upward.
- FX (_421): Hyper/Dimension → Distortion → Flanger → **Phaser** → **Delay ping-pong, L 10.89 ms / R 13.11 ms, filter ≈ 6.5 kHz, Q 0.8** → **Compressor multiband, gain 11.5 dB**.
