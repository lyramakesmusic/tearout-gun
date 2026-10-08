# 9H21J1q3yqA — ARTFX, "Making snares from scratch using synthesis - a DEEP DIVE into DNB DRUMS"
Archetype: modern DnB synth snare, Serum (2-3 instances). Said values — very decision-relevant. (Same creator as BaTmdjAmZwU; counts once.)

## Fundamental layer
- Analysis: real snare fundamentals are **not pure sines but "rounded squares" (like a clipped sine)**. Build it additively: sine + **bin 3 at ~35%** + a touch of **bin 5** to fill the dip, normalize. Odd harmonics only.
- **Pitch**: B2 ≈ 250 Hz ("standard for DnB a few years ago"), **E3 ≈ 330 Hz** tried, final **B3** (≈ 494 Hz... he says B, "perfect fifth above key E"). "Modern DnB snares tend to be higher pitched." **Tune to a chord tone of the track key (5th).**
- Volume LFO-env: **hold at max for the duration of the transient, then drop**; fundamental is short; **total snare ≈ a 16th note** (at 174 BPM ≈ 86 ms). DnB snares are as short as kicks (space for other sounds).
- Random phase 0 (consistency).
- **Pitch "punch" env: LFO env-mode unsynced at ~100 Hz rate (≈10 ms), hold then drop, unipolar to coarse pitch**; first version too high -> reduced to a "subtle extra pitch movement". Huge difference with vs without.
- Wants **more oscillation cycles in the fundamental** -> raise pitch rather than lengthen.
## Noise layer
- Stereo noise (or mono noise L/R offset), random phase 0. Envelope **starts with a gap: noise comes in when the fundamental ends**, curved decay. Timing is "finicky", most important to feel.
- Serum filter **HP 18 dB**, res 0.
- **Resonances from noise**: EQ with **5-6 maximum-Q bell boosts above the fundamental** (fund. at ~322 Hz; picked notes by ear, e.g. A3 relative to E3), each gain-tuned; then **EQ "scale" (global gain) automated over time: 0 during transient, rises in the tail where the noise peaks, then down** -> "sounds like there's an acoustic layer, there is none".
- Saturator drive ~4 dB on the snare group.
## Noise splash (3rd layer)
- Different stereo noise, ~1/8-note env, HPF; **pitch env falling on the noise and LPF cutoff falling over time** -> "noise of a snare starts brighter and gets darker toward the end of the tail". Boost highs, cut some mids.
## Group
- Transient Master: sustain down a bit. **Valhalla Room (Dense Room / LV1), no pre-delay, very short time, HPF'd, mixed barely audible**; "without it it suddenly sounds weak". Two saturation stages (+2 dB each): per-layer group then drum bus.
