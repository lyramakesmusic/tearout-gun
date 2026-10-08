# P55h-jIKinc — "The heaviest TEAROUT basses & how to make them!" — Code: Pandorum (13:13)

Serum 2 machine-gun preset pack breakdown. Very informative on one-LFO-drives-everything architecture.

## Sources (Serum 2)
- **Sample osc = snare** (from his "Trash Can" snare pack) — swap snares to get different guns.
- **Spectral osc** modulating the sample and also sounding.
- Wavetable osc: **very high pitched, very short** — only "supports the click".
- **Noise** osc also for click.
- Sub osc "doing weird stuff" (modulated).

## Envelope / the "hit"
- **One LFO controls everything** so everything has a hit at the start: "everything is falling down like this" (decaying shape, on screen). Targets: **sub level, FM from C → osc A, osc A volume, osc C volume, tube distortion on osc C, filter cutoff, convolver, convolver mix, delay mix+feedback, reverb, ASIM drive**. Adjusting LFO changes the impact.
- Another LFO → osc B WT position.

## Filters
- Filter 1 (off during demo); **Filter 2 = all-pass**, "pushing the sound quite a lot"; **blowout** knob beefs it up.

## FX (Serum 2)
- **Convolver** (factory IRs or load your own: snare, tonal hit) — mix automated.
- EQ: low-end boost.
- **Delay** with mix & feedback automated, **very fast rate** — changing it "drastically changes the sound pattern".
- Reverb **Hall** automated (wash).
- Distortion 1: **5-stack overdrive**. Distortion 2: **"ASIM"** (asymmetric) main distortion, mix **75%**, drive automated.
- Compressor **extremely aggressive**: threshold **-2.5 dB**? and gain **13 dB** → then **soft clip in mixer** ("overblown").
- Utility: **mono low frequencies**.
- **Unison 16 voices, very narrow spread**.

## Pattern / resample
- **Resample** guns and cut tails because built-in reverb/delay/compression make tails bleed into the next hit (sometimes bleeding is nice — drags into next sound).
- Macros for variation; pitch changes between hits.

## On-screen (frames/P55h-jIKinc_*.jpg) — preset "GUN 3", Serum 2, 120 BPM
- **Oscillators**: Sub on, oct **-2** (sine). **Osc A = SAMPLE "Snare 30"**, one-shot, **unison 16**, warp **FM (C)**. **Osc B = WT "Electric Guitar [SL]"**, oct -2, phase 180°. **Osc C = SPECTRAL "Snare 48"**, oct -2, one-shot, warp **TUBE**. Filter 1 **MG Low 12**. Macros named **Tail, Tone, Tail 2, Grit, Blowout**.
- **LFO1 (ENVELOPE, 1/4, 16 destinations)**: two variants seen. _318: smooth **exponential decay** — top → ~35% by ~7% of the cycle, asymptotically → ~0 by ~70%. _387: **fast drop to ~40% at ~10% of cycle (≈50 ms at 120 BPM)**, then a **straight gentle ramp to 0 at the end** (≈500 ms). "Everything falling down" — this one curve drives levels, FM, distortion, filter, FX mixes.
- **LFO3 (ENVELOPE, 1/4)**: very steep exponential to ~0 by ~12% of the cycle (~60 ms) — a short "click/blip" modulator [destination not visible; inferred pitch/FM spike].
- **FX chain (_387)**: **Convolve "Digital Chamber"** → **EQ** (low shelf **210 Hz +7.4 dB**, high band **2041 Hz −2.5 dB**) → **Delay Normal, L = R = 25.95 ms** (≈38.5 Hz comb), feedback ~mid, mix ~high → Reverb → Distortion → Distortion → Compressor → Utility.
