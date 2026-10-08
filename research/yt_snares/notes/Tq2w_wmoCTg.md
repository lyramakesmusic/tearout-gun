# Tq2w_wmoCTg — DNB Academy (Frags), "Making a Neurofunk Snare"
Archetype: neurofunk tight tuned snare, Serum (two instances). Said values. Same creator as IkkT48T-fBQ.

- Neuro snare must be **precise and tuned**; keep fundamental on MIDI to retune.
- **Body/transient instance**: sine, random phase 0, **phase 90°**, amp attack **~0.5 ms**. Env1 -> volume (typical snare shape). Env2 -> **coarse pitch bipolar, amount 24** (body drop). Env3 tiny/short -> coarse too (click). Transient "snap area" ~**2-3 kHz**.
- **Noise instance**: "Bright White"/"ARP White", envelope with a **release tail** (not closed abruptly — more natural). **Noise timing: "should not play on top of the transient, but not too far apart or it sounds fake"** (slight delay).
- RC-20 noise (white/pink) with "follow" envelope; HPF ~200 Hz.
- **EQ notch->band with gain enveloped, ~2 kHz, Q ~37%** -> "clappy sound... the felt of the snare with springs hitting the bottom".
- Glue: **waveshaper 100% wet on the sum** ("two layers not glued, use a waveshaper"). Then duck noise.
- Resample, play note **F**. Shorten and pitch up for old-school "mathewy" snare.
## Takeaways
- Noise onset slightly after transient (few ms). Waveshaping the summed layers is the glue.

## On-screen (frames/Tq2w_wmoCTg; 172 BPM, LFO rate 1/4 = ~350 ms cycle)
- Noise instance: noise "AC hum?"/white, LFO1 noise envelope: **near-vertical rise to peak at ~2-3% of cycle (~8-10 ms), then concave fall to 0 by ~15% (~50 ms)** with a short release tail added.
- SPAN of body/transient: **narrow spike at ~180-200 Hz**, flat broadband plateau ~300 Hz-10 kHz, roll-off above ~12 kHz.
- Serum EQ on noise: band (notch->peak) with gain on LFO2 (triangle-ish rise/fall over the 1/4 cycle) around 2 kHz.
