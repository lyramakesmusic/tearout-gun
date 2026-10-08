# WFGs91vSpIw — disphing, "the only drum sound design tutorial you'll ever need*" (9:16)
Source class: supplementary. Serum. Frame: frames/WFGs91vSpIw_178-188.png (4-up: short body+long tail / higher pitch / filtered noise / distortion).

## Snare architecture [said]
"EDM snare = tonal component (body) + noise component (tail)". Optional foley layer on top (on-screen title card "sine layer + noise layer + foley"). Example rendered file name `DSPHNG_snare_shawty_C#.wav` -> body tuned to **C#** [screen].
- Body: OSC A sine (Basic Shapes) [screen]
- Noise: Serum NOISE osc, **"BrightWhite"** (Analog folder) [screen]; he warns Serum's stereo noises have filtering baked in; FL 3xOsc noise is stereo [said].
- Optional slight pitch env on body "to make it more zappy" [said].

## Numbers [screen]
- ENV1 (amp): A **0.5 ms**, H 0, D **314 ms** (variant) / **380 ms** (variant), S -inf dB, R 15 ms.
- LFO1 in ENV mode, grid 8, 1/4 bar rate: fast-attack linear/curved decay shape — used to shorten body vs tail ("short body + long tail", "long body + short tail" variants).
- Noise through Filter **High 18** (HP 18 dB/oct), low-mid cutoff [screen 0:188].
- FX: Distortion **Tube**, Drive **49%** [screen].
- Variant "higher pitch" = same patch, higher key.

## Kick (context) [said]
Kick = sine + pitch env; too-high env range -> "laser"; HP at ~20 Hz or shorten env; shorten initial transient.

## Clap [said]
Noise + volume env with several short retriggers then longer tail (mimic 808 clap waveform), then distortion + reverb.

## Metallic [said]
Valhalla Supermassive with tiny size, warp/density max, mod rate 0, high feedback -> metallic resonance; FL Multiband Delay linear-phase + feedback; detuned square HP'd = "808 hat noise" alternative.
