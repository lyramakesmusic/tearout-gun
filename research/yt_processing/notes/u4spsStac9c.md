# u4spsStac9c — Why You Should Resample Your Basses
Channel: Kermode (on-screen "Komodo"/"Cremona" in captions) | 10:53

**Relevance:** Low-medium. Dubstep/bass producer explaining *why* to bounce basses to audio; editing workflow, not a crunch chain. No processing numbers.

## Chain / workflow (in order)
1. Bounce bass to audio at writing / mixdown stage ("commit"; frees CPU so more FX can be stacked after).
2. Manual fades at start/end of each bass hit instead of relying only on sidechain compression — removes clicks (compressor attack too slow) and sub tails overlapping the kick. Goal: "milliseconds of dead air" between hits (cites Sultan, Tasaki-style tightness [unverified by speaker]).
3. Phase alignment (last step, audio only): tune kick to same note as sub; nudge bass so its first cycle aligns with kick; polarity-invert kick if cycles are opposite. Any processing after alignment breaks it (plugin latency/phase shift).
4. Post-bounce edits: transpose, octave pitch drop via warp, stutter chops, reverse, per-chunk gain levelling (turn down an over-loud transient chunk), fades between chunks.

DSP: per-note amplitude envelope with short fade-in/out (a few ms) on the rendered audio; sample-offset alignment + polarity flip between kick and sub.

**Techniques tags:** resampling, sub_sidechain_or_split, pitch_env/gun_transient, utility_gain

## Quotes worth keeping
- "I almost always now resample my basses at the mix down stage"
- "manually fading sounds out of the way can make songs and sounds so much tighter"
- "if you have any processing on your sound after you do this it's going to change the phase of the sound"
