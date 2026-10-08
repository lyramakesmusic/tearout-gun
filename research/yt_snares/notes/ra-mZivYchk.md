# ra-mZivYchk — Au5, "Punchy Drums with Operator"
Archetype: generic synthesized bass-music snare, Ableton Operator (4 parallel osc). Values said on-screen/spoken.

## Model (said): transient (several ms click) / body (20-200 ms tone, punch) / tail (reverb or noise).
## Layers (said, Operator parallel algorithm)
- **Body**: sine, fixed **200 Hz**. Amp: attack **50 ms**(? said "50 millisecond attack", likely ~0.5; keep as said), decay **100 ms**, sustain 0. **Pitch env amount max (Operator +48 st peak)** , pitch env decay **350 ms** ("nice sweep").
- **Transient**: **Noise Looped** (deterministic: "same attack each time"; white noise is truly random), fixed 200 Hz, attack **10 ms**(said), very short decay; then shortened further & louder. Phase knob changes noise-loop start = different click.
- **Tail**: **white noise** (non-looped, else tonal), attack **~100 ms** fade-in, sustain down. "Transient and body hit and then tail fades in."
- Glue: filter **PRD with drive** cranked, resonance low ("higher harmonics ring out").
## Processing (said)
- EQ to shape noise highs; **Glue compressor soft clip** (zero-attack flat-topping) -> "transient gets snappier and glued".
- Resample, splice and crossfade to remove the sweepy section -> "more natural".
- Reverb with **automated wet/dry**: dry body+transient, wet tail = gated snare. Reverb low-cut.
- Multiband: upward compression to lengthen the mid-band tail, upward expansion to shorten very top of tail.
- **Retune with frequency shifter** (fine), not transpose: preserves timbre/length.
- Layer 808 clap with fade-in so the snare transient cuts through, clap full-wet reverb + chorus for lush tail.
- **"There shouldn't be any gaps or significantly low points of amplitude in the drum hit"** -> boost the dip and crossfade.
- Post: HPF tuned to the fundamental w/ resonance, filter env sweeping cutoff *up* so the tail loses lows but transient/body keep them.
- Layer metallic foley for ringers; overdrive whole snare.
- Global Time knob scales all envelopes together.

## On-screen (frames/ra-mZivYchk, 540p, partly legible)
- Operator env semantics: **Initial = Peak = 0 dB** on body and transient, so the "attack" time acts as a **hold at full level** (no fade-in).
- **Body** (osc A sine, fixed **200 Hz**, level -2.2 dB): attack(=hold) **50 ms**, decay ~**100 ms** (said; display partially), sustain -inf. Pitch env 100%.
- **Transient** (osc noise-looped, fixed **100-200 Hz**, level -12 dB): attack(=hold) **10.0 ms**, decay **22.1 ms**, sustain -inf.
- **Tail** (white noise, level **-8.6 dB**): initial -inf -> attack **100 ms fade-in**, decay **~600 ms**.
- Filter LP24 "Clean" 18.5 kHz res 20% (later swapped for PRD with drive).
