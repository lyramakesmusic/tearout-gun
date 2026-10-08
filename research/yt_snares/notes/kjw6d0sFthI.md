# kjw6d0sFthI — Maulik, "How to make Punchy Dubstep Snare in Serum"
Archetype: dubstep synth snare, Serum. Said values (rough).
- Sine body ~**200 Hz** fundamental; LFO1 (envelope shape) on level.
- Click: separate LFO on **coarse pitch**, tiny/fast -> "clicky".
- Noise osc level modulated by **LFO1 negative**: noise **ducked while the sine plays**, then enters after (complementary crossfade body->noise).
- Filter on noise: **HP 18 dB**, some drive + resonance, cutoff modulated around **800-1200 Hz** -> "the car[?]/crack sound a snare has".
- EQ band ~**1000 Hz with gain modulated by bipolar LFO** (shape rising over hit).
- Reverb with **tiny size, low decay, higher mix = metallic**; mix modulated negative so fundamental stays dry and only tail is reverbed. Size 0% gives metallic plate-ish ring.
- Post: OTT 20-25%, distortion, HPF ~100 Hz, dip ~500 Hz, clip. Notes **F2** (trap snare: go up).
- Transient weak after resample -> map transient env to level of an additional osc.
## Takeaways
- Body-then-noise crossfade (noise = inverse of body env). Tiny-room reverb as metal generator.
