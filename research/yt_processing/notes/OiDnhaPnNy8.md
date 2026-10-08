# OiDnhaPnNy8 — How to Tearout Dubstep like IVORY & CALCIUM - FL Studio 24
Phazen · 13:31 · FL Studio track breakdown ("Transmission")

**Relevance:** Partial. Tearout track breakdown; bass sound design is thin on numbers but documents a freq-shifter+delay comb texture and a live-resample loop. Master-chain section excluded per scope.

## Bass chain(s), in order
1. **"RX click" texture layer:** source = sounds made via "RX8 de-click trick" [inferred: running iZotope RX De-click aggressively on a synth to generate artifacts] → **MFreqShifter** (MeldaProduction) with internal delay feedback ~**70** (%) and delay time **~20–30 ms** → Serum patch (saw + noise, LFO'd EQ peaks, distortion) → "fat rack" from Discord (OTT/distortion rack, contents unspecified).
   - DSP: frequency shifter with short feedback delay = barber-pole / spiral comb (each recirculation shifted by Δf), i.e. inharmonic metallic comb at 1/(20–30 ms) ≈ 33–50 Hz spacing.
2. **Mono texture layer:** same sound, no movement, mono-only, for texture.
3. **Gritty layer:** saw + noise in Serum.
4. **"Switch" sound:** Serum saw with unison, **width and randomness = 0**, comb filter for texture → MFreqShifter with *modulated* shift + delay with feedback → fat rack.
5. **Variation section:** **MFreeformPhase** ("like a Disperser but draw your own shape") — user-drawn phase-vs-frequency curve = arbitrary allpass/group-delay shaping.
6. **Machine gun variation:** compressor set to make it "more transient heavy" (slow attack [inferred]) + "crunchy solid" texture layer.
7. **Feedback noise texture:** Serum + lots of delay + OTT.

## Resampling workflow
Serum → dedicated mixer track → effects → **Edison** recording constantly; stop, drag the take to playlist, chop volume, mute FX/MIDI to save CPU. Live-resample loop.

## Sub
Pitch automation ±**2 semitones** up/down on sub.

## Techniques tags
freq_shifter, comb, disperser/allpass, resampling, layering, OTT, unison, mono_low/sub_separate

## Quotes worth keeping
- "M frequency shift and the delay was set pretty high like at 70 and the milliseconds were pretty low around 20 to 30."
- "M Freeform Phase which is like a disperser but you can draw your own shape in it"
