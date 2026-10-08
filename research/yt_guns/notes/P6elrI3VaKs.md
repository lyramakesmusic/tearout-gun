# P6elrI3VaKs — "TEAROUT GUN USING STOCK PLUGINS - (TUTORIAL FL STUDIO)" — Silvyr (14:35)

FL Studio stock only (Sytrus). 4 layers: tail, sub, transient, white noise. Claims tearout is plugin-reliant in general.

## Body / "tail" (Sytrus FM)
- Init Sytrus; main pitch dropped. Op1: added harmonics; **LFO → pitch**, **envelope → pitch**, **envelope → modulation (FM index)**, volume envelope: "gun shape: a lot of start/percussive transient, then it fills" (no sustain). Op pitch not higher than **2** (ratio). Op2 "same thing but shorter", no harmonics. Op3 = transient copy, **pitch +15** "so it punches through".
- Insert chain: **frequency shifter** ("more atonal, evil") → EQ → distortion → **Fruity Convolver** ("underrated") → **very short delay** ("metallic") → reverb (width) → wave shaper (fizz) → compressor (OTT replacement) → multiband compressor → EQ → **Multiband Delay** ("disperser on steroids", linear phase, size **0.1x**, play with scale) → EQ.

## Transient
- Copy of the patch with only its **volume envelope** shortened (a click). Chain: transient processor → EQ low peak on resonance → same multiband-delay preset → convolver (stereo) → waveshaper. **Must hit exactly aligned** with the gun. Goes to master alone (not grouped).

## Sub
- Basic **sine**, **phase 0**, slight **pitch bend**, volume automation. Distortion → EQ (dip low mids) → EQ (loud) → soft clipper. Grouped with gun.

## Noise layer
- Sytrus 3 oscillators → white noise, volume automation follows gun's volume curve; distortion + 2 EQs.

## Group
- Patcher: **high-passed reverb** for width + EQ high boost.
- With 3rd-party: Multipass (multiband distortion with a little **bias**), dispersion, clipper, EQ, imager, clipper.

## Pattern
- Resampling then re-editing the audio gives "liquid" variants.
