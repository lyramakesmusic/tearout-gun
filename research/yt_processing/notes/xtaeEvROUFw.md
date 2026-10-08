# xtaeEvROUFw — MAKING THE HEAVIEST TEAROUT GUN (sound like NIMDA, yvm3)
Channel: DPS AUDIO | 11:57

**Relevance:** High. Full tearout gun inside Serum 2: stacked overdrive, spectral osc, split + convolver on highs, diffuser filter drive, notch EQ ~822 Hz, pitch envelope.

## Master (context)
- Pro-Q 3: **everything below 130 Hz mono**; then GClip at default settings.

## Serum 2 chain (in order)
1. Osc A: Analog **sine**, warp/drive param "dropped to 3", level/drive "up to **75**" "because tearout is quite loud … distorted crunch early" [inferred: a waveshape/warp amount].
2. FX: **Distortion: Overdrive, stacked (two overdrive instances)** — "meta for Serum 2 … this stacking overdrive is insane".
3. Osc B: **Spectral mode** loaded with a tonal snare/clap/perc sample (mid–high content), **-1 octave, +7 semitones**; warp FM from A (sine) for extra crush without added volume; **scan rate ~118** (speed through sample).
4. **Splitter** (Serum 2 multiband split): highs → **Convolver, "Tight Plate" IR** (placed before HyperDimension so the reverb gets widened) → **Hyper/Dimension** with hyper amount at 0, dimension mix/size up (stereo highs only).
5. **EQ before the filter**: narrow peak, swept to find an extra harmonic, boosted at **~822 Hz** ("you'll see this done a lot by tearout producers").
6. Filter: **S2 "Diffuser"**, cutoff near sub frequencies, **drive up** (≈ another distortion; equivalent to routing to a bass bus with Fruity Waveshaper saturation).
7. Compressor: **Multiband** (OTT).
8. Pitch envelope: LFO2 (envelope mode) → main tuning ~**8%** (classic gun pitch drop).
9. Unison random phase down. Small high shelf boost (was muddy).
10. Delay: **-35 ms** (negative offset / very short) [inferred: haas/ short slap].
- Layer 2: clone, remove sub, **+1 octave**, faster scan, swap filter to **Combs** for high-end character, lower level.
- Post: only bass bus with reverb + sidechain.

DSP: sine through two cascaded overdrive waveshapers (high crunch), FM by sine on resynthesized spectral perc; band split with short plate convolution on HF only; resonant peak ~820 Hz into a driven diffusion (allpass-ish) filter; 3-band OTT; pitch env ~ a few semitones down over the hit.

**Techniques tags:** overdrive_stack, waveshaper, hardclip, softclip, convolver, reverb_on_bass, chorus, mid_boost, eq_before_dist, disperser/allpass, OTT, multiband_comp, pitch_env/gun_transient, comb, layering, mono_low/sub_separate, spectral/warp, sub_sidechain_or_split

## Quotes worth keeping
- "everything below 130 Hz is mono"
- "put this on overdrive. A lot of people would call this meta for Serum 2 at the moment. This stacking overdrive is insane."
- "our equalizer is there at around 822 hertz"
- "the absence of this convolver is quite noticeable"
