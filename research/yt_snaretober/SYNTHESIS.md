# Snaretober research: snare synthesis architecture and numbers

## What the corpus is

CamaCon's daily Snaretober clips (2023, 2024, 2025) are not on YouTube. The short per-day breakdowns live in his Discord, and the packs (Ko-fi/Gumroad) contain audio only. His YouTube channel (@itscamacon) holds:

- **sHEcJN2Z16w**: Snaretober 2025 part 1, days 1–9, recorded while making them, 28 min.
- **zybsXNyqJsc**: Snaretober 2025 part 2, days 10–15, 26 min.
- **ZsUj5goDCnk**: a third-party video of the Snaretober 2024 pack, layered in FL.

Together they cover about 15 distinct CamaCon snares. Parameter values for them come from three sources:

1. 1080p frames read directly.
2. **OCR of FL Studio's hint bar**. It prints the exact value of every knob he touches, roughly 420 parameter readings in total, stored in `ocr/`.
3. The transcripts.

To reach a sample size where consensus means something, I added 33 bass-music snare-design tutorials (Floure District's "FloureTober", which explicitly copies Snaretober; Virtual Riot; Tealx1, who is a CamaCon collaborator; Code:Pandorum; Dripment; and others). That gives **36 notes in total**, with frames read for 15 of the videos (`frames/`, 60 PNGs).

Tags used below:
- **C-dN** = CamaCon Snaretober day N (d1–d9 from sHEcJN2Z16w, d10–d15 from zybsXNyqJsc).
- Plain IDs refer to supplementary videos.

Counting units: "videos" means the 33 supplementary videos plus the 2 CamaCon videos. "Snares" means individual CamaCon days. Of the 36 notes, 16 involve actual synthesis (oscillator plus noise). The rest are sample layering, which contributes roles, EQ and processing but no envelope numbers.

---

## 1. Recurring architecture

| Layer | How it is made | Prevalence |
|---|---|---|
| **Tonal body** | Sine (most), triangle (98Ky4Qq1MBk, FYOPiPICCMA), or square through a 365 Hz LP (ooiy0mVY3Ow). One voice, **phase randomization 0** so every hit is identical (C-d1, 98Ky, nglK, IWUV). | 16/16 synth videos; every CamaCon day except d12 (plank) |
| **Pitch transient** | A fast exponential pitch-down envelope on the body. **This is the click.** It is not a separate click sample in most patches. | 14/16 synth videos. The exceptions are C-d8 (old-school, "no clear pitch env") and C-d14 (acoustic, "we should not do a lot of this") |
| **Noise** | White noise, always filtered. CamaCon's preferred trick is a white-noise *wavetable* transposed −36 st with 16 unison voices at detune range 48 (C-d2, d3, d4, d8, d10, d12; also 98Ky with 2 voices). | 16/16 synth videos; all CamaCon days |
| **Tone / pan / metal** | Pan or foley sample, comb filter, frequency shifter, FM/ring-mod, or a narrow resonant EQ peak. | 6 of 15 CamaCon days (d2, d9, d12, d13, d14, d15); 2024 pack ships a separate "Metal" layer (ZsUj); "tone" is one of the three T's in N1DF, -vI0, QmukR, 2JviQ and xnc5 |
| **Clap** | Noise with 3–4 retriggered bursts and then a tail. | C-d4 (808 rebuild), disphing, x5OP; used as a sample layer in e9Tb, towA, WMee, LVme |
| **Stereo tail** | Chorus or detuned unison whose **mix fades in with an 85–140 ms attack**, so the transient stays mono and the tail goes wide. | C-d1 (85 ms), C-d8 (139 ms), C-d10; -vI0 (mono impact + reverb tail), 98Ky (2-voice detune), 2kn3 (mono low end) |

The canonical three roles are stated almost word for word in 7 videos:

- **"talk / tail / tone"** (N1DF, -vI0)
- **"transient / tonal / glue"** (mP8O, QmukR)
- **"bottom / transient / tail"** (ooiy)
- **"sine + noise + foley"** (WFGs)
- **"attack / noise / tail"** (LVme)

---

## 2. Consensus numbers

### Body pitch / tuning
- CamaCon's default is **A3 ≈ 220 Hz**: "A3 is usually a good root note for snares. Nice middle ground" (C-d1).
  - Low pan snare: **D** (C-d9).
  - High tonal snare: **D5/E5** (C-d10).
  - The ISOxo-style snare is around A4–A5, with an extra **+12 st** sine (C-d3).
- Other videos:
  - **~185 Hz** (nglK, matched to a reference)
  - **~200 Hz, range 150–400 Hz** (FYOP, Virtual Riot)
  - **F3 ≈ 175 Hz** (ooiy and kjw6 say "F2" in Ableton naming)
  - **Ab3 208 Hz** (eDHO)
  - **G4 392 Hz** (98Ky)
- **Median ≈ 200–220 Hz. Usable range 150–400 Hz, plus tonal outliers up to about 660 Hz.**
- Snares are **tuned to the song key**. The 2024 pack ships each snare in C, A and Gb versions (ZsUj). Tealx1 renders G#3/C#3/D3 versions. disphing names its file "C#". Others find the peak note with EQ (mP8O) or snap the pan peak to key (-vI0). CamaCon matched an 808 to **+25 cents** (C-d4).
- 808 snare body: **two sines an octave apart, with the upper one decaying faster** (C-d4).

### Pitch envelope

| Source | Depth | Time |
|---|---|---|
| C-d1 / d2 / d9 | Vital mod 0.6–1.0 of the transpose range (≈ 50–90 st) | **LFO-as-envelope period 9–12 ms** (9.0, 9.7, 12 ms) |
| C-d3 | – | 5.4–7.3 ms |
| C-d5 (ominous) | – | 27 ms |
| 6Kyy | +48 st → +24 st | 5.66 ms, then 67.5 ms to 0 |
| ooiy | +48 st → +24 st | 1.44 ms, then 20.3 ms |
| nglK | 65 st (70 st left a gap) | 1/32 note |
| LVme | 69.9 st, then reduced as "too laser-y" | fast |
| IWUV | – | 35 ms cycle, 90% of the drop done by ~12 ms |
| 98Ky | "until the top is inaudible" | 50 ms |

**Consensus: start 48–70 st above the note, use a concave (exponential) curve, and reach about 90% of the drop in 6–12 ms.** A two-stage version (very fast to +24 st, then about 20–70 ms to 0) appears in 2 Operator patches.

Feel rules (C-d1):
- Shorter gives "attack".
- Longer gives "zappy / juicy / laser".
- Deeper gives "clicky / unnatural".

### Body amp decay — bimodal
- **Short (CamaCon style): 54 ms** (C-d1), 77 ms (C-d14; Tealx1 9St6), 60 ms (98Ky).
  - CamaCon: "A big mistake a lot of people make is having their body way too long. It can be a lot shorter than you'd expect."
- **Medium: 133–176 ms** (Floure SM87 and xnc5), ~145 ms (IWUV).
- **Long: 256–400 ms** (Virtual Riot, disphing 314–380, Letsynth 400).
- Attack is always 0–0.5 ms. Decay curve: exponential, with Vital decay power about −0.7 to −3.

### Noise envelope and the body→noise stagger
**Noise lagging the body is the single most repeated trick. It appears in 11 videos / 4 CamaCon days:**

| Source | Noise attack / onset |
|---|---|
| C-d1 | **6–9 ms attack** (power −1.1…−1.9): "add a bit of attack to the noise's volume… makes the transient punch more, more synthetic" |
| IWUV | spike at 0, dip to 0 at ~13 ms, swell back to peak at **~25 ms**, linear tail |
| 6Kyy | **23 ms** attack |
| SM87 | **24–26 ms** attack |
| 98Ky | attack **= body decay (60 ms)**, a crossfade |
| kjw6 | noise level modulated *negatively* by the body env |
| LVme | attack "so it comes in right after the punch" |
| Tealx1 9St6 | spliced: noise clip starts where the tone's first loud cycles end |
| ooiy | the reverse: the body is delayed a few ms behind the transient |
| C-d5 (ominous) | **93–100 ms** swell |
| C-d8 (old school) | chorus/noise swell **139 ms** |
| xnc5 | **90 ms** swell |

**Default: noise peak 5–25 ms after the hit. Swell archetypes use 90–140 ms.** Zero attack sounds "natural / acoustic" (C-d1).

**Noise decay:**
- CamaCon: **85–150 ms** (d1 84–95, d1 initial 151, d2 107), with decay power +2.
- IWUV: 140 ms.
- Floure: 242–449 ms.
- Letsynth: 400 ms plus a 332 ms release.
- Au5: two layers, 33 ms bright and 66 ms dark, plus a 333 ms release.
- **Median about 150 ms (CamaCon), 250–400 ms for "big" snares.** Noise is typically **1.5–3× the body length** (Tealx1: "1.5–2×").
- Warning (C-d1): too-short noise sounds "hilariously gated".

### Noise colour / filtering
- **High-pass 150–700 Hz** (9 videos):
  - CodeP 150–300
  - C-d8 290
  - Letsynth 428
  - Au5 666 (resonance 44%)
  - e9Tb 700 Hz and 1 kHz
  - Maulik 911 (HP18)
  - disphing HP18
  - IWUV HP12 with drive
  - C-d1 HP into LP
- **Presence peak 1–2 kHz** (8 videos):
  - C-d1 ≈ 1.0 kHz (Q / resonance 36, +2.8 dB high shelf)
  - C-d8 1.4–1.6 kHz
  - Letsynth 1.35 kHz
  - VR +6.8 dB @ 1066 Hz, Q 1
  - e9Tb 1.11 kHz
  - CodeP **1.6–2 kHz** ("boost snare at 1.6k")
  - KelSounds 774 Hz
  - Kasanov "vowel / formant"
- 808-style band-pass on the noise: 1 kHz (24 dB) + 2.5 kHz (12 dB) for the clap, and ~5 kHz for the snare wires (C-d4).
- Cuts: ~500 Hz (VR, Maulik), 587 Hz −6.3 dB (QmukR), ~3 kHz harshness (VR).
- **Moving formant**: EQ gain driven by an envelope.
  - C-d1 ties it to the noise env so the transient isn't mid-boosted.
  - 98Ky uses a 0.5 s saw-up, so the formant blooms late.
  - VR sweeps the peak *upward* so the tail ends in the highs.
  - IWUV follows the noise LFO.

### Balance
- Tealx1: "noise should be the same loudness as your snare [tone]". Overpowering hat-like noise is the failure mode.
- Au5: the second (dark) noise sits 11 dB under the bright one.
- KelSounds: metal sine layer at **−15 dB** under the snare.
- C-d10: chord-tone osc blend at **~4%**.
- C-d14: sizzle layers −1.3 to −6.5 dB.
- Dripment: pan layer pulled down 8.6 dB.
- **Rule of thumb: body ≈ noise peak, tone/metal 6–15 dB under, decorative layers ~−25 dB.**

### Clap
- 808 clap (C-d4 analysis): **3 short bursts, then a tail, all inside about a 100 ms LFO cycle**, made from two band-passed noises (1 kHz steep, 2.5 kHz shallow) plus a steep HP.
- disphing and Stinger (x5OP) describe the same thing: several short retriggers, then a longer tail. Implied burst spacing is about 8–12 ms.

### Processing chain (order as observed)
1. **Compression with slow attack to accent the transient.** Present in CamaCon days d1, d2, d3, d8, d12, d14.
   - C-d1: Vital single-band, attack about 70%, "let a little bit of the transient in".
   - MCompressor at −16 to −27 dB threshold, ~4:1 (d2), and 3.9:1 with release 141 ms (d3).
   - **−64.8 dB threshold, 2.7–4:1** (d8, old school).
   - Others: Glue at 30 ms attack, ratio 10, −20 dB (LVme); 30 ms (Au5); 6 ms "for a clickier transient" (xnc5); 1 ms, −29.8 dB, +20 dB makeup after the reverb, which squashes the tail up (6Kyy).
2. **Saturation / distortion**: Vital SoS soft clip at **+4.3 → +14.6 dB drive** (C-d1); diode filter drive (C-d11); MSaturator threshold −2.4 dB (C-d3); bias 48% (asymmetric, C-d9); Tube 30–49% (VR, Maulik, disphing); hard clip (IWUV). VR envelopes the distortion amount so it is heavy on the attack and lighter on the tail.
3. **OTT**: 6 videos (C-d10, C-d13 in parallel, VR, Maulik 24%, e9Tb, kjw6).
4. **Short convolution / reverb as tone, not space**:
   - Plate IR at 100% wet (C-d3).
   - Tape-echo IR (C-d10).
   - Vowel IR (C-d12).
   - Reverb decay **15.6 ms** (C-d5).
   - Letsynth: reverb 491 ms, 44% wet, input low-cut 2.59 kHz, then glue, which turns the reverb into the tail.
   - Dripment warns against stacking reverbs.
5. **Final clip**: kHs Clipper (C-d10), "clip it" (C-d12, d15), +11 dB into a clipper (xnc5), soft clipper as level control (-vI0), limiter clipping (VR, 98Ky). **Present in about 75% of videos.**
6. **Transient shaper**: FL Transient Processor attack +100% / release −100% (-vI0); "blow out the transient… plasticky" (C-d9, used *instead of* a noise click); Drum Buss transients ±.

---

## 3. CamaCon's snare archetypes and what distinguishes them in DSP

| Archetype (source) | Body | Pitch env | Noise | Tone layer | Processing signature |
|---|---|---|---|---|---|
| **Clean synth snare** (C-d1) | Sine, A3, **54 ms** | 12 ms, deep | White HP→LP, **6–9 ms attack, ~90 ms decay**, ~1 kHz resonant bump | – | Chorus tail fading in over 85 ms, compressor letting the transient through, soft clip +4…+15 dB |
| **Metal/pan hybrid** (C-d2) | Sine, band-passed by spectral morph | 9 ms | Noise wavetable −36 st, 16 voices, 107 ms | Freesound pan sample, 3 ms attack / 233 ms decay, +2 st | HP, EQ at 684 Hz, comp −16…−27 dB 4:1, saturator |
| **ISOxo zappy** (C-d3) | Sine + sine **+12 st** | 5–7 ms, then a linear ramp | Noise wavetable | – | Comp 3.9:1, **plate IR 100% wet**, saturator |
| **808 / "chop"** (C-d4) | **2 sines an octave apart, upper decays faster**, no pitch env, +25 cents | none | BP + HP noise around 5 kHz | Clap: **3–4 bursts in ~100 ms**, BP 1 kHz + 2.5 kHz | Light distortion |
| **Ominous / swell** (C-d5) | Body **hold 31–59 ms** | 27 ms (slower) | 2 noise layers, **~100 ms attack** | – | +7 dB high shelf, **comb flanger**, 35 ms delay, 15.6 ms reverb |
| **High screech / tonal** (C-d6/7, d10) | Resynthesised wavetable, high (D5/E5) | – | Noise | **"Minor Chord" unison stack on a harmonic-series wavetable at ~4% blend** | Chorus with 45–51% feedback, tape-echo IR, OTT, clipper |
| **Old-school 2010–12 dubstep** (C-d8) | Weak: "puffy, no clear pitch env… a lot of noise" | ~none | **Supersaw noise** (16 voices, detune 48) dominates | – | Chorus mix fading in over 139 ms, HP 290 Hz, EQ 1.4/1.6 kHz, **comp threshold −65 dB** (heavy squash) |
| **Tearout pan** (C-d9) | **Low root (D), longer** body | 9.7 ms | Optional | Pan IR convolved with a **saw** | **Transient shaper for a "plasticky" attack**, +8.6 dB into a clipper, **parallel biased saturation (48%)** |
| **Terror FM** (C-d11) | Long body | LFO→transpose | Noise | **Audio-rate PM: key-tracked sine LFO → osc phase, index enveloped by the body env** | Diode-filter drive ("smash it"), convolution tail |
| **Plank / clank** (C-d12) | none | – | **10–20 ms noise burst** | **Two combs in series, 79 Hz, resonance 75.7%** | HP, vowel IR + frequency shift, slammed comp, clip, LP |
| **Rim / Complexity blip** (C-d13) | Rim-shaped wavetable, no unison | – | "Not too noisy" | **Osc FM-from-osc distortion 0–100%**, random amp | Reverb, **parallel OTT**, HP |
| **Fake acoustic** (C-d14) | Short body, 77 ms | **None** ("no obvious pitch thing") | **Mostly above 1 kHz** | **Inharmonic triangles detuned −4 / −2 / +5 st**, FM'd; snare-wire sample at −10 st with resonant filter | Frequency shifter 12% wet, HP 1.4–3.1 kHz on sizzle, comp, limiter |
| **Foley** (C-d15) | – | – | White noise | **Frequency-shifter feedback 67–73%, delay 0.4–6 ms** (metallic comb), wavefolder | Band-pass 2.17 kHz, LP swept 8 kHz → 2.6 kHz, clip |

Archetypes from the other videos:
- **Riddim** (towA, WMee, phhoenix): **kick transient (sub removed) + clap, fused and clipped together.** "Crunchiness is what makes a riddim snare." There is essentially no tonal snare body; the kick click is the body.
- **Tearout pan from any snare** (-vI0, N1DF, QmukR): narrow **+16 dB, very high-Q peak at about 1064 Hz snapped to key**, then distortion. Alternatives: frequency shift −231 Hz to put it in key, a static phaser comb (2JviQ), or ring mod with a triangle at +36 st (xnc5).
- **Metallic**: frequency shifter → convolver → distortion → HP sine at −15 dB (W_n8). Supermassive with tiny size and high feedback, or a flanger with maxed feedback (WFGs, MuD_).

---

## 4. Implications for a snare synth (knob list with defaults from the data)

1. **Note** (default A3 / 220 Hz; range 150–660 Hz), plus fine cents. Snares are tuned to key, so this is a first-class control.
2. **Body**:
   - wave: sine → triangle → square-through-LP
   - decay: 50–400 ms, default ~60 ms (CamaCon) or ~150 ms
   - curve power
   - optional hold (0–60 ms)
   - octave partial (+12 / −12 st, the upper partial with faster decay; 808 / ISOxo)
3. **Pitch env**: depth 0–72 st (default ~48), time 3–30 ms (default 10 ms), curve. Optional two-stage mode (fast to +24, then 20–70 ms to 0).
4. **Noise**:
   - level relative to body (default 0 dB)
   - **onset delay / attack** 0–140 ms (default 6–10 ms), plus an optional click spike at t=0 (IWUV)
   - decay 30–450 ms (default ~120 ms), curve
   - HP 150–1000 Hz, LP 3–12 kHz
   - formant peak 0.8–2 kHz with Q and gain, plus "formant follows env" amount
   - colour via unison-detuned noise (supersaw noise)
5. **Clap**: burst count (1–4), spacing ~10 ms, BP 1 kHz / 2.5 kHz.
6. **Tone / metal**:
   - mode: comb (freq 60–400 Hz, resonance up to ~0.8, excited by a noise burst), FM / phase-mod clang (ratio, index env), frequency-shift feedback (0.4–6 ms, ~70%), inharmonic partials (odd semitone offsets)
   - level −15…0 dB
   - snap-to-note
7. **Stereo**: width of the tail only, with a fade-in time of 85–140 ms; transient forced mono.
8. **Post**:
   - comp: threshold, ratio ~4, attack 6–30 ms for "let the transient through"
   - drive: soft-clip / tube / diode / hard, 0–15 dB, enveloped heavier on the attack
   - bias (asymmetry)
   - OTT amount (parallel)
   - short "tone" reverb or comb, ≤500 ms, low-cut ~2.5 kHz into the reverb
   - final clipper with input gain (default +6…+11 dB)
9. **Transient shaper**: attack and sustain, an alternative way to make the click (C-d9).

### Source index
- **CamaCon:** sHEcJN2Z16w, zybsXNyqJsc, ZsUj5goDCnk.
- **Synthesis (with numbers):**
  - IWUVd0fiyH4, WFGs91vSpIw, 98Ky4Qq1MBk
  - FYOPiPICCMA, nglKhCUJiJY, LVmEsVN6oDQ
  - 6Kyy8fGG4B4, ooiy0mVY3Ow, kjw6d0sFthI
  - xnc5HQ-yV7E, SM87V2oQ6OQ, TNl6CgU6EcE
  - 9St6hB0c-HU, 7cqh0vjf-4M, eDHOCLCUjv8
  - YvEIKc2Eg9Q, x5OPiKNA-BI
- **Layering / processing:**
  - mP8O1Zi7TSs, QmukRzQGu_Y, N1DFAI3LXoY
  - -vI0SYRl7GQ, 2JviQsn94kU, W_n8d7RQC30
  - e9Tbg2mY20c, towAIi6h4_8, WMeeMBw3Fqw
  - J0IHZ90eA7M, MuD_Rq04tic, 2kn34rK62Hc
  - 5qAofSlivik, hsJcvIEmDBs, mXlKIvrPecw
  - k9hwL-E-fR4 (no usable content)
- **No captions or note:** wMi9OEt91Bo, EUxzUl_IxmY.

Vital modulation amounts are a fraction of the destination range (transpose spans ±48 st), so the CamaCon pitch-env depths in semitones are approximate.
