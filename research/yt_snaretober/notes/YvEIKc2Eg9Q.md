# YvEIKc2Eg9Q — oddprophet, "HOW TO TURN ANY SNARE INTO A TEAROUT GUN" (17:18)
Source class: supplementary; snare-as-gun-source (Serum + Kilohearts Multipass). No frames.

## Synth stage (Serum) [said]
- Snare sample dropped into Serum NOISE osc, **key-track on**, **one-shot on** -> snare pitch follows notes.
- OSC A saw -> **sine at -3 octaves** = sub under the snare. Noise level max.
- Filter: **band-pass/notch combo** ("BP Notch"), cutoff driven by LFO1 in ENV mode with a "gun shape" (peak at t=0 = transient, decays into lows), drive added.
- Pitch: LFO2 (ENV mode) -> Global **Master Tune**, **unipolar** (bipolar "goes crazy downward"), short downward pitch drop.
- LFO3 "gun shape" on level to shorten the tail when playing lower notes (lower pitch = longer snare).
## Post (Multipass) [said]
- Post (full band): convolver with real-world IR, shortened + faded, "tone" knob up (cut lows / add highs); IR stretch for tone variation; waveshaper (drawn) for loudness; distortion per band, sub band processed separately.
- Pre: phaser as **disperser** (delays highs -> emphasizes transient); **frequency shifter -25 Hz** to pull new tones.
- Remove low-end stereo (mid/side EQ); boost missing treble; boost resonant tones. Goal: fill the whole spectrum.
Relevance: "snare transient ≈ gun transient": tonal element + tail; tearout guns = snare + sub + convolution + clip.
