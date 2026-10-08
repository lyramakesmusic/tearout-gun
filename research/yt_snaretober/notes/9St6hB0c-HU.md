# 9St6hB0c-HU — Tealx1, "Tealx1 on splicing dubstep snares" (4:42)
Source class: supplementary; Tealx1 is a CamaCon collaborator (K-DUB ft. Tealx1). Frames: frames/9St6hB0c-HU_{0,90}.png

## Method: splice (sequential) rather than stack [screen+said]
Render a tonal hit and a noise hit separately from Serum, place them on adjacent playlist tracks; noise clip starts **after** the tone's onset, around where the tone's initial loud cycles end, overlapping its decay. Changes PPQ (time base) 96 -> **960** for fine nudging of the splice point [said]. Records "like 40 different versions" and keeps good ones [said].

## Tone layer (Serum, frame 0:00) [screen]
- OSC A "Custom" skewed sine-ish wave, octave -1? (osc oct field shows -1), unison 1.
- Noise osc "AC hum1" enabled into filter.
- Filter **High 12** with resonant bump (HP with peak).
- ENV1: A **0.5 ms**, H **0.0 ms**, D **77 ms**, S **0%**, R **15 ms**.
- LFO2 (ENV mode, 1/4, grid 8): fast concave drop to 0 by ~25% of window -> pitch/level sweep.
- Rendered clips named G#3 / C#3 / D3 -> tone tuned to scale notes (G#3, C#3, D3) [screen].

## Noise layer [screen 1:30]
Separate Serum render (C#3), clip gain **+5.8 dB**, longer than tone (~1.5–2x tone length), slow fade.
[said] "you want your noise to be the same loudness as your snare (tone)" — hi-hat-ish noise overpowering is the failure mode.

## Processing
Insert labeled "GClip" (Gross Beat? -> likely GClip clipper) on mixer [screen]. Not discussed in detail.
