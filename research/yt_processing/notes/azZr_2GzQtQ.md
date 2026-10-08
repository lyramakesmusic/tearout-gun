# azZr_2GzQtQ — How to make metallic Riddim Dubstep bass in Serum (PHENOMSOUND, 9:00)

**Relevance:** Medium-high. Metallic riddim bass, in-Serum FX with a few numbers; "reverb filter" is the metallic element.

## Chain
1. Osc A "Distorted Sub" table, level 0, FM from B; Osc B basic CJW, +1 oct +1 semi, level 0. LFO1 (heartbeat curve) → A WT pos, A level, FM amount (~1 o'clock), B WT pos (sync).
2. **Filter HP12**, cutoff ~1 o'clock, LFO1 bipolar → cutoff, res ~9 o'clock, drive ~2 o'clock.
3. Noise "bright", LFO1 → level (cymbal-ish). Sub osc on, LFO1 → level.
4. FX: **Hyper/Dimension** (LFO → mix/dim) → **Distortion soft clip, drive max** → **EQ bell +12 dB, LFO-swept frequency** (two bands, LFO → both freqs) → **Filter: Reverb filter** (Serum comb/allpass-network filter; "the magic of riddim"), LFO → cutoff, res ~11 o'clock → **Compressor multiband** with gain boost.
5. LFO2 → master tune +2 semis (pitch slide), → B sync & WT pos.
6. Mixer chain: **Fruity Multiband Compressor gain all the way up** → **transient processor, release ~8–9 o'clock** (gates tails) → **Maximus mid boost** → **OTT** → EQ cut lows/mud.
- Play around C3/D3 ("third octave ... D or C").

DSP: soft-clip → LFO-swept +12 dB bell (moving resonance) → resonant comb-like "reverb" filter = metallic ringing.

**Techniques tags:** softclip, comb, OTT, multiband_comp, mid_boost, filter_env, eq_after_dist, lowmid_scoop, pitch_env/gun_transient, chorus

## Quotes worth keeping
- "here goes the magic of riddim, turn on the filter and select the reverb filter"
- transient processor release "around eight or nine o'clock ... makes sound a lot cleaner and sharper, removes all the unnecessary tails"
