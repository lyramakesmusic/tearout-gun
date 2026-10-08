# 6Kyy8fGG4B4 — Letsynthesize, "Designer/Experimental Dubstep/Tearout Snares Rack" (Ableton Operator)
Supplementary. Frames: frames/6Kyy8fGG4B4_{135,150,160,175,200,240}.png

## Architecture (Operator, two parallel stacks) [said+screen]
- **A (body)**: sine, coarse 1, level 0 dB, **FM'd by B (coarse 5) only during the transient**.
- **C (noise)**: Wave **"NoL" (noise looped)**, coarse **4** = "noise color" macro (pitching sample noise changes color) [said+screen], level **-1.9 dB**, modulated by D (D level -inf, unused).
- Operator global filter: LP 24 dB at 18.5 kHz (effectively off). Tone 70%, Volume -15 dB.

## Envelopes [screen]
- Body amp (A): A 0 ms, D 400 ms, R 50 ms, peak/sustain 0 dB (held while note on).
- **Pitch env** (dest A only, 100%): **initial +48 st → attack 5.66 ms → peak +24 st → decay 67.5 ms → 0 st**. "Trans Length" macro = 5.66 ms. Most important part [said].
- **Noise amp (C): attack 23 ms, decay 400 ms, sustain 0 dB, release 332 ms** ("Noise Tail" macro). Noise swells in ~23 ms after the hit.

## FX chain [said+screen]
1. EQ Eight: boost a noisy band = "Noise EQ" macro **1.35 kHz** ("color of the snare").
2. **Reverb — the most important part**: input lo-cut **2.59 kHz**, predelay 2.5 ms, size 1.40, **decay 491 ms**, diffusion HS 4.5 kHz, chorus on, **dry/wet 44%**.
3. **Glue Compressor** after reverb, heavy: attack 1 ms, ratio 4, **threshold -29.8 dB, makeup +20 dB**, soft clip on — squashes the reverb tail up into the snare.
4. **High-pass "HP Freq" 428 Hz**, keep in mids/low-mids [said].
5. Second compressor, then Saturator "to make it loud" [said].

## Macros (useful knob set): Noise Color, Trans Length, Noise EQ freq, Reverb Length, HP Freq, Noise Tail.
