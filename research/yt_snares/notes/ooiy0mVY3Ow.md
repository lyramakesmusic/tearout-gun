# ooiy0mVY3Ow — W. A. Production, "Make Your Own EDM & Dubstep Snare From Scratch" (part 1 of 4: theory + bottom)
Archetype: big EDM/dubstep synth snare, Ableton Operator. Said values.

## Theory from analyzing a reference snare waveform (said)
- **Bottom** = looks like a *square or saturated sine*, flat tops, becomes smooth (low-passed) at the end -> **square wave + LPF with envelope closing** over the body.
- Bottom has **pitch bend at start** (first cycles longer period... note: he describes first cycle as lower freq then transient; implements pitch env *down* from high).
- **Transient** = separate click; start of bottom is delayed relative to transient: put a **few ms delay (100% wet, feedback 0) on the bottom** layer.
- **Tail** = noise *with metallic content* (like splash/hat/crash layers in sampled snares) — plain white noise sounds like "just white noise".
## Bottom synthesis (said)
- **Square wave at F2 (~87 Hz)** — "snare bottom always placed a little higher than the kick"; later transposed +2 st (G2 ~98 Hz). Reference snare bottom peak at **E2/~170 Hz** (he said "e2 ... around 170 Hz", inconsistent; ~165 Hz = E3).
- LPF with envelope (env amount up, short decay) so the square smooths into a sine-ish tail.
- Pitch env: initial up, **peak +24 st**, short attack/decay tuned for click.
- Saturation *before* EQ; EQ cuts highs. **Warns: cutting the very lows of the snare can ruin the phase/punch** — keep them.
- Multiband: split ~200 Hz / ~300-400 Hz, **saturate only the 200-400 Hz mid band**. Drum Buss transients up (no crunch).

## On-screen (frames/ooiy0mVY3Ow) — reference-snare waveform + Operator values
- **Reference dubstep snare waveform (timeline 0-110 ms)**: flat-topped square-ish body oscillation, ~11 cycles from 0 to **~47 ms**, peak spacing ~4.0 ms early -> ~4.5 ms later => **body ~250 -> ~220 Hz (small downward glide), clipped/squared**. Noise grows in from ~30 ms, dominates from ~47 ms, decays to ~0 at **~120 ms**. Peak amplitude roughly constant through the body (no decay during body: clipped).
- Operator (osc A square-ish, osc D level -12 dB visible): **pitch env Initial +48 st, Peak +24 st, attack 1.75 ms, decay 600 ms, sustain 0 st**, release 50 ms; pitch env amount 100%.
- **Filter: LP 24 dB "Clean", freq 119 Hz (later 365 Hz), res 20-31%; filter env A 0, D 40.3 ms, S 0, envelope amount 60%** -> square smooths toward sine over ~40 ms.
