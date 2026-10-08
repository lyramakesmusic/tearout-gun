# Xgr39T5MXwg — SERUM 2: This NEW CONVOLVER Trick BREAKS Dubstep (Multiband Delay IR Hack)
Channel: oddprophet | 12:00

**Relevance:** High. Tearout/riddim producer; custom "dispersion" impulse responses for the Serum 2 Convolver — the "whip" on guns.

## The IR trick
- Origin: Phase Plant convolver "phaser" IRs used on guns. Those IRs delay top frequencies relative to bottom (frequency-dependent delay) → "disperser whipping sound".
- Make your own IR: FL **Fruity Convolver "trigger impulse"** (a click/Dirac) → **multiband delay** where each higher band is delayed more than the lower (top delayed from bottom; shape drawn as rising delay vs frequency; keep total time short — "too long on the scale" sounds bad) → set **linear phase** → record result (Edison) → that's the IR. Drag into Serum 2 Convolver.
- Result looks like a frequency sweep (chirp) in the spectrogram: "you can see that laser coming through".

## Riddim patch chain (Serum 2)
Square-ish wavetable ("Square 4") → Distortion → split (multiband) → **Convolver #1: dispersion/"phaser" IR on high band** → **Convolver #2: reverb IR (fattening), size down** → Compressor → sub added.
- Convolver **Size** knob resizes IR with few artifacts (some glitch when moved = sometimes good).

## Gun patch
Wavetable → Distortion → split → Convolver with dispersion IR, **mix 100%** ("very important") and **gain up** → second convolver → notch filter → **pitch bend on amp/main tuning** (pitch env). Toggle: "no whip" vs "whip" — transient becomes "squelchier" and pops out.

DSP: IR = chirp (group delay increasing with frequency) ⇒ convolving = dispersive allpass-like filter (Disperser-equivalent but arbitrary group-delay curve); Size = time-stretching the IR = scaling group delay. 100% wet = pure allpass phase smear.

**Techniques tags:** convolver, disperser/allpass, reverb_on_bass, multipass, overdrive_stack, pitch_env/gun_transient, multiband_comp, sub_sidechain_or_split

## Quotes worth keeping
- "this phaser is basically delaying the top frequencies from the bottom … it creates this disperser whipping sound"
- "you need to also set this to linear phase"
- "put the mix all the way up. That's very important. And put the gain up as well"
- "If I turn this off, no whip. Turn it on. And that transient is now coming out."
