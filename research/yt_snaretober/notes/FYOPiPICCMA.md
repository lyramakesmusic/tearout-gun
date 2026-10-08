# FYOPiPICCMA — Virtual Riot snare-from-scratch in Serum (uploaded by Trap Therapy)
Supplementary. Frames: frames/FYOPiPICCMA_{250,400,520}.png

## Layers
1. **Body**: one osc, **triangle or sine**, fundamental **150–400 Hz, "somewhere around 200 Hz"** [said]. Serum OSC A Basic Shapes triangle [screen].
2. **Noise**: Serum noise osc (ARP circuit sample at one point) [screen]; noise pitch also enveloped down [said]; "noise pitch knob" used as a variation control [said].
3. Optional: transient borrowed from another snare/kick sample on top, then master limiter [said].

## Envelopes
- Env1 (amp) [screen]: **A 0.5 ms, H 0, D 317 ms (later 256 ms), S 0 dB-ish, R 15 ms**.
- LFOs in envelope mode, tempo-synced (1/4 at 150 BPM) for per-layer volumes [said+screen]. Because synced, he changes project tempo to make the snare shorter/longer [said].
- Body pitch env: "a little, not as extreme as a kick" — slight clicky transient [said]; amount not read.
- Noise volume envelope reduced at the end (less noise in tail) [said].

## Processing
- Serum FX order [screen]: **Distortion (Tube)** → **EQ** → **Compressor**. Distortion amount enveloped: **heavy at start, less by the end** [said].
- **EQ peak around 1 kHz** ("where a clap sits, vocal 'r' formant") with an envelope sweeping a peak **upward** so the tail ends only in highs ("whip") — upward-evolving tail [said].
- After Serum: **OTT** (split ~2.5 kHz high band) [screen], **Pro-Q2**: steep low cut, small low bump, **bell +6.8 dB @ ~1066 Hz Q 1**, cut "annoying" ~3 kHz, avoid 500 Hz [said+screen 520].
- More distortion, **transient shaper** to lift main transient, **limiter clipping hard** ("OK as long as transient sounds fine") [said]. Another light OTT + saturation at end [said].
- Frequency shifter for pitch variants instead of pitch shifter [said].
