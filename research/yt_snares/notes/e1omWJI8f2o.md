# e1omWJI8f2o — QUIVILE, "Neurofunk snare from scratch in 5 minutes"
Archetype: neurofunk snare, Serum. Said values.
- Sine (bend- for variation) + stereo noise (needs good high end).
- LFO1 -> sine level AND EQ band gain. **LFO4 -> coarse pitch +12 st (starts an octave up), envelope mode rate 132 (very fast)**. LFO2 -> noise level (different shape). **LFO3 -> hard-clip distortion drive and mix** (enveloped distortion).
- Filter trims a bit of sine highs. EQ: **mid boost ~1-2 kHz ("punch")**, high shelf.
- Post: EQ HPF **107 Hz**, light comp, imager widening mids/highs, **multiband transient shaper (Oxford Spiff) huge boost** then level down, Valhalla reverb small, exciter (tube) on mids/upper-mids, **clipper** last.
## Takeaways: pitch env +12 st very fast; distortion amount on its own envelope (as VR).

## On-screen (frames/e1omWJI8f2o; project ~134 BPM)
- Osc A sine (Basic Shapes) OCT -1, coarse modulated +12.00 by LFO4; Bend- warp. Noise "Noise Stereo". Filter MG Low 12 on A only (gentle HF trim).
- **LFO4 pitch: 1/32 BPM envelope (~110 ms cycle)**: exponential fall to ~10% within ~15% of cycle (**~15 ms**), ~0 by ~35 ms.
- **LFO1 (sine level + EQ gain): 1/32 BPM (~110 ms)**: starts ~90%, **dips to ~60% at ~12% of cycle (~13 ms) — a post-transient notch**, recovers to 100% by ~25% (~28 ms), holds/declines slowly, then **steep fall to 0 at the end (~110 ms)** — a gated body.
- **LFO2 (noise level): 4.4 Hz envelope (~227 ms)**: fast rise to peak at ~10 ms, convex decay to ~35% at ~110 ms, to 0 at ~227 ms. Noise outlasts body ~2x.
- Post chain: EQ8, compressor, Ozone imager, Oxford Spiff (multiband transient), Valhalla, exciter, clipper (as said).
