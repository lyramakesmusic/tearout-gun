# Drum-synthesis theory for tearout guns

Source material for designing a gun/808/chug synth as DSP building blocks. Every claim carries a source tag; **[inf]** marks my own inference or synthesis across sources rather than something a source states.

Source key (YouTube IDs are `https://youtu.be/<ID>`; transcripts in `txt/<ID>.txt`, written-source notes in `written_sources_notes.md`):

| Tag | Video | What it's good for |
|---|---|---|
| MK | hULEn2_4Unw · Moritz Klein, "Designing a TR-808 style snare drum from scratch" | bridged-T snare circuit, component values, noise HPF |
| TH | suIDYns5evQ · thilk, "Advanced Snare Synthesis" | sine-phase click, dynamic-EQ transient, gating, Corpus metallic layer |
| LS | tofBTvc3uT8 · Light Shard, "How To Synthesize Snare Drums" | 4-element snare model, envelope times, RM metallic layer |
| ZJ | 1Db9rGbth_o · Zion Jaymes, "Synthesize SNARES ... FEEDBACK" | feedback-delay drum-head resonator |
| VT | DFhFDrlYTng · Venus Theory, "FM Snare Drum Synthesis" | measured snare partials (198/295/451/722/1340/2090 Hz) |
| AB | fzFSm4frIio · Astrobear, "Making drums like Noisia with Phase Plant" | exact kick/snare envelope numbers, zero-crossing harmonic trick |
| MZ | M3y-AHFu7CI · Mat Zo (DNB Academy) snare | noise-modulated triangle transient |
| TK | dG_iLMBvXZs · Teddy Killerz, "Snare 101" | synth + acoustic layering, resample-and-clip |
| SO | MuD_Rq04tic · dnksaus, "Metallic snares like SOPHIE" | delay/flanger/grain-delay metallic chain |
| CE | rmTEnxR0f8I · Celestric, metallic snare layer (Serum) | unison+octave metal layer, ring-mod filter, tuning the metal |
| OS | 8eAQ0VdW0Cw · Oscillator Sink, "How To Kick" | pitch-env amount vs rate tradeoff, separate attack layer |
| HB | 0MYc37NWEvY · Thee HB, "909 Kick ... Scientific Method" | breakpoint pitch curve for 909-style kick |
| AH | Z07NWhI2oQQ · Andrew Huang, "HUGE 808s" | single-cycle-loop 808, compressor attack 84 ms |
| ITM | H6i55lXAMh0 · In The Mix, "Bass Sound Design: 808s..." | 808 envelope values, pitch-env 34 ms |
| AK | S1PkHMA0cYo · Akito, "808 glide in Serum" | mono/legato portamento 808 |
| AP | qTmwVEh2xgM · The AudioPhool, "DIY Metallic Percussion Synth" | XOR ring-mod of square oscillators |
| DG | 0i1LCrC7dZw · Dash Glitch, "Modal synthesis in Phase Plant" | noise -> parallel bandpass modal bank |
| OP | YvEIKc2Eg9Q · oddprophet, "Turn any snare into a tearout gun" | snare-in-noise-osc + sub, convolver, phaser-as-disperser |
| DPS | xtaeEvROUFw · DPS Audio, "Heaviest tearout gun" | spectral-osc gun in Serum 2, notch-boost ~822 Hz, comb layer |
| GH | ODdHap_SYM0 · Ghosthack, "Tearout/dubstep gun" | disperser with swept split frequency, 1.6 kHz reverb boost |
| DR | fJZYQPGFXB4 · DRIZKO, "Tearout drop (gun)" | 3-layer gun (tonal/transient/sub), freq-shifter -100/-35 Hz |
| WM | qskyWy479Ac · Wheysted, "3 dubstep basses" | chug pitch-bend ±2/5/7 st, IR-as-resonator |
| BU | ibdbe1nxAhs · Bunting, "Tearout/riddim basses in Vital" | machine-gun pluck bass, FM/sync screech |
| WO | vZPdAnSjEQM · Wheysted, "Wooli's secret bass technique" | arpeggiated one-shot chug rhythms |

Written sources are tagged **[W:...]** with the URL given inline or in `written_sources_notes.md`.

Main written sources (all URLs and extracted numbers are in `written_sources_notes.md`):

- **[W:SOS-BD]** Reid, *Synthesizing Drums: The Bass Drum*: https://www.soundonsound.com/techniques/synthesizing-drums-bass-drum
- **[W:SOS-PBD]** Reid, *Practical Bass Drum Synthesis*: https://www.soundonsound.com/techniques/practical-bass-drum-synthesis
- **[W:SOS-SD]** Reid, *Synthesizing Drums: The Snare Drum*: https://www.soundonsound.com/techniques/synthesizing-drums-snare-drum
- **[W:SOS-PSD]** Reid, *Practical Snare Drum Synthesis*: https://www.soundonsound.com/techniques/practical-snare-drum-synthesis
- **[W:SOS-PERC]** Reid, *Synthesizing Percussion*: https://www.soundonsound.com/techniques/synthesizing-percussion
- **[W:SOS-TIMP]** Reid, *Practical Percussion Synthesis: Timpani*: https://www.soundonsound.com/techniques/practical-percussion-synthesis-timpani
- **[W:SOS-METAL]** Reid, *Analysing Metallic Percussion*: https://www.soundonsound.com/techniques/analysing-metallic-percussion
- **[W:SOS-CYM]** Reid, *Synthesizing Realistic Cymbals*: https://www.soundonsound.com/techniques/synthesizing-realistic-cymbals
- **[W:SOS-PCYM]** Reid, *Practical Cymbal Synthesis*: https://www.soundonsound.com/techniques/practical-cymbal-synthesis
- **[W:SOS-BELL]** Reid, *Synthesizing Bells*: https://www.soundonsound.com/techniques/synthesizing-bells
- **[W:SOS-COW]** Reid, *Synthesizing Cowbells & Claves*: https://www.soundonsound.com/techniques/synthesizing-cowbells-claves
- **[W:WAS-BD]** Werner, Abel and Smith, *TR-808 bass drum model* (DAFx-14): https://dafx14.fau.de/papers/dafx14_kurt_james_werner_a_physically_informed,_ci.pdf
- **[W:WAS-CY]** Werner, Abel and Smith, *The TR-808 cymbal* (ICMC/SMC 2014): https://speech.di.uoa.gr/ICMC-SMC-2014/images/VOL_2/1453.pdf
- **[W:RW909]** Robin Whittle, *TR-909 sound mods*: https://www.firstpr.com.au/rwi/tr-909/TR-909-Sound-Mods.pdf
- **[W:ED909]** Electric Druid, *TR-909 noise generator*: https://electricdruid.net/tr-909-noise-generator/
- **[W:JOS]** J.O. Smith, PASP comb and T60 pages: https://ccrma.stanford.edu/~jos/pasp/Feedback_Comb_Filters.html and https://ccrma.stanford.edu/~jos/pasp/Achieving_Desired_Reverberation_Times.html
- **[W:KS]** Karplus and Strong patent US4649783: https://patents.google.com/patent/US4649783A/en
- **[W:CS-MODAL]** Csound modal frequency-ratio table: https://csound.com/manual/misc/modalfreq/
- **[W:STK]** STK `ModalBar.cpp`: https://github.com/thestk/stk/blob/master/src/ModalBar.cpp
- **[W:BELL]** Bell partial table: https://en.wikipedia.org/wiki/Strike_tone (Fletcher & Rossing)
- **[W:AVZ]** Marogna and Avanzini, DAFx-09: https://avanzini.di.unimi.it/downloads/publications/marogna_dafx09.pdf
- **[W:CRED]** Credland, *Kick drum theory*: https://www.credland.net/kick-drum-theory.html
- **[W:MODE]** ModeAudio, *Drum synth: kick and snare*: https://modeaudio.com/magazine/drum-synth-sound-design-kick-snare
- **[W:SOS-LPX]** SOS, *Designing Kicks In Logic Pro X*: https://www.soundonsound.com/techniques/designing-kicks-logic-pro-x
- **[W:ISMIR808]** arXiv 2502.07524, *TR-808 bass drum constraints*: https://arxiv.org/html/2502.07524v1
- **[W:ATK808]** Attack Magazine, *808 bass with saturation*: https://www.attackmagazine.com/technique/synth-secrets/808-bass-with-saturation/
- **[W:EDMT]** EDM Templates, *Tearout bass in Serum 2*: https://edmtemplates.net/blogs/edm-templates-blog/how-to-make-tearout-dubstep-bass-in-serum-2

The corpus is 24 transcripts. YouTube bot-blocked further caption pulls partway through, so dedicated "chug" tutorials are thin. The chug section leans on the gun and bass videos plus inference.

---

## 1. Snare model

### 1.1 Anatomy: four or five layers, each with its own envelope

Every source decomposes the snare into the same parts:

| Layer | What it models | Typical design | Sources |
|---|---|---|---|
| **Click / transient** | stick on head | 1–15 ms burst: noise, a square for low-mid weight, a phase-offset sine, or a very fast pitch drop | LS, ZJ, TH, MZ |
| **Body / shell** | drum-head modes, mostly the (0,1) pair | sine/triangle at 100–250 Hz with a pitch envelope; decay 50–300 ms | MK, LS, VT, AB, [W:MODE] |
| **Wires** | snare wires rattling | high-passed or band-passed noise; decay longer than the body | MK, LS, VT, AB, [W:SOS-SD] |
| **Tonal ring / metal** | shell ring, inharmonic overtones | inharmonic partials, RM/FM, a resonator, or Corpus | LS, VT, TH, CE |
| **Room** | the room the drum is in | small-room reverb or short convolution, with the transient removed from the send | ZJ, TH, VT, AB |

- LS's summary: "four elements: transient, body (saturate with FM / wavetable / wavefold), noise tail, tonal layer."
- ZJ: "the entire plethora of sound is initiated by a tiny 5 to 10 ms reaction."

### 1.2 The body (tonal shell)

**Real snare modes** [W:SOS-SD]:
- Nine frequencies come from the first seven modes. The (0,1) mode is doubled, at about **180 Hz and 330 Hz**.
- Spectral lines (read off a figure) are at about 180, 280, 330, 342, 400, 440, 510, 550 and 618 Hz.
- The (0,1) pair decays "sometimes at more than twice the rate" of the other partials, so it needs its own VCA.

**Reid's synthesis model** [W:SOS-SD]:
- A **111 Hz triangle** goes through two frequency shifters, **+175 Hz** and **+224 Hz**. That gives inharmonic quasi-series at 286, 397, 508, 619… Hz and 335, 446, 557… Hz.
- Add sines at **180 Hz and 330 Hz** for the (0,1) pair.
- This is the "frequency shifter turns a harmonic series into an inharmonic one" trick:
  - Shifting a 100 Hz harmonic series by +25 Hz gives 1 : 1.8 : 2.6 : 3.4 [W:SOS-BD].

**Measured partials of a real snare with the wires off** (VT):
- **198, 295, 451, 722, 1340, 2090 Hz**, which have no harmonic relationship.
- VT puts one FM8 operator on each partial and gives each a slightly lower level than the one before.
- He adds light cross-FM between them for "in-between random harmonics".
- Higher partials get shorter decays ("high frequencies decay faster than low frequencies").
- The fundamental's amplitude envelope is about 300 ms.
- The partial ratios (198 Hz = 1) are 1, 1.49, 2.28, 3.65, 6.77, 10.6.
- **[inf]** These sit near the membrane ratios 1, 1.59, 2.14, 2.30… only at the bottom; the upper partials are shell and air modes.

**Ideal circular-membrane mode ratios** (Bessel zeros j_mn/j_01) [W:SOS-PERC], recomputed with scipy:

| mode (m,n) | 0,1 | 1,1 | 2,1 | 0,2 | 3,1 | 1,2 | 4,1 | 2,2 | 0,3 | 5,1 | 3,2 | 1,3 |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| f/f01 | 1.000 | 1.593 | 2.136 | 2.295 | 2.653 | 2.917 | 3.155 | 3.500 | 3.598 | 3.647 | 4.059 | 4.230 |

How the modes behave on a real membrane [W:SOS-PERC]:
- Air loading pushes the radial (n,1) modes up toward harmonic. In a kettle drum they land at 1 : 1.50 : 1.98 : 2.44 relative to the (1,1) principal.
- The circular modes, (0,n) and (2,2), are short-lived and give the "noise-like burst".
- The radial modes ring.
- A center strike gives a "dull, toneless thump".

**Body pitch envelope**:
- **AB, a Noisia-style snare body** (exact Phase Plant values):
  - A slow envelope of about **+4 semitones** over the body.
  - A fast transient envelope of **+37 semitones, 1 ms hold, 3.4 ms decay**.
  - The kick in the same video uses +24 st, so the snare's slow drop is much smaller.
  - Amplitude: no attack, **40 ms hold, 40 ms decay**, near-linear shape.
- **TR-909**: both snare oscillators share **one pitch CV**, the Tune pot plus a short pulse at note start [W:RW909]. Reid calls a pitch envelope on a snare "anomalous" for a real drum but it is part of the 909 sound [W:SOS-PSD].
- **LS**: a small, fast pitch fall, motivated by the two heads being tuned differently.
- **LS body amplitude**: about 20 ms hold plus 100 ms decay, with a range of **50 ms (tight) to 200 ms (doofy)**.

**Body timbre beyond a sine**:
- LS: FM a sine with a 2:1 modulator (modulator an octave up) and sweep the FM index from high to zero over the hit. The waveform starts square-ish and decays to a pure sine.
- LS also bakes that FM shape into a wavetable frame.
- AB: drive the body into a clipper with a separate short envelope:
  - Clip envelope: 5 ms attack, 5 ms decay, 30 ms hold.
  - The output is flat-topped for the first ~35 ms, then clean.
  - Use two clip stages, the first with its mix turned down.

### 1.3 Click / transient

- **Phase-offset sine** (TH): start the oscillator at **90°**, with attack at zero. The instant jump from 0 to the peak is a broadband click.
  - Reid makes the same point about analog hardware: an instant VCA attack makes a discontinuity, which is an HF burst [W:SOS-PBD].
- **Pitch-drop click**: a very fast, very large pitch envelope reads as a click rather than as pitch.
  - AB: +37 st over 3.4 ms. OS: "as we make it go faster that very fast pitch almost doesn't sound like a pitch… sounds like a [click]".
- **Noise + square transient** (ZJ): **4–15 ms** long. The square adds low-mid body that noise alone lacks.
  - Randomize the noise part per hit, not the oscillator. The oscillator sets "the phase and the initial impact".
- **Noise-modulated triangle** (MZ, Mat Zo): a triangle modulated by white noise, then high-passed, gives a clap-like transient.
  - The tail uses the same structure with unison spread.
- **Separate attack layer** (OS): a saw with a very fast pitch envelope, or a short band-passed noise burst, kept separate so its tonality and level can be set independently of the body.
- **LS**: about **10 ms** steep envelope on the click.
- **Sampled stick click layered in** (AB, TK). TK cuts the attack from one sample and fades it onto another layer's body.
- **Velocity** (ZJ): for ghost notes, turn the transient down more than the wires.
- Dash Glitch's modal model uses the same rule: velocity changes the upper partials most [inf].

### 1.4 Wires (noise)

**MK (808-style circuit)**:
- White noise goes through a VCA with an **instant-attack** envelope ("we don't want the noise to come in gradually but hit at full volume").
- The decay is variable to simulate tighter or looser wires.
- Then an **aggressive Sallen-Key high-pass at 3.4 kHz with a resonant bump** at the cutoff (2 × 1 nF, 100k to ground, 22k feedback).
- Without the HPF it sounds like "a weird combination of a tom and a terrible hand clap. The noise is way too bottom heavy."
- Mix ratio: drum through 100k, noise through 10k, so the noise gets about 10× more mixer gain.

**TR-808 snare** [W:SOS-PSD]:
- The "snappy" envelope does double duty. It drives the noise VCA, and an attenuated copy is added to the trigger that kicks the two bridged-T oscillators.
- The noise is **high-passed** after the VCA.

**TR-909 snare** [W:SOS-PSD], [W:RW909]:
- Noise is low-passed, then split into two paths:
  - A high-passed narrow band with its own short envelope.
  - An unfiltered band with a longer envelope.
- So the high and low parts of the noise decay differently.
- The snappy envelope **holds at max for about 24 ms**, then decays **faster than exponential**: it discharges toward a negative rail, so it hits zero in finite time.
- The noise source is a **31-stage LFSR, taps 31 and 13, clocked near 300 kHz** [W:ED909].
- 808 noise is an avalanche transistor, which is spikier.

**Reid's wire model** [W:SOS-SD]:
- Band-limited noise through a velocity-controlled LPF, plus notch filters for spectral holes.
- Crossfade between the modal path and the noise path by velocity.
- Harder hits mean more HF and wider modal peaks, eventually a broad noise spectrum.

**Wire envelope details**:
- **Delayed attack** (LS, AB): let the body speak first.
  - AB: **25 ms attack** at 50% curve, **30 ms hold**, **61 ms decay** at a 100% (fast) curve.
  - The noise **outlasts** the fundamental.
- **Filter sweep** (LS, TH): the noise starts brighter and tapers. Sweep the cutoff from high to low.
  - TH: "if you give it just a bit of filter movement it can really sound live."
  - LS: also bring the filter resonance in after the click, so the click stays clean.
- **Two noise layers** (VT): a **band-pass "clappy" middle** and a **high-pass "bright buzz"** layer.
- **AB**: white noise boosted around **1 kHz**, saturated with **+20 dB drive**, with stereo spread.
- **ZJ**: a boost around **2 kHz** on the wires. A lower-tuned drum needs a **longer** wire decay.

### 1.5 Feedback-delay "drum head" resonator (ZJ)

This is a cheap waveguide, and the most physically meaningful trick in the video corpus.

- Send the transient into a bus that loops back on itself through: sample delay → EQ → gain < 1 → limiter.
- **Delay time sets pitch.** Plugin latency in the loop also shifts pitch.
- **Loop gain sets ring time.** It must stay below unity.
- **EQ in the loop acts as damping**, i.e. frequency-dependent loss. Mostly cut; if you boost, lower the gain.
- A **filter in the loop retunes the drum**, because its phase shift changes the effective loop delay.
- **Tune the transient's EQ** so it excites the loop's fundamental.
- **Comb decay formula** [W:JOS]: for a loop of M samples, g = 10^(−3M/(fs·T60)), and resonances sit at k·fs/M.
- **[inf]** In DSP terms this is a feedback comb with a loop filter: an extended Karplus–Strong voice where the exciter is the click layer.

### 1.6 Karplus–Strong drum [W:KS]

- Recurrence: y[n] = ±½(y[n−N] + y[n−N−1]). The sign is + with probability b (the blend factor), otherwise −.
  - b = ½ is drum-like.
  - b = 1 is a plucked string.
  - b = 0 drops an octave and leaves only odd harmonics.
- At b ≈ ½, **N sets decay, not pitch**: N ≈ 200 at fs ≈ 20 kHz is a snare, N ≈ 20 a tom.
- A stretch factor lengthens the "snare" sound.
- Csound's `pluck` method 3 calls this "roughness = 1 − b". At 0.5 it is "an optimum snare drum".

### 1.7 Modern processing

- **thilk's view** (TH): "it's all about the post-processing". A sine plus noise "literally just sounds like a synth playing a sine wave and some noise."
  - **Distortion with an envelope on drive** gives a crunchy transient (LS does the same: "this will essentially be our transient processor, but also… glue").
  - **Envelope-modulated EQ bands** (gain and frequency), including an upward sweep, placed before the distortion.
  - **A resonant peak around 1 kHz**, a "high whistle layer", gives the snare character.
  - **Transient-removed reverb send**: a transient shaper on the send kills the attack, so the reverb is fed by the body and tail.
  - **Dynamic EQ**: boost 1–2 kHz only on the transient, ducked after about **13 ms** ("10 to 20 ms is the time scale transients exist on").
  - **Corpus plate** with an inharmonic control. Its "material" control sets frequency-dependent decay (lows die fast or highs die fast).
  - **Lookahead gate**, hold and release, to cut the ring.
  - **Multiband**: a transient shaper or gate on the low band only, to shorten the fundamental.
- **Teddy Killerz** (TK): synth snare plus an acoustic sample layer (pitched, low-cut), resampled, soft-clipped, then a transient shaper.
- **AB on Noisia**: their snares have a "nice chunky waveform with no dips". Shape it with careful synthesis rather than heavy EQ, because EQ dips amplitude while the swept sine passes through the cut frequencies.

### 1.8 Classic analog circuits

- **Bridged-T percussive oscillator** (MK, [W:WAS-BD]):
  - A bridged-T RC network in an op-amp's feedback path is a high-Q band-pass. A trigger pulse "pings" it into a decaying sine, so no VCA or envelope is needed.
  - Center frequency: fc = 1/(2π√(R_eff·R_a·C1·C2)).
  - MK's snare values: 910k bridge, 2 × 33 nF, 470 Ω + 1k pot to ground → **100–200 Hz**.
  - **Positive feedback** lengthens decay (MK: an inverting amp with gain 0–6). Too much feedback gives endless oscillation [W:SOS-PBD].
- **Pitch envelope in a bridged-T**:
  - A transistor shunts part of the ground resistance, which raises fc. A 330 Ω series resistor caps how high it goes (MK).
  - A resistor before the envelope cap softens the attack. MK's range runs from "pretty punchy" to "subtle".
- **808 swing-type VCA**: heavy distortion is part of its character. A small cap tames the HF (MK).
- **808 snare oscillator pair**: about **238 and 476 Hz**, an octave apart. This is [W] secondary, from service-manual figures. Note that 180/330 Hz is Reid's measurement of an *acoustic* snare, not the 808. The Tone control crossfades the two oscillators.

---

## 2. Kick / 808 model

### 2.1 Kick structure

A **sine (or near-sine) body with a fast-falling pitch envelope**, plus an **attack** layer.

- [W:SOS-PBD], [W:CRED], OS, AB, HB.
- Credland: the body settles "**usually between 40 and 60 Hz**". It sounds like a kick from about 35 Hz (system roll-off) up to about 65 Hz [W:CRED].
- Real dual-head kick drum modes [W:SOS-BD]:
  - **50, 93, 136, 182, 225, 273 Hz**. That is a 43 Hz harmonic series shifted up by 7 Hz, so a frequency shifter models it.
  - With the resonant head loosened or holed: near-harmonic 44, 91, 138, 184, 232, 282 Hz.
  - Plus "scores of partials between 250 Hz and 1 kHz", modeled with FM or noise through a band-pass.
- **909 kick** [W:SOS-PBD], [W:RW909]:
  - A saw/triangle VCO rounded to a near-sine by a diode shaper.
  - The pitch is set by an instant-attack, slow-decay envelope.
  - Separately, LP-filtered noise plus a short pulse "click" go through their own VCA.
  - **The Tune knob sets the pitch-sweep decay time, not the base pitch.** Sweep depth is set by a resistor: doubling R28 roughly doubles the sweep.
- **808 kick** [W:WAS-BD]:
  - A 1 ms trigger pings a bridged-T resonator at about **49.5 Hz** (Roland's chart says 56 Hz).
  - **During the first ~6 ms the center frequency jumps up by more than an octave**, less than one period at the higher frequency, which is heard as "punch", not pitch. The bridged-T is then re-kicked so there is no amplitude hole.
  - Then a slow "**pitch sigh**" from about **56–57 Hz down to about 50 Hz** over roughly 150–200 ms (read off a figure).
  - Measured across real samples: median f0 is **49.48 Hz**, the median initial sweep is about **1 semitone**, and it is a pure sine after about **0.4 s** [W:ISMIR808].

### 2.2 Pitch-envelope shapes and numbers

| Source | Start | End | Timing | Shape |
|---|---|---|---|---|
| AB (Noisia kick, Phase Plant) | fundamental +24 st (env 1) +26 st more (env 2) | G0 in Ableton naming (MIDI 31 ≈ 49 Hz) **[inf]** from the octave convention | env 1: decay 86 ms, curve +58%; env 2: hold 1 ms, decay 2.5 ms | **sum of two exponential-ish envelopes**: a fast "click" one and a slower "body" one |
| HB (909-style, measured breakpoints) | **1396.9 Hz** at 0 | **43.65 Hz** at 238 ms | **329.6 Hz at 1.86 ms**, **103.8 Hz at 59.5 ms** | piecewise: about 2 octaves in 2 ms, then about 1.7 octaves over 58 ms, then 1.25 octaves over 180 ms; **[inf]** roughly exponential with a slowing rate |
| [W:SOS-LPX] 808-style Logic | about 5 octaves above base | base | pitch decay **7.6 ms**; amp decay **720 ms** | faster curve → more HF click; slower → more mid "thump" |
| [W:MODE] | — | ~58 Hz | 1 ms attack, ~50 ms decay | — |
| [W] secondary guides | 150 Hz | 48 Hz | about 40 ms; 909-like about 4–5 octaves over about 200 ms | "steep start" |
| [W:WAS-BD] 808 | above 1 octave over 56 Hz | ~50 Hz | 6 ms jump, then sigh over 150–200 ms | step + slow sag |
| ITM (808 bass) | +N st | note | env attack 0.5 ms, decay **34 ms**, release 15 ms on master tune | — |
| DR (gun sub) | +19 st on coarse pitch | note | short envelope | gives the sub its transient |

Principles:
- **Amount vs rate** (OS): "everything about the character of the sound comes from balancing your mod amount and your mod rate."
  - Big and slow gives a "zappy" tom sound.
  - Big and very fast gives a click.
  - Small and slow gives a soft "boof".
- **Coupling pitch to amplitude** [W:SOS-BD], [W:AVZ]:
  - In a real drum, membrane tension grows with displacement squared. Pitch therefore follows the energy decay: Reid says "the gain changes 100%, pitch only ~10%, a couple of semitones", so one envelope can drive both.
  - Measured partial glides sit in the **first 200–300 ms** and grow with strike velocity.
- **[inf]** For a synth: pitch(t) = f_end · 2^((A1·e^(−t/τ1) + A2·e^(−t/τ2))/12).
  - τ1 ≈ 0.5–3 ms with A1 = 24–37 st gives the click.
  - τ2 ≈ 20–90 ms with A2 = 3–24 st gives the body. Use the low end for snares and guns, the high end for kicks.
  - Optionally add a third, slow term of about 1 st over 200–400 ms (808 sigh, velocity-scaled).
  - Phase must be integrated from frequency, never computed as sin(2π f(t) t).

### 2.3 Attack / click design

- **909**: a short white-noise burst plus a short pulse, through their own VCA [W:SOS-PBD], [W:CRED].
- **808**: the trigger pulse itself is mixed into the audio, a single-sample-ish impulse [W:SOS-PBD], [W:CRED].
- Harder attacks carry more content around **300 Hz** and last slightly longer [W:CRED].
- **AB's zero-crossing harmonic** (Noisia kick):
  - A sine at the **12th harmonic**, decay about 40 ms, gets a tiny pitch drop of its own.
  - It is amplitude-modulated by the inverse of the fundamental's magnitude, done either by slamming a parallel copy of the fundamental into a saturator or by multiplying by (1 − |sin|).
  - So the HF detail exists only near the fundamental's **zero crossings**. "It's keeping headroom": the peaks stay clean and the transient detail sits where there is headroom.
  - **[inf]** As DSP: h(t) = sin(φ_h) · (1 − |sin φ_f|)^k.
- **Clipping as transient** (AB): the body sine is pushed **+25% over 0 dB** into a hard clipper for **40 ms hold + 12 ms decay**, so the first ~50 ms are a squarish wave, then clean. The amp envelope is a 45 ms hold and 125 ms decay.

### 2.4 808 bass specifics

**Envelope** (ITM):
- Sine, attack near zero but raised just enough to kill the click.
- **Hold 300–400 ms, decay 1.5 s**, sustain 0.
- A wavetable "bend" warp adds harmonics.
- The pitch envelope (34 ms decay) on master tune gives the punch.
- Tube distortion, then an EQ boost around 100 Hz.
- With a punchy kick, give the 808 **80–100 ms attack** so the kick supplies the punch.

**Distortion of 808s**:
- Distort the **upper harmonics only**:
  - Serum's distortion in high-pass mode, followed by a filter (AK).
  - Multiband saturation split at **123 Hz** with much less drive below [W:ATK808].
- The point is audibility on small speakers: 2nd and 3rd harmonics [W] (secondary).
- Andrew Huang's version (AH):
  - A single-cycle loop of any sample is the oscillator.
  - Amp and pitch envelopes "mimic a drum skin… vibrate loud and quickly, then slow down".
  - An HPF just below the lowest note, and a mid dip.
  - A compressor with **84 ms attack** lets the hit through, then clamps.

**808 glide**:
- Mono plus **legato** portamento (AK). Glide time is about **300 ms** [W:ATK808], or 80–150 ms per secondary guides.
- Glide happens only on overlapping notes.
- **[inf]** Implement as a one-pole or linear-in-pitch slew on the note's log-frequency, so a glide does not retrigger the amp or pitch envelopes.

**Tail gating** (TH, OP, WO):
- Lookahead gate with hold and release.
- An extra level envelope to shorten the tail when pitched down (OP).
- Note length or an arpeggiator gate controls the tail ("you control the tail", WO).
- Mono voice stealing chokes the previous tail (ITM).
- **[inf]** In the synth, a gate-off should trigger a short release (5–15 ms). The ITM 808 used 15 ms; Reid's 909 snappy discharges faster than exponential, so it hits zero in finite time. The release should be rate-limited so the cut doesn't click.

---

## 3. Metallic / modal model

### 3.1 Why inharmonicity

- Subtractive synthesis can't create inharmonic partials because filters only remove what is already there. You need multiplication (ring mod/FM), shifting, or resonators (AP, DG).
- Snare shells, bells, plates and cymbals all have **non-integer partial ratios** (VT, [W:SOS-METAL]).
- LS: adding a metallic tonal ring is what makes the brain "decode this as a more realistic snare".

### 3.2 Mode-ratio tables (f/f₁)

| Object | Ratios | Source |
|---|---|---|
| Ideal membrane | 1, 1.593, 2.136, 2.295, 2.653, 2.917, 3.155, 3.500, 3.598 | [W:SOS-PERC], computed |
| Kettle drum (radial, vs principal) | 1, 1.50, 1.98, 2.44; decays 45/73/91/84%; amps 5:4:3:1 | [W:SOS-TIMP] |
| Free-free uniform bar | **1, 2.757, 5.404, 8.933, 13.344, 18.638** (from cos βL·cosh βL = 1, βL = 4.730, 7.853, 10.996…; f ∝ (βL)²) | computed; measured aluminium bar 1, 2.756, 5.423, 8.988 [W:CS-MODAL] |
| Clamped-free bar | 1, 6.267, 17.547, 34.386 | computed |
| Xylophone (measured) | 1, 3.932, 9.538, 16.688, 24.566 | [W:CS-MODAL] |
| Marimba (STK) | 1, 3.99, 10.65, + fixed 2443 Hz; r = 0.9996/0.9994/0.9994/0.999 | [W:STK] |
| Vibraphone | 1, 3.984, 10.668, 17.979, 23.679 | [W:CS-MODAL] |
| Agogo (STK) | 1, 4.08, 6.669, + 3725 Hz | [W:STK] |
| Wood block (STK Wood1) | 1, 2.777, 7.378, 15.377 | [W:STK] |
| Clamped circular plate | 1, 2.081, 3.414, 3.893, 4.995, 5.954 | computed |
| Flat plate (center-mounted) | 1, 2.90, 4.20, 8.90, 10.3, 17.3 | [W:SOS-METAL] |
| Real cymbal | 1, 3.3, 8.9, 12.6, 15.6, 17.9 (only 6 of ~300 modes) | [W:SOS-METAL] |
| Tibetan bowl | 1, 2.778, 5.181, 8.163, 11.661, 15.638 | [W:CS-MODAL] |
| Pot lid | 1, 3.2, 6.23, 6.27, 9.92, 14.15 (6.23/6.27 is a beating doublet) | [W:CS-MODAL] |
| Church bell (to prime) | hum 0.5, prime 1, tierce 1.183, quint 1.506, nominal 2, 2.514, 2.662, 3.011, 4.166, 5.433, 6.796, 8.215 | [W:BELL], [W:SOS-BELL] |
| Measured snare, wires off | 1, 1.49, 2.28, 3.65, 6.77, 10.6 (198 Hz base) | VT |
| 808 cymbal oscillators | 205.3, 304.4, 369.6, 522.7, 540, 800 Hz → 1, 1.483, 1.800, 2.546, 2.630, 3.897 | [W:WAS-CY] |

**Chladni's law** [W] (Rossing): f ≈ C·(m + 2n)^p.
- m = nodal diameters, n = nodal circles.
- p ≈ 2 for flat plates; p = 1.4–2.4 for cymbals and bells.
- **[inf]** This is a one-parameter "plate stiffness" control for a generated mode set.

**Real-object detail**:
- Mode **doublets** a fraction of a Hz to a few Hz apart cause beating or "warble". A measured bell nominal is 831.7 vs 832.3 Hz [W:BELL], [W:SOS-BELL].
- **[inf]** Detune each mode into a pair at ±0.1–2 Hz for life.

### 3.3 TR-808 cymbal / hat / cowbell [W:WAS-CY], [W:SOS-PCYM], [W:SOS-COW]

**Oscillators**:
- **Six Schmitt-trigger square oscillators** (one 40106/HD14584 chip) at **205.3, 304.4, 369.6, 522.7, 540, 800 Hz**.
- Duty cycle is 47.98% (very slightly off square, so weak even harmonics).
- The cowbell uses only **540 + 800 Hz** (ratio 1.48).

**Cymbal signal path**:
- Sum → two band-pass filters at about **3440 Hz** and **7100 Hz**. These emphasize the upper overtones and suppress the fundamentals.
- → three swing VCAs with separate envelopes. Peaks are roughly 1–3 ms (short band), 30–50 ms, and 0.2–0.3 s (the decay-controlled band) — read off a figure.
- → Sallen-Key HPFs; the last one resonates near **10.5 kHz**.
- → a Tone control that mixes the bands.
- The attack is smoothed with τ = 0.1 ms.

**Hats**: the same six oscillators → one BPF → VCA → HPF. The closed hat chokes the open hat.

**Cowbell envelope**: two stages, an "abrupt level decay at the initial trailing edge to emphasise attack", then a longer tail.
- Reid's version: triangles, not pulses, through a **2.64 kHz BPF** at 12 dB/oct with a little resonance.
- A real CR-8000 cowbell measures **587/845 Hz**, ratio 1.44, and "small deviations destroy the illusion".

**Other cymbal approaches**:
- **TR-909 cymbal** is a 6-bit ROM sample, not synthesis.
- **Reid's ride**:
  - A 1 kHz pulse FMs a 2.5 kHz square.
  - "Ping": a band-pass swept down to 1 kHz over about 200 ms.
  - "Tail": high-pass at **2.64 kHz**, opening over 200 ms and closing over 3.7 s.
- **Cymbal time course**: energy starts at a few hundred Hz, moves up to a few kHz over about 100 ms, then the HF dies first [W:SOS-METAL].

### 3.4 Synthesis methods for the metal layer

- **Modal bank** (DG):
  - Noise burst (or any exciter) → **parallel band-pass resonators**, key-tracked.
  - Resonator Q sets each partial's decay. A two-pole resonator: a1 = −2r·cos(2πf/fs), a2 = r² [W:STK].
  - T60 = −3·ln10 / (fs·ln r). At 44.1 kHz, r = 0.999 gives about 0.16 s and r = 0.9999 about 1.6 s [W:STK], [W:JOS].
  - DG's craft tips:
    - Keep the low partials key-tracked and leave the upper partials **un-key-tracked and randomly detuned** for "woody" or metallic texture.
    - Velocity scales the upper partials far more than the fundamental.
    - A delay before the bank turns one strike into a strum or flam.
    - A resonator adds "a glossy click to the transient".
- **Ring modulation / XOR** (AP, LS, CE):
  - The XOR of two ±1 square waves equals their product, so it is a ring modulator.
  - Every odd harmonic of each square forms sum and difference tones, so two squares give a dense inharmonic spectrum.
  - AP: six oscillators into four XOR gates. Four are "set and forget" and two are fine controls. The result goes from church bell to china/ride to cowbell.
  - LS uses RM rather than FM for a snare's metallic tail and pushes it high. CE uses Serum's ring-mod filter.
- **Frequency shifting** [W:SOS-SD], [W:SOS-BD]: shifting a harmonic series by Δ gives f_k = k·f0 + Δ, which is inharmonic.
  - The tearout producers use a frequency shifter on guns at −25 to −100 Hz (OP, DR). **[inf]** That is the same operation applied to a whole processed sound.
- **FM with inharmonic ratios** (VT, Reid's ride). Light cross-FM between inharmonic sine partials gives "in-between" metallic partials.
- **Comb / delay** (SO, ZJ, DPS, OS-comb):
  - SOPHIE-style chain: flanger at full feedback with an S&H LFO, then a grain delay at full feedback and short time, then a resonator. "If you want to make anything metal, use delays."
  - Feedback comb: decay g = 10^(−3M/(fs·T60)) [W:JOS].
  - Several **mutually inharmonic** comb delays in parallel approximate a plate **[inf]**.
- **Wide-detune unison** (CE):
  - 10 unison voices with the detune range at **3 octaves**, plus a few added partials, gives a bell or triangle.
  - Then a ring-mod filter, an HPF, and a cut above about 6.5 kHz.
  - To make it read as the note, boost an EQ band at the note's frequency in the metal's range: D#7 ≈ 2.5 kHz.
  - Duck its volume during the main transient so it "plays after the attack".
- **Physical-model plate** (Corpus, TH): "inharmonic" and "material" controls. Material tilts the decay so lows die first or highs die first.
- **Convolution with tonal IRs** (WM, OP, DPS, GH):
  - Short real IRs, tight plate, car garage.
  - Wheysted uses a **bass sample as the impulse response**, which acts as a resonator bank with that sample's spectrum.
  - Ghosthack boosts about **1.6 kHz** in the reverb return.

---

## 4. Chug / percussive bass

What the sources say (thin; these are gun and bass videos rather than dedicated chug tutorials):

- **Chugs are the same patch played short.**
  - WM: "this is something I like to do with a lot of my chugs." Pitch bend of **±2 st, sometimes 5 or 7** on fills, or an octave drop. A quarter-note "jab".
  - **MIDI note length changes the tone**, because the envelopes and LFOs are cut at different phases.
- **Machine-gun bass** (BU):
  - A **snappy level envelope** set to envelope mode, so it hits once per note.
  - A clicky, fat wavetable ("clicky robot").
  - Heavy distortion and a multiband compressor.
  - A small **pitch envelope** gives the "glazier puke" character.
- **Arpeggiated one-shots** (WO):
  - An arpeggiator re-triggers a one-shot bass. Rate and gate set the rhythm and the tail length.
  - Then OTT, hybrid reverb and pitch-bend automation.
- **Gun as bass** (DPS, DR, OP): the guns in these videos are played as **pitched notes in a drop**.
  - The snare/perc content is key-tracked (OP: "enable pitch tracking… one-shot").
  - A sub sine plays the note.
  - So a gun is a **pitched percussive bass hit**, which is what a chug is too.
- **[inf]** A chug in this synthesis frame is:
  - the gun voice with a short gate (1/16–1/8 note);
  - a moderate body pitch drop (2–12 st over 20–60 ms) and a constant sub at the note;
  - the noise/metal layers mixed lower and more band-limited than in a gun, so the hit stays "bassy";
  - heavy pre-filter distortion, with a formant/notch filter sweeping per hit (the BU/OP notch, the DPS notch-boost around 822 Hz).
  - Fast repetition makes the retrigger behavior (envelope reset, phase reset, choke) audible, so per-hit phase reset of the sub and body oscillators matters for consistent punch.
- **808 vs chug** **[inf]**: both are "sub sine + short pitch drop + distortion of the harmonics only". The 808 has a long hold (300–400 ms) and decay (1.5 s) and glides. The chug is gated short and leans on mid-band distortion and formant movement.

---

## 5. What transfers to tearout guns: DSP building blocks

The collaborator's model (transient + noisy atonal body + sub + optional spice) matches the classic 4–5 layer snare decomposition (§1.1). The tearout videos add a **pitched, key-tracked** body, a **separate clean sub**, and **heavy multiband post-processing**. EDM Templates' only written description of guns: "short, explosive, pitchy, transient-heavy… fast-dropping pitch envelope + short amp envelope + FM movement + distortion/clipping; separate clean mono sub" [W:EDMT]. DR: "you need three elements: a tonal layer, a transient layer, and a sub bass."

### B1. Exciter / transient

- **Sources** (mixable):
  - A noise burst: white, or a 31-bit LFSR for a 909 character [W:ED909].
  - An impulse or one-sample spike (808 style).
  - A phase-offset sine (start at 90°).
  - A square for low-mid weight (ZJ).
  - A noise-modulated triangle (MZ).
  - An ultra-fast pitch drop.
- **Length**: **1–15 ms**. Typical values: 4–15 ms (ZJ), about 10 ms (LS), 2.5–3.4 ms pitch tau (AB). Use an exponential decay, or a linear decay that ends in finite time.
- **Pitch-drop click**: **+24 to +37 st**, hold 0–1 ms, decay **1–3.5 ms** (AB).
- **Filtering**: HP or BP 1–8 kHz. A dynamic boost of 1–2 kHz that ducks after **10–20 ms** (TH).
- **Randomization**: per hit, randomize only the noise seed, not the oscillator phase (ZJ).
- **Routing**: send it to both the output and the resonators (B3) — the exciter drives the modes.

### B2. Tonal body ("shell")

- **Oscillator**: sine, triangle, or a diode-rounded triangle (909), with an optional FM 2:1 whose index decays to 0 (LS).
- **Pitch**:
  - Key-tracked note, transposed. DPS: oscillator B −1 octave +7 st; OP: the snare pitched with the keys.
  - For a snare-like gun, body fundamentals are **150–330 Hz**: [W:SOS-SD] 180/330, VT 198, 808 238/476, MK 100–200.
  - **[inf]** For guns, key-tracking an octave or two above the sub keeps the body in 100–400 Hz.
- **Pitch envelope**: two-term exponential (§2.2).
  - Click term: +24 to +37 st, τ ≈ 1–3.5 ms.
  - Body term: **+3 to +5 st over 40–90 ms** for snare and gun (AB snare about 4 st); up to +24 st for kick-like guns. Serum gun pitch-env depths seen: about "8%" of the main-tuning range (DPS), +19 st coarse (DR sub).
  - Curve: exponential or convex ("shape +58%", AB). Do it on log-frequency.
- **Amplitude**: attack 0, **hold 20–45 ms**, **decay 40–200 ms**. Snares are 50 ms (tight) to 200 ms (loose) (LS); AB snare 40/40; AB kick 45/125.
- **Drive envelope**: push gain **+25% (≈+2 dB) over clip** for **30–40 ms hold + 5–12 ms decay** into a hard clipper, so the first ~50 ms are flat-topped (AB). A soft first stage with partial mix, then a hard stage.
- **Inharmonic partials**: optional extra sines from the measured-snare ratios (1, 1.49, 2.28, 3.65, 6.77, 10.6), with level and decay falling as frequency rises (VT). Or Reid's frequency-shifted triangle: 111 Hz base, +175/+224 Hz shifts.
- **Zero-crossing detail**: h(t) = sin(φ_h)·(1 − |sin φ_f|)^k, harmonic ≈ ×12, decay about 40 ms (AB) — adds bite without eating headroom.

### B3. Resonator / modal "metal spice"

- **Modal bank**: 4–16 two-pole resonators driven by B1.
  - Coefficients: a1 = −2r cos(2πf/fs), a2 = r². Per-mode T60 → r = 10^(−3/(fs·T60)).
  - **Ratio presets**: membrane, free bar, clamped plate, cymbal, bell, measured snare, 808-six-osc (tables in §3.2).
  - Rule: **higher modes decay faster** (VT, Corpus "material", Reid's cymbal).
  - Velocity raises the upper-mode gains more than the fundamental (DG, [W:SOS-SD]).
  - Doublet detune: ±0.1–2 Hz.
  - Key-track the low modes. Optionally leave the upper modes fixed or randomly detuned (DG).
  - **Stiffness macro**: generate the ratios from Chladni's law, f ∝ (m + 2n)^p with p = 1.4–2.4.
- **Feedback comb / waveguide**:
  - Delay M = fs/f.
  - Loop gain g = 10^(−3M/(fs·T60)), giving **T60 of 30 ms–1 s**.
  - A loop LPF or EQ sets damping; an allpass in the loop retunes the drum (ZJ).
  - A Karplus–Strong drum variant with b = 0.5 for rattly noise-pitch hybrids [W:KS].
  - **Parallel combs at inharmonic delays** for a plate-like sound **[inf]**.
- **808 metal bank**:
  - Six squares at **205.3, 304.4, 369.6, 522.7, 540, 800 Hz** (scalable by a transpose factor).
  - → BPFs at **3.44 kHz / 7.1 kHz** (Q is high but I could not get a precise value; **[inf]** Q ≈ 3–6).
  - → HPF with a resonance near 10.5 kHz.
  - → three envelopes: ~2 ms, ~40 ms, and 0.2–1 s.
  - Use 4× oversampling or BLEP squares; the paper simulated at 176.4 kHz [W:WAS-CY].
- **Ring mod / XOR**:
  - Multiply two squares or saws (any ratio, preferably irrational). Each odd-harmonic pair gives f_a ± f_b.
  - Envelope it on its own, HPF it at about 1 kHz, and duck it during the transient (CE, LS).
- **Random sine bleeps** (the collaborator's term): **[inf]** treat them as single resonator modes or short sine grains (5–40 ms) at random inharmonic frequencies in 1–8 kHz, triggered at or just after the hit. Equivalent to sparse PhISEM-style particle excitation of resonators [W] (Cook, abstract only).

### B4. Noise body ("wires", mostly atonal)

- **Source**: white or LFSR noise, or a sample/spectral source (OP and DPS load a snare/perc into the noise or spectral oscillator, key-tracked).
- **Filters**, one or both of:
  - (a) **HPF 1–3.4 kHz with a resonant bump** (MK: 3.4 kHz Sallen-Key).
  - (b) **BP around 1 kHz "clappy"** plus a **HP bright** band, each with its own decay (VT, 909).
  - The cutoff sweeps from high to low over the hit (LS, TH, OP's "gun-shaped" filter envelope: peak at the transient, sliding down).
  - A notch or band-pass-notch combination on top (OP).
- **Envelope**:
  - Attack 0 (808/MK), or a **delayed onset of 10–25 ms** (AB, LS) so the body speaks first.
  - **Hold ~24–30 ms** (909, AB).
  - **Decay 60–300 ms**, longer than the body.
  - Shape: faster than exponential (909) or a steep curve.
  - A lower-pitched body needs a longer noise decay (ZJ).
- **Drive**: saturate the noise hard (AB: +20 dB) before mixing; stereo spread OK above about 150 Hz.
- **Mix**: in the 808-style circuit the noise gets about 10× the drum's mixer gain (MK). Guns are "mostly noise", consistent with that.

### B5. Sub

- **Oscillator**: a pure sine at the note, often an octave or more below the body. DPS uses "analog sine down 3 octaves", with Serum's warp used for crunch.
- **Range**: kick-like sub **40–60 Hz** [W:CRED]. Keep it **mono below about 120–150 Hz** (DPS: mono below 130 Hz; [W:ATK808] split at 123 Hz).
- **Pitch**: a short punch envelope (DR: +19 st coarse; ITM: 34 ms decay), plus an optional 808 "sigh" of about −1 st over 150–400 ms.
- **Processing**: kept separate from the destroyed layers (OP, DR, GH, [W:EDMT]). OP argues an in-synth sub is fine if multiband keeps it clean. Light tube or harmonic distortion is allowed.
- **Phase**: phase-reset on each hit for consistent punch **[inf]**.
- **Glide**: legato portamento for 808 slides, 80–300 ms.

### B6. Nonlinear stage

- **Clipper with a drive envelope** (B2), and a waveshaper with drawable curves (OP "shaper").
- **Stacked overdrive** (DPS: "two stacks of overdrive").
- **Diode/wavefold** (LS, OS).
- **Multiband**: split at **~120–150 Hz**. The sub stays clean; the mid band gets distortion, OTT-like upward/downward compression and a convolver; the high band gets width.
- **Notch-boost "tone finder"**: a narrow boost swept to find an extra harmonic, e.g. **~822 Hz** (DPS). Also a mid notch on the sub layer (DR).

### B7. Post / space (tearout-specific)

- **Short convolution**: tight plate, real short IRs, or **tonal IRs** (a bass or snare sample as the IR, WM). Shortened with a fade-out; a "tone" control tilts toward the highs (OP).
  - Room for snares: about 15% wet small room (VT).
  - Send the reverb from a **transient-removed** copy (TH).
- **Disperser**: an allpass chain. OP uses a phaser to "delay the top frequencies… emphasizes transients, gooey". GH notes the **swept split frequency** (modulated per hit by an envelope) is what makes it.
- **Frequency shifter**: **−25 to −100 Hz** on the processed mid layer, plus a second shifter around −35 Hz (DR, OP). Automate it for per-hit variation.
- **Transient shaper with clip** and OTT (DR: time about 1000%, amount about 30%).
- **Pre-delay trick**: one layer offset by **−35 ms** (DPS).
- **Spectral fill**: "fill up every single frequency" with an EQ check (OP).
- **Tail control**: a lookahead gate or an extra level envelope (TH, OP).

### B8. Parameter cheat sheet for a gun voice **[inf]** (combining the cited numbers)

| Block | Param | Range | Default |
|---|---|---|---|
| Exciter | length | 1–15 ms | 5 ms |
| Exciter | HP cutoff | 500 Hz–8 kHz | 2 kHz |
| Click pitch env | depth / τ | +12 to +37 st / 0.5–4 ms | +30 st / 2 ms |
| Body | freq (key-tracked) | 80–400 Hz | note +12 st |
| Body pitch env | depth / τ | +2 to +24 st / 15–100 ms | +5 st / 40 ms |
| Body amp | hold / decay | 0–60 ms / 30–300 ms | 30 / 80 ms |
| Body drive env | over-clip gain / hold / decay | 0 to +6 dB / 0–50 ms / 5–20 ms | +2 dB / 35 ms / 10 ms |
| Noise | onset delay / hold / decay | 0–30 ms / 0–40 ms / 40–400 ms | 5 / 25 / 150 ms |
| Noise filter | HP / BP center / sweep | 500 Hz–5 kHz / 800 Hz–3 kHz / down 1–3 oct over decay | HP 1.5 kHz, BP 1.2 kHz |
| Noise drive | gain | 0–24 dB | 12 dB |
| Modal bank | ratio preset / N modes / T60 base / HF damping | see §3.2 / 4–16 / 30 ms–1.5 s / T60 ∝ f^−α, α 0–1.5 | snare-measured / 8 / 200 ms / 0.7 |
| Comb | f / T60 / loop LPF | 60 Hz–4 kHz / 20–800 ms / 1–12 kHz | — |
| 808 metal | transpose / BPF1/BPF2 / env times | 0.25–4× / 3.44 k / 7.1 k Hz / 2, 40, 300 ms | 1× |
| Ring mod | ratio fb/fa | 1.1–4.7 (irrational) | 1.414 |
| Sub | freq / punch / sigh | note (30–80 Hz) / +12 to +24 st, 5–40 ms / −1 st over 300 ms | +19 st / 15 ms |
| Multiband split | crossover | 100–200 Hz | 130 Hz |
| Freq shifter | shift | −150 to +50 Hz | −35 Hz |
| Gate / release | release | 2–30 ms | 10 ms |
| Chug mode | gate length / bend | 1/32–1/4 note / ±2, 5, 7, 12 st | 1/16 / 0 |

### B9. Design notes

- **Everything is one envelope family.** Reid and Avanzini: pitch follows energy. **[inf]** Let a single "hit energy" envelope (velocity-scaled) drive body pitch depth, mode brightness, noise cutoff and drive. That gives consistent hard-vs-soft hits for free.
- **Layer separation matters more than the oscillator** (Mat Zo, DR, OS). Keep each layer's envelope and level independent so the click, body, noise, metal and sub can be balanced separately.
- **Headroom placement** (AB): put HF detail at the fundamental's zero crossings and clip the body for its first ~50 ms. That is how "thicc" guns stay loud without mush.
- **808 vs gun vs chug** are presets of the same graph: change the sub/body/noise balance, the body pitch-envelope depth, and the gate length.
