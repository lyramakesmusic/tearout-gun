# pzxGMhXMt9s — Zens, "How I Make & Design Trap Snares from Scratch (Drum Synthesis pt 1)"
Archetype: trap/hybrid-trap synthesized snare by reference-matching. Serum + FL. Said values.
- Method: analyze reference with EQ analyzer, read **two fundamentals** (here **B4 and C4** — B dominant, C quieter, a semitone-ish class apart) -> recreate with **two sines (osc A, osc B)**, relative levels matched to reference. "Snares are a tonal note stacked with noise."
- Tone envelope is a **blip** (short) shared by both oscs. White noise added for grit/body.
- Filter: everything through a **high-pass with resonance up and drive up**.
- Serum distortion: **tube** (subtle) or **soft clip** (can drive harder); EQ in Serum cuts highs (noise added back later).
- Post: bitcrusher/Decimort, then **resample at several pitches** and pick; drive into clipper ("Pogo"); stereo white-noise layer mixed **between mono and stereo** ("too stereo is weirdly wide for a snare"); **fundamental must not be too loud — "don't want it to sound too melodic, it has to stay a drum"**.
- Steep (40-48 dB/oct) EQ bands to match the reference's spectral envelope; narrow boosts on harmonics add character.
- Layer a clap for more noise; soft-clip mode gives more loudness than limiter; **Lossy (MP3-degradation)** at low mix makes trap drums sound "cleaner"; fade-in variants for fill/roll accent snares.
