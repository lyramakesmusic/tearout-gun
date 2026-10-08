# ewMTlw6VeAM — Letsynthesize, "Skrillex snare drum tutorial (only FM8), part 03" (tail and top)
Archetype: fully-synth Skrillex-style snare (claims Skrillex used his snare on a Schism remix). FM8 values said. See frames.

## Snare = 4 layers (said): **transient, "bounce/bottom" (FM8 or Virus), tail (two FM8 patches: bottom-mid + top), top (hi-hat-like)**.
## Top / hat layer FM8 (said)
- Op1 **sine ratio 5**, Op E **square ratio 64** (max), third **triangle ratio 2**; short decay/release envs; **HPF**. Post: **frequency shifter** ("gives a lot more metallic"), auto filter, HPF, EQ.
## Tail layer 1 FM8 (said)
- Op **sine ratio 37**; op **"1+3+5" square, ratio 64**; saturator; **operator self-feedback (self-oscillation -> noise)**. Pitch envelope on every patch (small transient). FX: LP with high res (disabled), saturator ("gives the metallic sound"), EQ, freq shifter.
## Tail layer 2 FM8 (said)
- B: ratio **21.1**, very short decay, no sustain; C: **sine ratio 18.6**, very short decay; both into **X (noise op)** + saturator -> Z filter with short filter env; short pitch env. FX: HPF, EQ boosting upper-mids.
## Rules (said)
- For noisy/metallic tops: **lots of operator feedback, very high (non-integer) ratio modulators (18.6, 21.1, 37, 64), saturator, HP filters, frequency shifter (pushed down)**.
## Takeaways
- Synth "noise" for snare tails can be FM noise with inharmonic high ratios rather than white noise — gives metallic content. Two tail layers split by band (mid noise vs top).

## On-screen (frames/ewMTlw6VeAM, 175 BPM)
- Arrangement shows the 4-layer stack, **all clips start together at 0 ms** (layering offset handled inside envelopes): Transient Bounce (~5 ms), Transient (~40 ms), Top (~160 ms), Bottom FM8 (~75 ms), Bottom Virus audio (~65 ms, visible as ~15 cycles of a decaying tone), Snare Tail (~330 ms with fade).
- Tail FM8 "TAIL_01": operator ratio **64.000**, waveform "1+3+5 Square", short decaying env; post chain Auto Filter (band, env-follower), Saturator in **Waveshaper** mode, EQ Eight high boost.
