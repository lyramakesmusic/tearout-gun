# FeWnYudwCno — "How to make high-pitched machine gun Dubstep sound in Serum" — PHENOMSOUND (6:46)

Serum; high-pitched "squeaky" rapid wobble/gun (Kompany/"bom" style per captions). Not tearout-heavy but a clean rate recipe.

## Body
- Osc A: wavetable **"Trilobyte 3"**, octave **-2**, level down (driven by LFO).
- Osc B: sine, octave **+2**, semi **+7**, level down, **FM from B** on osc A automated ~**50%**.
- Noise osc: "Sim Mic Bleed", pitch **70**, level **30**.
- Filter: **LP12**, cutoff automated (~80), res 0, drive **30%**.

## Pattern / retrigger (the "gun" part)
- **LFO 1**, custom decaying shape, **retrigger (Trig) mode**, **1/8 triplet** rate, mapped to osc A **level** and osc A **wavetable position**. => the rapid-fire comes from an LFO gating level + WT pos at 1/8T. [the "triplet" sub-division is the rhythm signature]
- Pitch: Matrix LFO 2 → **master tune**, amount 1, **bipolar** — small pitch movement "makes it more aggressive".

## FX (Serum)
- Distortion **Tube**, drive **80**.
- Compressor **multiband**, gain ~**15 dB**.
- Delay, BPM unlinked: L **10 ms**, R **15 ms**, **ping-pong**, filter freq **3200 Hz**, Q **2.4**, feedback **60%**, mix **30%** — "does a huge amount of the job": a very short stereo delay = comb/metallic width.
- Reverb **Plate**: size 20%, predelay 0, low cut ~40%, high cut 50%, damp 50, width 60%, mix 20%.

## Group/master
- OTT (downward 20%), EQ to cut unneeded frequencies, maximizer.
