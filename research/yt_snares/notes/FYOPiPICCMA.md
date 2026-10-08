# FYOPiPICCMA — Virtual Riot (stream clip via Trap Therapy), "How to create snare samples" in Serum
Archetype: VR/neuro-style heavy synthesized snare, one Serum patch + processing. Mostly said; values approximate.

## Synthesis (said)
- **"All you need is one oscillator and the noise."**
- Body: **triangle or sine, fundamental 150-400 Hz, "somewhere around 200 Hz"** (checked on EQ analyzer).
- Amp: all LFOs in **envelope mode** (curved shapes), separate volume shapes for osc and noise. Noise gets its own fade-out ("a lot of noise at the end... noise volume gets a fade").
- Body pitch mod: "comes down from a high pitch a little bit like a kick drum... but not that extreme" -> "slightly clicky initial transient".
- **Noise pitch also enveloped** (noise osc pitch automated by env), result depends on the noise sample.
- **Distortion modulated by envelope**: more distortion at the start (crunchy transient), less by the end.
- OTT (several times).
- **Swept EQ peak** (wide bell) modulated by envelope: one sweeps *up from below* to **~1 kHz**, another sweeps *down from top* to 1 kHz. "Starts with heavy transient and lots of low end, by the end it's only higher frequencies, like a whip." The snare should end on the ~1 kHz "formant" (clap region / vocal 'R').
- Keytracked: play in any key -> tuned snares. LFO envelopes tempo-synced, so changing project tempo lengthens/shortens the snare.
- "No sustain, a little bit of hold" on envelopes; "all about the envelopes and the right exponential or linear curves".

## Processing (said)
- Pro-Q2: low cut up to body, cut **~500 Hz** ("where instrument fundamentals sit... drums: low thump, nothing, then 1 kHz"), **bump 1 kHz**, cut harsh ~3 kHz. More distortion, transient shaper, **limiter clipping hard** ("OK to clip as long as transient sounds fine").
- Optional: paste first ~1 ms of another snare's (or kick's) transient on top, limiter on master.
- Frequency shifter to retune after.
## Takeaways
- Spectral trajectory: low+broad at hit -> narrows to ~1 kHz band at end. A time-varying bandpass/peak on the whole snare is a defining VR move.
- Modulated distortion amount (high at transient, lower in tail).

## On-screen (frames/FYOPiPICCMA, 150 BPM project)
- Osc A = **triangle** (Basic Shapes), Noise = **"ARP circuit"** then **"ARP white"**. Filter MG Low 12 present but off.
- ENV1 (amp, held open): A 0.5 ms, H 0, D 317 ms, S 0 dB(…), R 15 ms — the actual shaping is done with LFOs.
- LFO1/LFO2 (envelope mode, **rate 1/4 BPM = 400 ms cycle at 150 BPM**): exponential-decay shapes; LFO1 (osc level) convex decay over ~whole cycle; LFO2 (noise level) steeper, reaching ~0 by ~40% of cycle (~160 ms). LFO3 (pitch) a near-linear ramp down over the cycle (gentle).
- FX: **Distortion = Tube**, EQ (one band, LFO4 -> EQ high-band gain "VolH"), Compressor (multiband toggle visible).
