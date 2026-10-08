# gdY4EygN4qY — Matt Lange — "Kick Drums (and 808s) With NI FM8" (16 min) — **FM reference**
Source: transcript.
- Osc: operator F sine; key **G** (relative to D minor / C minor; resonates well in clubs); dropped an octave. Long release to kill note-end click. (S)
- Pitch env (FM8 global pitch env): amount ~**88** (FM8 units), attack segment = start point, **exponential curve** ("I really like exponential curves for drums"), short; end level exactly 0 so it lands on G. Less pitch env for the 808 version than for the kick. (S)
- **FM transient**: operator E modulates F with a **very short envelope** → "pop"; **higher ratio = higher-pitched pop**; level reduced so it's a transient not metallic tone. Reduced further for the 808 version. (S)
- Noise: operator X noise, high-passed, very short env, low level; removed for 808 version. (S)
- **Free-running operator phase → every hit differs**; he prints several and picks one. (S)
- Chain: Neve 1073 pre (slight drive) → API 550: **+6 dB @ 10 kHz, +2 dB @ 30 Hz, −2 dB @ 200 Hz** → 1176 **4:1, ~3 dB GR, medium-fast attack** (let transient through) → Pro-MB: **sub −3 dB, ~200 Hz −3..−4 dB, + highs** → Massive Passive **+8 dB @ 3.9 kHz, −3 dB @ 390 Hz** → limiter (Elevate) always on during design. (S)
- Kick/808 interaction: no sidechain; **fade the sub in after each kick with an exponential fade-in mirroring the kick's log decay**. (S)
- Length: club kick ≤ 1/8 note; 808 version long. (S)

On-screen (frames 2:20–5:50, 360p): modulator operator **E ratio set to 1.0000, then changed to 2.0000** (OS) while dialing the pop; E's amp envelope is a near-vertical spike (a few ms, OS). Pitch-envelope graph: fast drop with a strongly bowed exponential segment back to 0 (OS). Noise op X with saturator, envelope a short block (OS).
