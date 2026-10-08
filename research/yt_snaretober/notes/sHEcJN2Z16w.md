# sHEcJN2Z16w — CamaCon, "Making snares with only FREE plug-ins" (Snaretober 2025, part 1, days 1–9)

Uploaded 2026-02-06, 27:52. The Snaretober 2025 rule was free plugins only: Vital, the Melda free bundle (MEqualizer, MCompressor, MConvolutionEZ, MSaturator, MWaveFolder), and stock FL. Each snare ships in the free "Snaretober 2025" pack (Ko-fi/Gumroad) with variations. The short per-snare breakdowns live in his Discord and are not on YouTube.

Sources:
- [said] means from the auto-transcript.
- [screen] means read from a 1080p frame (`frames/sHEcJN2Z16w_<sec>.png`).
- [hint] means OCR of FL Studio's hint bar, which prints the exact value of the last-touched parameter. The full log is in `ocr/sHEcJN2Z16w_fl_hintbar_params.txt`, with timestamps in mm:ss.

On Vital modulation amounts: the amount is a fraction of the destination's range. Osc transpose spans ±48 st, so an amount of 0.6 on transpose is roughly +50–58 st. Treat the semitone conversions as approximate.

## Global drum-patch template (day 1, 0:42–1:41) [said + hint]
- All smoothing set to 0: LFO smooth time 0.00098 s [hint 0:59].
- All LFOs set to **Envelope** sync mode, so they fire once per note.
- **Phase randomization 0%** on every osc, so each retrigger is identical and click-free [said, hint 1:11].
- All envelopes at 0 attack. ENV1 runs the master amp, and each layer gets its own ENV.
- LFO rate set in seconds, not tempo. He saves this as a "Drum Starter" preset, used on every later day [screen].

## Day 1 — "simple tutorial snare", all inside Vital (0:42–7:40)
- **Root note A3** [said]: "A3 is usually a good root note for snares. Nice middle ground." FL shows "Note on: A4" in its own octave naming [screen 175], which is A3 in Vital/scientific naming, about 220 Hz.

### Body
- Sine (Basic Shapes) [screen 150].
- **ENV2 decay 54 ms** [hint 2:25].
- He says: "A big mistake a lot of people make is having their body way too long. It can be a lot shorter than you'd expect."

### Pitch transient
- LFO1 in envelope mode with a fast exponential-down shape drives osc pitch [screen 175, 412].
- **LFO period tried at 9.7 ms, 12 ms and 23 ms; final 12 ms** [hint 2:39–2:56, screen 412 "0.012 secs"].
- Mod amount 1.0 at first [hint 2:45], later "extend the range of the pitch bend a bit… It's going to make it sound clicky", mod 2 at about 0.6 [hint 6:42].
- [said] When it is short it "adds an attack". When too long it gets "zappy and juicy". "Pull the tension all the way down."

### Noise
- Vital sampler "White Noise" goes to Filter 1, which is a high-pass (blend 2), and then to Filter 2, a low-pass: "roll off the highs" [said, screen 255].
- **ENV3 decay 151 ms, then settled at 84–95 ms**, decay power +2.0 to 2.5 [hint 3:42–4:34].
- **Noise has an attack of 6–9 ms**, with attack power −1.1 to −1.9 [hint 3:55–4:22].
- [said] "If you want it a bit more synthetic and make the transient punch even more, add a bit of attack to the noise's volume." With zero attack the noise sounds natural. With a short attack it reads as synthetic and juicy.
- [said] A very short decay sounds "hilariously gated", and he warns not to overdo it.

### Noise EQ
- Vital EQ band with resonance 35.8 at cutoff 83.4 (Vital uses MIDI-note units, so about **1.0 kHz**) [hint 5:07–5:09], plus a **high shelf of +2.8 dB** [hint 5:20].
- The EQ gain is modulated by the noise envelope, "so it won't apply the gain to the transient… won't make the snare sound too boosted in the mids" [said].
- He adds a little resonance for a "dusty" tone.

### Stereo chorus on the tail
- Chorus with 16 voices [screen 345].
- Its **mix is ducked in by an envelope with an 85 ms attack** (ENV4 attack 0.085 s) [hint 5:34], so the transient stays mono and the tail goes wide.
- Chorus filter spread 0, so the chorus only covers one band. Chorus filter cutoff 69, then 89–92 (about 1.4–1.7 kHz) [hint 5:43–6:22]. Feedback 40% "almost adds a bit of reverb" [hint 6:05].
- Optional step [said]: a mid/side EQ after it, cutting low end and transient from the sides.

### FX order and output
- Chain: Chorus → EQ → Compressor (Single Band) → Distortion "SoS Clip" (soft clip) [screen 412, 440].
- **Distortion drive 4.3 dB, later raised to 14.6 dB** [hint 6:32, 7:11]. Volume −4.6 dB.
- Compressor upper ratio around 0.75 and attack about 70%: "let a little bit of the transient in", which emphasizes the transient [said, hint 6:54–6:58].

## Day 2 — "noise wavetable and sampler shenanigans" (7:43–12:53)
- **Noise as a wavetable osc**: factory "White Noise" wavetable with key tracking off and **transpose −36 st** (tried −48) [hint 8:41–8:42, screen 590].
- **16 unison voices, detune range 48** ("slam the detune range") makes it real noise rather than "chunky" noise [said, hint 8:53]. This frees the sampler slot.
- **Body:** sine. The spectral morph "Band Pass" is used as a filter on the body, cutting lows [said]. Pitch LFO **9 ms** [hint 9:15, screen 590]. Body ENV2 attack 12.8 ms with decay power −0.68 [hint 9:18–9:19]. Noise ENV3 decay **107 ms** [hint 9:14].
- **Third layer:** a freesound.org metal/pan sample ("96492__alienxxx__pan_8-12") in the sampler, loop off, key-tracked. **ENV4 attack 3 ms, decay 233 ms**, sample transpose +2 [hint 10:02–10:06, screen 590]. The noise is high-passed through Filter 1 (12 dB).
- **FX (Melda free):**
  - MEqualizer high-pass, with a band at 684 Hz [hint 10:56].
  - MCompressor at **threshold −16 to −27 dB, ratio about 4.3:1** [hint 11:21–11:29].
  - MSaturator.

## Day 3 — ISOxo-style snare ("someone asked on Twitter") (13:10–15:30)
- Built on the "Drum Starter" preset [screen 822, 842].
  - **Osc1:** sine.
  - **Osc2:** white-noise wavetable at −36 st, 16 voices, phase 100%.
  - **Osc3:** sine **+12 st**, an octave-up partial [screen 842].
- Played around A5 [screen 822, FL naming].
- LFO1 pitch envelope period **5.4–7.3 ms** [hint 13:35–13:36]. The LFO1 shape is a near-linear ramp down [screen 822].
- He jokes that it is "just a sine and an envelope". The character comes from processing:
  - **MCompressor**: threshold −19.7 dB, ratio 3.9:1, release 141 ms [hint 14:36–14:38].
  - **MConvolutionEZ** with a **Plate IR ("Medium 06")** at 100% wet, used as body/tone shaping rather than as a room [screen 882]. He also says "Fruity Convolver clears so hard".
  - **MSaturator**: even harmonics 0%, threshold −2.4 dB [hint 15:04–15:06]. "That's the whole snare."

## Day 4 — "chop snare" = TR-808 clap + 808 snare rebuilt (15:31–18:23)
### Clap
- Analysed from 808_CA7 [screen 972]. The waveform shows **3 short bursts followed by a decaying tail**.
- Two noise layers [screen 1003]:
  - Noise A: "shallow band-pass centred around 2.5 kHz", 12 dB.
  - Noise B: "steeper band-pass around 1 kHz", 24 dB.
  - Then a "pretty steep" high-pass [said].
- The burst envelope is an LFO with **4 spikes, "1, 2, 3 and then four"** [said], with an LFO period of about **100 ms** [hint 16:19].
- FX: filter at 24 dB, compressor with band gain +15.6 dB [hint 16:22–16:34].
### 808 snare
- Analysed from 808_SB7 [screen 1072, 1082]: "Doesn't look like the body has any envelope on it… just noise layered on top… distorted a little".
- **Two sines, the second at −12 st** (an octave down), plus noise at −36 st. Noise goes through a band-pass (Filter 1) then a high-pass (Filter 2), centred around **5 kHz**, "another pretty shallow one" [said, screen 1082].
- [said] "The higher one lasts shorter than the lower."
- [said] Tuning: the 808 source is "**25 cents out of tune**", so he matched it.
- Post: EQ at 4.6 kHz [hint 18:26].

## Day 5 — "ominous" snare (18:41–19:46)
- Body with **ENV2 hold 31 ms, then 59 ms**, decay power −1.34 [hint 18:43–18:46].
- Pitch LFO **27 ms** (tried 87 ms and 81 ms) [hint 18:46–18:48].
- "Bit of a longer attack… two layers of noise this time" [said]: noise ENV5 **attack 93–100 ms** [hint 19:22, screen 1162 "Envelope 5 Attack: 0.100775 secs"]. The noise swells in after the hit.
- Unison detune 9%.
- FX:
  - EQ high gain **+7.1 dB**.
  - **Comb filter "Low High Flange+"** as an effect.
  - Delay 35 ms.
  - Reverb with **decay time only 15.6 ms**, used as thickening [hint 19:11–19:24, screen 1162].
  - Chorus-envelope ENV4 decay 54–139 ms.

## Days 6/7 — "really high, tonal, screechy" snare (19:46–21:41)
- A Serum wavetable is resynthesised in Vital. Osc3 transpose −48, with frequency-morph amount swept between 1% and 81% and wave-frame scanning [hint 20:27–21:04].
- Chorus feedback 45–51%. Clipped "again with M…" and a little high-end reduction [said].

## Day 8 — old-school (2010–12) dubstep snare (22:00–23:24)
- [said] "These have really puffy, poofy transients… there's not a very clear pitch envelope from a sine wave. So I'm thinking this is like a lot of noise."
- The noise is unison-supersaw noise: **osc3 at 16 voices, detune range 48** [hint 22:32–22:33].
- ENV4 attack **139 ms** on the chorus mix [hint 22:44]. "Let's try to do some of the chorus stuff", ending on 6 voices.
- EQ bands at **1.4 kHz and 1.6 kHz**, high-pass at **290 Hz** [hint 22:45–22:55].
- Very heavy compression: **threshold −64.8 dB, ratio 2.7–4:1** [hint 23:00–23:03].

## Day 9 — pan snare (23:24–27:43)
- Freesound pan IR loaded into "Free Convolver". A saw wave is convolved: "probably should be using like a standard saw wave".
- **Root D**: "Maybe not that low… let's do like D" [said]. The body is lower and longer than usual "since it's like a [pan]".
- He weighs two options for the attack: "blow out the transient with the transient shaper" or "add a white-noise transient". He chooses "a really plasticky transient" made with the transient shaper [said].
- Pitch LFO **9.7 ms**, ENV2 decay power +2.44 [hint 25:17–25:19].
- EQ band +3.5 dB. **Out gain +8.6 dB** into the clipper.
- Parallel distortion layer: MEQ, then a saturator with **bias 48%** (asymmetric, even harmonics) and an HP of 6 dB/oct [hint 26:26–27:19].
- Result is "like one of those Joe Jo kind of snares".

## Cross-day takeaways
- Root note is A3 by default, with D for low pan snares. Pitch is matched to source to the cent (25 c).
- Pitch transient is an envelope-mode LFO with a fast exponential-down shape. **Typical period 9–12 ms** (range 5–27 ms). Depth is large, about 0.6–1.0 of Vital's range, which is tens of semitones.
- Body amp decay is short, **about 55 ms**. He explicitly warns against long bodies.
- Noise decay is **about 85–150 ms**, with power-curve shaping. A noise attack of **6–9 ms** gives the "synthetic punch". A noise attack of **90–140 ms** gives an "ominous" or old-school swell.
- Noise colour is always filtered: HP then LP, or band-pass at 1–2.5 kHz or around 5 kHz.
- Stereo comes from a chorus or detuned 16-voice unison whose mix is ducked by an envelope with an 85–140 ms attack. The transient stays mono.
- Final stage is always compression (−16 to −65 dB threshold, about 3–4:1, slowish attack to let the transient through), then soft clip or saturation (+4 to +15 dB drive).
- Plate or short-IR convolution at 100% wet is used as a tone shaper.
