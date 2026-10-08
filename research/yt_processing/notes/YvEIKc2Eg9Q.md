# YvEIKc2Eg9Q — HOW TO TURN ANY SNARE INTO A TEAROUT GUN
Channel: oddprophet | 17:18

**Relevance:** High. Tearout producer; Serum (snare in noise osc + sine sub) → Kilohearts Multipass with convolvers, shaper, phaser-as-disperser, frequency shifter.

## Serum (in order)
1. Osc A: **sine at -3 octaves** = organic in-synth sub ("if there's no sub in your basses give up").
2. Noise osc: a **snare sample** (key-tracked, one-shot), level max, pitch up a bit. Snare transient + tonal body + tail ≈ gun.
3. Filter: **Band-Pass/Notch combo** ("most goated subtractive filter in Serum") with drive; LFO1 in envelope mode shaped like a gun (peak at hit, decay down) → cutoff.
4. Pitch env: LFO2 (envelope, **unipolar** — bipolar "goes crazy downward") → global master tuning, gun shape.
5. LFO3 envelope → level/tail shortening when pitched lower.
6. A little distortion.

## Multipass (in order)
- **Pre-FX (whole signal before band split):** **Phaser** — "acts like a disperser … delays the top frequencies down to the bottom", emphasises transient, "gooey"; plus a convolver.
- **Bands (low processed separately/cleaner — "we don't want to be processing the sub, or process it differently"):**
  - **Convolver** with Kilohearts "Real" IR category, random IR shortened + fade so it's not reverb-like; **Tone** knob up (cut lows, add highs — "extra crispy"); **Stretch** the IR for different tones; gain up. "All these guns have convolvers on it."
  - Distortion, **Shaper** (draw-your-own waveshaper) — "absurdly loud".
  - More distortion on a band.
  - **Frequency Shifter**: down **-25 Hz** on mid band for alternate tones (meh on that example; keep as option).
- Nested Multipass ("Multipass inception") in post-FX slot for global dynamics.
- Slice EQ: remove **low-end stereo** (mono lows).
- Fill the spectrum: frequency response should "fill up every single frequency"; boost missing treble; boost resonant tones.
- In-synth sub is fine; separate sub layer not required (disagrees with others).

DSP: snare = broadband transient + resonant body; bandpass/notch envelope sweep; phaser (allpass stages) as dispersion; short faded IR convolution ≈ fixed resonant coloration; waveshaper; frequency shift (SSB) -25 Hz makes partials inharmonic.

**Techniques tags:** multipass, convolver, disperser/allpass, phaser/flanger, waveshaper, overdrive_stack, freq_shifter, pitch_env/gun_transient, filter_env, mono_low/sub_separate, sub_sidechain_or_split, eq_after_dist

## Quotes worth keeping
- "if you're listening to tearout all these guns have convolvers on it it is super important"
- "phaser … it basically acts like a disperser … it delays the top frequencies of the spectrum … all the way down to the bottom"
- "we want to make sure that our frequency response of the gun is pretty much filling up every single frequency that's how you get things to sound big"
- "do I need a separate sub … if it's fine in the synth you don't need to"
