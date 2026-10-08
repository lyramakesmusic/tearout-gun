# zybsXNyqJsc — CamaCon, "Making Snares is STILL FREE and EASY" (Snaretober 2025, part 2)

Uploaded 2026-03-27, 25:58. The rules are the same as part 1: free plugins only, Vital plus Melda free plugins plus Convology XT, OTT and kHs Clipper. The first half was recorded with a webcam mic and has a HyperCam/meme overlay, so the transcript is thin and the frames are partly covered.

Sources:
- [said] is from the transcript.
- [screen] is read from 1080p frames (`frames/zybsXNyqJsc_<sec>.png`).
- [hint] is OCR of the FL hint bar. See `ocr/zybsXNyqJsc_fl_hintbar_params*.txt`.

## Day 10 — "white noise stuff" plus chord-unison texture (1:21–3:45)
- Vital "Drum Starter" preset.
  - **Osc1:** sine body.
  - **Osc2:** white-noise wavetable at −36 st.
  - **Osc3:** "Harmonic Series" wavetable at **−12 st, 16 voices, 0% detune**.
  - **Sampler:** white noise.
- Filter 1 is an **Analog 24 dB band-pass** [screen 112].
- **The osc3 unison stack is set to "Minor Chord"**, the osc2 unison to the "Unison" stack at detune range 48, and the table spread to 99 [screen 205].
- [said] "It's got more of a chord tone in it. The higher you go, unless you blend it in, it starts to be more textural."
- Osc3 unison frame spread swept 20–126 [hint 1:40–3:24]. Osc3 blend about 4% [hint 3:21], so the chord tone sits very low in the mix.
- A text overlay notes that Vital's spectral filter can make custom filter shapes [screen].
- Played at D6 / E6 in FL naming, i.e. **D5/E5 in Vital naming, a high tonal snare** [screen 145, 205].
- Body envelope is a fast attack with a roughly 1-second linear-ish LFO (1.037 s) used as a slow sweep [screen 205].
- Chorus with 16 voices on "Freeze" tempo [screen 145].
- Mixer chain: **Convology XT** (vintage delay IR "German Echo Super B / Oil Can Delay") → **OTT** → **kHs Clipper** [screen 172].
- Second variant: "a keys jingling" foley layer was blended in [said 3:20].

## Day 11 — old-school "terror" snare with long tail, FM (3:49–7:30)
- [said] "I want to FM… set it to trigger. If you go on the phase, we are doing phase modulation which is what most digital FMs do."
- Matrix [screen 282]:
  - **LFO1 → Osc1 Transpose**. This is the pitch-drop transient.
  - **LFO2 (Sin, Trigger mode, keytrack 17 / 0, i.e. audio-rate and key-tracked) → Osc3 Phase**. This is phase modulation by an audio-rate modulator at a fixed ratio.
  - **ENV2 → Modulation 2 Amount**. The FM index is enveloped by the body envelope.
  - **ENV2 → Osc3 Level**.
- [said] "Play with the ratio… maybe band-pass it… maybe you don't want too much stereo unison."
- "Want something pretty long for the body" [said]. The body is long here, unlike day 1.
- Then a filter **in diode mode**: "Just smash the [heck] out of it" [said]. Then Convology again for the tail.
- Filter 2 is a 12 dB filter with its drive used as distortion [screen 330]. On-screen text: "Vital's filters have different unique distortion modes!"

## Day 12 — "big old plank", comb stacking on noise plus convolution (7:35–12:50)
- [said] "Last time I was doing a bunch of comb filter stacking on noise and then a lot of convolution… keep it relatively simple."
- Noise wavetable at −36 st, 16 voices, into **two comb filters in series**: Vital "Comb : Low High Flange+", Filter 1 into Filter 2 [screen 495].
- **Comb cutoff 79 Hz, resonance 75.7%** [screen 495 tooltip].
- **The noise amp envelope is a very short burst**, decaying in roughly 10–20 ms per the envelope display [screen 495]. The comb does the ringing.
- [said] "You don't want a very clear tone to it, cuz then it sounds really goofy. You want to compress it and make it interact so it sounds kind of atonal."
- FX:
  - High-pass "to get rid of all that low-end gunk".
  - MConvolutionEZ "sound design / vowel" IR, frequency-shifted because the IR cannot be pitched.
  - MCompressor, slammed.
  - Clip.
  - Low-pass.
  - Optionally a second MConvolution.
- Workflow tip [said]: put Edison on the master, record lots of hits while tweaking, then chop out the good ones.

## Day 13 — rim-ish "high clang" snare, Complexity-style blips (13:00–17:20)
- "Really high clang… rim-ish hit. Not too noisy" [said]. He uses a WaveEdit Online wavetable with a "nice rim-like shape".
- Unison was rejected because it made the sound smeary. Played at D2 in FL naming [screen 905].
- **"Random amp"**, an envelope or random modulation on level: "I want it to ring out a bit longer".
- Reverb, then **parallel OTT** via Patcher, then a high-pass.
- Hint log: Osc1 Distortion type "FM ← Osc" with amount 0–100%, osc transpose swept −38 to +31, Osc2 transpose +24, Filter resonance 66%, a delay of about 7 ms [hint 11:17–12:34 in the part-1 layout log]. The osc-internal FM-from-osc distortion is where the clang comes from.

## Day 14 — "acoustic snare" request from a Discord mod (17:28–22:00)
- Analysis [said]: "Past 1k is just noise." "There's no obvious pitch thing" in the waveform, "so we should not do a lot of [pitch envelope]". It is built across multiple Vital instances.
- Snare-wire sizzle layer: noise plus **two triangle waves "at weird messed-up ratios… detune them a lot"**, plus FM ("this one's going to be FM'd").
- [hint 15:02–15:48]:
  - Body ENV2 decay 77 ms, decay power −3.14.
  - Mod amount 0.38.
  - Filter resonance 41%.
  - EQ around 230–290 Hz.
  - Chorus mod depth 95%, feedback 0.
- Layer 3 is a sample "Snare Hit HQ Metal #7" at transpose −10, Filter 2 resonance pushed 2% → 78% [hint 18:06–18:50].
- Osc distortion amount 32–49%, unison 2 → 4 → 6 voices, osc transposes −2 / +5 / −4 st (the "weird ratios") [hint 19:58–21:23].
- Frequency shifter (Shift −70.8, then +8.8, then −0.5 Hz, and later −806 Hz), dry/wet 12–14% [hint 20:52–21:39].
- EQ at **806 Hz**, high-pass at **1.4–3.1 kHz** on the sizzle layer [hint 21:29–21:35]. Layer volumes are −1.3 to −6.5 dB.
- MCompressor, then Maximus.

## Day 15 — Foley snares with frequency echo (22:22–25:40)
- "Pre-shifted thingamabob": a frequency shifter with feedback at **67–73%** and **delay 0.43 ms → 6.2 ms** [hint 22:53–23:42]. This forms a short metallic comb or echo.
- MWaveFolder on the modulated layer. White noise added; "Supermassive, but only in like [a short setting]".
- Then EQ:
  - A peak at 642 Hz.
  - A band at **2.17 kHz with tension −24%** (the band-pass he mentions).
  - Gain +6.6 dB, then +3.2 dB.
  - A low-pass **High Cut swept 7.96 kHz → 2.61 kHz** [hint 24:00–25:12].
- Clip at the end.

## Takeaways
- FM / phase-mod "clang": an audio-rate sine at a fixed ratio modulates the body's phase, and the index is enveloped by the body's amp envelope.
- Plank or clank: a short noise burst (10–20 ms) into **comb filters at about 79 Hz with resonance around 75%** (two in series), then convolution and slammed compression.
- Chordal tonal snare: a unison "Minor Chord" stack on a harmonic wavetable, kept very low in the mix (about 4% blend).
- An acoustic snare has little or no pitch envelope. Its sizzle is band-limited noise above about 1 kHz plus detuned inharmonic triangles at odd small-integer semitone offsets.
- Frequency shifter with short feedback delay (0.4–6 ms, 70% feedback) gives Foley/metal colour.
