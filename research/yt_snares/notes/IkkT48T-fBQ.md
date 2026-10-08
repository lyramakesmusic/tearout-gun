# IkkT48T-fBQ — DNB Academy (Frags), "How to Make a Hard DNB Snare in Serum 2"
Archetype: hard DnB / skullstep "ringy bang" snare, single Serum 2 patch. Said; see frames.

- **Sine, random phase 0, phase 90°** (starts at peak -> click).
- Env1 (LFO env) -> level: "big fat" shape.
- Env2 -> **global main tuning**, amount **24 then reduced to ~1-2** (st, scaled); target "harsh **180-200 Hz**" body.
- Env3 super short -> **osc A coarse pitch, bipolar, large amount** = click. Body tuned ~4-5 st by ear.
- Env4 -> noise level: **pink noise stereo**, snare-shaped.
- FX: **soft-clip distortion** light (blend layers); **EQ notch band whose gain is modulated by the pitch env** (noise color changes over time).
- **Key trick**: noise routed 100% to a bus with **Convolve reverb (short IR, e.g. "bride vocal", lots of harmonics), mix ~50%, decay very short, size reduced**; then EQ after convolve: **narrow boost picked by ear at ~1-2 kHz** = "skullsteppy bang" ringing tone.
- Play lower/higher; longer pitch env = more "acoustic".
## Takeaways
- Hard DnB ring = noise -> short convolution with a harmonically rich IR -> narrow resonant boost 1-2 kHz. A resonated noise layer rather than tonal osc.

## On-screen (frames/IkkT48T-fBQ)
- Osc A sine (Default Shapes), **phase 90°, rand 0**, OCT -2 SEM +4 (tuned body). Noise osc = **"HP12 ...stereo"** (Serum 2 high-passed noise).
- ENV1 held (A 0.5 ms, H 0, D 1.00 s, S 100%, R 15 ms) — shaping by LFO envelopes at **rate 1/4 (BPM)**.
- LFO2 (pitch -> main tuning): exponential drop reaching ~0 by ~12-15% of the cycle (at ~174 BPM, 1/4 = 345 ms -> **~40-50 ms**), steep convex.
- LFO3 (click -> osc A coarse): essentially a spike, ~1-2% of cycle (**~3-5 ms**).
- LFO4 (noise level): **linear-ish attack to peak at ~7% of cycle (~25 ms)**, fast fall to ~10% level by ~30% (~100 ms), then low tail.
- FX main: **Distortion SOFTCLIP** (drive ~1/3), Equalizer low band **210 Hz Q60** (gain 0, later modulated), high shelf **2041 -> 3013 Hz, +2.7..+4.6 dB**.
- Bus1 (noise only): **Convolve IR "Caster Big Verb", size 46%**, then **EQ peak 996 Hz, Q 84 (very narrow), +22.7 dB** = the "skullstep bang" resonance.
