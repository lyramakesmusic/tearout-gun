# KkXF4T_WPbY — "SERUM TEAR-OUT GUN BASS TUTORIAL [Phaseone/Svdden Death/Marauda]" — Wheysted Music (13:14)

Serum. Usually he Frankensteins gun samples; here all-Serum. Good envelope description + **inverted FM sweep** idea.

## Envelope shape (verbal)
- "A gun: the sound is there instantly as soon as you shoot" → **LFO1 = very quick transient then goes down** (instant attack, fast fall) [on screen].
- **Pitch bend very quick as well.**
- **LFO ramp rate 1/16**, another at **1/8** ("my style is very quick").

## Body
- Wavetable important; basic/**Digital** tables work well (e.g. "Splat"?; "Has Kick" "always clutch"). Harmonic/noisy tables get crazier.
- Osc A **+1 octave**, FM'd by a lower osc B (B ~2 octaves lower than A, level 0).
- **Reversed FM sweep**: "start with a higher FM and as soon as it triggers it goes lower" — FM index high→low (opposite of usual rising FM). 
- LFO1 → WT position (movement). Noise osc (any). Serum sub lower.
- **High Notch filter** → "adds the res on the low one — the **concussive side of the gun**; really important".
- FX: distortion ("ring from the sub"), light flanger, **compression doing a lot** (lows/mids boosted), **very short "tunnel" delay ~30 (ms)** adds stereo/space; macro LFO2 → filter + **master tune** — sweeping master tune per print gives "10 different gunshots" from one patch (tune 3, 5,…).

## Post
- OTT (light) → "fat rack" → "**JK disperser = a ton of EQ Threes**" (amount, freq low/high).
- EQ high boost.

## Sub (separate, needs to bend)
- Sine, -1 oct; small **attack** because "a gunshot is a hair of mid-highs then the low end comes in"; **pitch bend very important** (static otherwise), distortion, compressor (lots of upward), LP.

## Print / group
- Print layers → EQ low boost → **Valhalla Supermassive** small-wide preset ("more metallic, wider") → **Hybrid Reverb 100% hybrid, size down, small room** → GClip → low cut. Fresh Air (brightness). Sub low cut, glued with distortion. Widened slightly but mono-compatible.
- "**No-sub shifter rack**" (frequency shift rise, keeps sub) "super important for tearout".

## On-screen (frames/KkXF4T_WPbY_*.jpg) — preset "WHEYGUN 2024", Serum, Ableton, **150 BPM, F minor**
- Osc A **"FM_Splat"**, oct **+1**, warp **FM (from B)**. Osc B **"FlangeSquare"** (alt. "Virus_WT_004"), oct **-1**, **sem -7**, level ~0 (modulator). Sub **sine oct -1** (in-patch, alongside separate sub). Noise **"CymMicBleed"**. Filter **HN 12** (high notch).
- **LFO1 (TRIG mode, 1/8 ≈ 200 ms at 150 BPM)**: smooth concave **exponential decay** — top → ~35% at ~30% of cycle → ~10% at ~60% → ~0 at end. 6 destinations.
- **LFO2 (TRIG, 1/16 ≈ 100 ms)**: same exponential-decay family → pitch/filter macro (3 destinations). Macros: Tone, FM, WT, "Make Brostep".
- **FX order**: (Hyper off) → **Distortion → Flanger → Compressor (multiband) → Delay (L 13.28 ms / R 16.67 ms, ping-pong, filter 884 Hz, Q 1.1) → Filter "Combs"**. Ableton inserts after Serum: **OTT → VR_PHAT → Disperser → EQ Eight → Valhalla**.
