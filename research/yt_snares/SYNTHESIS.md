# Bass-music snare synthesis: consensus from 45 tutorials

This is the design input for a JavaScript snare synth with per-family knobs. It builds on `../drum_synthesis/SYNTHESIS.md`, which covers the acoustic/808/909 snare model, modal ratios and the Noisia/SOPHIE/Teddy Killerz videos. That document's IDs are left out of the counts here.

## Corpus

- **45 videos, 38 distinct creators.** Each has a note in `notes/<id>.md` and a transcript in `txt/<id>.txt`.
- **Frames were read for 11 videos**, all in `frames/<id>/`: FYOPiPICCMA (Virtual Riot), IkkT48T-fBQ and Tq2w_wmoCTg (DNB Academy), fIZHjnAlKW8 (Art1fact), e1omWJI8f2o (QUIVILE), EZV6Ignx_mI (KelSounds), ooiy0mVY3Ow (W.A. Production), 98Ky4Qq1MBk (Fragmental), ra-mZivYchk (Au5), ewMTlw6VeAM (Letsynthesize), 7cqh0vjf-4M (Fletch).
- The most precise timing data comes from three places:
  - **Art1fact**: the Phase Plant LFO editor shows a millisecond axis.
  - **KelSounds**: an ms-axis volume shaper.
  - **W.A. Production**: a reference dubstep snare waveform on a 0–110 ms timeline.
- **Tags**: [said] means spoken, [screen] means read off a frame, and [inf] marks my own inference.
- **Videos per family**:
  - Synthesis-from-scratch: 24.
  - Sample-layering: 15.
  - Mixed: 6.

---

## 1. Consensus architecture

A bass-music snare is **three to five layers, each with its own envelope, staggered in time by 0–50 ms.** Then the whole stack goes through **distortion/clipping**, a **1–2 kHz emphasis**, and optionally a **short, high-passed room** and a **tuned resonance**.

| Layer | Role | Videos (count) | Typical design |
|---|---|---|---|
| **Body / "talk" / "donk" / "fundamental"** | the pitched thump | 30+ (nearly all) | sine/triangle, or a "rounded square"; 180–330 Hz; pitch drop; 40–130 ms |
| **Click / transient / "impact"** | stick hit, mix cut-through | 28 | a pitch spike on the body osc, a 90° phase start, a short noise burst, or a sample/kick click |
| **Noise / "wires" / "tail"** | rattle, snare identity | 40+ | white/pink, high-passed; **onset delayed** relative to the body; 80–300 ms |
| **Clap / character** | upper-mid energy, formant | 14 | a clap or a second noise with a band/formant peak at 1–2 kHz; faded in after the transient |
| **Tone / ring / pan / metal** | tuned resonance, genre signature | 17 | a narrow resonant peak, Corpus/Hollow resonator, ring-mod, high-ratio FM, or convolution with a harmonic IR |
| **Room** | "out of the digital void" | 15 | very short room or plate, high-passed, mixed low or wet-automated |

### Consensus counts on structural choices

**Body and noise sources**
- **One tonal osc plus one noise is enough** for the core. Virtual Riot said this in those words; Art1fact, KelSounds, QUIVILE, Fragmental, KULTURE, ARTFX-DnB and In The Mix all build it that way (**8+ videos**).
- **Noise-only snares** (driven or resonant filter on noise, pitch-enveloped): Fletch, Au5-1min, and VR's rim shot (**3**). The filter resonance supplies the "body".

**Timing between layers**
- **Noise onset delayed until the body/click has spoken: 14 videos.** Fragmental, Maulik, Art1fact, Ceptre, ARTFX-DnB, DNB Academy neuro, Au5, XLNTSOUND, ARTFX-dubstep, RPS, Frshmlk, Cure, KelSounds, QUIVILE.
- **Non-transient layers faded in after the lead transient** ("don't stack transients"): ARTFX-dubstep, Au5, RPS, Frshmlk, Ceptre, Cure, SPORTMODE (**7**).

**Pitch and tuning**
- **Separate fast click envelope plus a slower body pitch envelope:** DNB Academy ×2, KULTURE, Maulik, W.A. Production (**5**). The drum_synthesis corpus has the same split from Astrobear.
- **Tune the snare to the song key:** Dripment, Code: Pandorum ×2, Letsynthesize, ARTFX-DnB, Ceptre, SPORTMODE, bayesonic, RPS, Zens (**10**).
  - Usually the fundamental lands on a chord tone, often the **5th**. ARTFX-DnB used B over an E key.
  - The resonance goes on a related interval (§3.6).

**Processing**
- **Distortion or clipping as the glue between sine and noise:** 30+ videos. Art1fact: "smashing the sine and noise together with a really high drive is what glues them".
- **Time-varying EQ band** (gain or frequency enveloped over the hit): VR, Fragmental, Maulik, QUIVILE, DNB Academy ×2, ARTFX-DnB, XLNTSOUND (**8**).

---

## 2. Consensus numbers

### 2.1 Body

**Fundamental frequency**
- **Dubstep, brostep and tearout: 180–250 Hz** (around 200 Hz is the mode).
  - VR: 150–400, "around 200". Au5: 200. Maulik: 200. XLNTSOUND: 200. Art1fact: ~200.
  - DNB Academy: 180–200, with an SPAN spike at ~190 Hz [screen].
  - Mutrix: 180–250. The Mavrik: 200–300, aiming for 200–250. Producertech: 200–300, 250 ideal. ARTFX-dubstep: boost at 200. Arcade: 200.
  - W.A. reference waveform [screen]: about 250 → 220 Hz.
- **DnB and neuro: 250–500 Hz.**
  - ARTFX-DnB: "250 Hz was standard, modern DnB is higher". He tried 330 and ended on B.
  - Mr. Bill: 235–300. EEK: 200–500.
  - Teddy Killerz (prior corpus) and Ceptre pitch acoustic layers up.
- **Hybrid trap: 200–800 Hz** (SPORTMODE). Zens matched B4/C4 partials, about 500 Hz.
- **Body pitch fixed or tracking the note**: Mr. Bill fixes it ("changing the snare fundamental makes it harder to mix"). Most others play it from MIDI to retune it. Both should be possible.

**Waveform**
- Sine: about 15 videos. Triangle: VR, Fragmental, XLNTSOUND, bayesonic.
- **"Rounded square" / clipped sine**: ARTFX-DnB builds it additively, as sine + harmonic 3 at ~35% + a little of harmonic 5, normalized.
- W.A. Production [screen]: the reference body is flat-topped. He synthesizes it as a square into a low-pass filter (LP24 at 119–365 Hz, env amount 60%, decay 40 ms), so it smooths toward a sine.
- Cure: ripped Skrillex snares are flat ("compressed"). Ceptre: "square it off a little with saturation".
- **[inf] Consensus: a sine driven into a soft clipper, with drive on its own envelope**, so it is squared during the body and cleaner in the tail.

**Phase**
- Start the osc at a **90° phase offset with random phase off**, so the hit begins at peak amplitude for an instant click and identical hits. DNB Academy ×2 [screen: "phase 90°, RAND 0"]. ARTFX, Fragmental and QUIVILE also set random phase off.
- EEK inverts the phase "for more snap". **[inf]** This only changes the first half-cycle's direction.

**Body pitch envelope** (the slow drop)

| Source | Depth | Time |
|---|---|---|
| Art1fact [screen ms-axis] | +12 st | 50% at ~20 ms, 25% at ~40 ms, ~0 by 160 ms (≈20 ms half-life exponential) |
| QUIVILE [screen] | +12 st | to ~10% by ~15 ms, 0 by ~35 ms |
| XLNTSOUND [said] | ~+11 st | very fast; "laser attack, but not too lasery" |
| DNB Academy hard [screen/said] | 24 st then reduced to 1–2 st of main tuning | ~40–50 ms convex |
| Fragmental [said] | until no longer heard as pitch | decay ~50 ms, no longer than the amp decay |
| W.A. [screen] | Operator initial +48, peak +24 st | attack 1.75 ms, decay 600 ms on Operator's curve |
| VR [said+screen] | "a little, not like a kick" | near-linear over ~400 ms (small depth) |
| ARTFX-DnB [said] | small, "subtle extra pitch movement" | ~10 ms (100 Hz one-shot) with a hold at the start |

**→ Default**: +12 st with an exponential τ of 12–20 ms. Range 0–24 st and 5–60 ms.

**Click envelope** (separate from the body drop)
- DNB Academy: a spike of about 3–5 ms on osc coarse pitch, large depth [screen].
- KULTURE: +36 st on master tune with a very short decay.
- Astrobear (prior corpus): +37 st, 1 ms hold, 3.4 ms decay.
- **→ Default**: +36 st, 0.5–1 ms hold, 2–4 ms decay.

**Body amplitude**
- **Hold, then decay** is the consensus shape:
  - Au5 [screen]: Operator initial equals peak, so his "attack" acts as a **50 ms hold**, then **100 ms decay**.
  - ARTFX-DnB: "hold at max for the duration of the transient, then drop".
  - QUIVILE [screen]: a gated shape, flat with a dip, then a **steep cut at ~110 ms**.
- Fragmental: 60 ms decay, exponential (power ~3).
- In The Mix: 1 ms attack, 100 ms decay.
- KelSounds [screen]: about a 130 ms body.
- **DnB total length is about a 16th note**, roughly 86 ms at 174 BPM (ARTFX-DnB).
- **Count cycles** (Ceptre): "one wave cycle too long or short changes the vibe". Heavy snares run **about 5–6 cycles** of body. At 200 Hz that is 25–30 ms of strong body.
- W.A. reference [screen]: body about **47 ms, roughly 11 cycles**, at near-constant (clipped) amplitude.

**→ Default**: hold 30–50 ms, then exponential decay with τ ≈ 30–60 ms. The knob range is a total body of 40–200 ms.

### 2.2 Click / transient

- **Noise burst** (Au5 [screen]): Operator noise-looped at a fixed 100–200 Hz, **10 ms hold + 22 ms decay**, at −12 dB.
  - "Noise looped" is deterministic, so every hit has the same attack. Au5 said this is better for the click than true random noise.
  - **[inf]** For the JS synth: seed the click noise with a fixed seed per voice, and let the tail noise be free-running.
- **Phase-90° sine start** plus a pitch spike (see above).
- **Kick click as the snare impact**: Code: Pandorum ("something reminiscent of the click of a kick"), Kasanov (riddim), RPS (a kick under the snare, low-mids only), mic black (kick as the body).
- **Post-transient dip** (a deliberate notch):
  - KelSounds [screen ms-axis]: 100% for 0–4 ms, **dip to ~50% at ~6 ms, back to 100% by ~12 ms**.
  - QUIVILE [screen]: body level dips to ~60% at ~13 ms, recovers by ~28 ms.
  - Both say it makes the snare snappier.
  - Au5 says the opposite: "there shouldn't be any gaps or low points".
  - **[inf]** These fit together if the dip is short (under 10 ms) and right after the click. A dip in the middle of the body is what Au5 warns about.
- **Transient shaping**: Art1fact and others say not to boost the attack with a transient shaper ("attack should be designed with pitch and envelopes"). Cut the sustain instead.

### 2.3 Noise

**Onset delay**
- Fragmental: noise attack equals the body decay, 60 ms.
- Art1fact [screen]: rises from 0 to a peak at **~20 ms**, a second peak at **~45 ms**, gone by ~160 ms.
- DNB Academy hard [screen]: linear rise to a peak at **~25 ms**, fast fall to ~10% by ~100 ms, then a low tail.
- DNB Academy neuro [screen]: peak at ~8–10 ms, gone by ~50 ms. "Not on top of the transient, not too far or it sounds fake."
- ARTFX-DnB: "noise comes in when the fundamental ends."
- Au5: tail fade-in of **100 ms** over 600 ms.
- QUIVILE [screen]: peak at ~10 ms, convex decay, gone at ~227 ms. That is about 2× the body.
- **→ Default**: noise attack 5–25 ms (linear or concave) and decay 80–200 ms. The noise must outlast the body. Range: delay 0–60 ms and decay 40–600 ms.

**"Flutter" / double peak** (Art1fact [screen])
- Two bumps about 25 ms apart, at 95% and 100%. "Without it, not as dynamic."
- **[inf]** It works like a minimal clap: a 2-burst noise envelope.

**Filter**
- A high-pass is near-universal: Fragmental 24 dB, Maulik 18 dB, In The Mix 18 dB, ARTFX-DnB 18 dB, KelSounds HP at about 2 kHz, Maulik around 800–1200 Hz with resonance.
- KULTURE: don't high-pass above 200 Hz on the whole snare.
- **Brightness falls over time**: ARTFX-DnB's noise-splash layer uses a falling pitch envelope plus a falling LP cutoff ("noise starts brighter and gets darker toward the end"). VR and Fletch drop the noise pitch.

**Noise type and stereo**
- White, pink, "HP12 stereo" (Serum 2), "ARP white", "JP106 HP noise", "Bright White". Real-world recordings also appear: fizz, air cans, AC hum, and a Geiger counter (VR).
- **Stereo**: decorrelated L/R noise. Fragmental: 2 unison voices at full detune. ARTFX: a mono noise offset between L and R.
- Zens: "mix between mono and stereo; too stereo is weirdly wide for a snare."
- Pandorum: the transient is mono.

**Noise "with metal in it"**
- W.A.: plain white noise "will always sound like regular white noise". Real snares and cymbal layers carry metallic content.
- Fixes for this are in §3.6. Letsynthesize uses FM noise with high inharmonic ratios (18.6, 21.1, 37, 64) plus operator feedback.

### 2.4 Spectral shaping (the "snare formant")

**1–2 kHz emphasis** (11 videos)
- VR: a wide peak at 1 kHz. "The snare should end on the 1 kHz formant, like the vocal R."
- Code: Pandorum ×2: **1.6–1.8 kHz** ("crucial for any snare", "the sound you hear in the mix").
- KULTURE: a big wide boost at ~1 kHz ("low mids is where the snare comes to life").
- QUIVILE: 1–2 kHz. Maulik: ~1 kHz with modulated gain. DNB Academy neuro: ~2 kHz enveloped band ("clappy, the springs hitting").
- DNB Academy hard [screen]: **996 Hz, Q 84, +22.7 dB** after a convolution. This is a resonance, not a tone control.
- In The Mix: 2–4 kHz. RPS: dynamic boosts at **200 Hz and 2–4 kHz**. SPORTMODE: the peak layer at 1–3 kHz.

**Cuts**
- **~500 Hz scoop** (VR: "where instrument fundamentals sit; drums go low thump, nothing, then 1 kHz"). Maulik also cuts 500. ARTFX-dubstep uses a mid scoop for a "hollow" sound.
- HPF at **100–110 Hz** (Producertech 100, QUIVILE 107, Maulik 100).

**Time-varying formant** (VR)
- Two enveloped peaks: one sweeps *up* to 1 kHz and one sweeps *down* to 1 kHz.
- "It starts with a heavy transient and lots of low end; by the end it's only higher frequencies, like a whip."
- Fragmental and Maulik envelope an EQ band's gain *upward* across the hit.
- **[inf]** This is the signature of the VR/neuro-dubstep family: the spectral centroid rises over the hit while the body decays.

### 2.5 Distortion and dynamics

**Types**
- Soft clip: DNB Academy, Fragmental, Zens, Au5 (Glue soft clip), KULTURE.
- Tube: VR, KULTURE, Zens.
- Hard clip: QUIVILE, Convexity.
- Waveshaper: DNB Academy neuro "M Waveshaper 100% wet as the glue", Slime Cinema, Arcade.
- Saturate: Art1fact (~15 dB drive), ARTFX-DnB (~4 dB + 2 dB, in two stages).

**Enveloped distortion**: VR and QUIVILE give the drive and mix their own envelope, with more distortion during the transient and less in the tail.

**OTT**: VR, Convexity, Slime Cinema, Cure, KelSounds, RPS.
- Convexity's settings: output **+13 dB, time 1000%, amount 70%**.
- RPS: time 667%, downward 24%.
- **Long OTT time avoids smearing and wavefolding artifacts** (Cure, RPS, Convexity).

**Compressors**
- KULTURE: LA-2A style, threshold −10, 4:1, **15 ms attack** (lets a 15 ms snap through).
- In The Mix: attack ≥100 ms lets the transient smash through.
- Fletch: attack 0 / release 100 for a sharp attack.

**Final clipper or limiter** pushed hard: VR ("OK to clip as long as the transient sounds fine"), ARTFX-dubstep (+8 dB maximizer), Frshmlk (redline into the master clipper), Convexity, KULTURE, Zens, Slime Cinema, QUIVILE.
- **Limiters used as tail shorteners** (Slime Cinema).

### 2.6 Room / reverb

**Short and subtle**
- Art1fact: **~4% mix**, "from a digital void to a room".
- ARTFX-DnB: Valhalla Room, no pre-delay, very short, high-passed, barely audible ("without it, it sounds weak").
- Fletch: "dark snare room". Mr. Bill: reverb on a noise copy as a "clap", with less pre-delay.

**Gated: wet on the tail, dry on the body**
- Au5: automated wet/dry.
- Maulik: reverb mix modulated negative so the fundamental stays dry.
- In The Mix: reverb mix starts at 100% and closes over about 500 ms ("adds harmonic energy").

**Long, high-passed tails** (brostep and tearout)
- ARTFX-dubstep: room on the noise layer, EQ'd to highs only, boosted.
- Mutrix: HP'd 100%-wet return.
- RPS: **input low-cut 1.54 kHz, decay ~800 ms, extra HPF at 403 Hz.**

**"The reverb tail" is often an enveloped noise layer**, not a reverb: Dezolent, Deucez (a resampled clap-reverb tail faded in), Producertech (a clap through 100%-wet reverb).

**Tiny room = metal**
- Maulik: Serum reverb at size 0, low decay, high mix gives a metallic plate.
- XLNTSOUND: a short room/delay with reflections up for the metallic part of Skrillex QFF snares.

**Never put reverb on the low end** (RPS, Au5, ARTFX).

### 2.7 Retuning

- Use a **frequency shifter** rather than a pitch shift or transpose to retune a finished snare: Au5 ("preserves timbre and length"), VR, KelSounds, Slime Cinema.
- Tempo-synced envelopes let the project tempo lengthen or shorten the snare (VR).
- A global "time" or "tightness" macro scales all envelope rates together: Au5 (Operator Time), Art1fact, KULTURE, XLNTSOUND.
- XLNTSOUND: going up an octave requires shortening the length.

---

## 3. Archetypes and the DSP that defines each

### 3.1 Brostep / Skrillex-era (2010–13)

**Sources**: Cure, The Mavrik, ARTFX-dubstep, Mutrix, Letsynthesize ×2, Producertech, Arcade/Koan, XLNTSOUND (QFF era).

**Shape**
- A **noisy transient first, then a sine-shaped "doofy" body** at **~200–250 Hz** that comes in after the transient (Cure, from ripped Scary Monsters snares).
- The body is **heavily compressed, nearly sustained and flat-topped** for ~40–60 ms.
- Then a white-noise or **China cymbal** tail (Cure), and a **long high-passed reverb** (ARTFX-dubstep, Mutrix).

**Construction**
- Built from 909s (Skrillex, per Cure).
- Or a "ladder" of layers rising in frequency: tom pitched up to 200–250 Hz, 909 low, 909 high, clap, then a parallel copy HPF'd at 400 Hz (Mavrik).
- Everything is compressed.

**Defining DSP**
- Body sustain/hold of 40–60 ms with clipping.
- Delayed body onset.
- Noise or cymbal tail at 150–400 ms, plus HP'd reverb at 0.8–1.5 s.
- Heavy bus limiting.

### 3.2 VR / neuro-dubstep "whip" snare

**Sources**: Virtual Riot, Fragmental, Maulik, KelSounds, Fletch.

- **One osc (triangle ~200 Hz) plus noise in one patch.**
- Envelopes come from LFO one-shots, often tempo-synced.
- **Distortion amount enveloped.**
- An **EQ peak swept toward 1 kHz** (the centroid rises).
- OTT, then a hard limiter.

**Defining DSP**
- A time-varying peak filter at 300 Hz → 1 kHz, with Q ~1–2 and +6–12 dB.
- Enveloped drive.
- A noise pitch envelope.

### 3.3 Riddim

**Sources**: Kasanov, Dripment, RPS (kick under the snare), ovasenpai (not transcribed).

- A **kick's click/transient** (sub removed) **plus dry claps** with their tails cut.
- The crunch comes from clipping: "that crunchiness is what a riddim snare is".
- A **vowel formant ("ah") peak in the clap**.
- Mostly dry and short.
- The kick plays under the snare on 3, so it is "already sidechained".

**Defining DSP**
- A low thump click at 60–120 Hz, at kick-like pitch.
- Clipped/crunchy dry noise with a formant bandpass around 700–1200 Hz **[inf from "ah"]**.
- Little or no tail.

### 3.4 Tearout / deathstep "pan" snare

**Sources**: Code: Pandorum ×2, neutrinia, KelSounds-metal, RPS, Dripment, oddprophet (prior corpus).

- **"Talk, tail, tone"** (Pandorum):
  - **Talk**: a mono, transient-processed click with its fundamental boosted.
  - **Tail**: noise, low-cut, **boosted at 1.6–1.8 kHz**, reverbed.
  - **Tone (pan)**: one spectral peak boosted with a **very narrow bell, snapped to the song key**, then **distorted**, then low-cut again.
- RPS uses **Corpus (membrane) tuned to the kick's pitch class, decay 325 ms, band-limited to 318 Hz–4.8 kHz, peak-compressed, 32% reverb**, in parallel with the dry stack.
- RPS also puts a **kick's low-mids under the snare**.
- KelSounds: a resonant peak at **~774 Hz**, a frequency shifter (automated per hit for variants), and a shaper at +8 dB drive, 58% mix.
- neutrinia: **ring-mod or static phaser (comb notches) plus 2× distortion** on a snare.

**Defining DSP**
- A **high-Q resonance (or resonator bank) at 500 Hz–2 kHz with 150–400 ms decay, tuned to key, then distorted**.
- A body that is partly the kick.
- Loud.

### 3.5 DnB / neurofunk tight-tuned snare

**Sources**: ARTFX-DnB, DNB Academy ×2, QUIVILE, Art1fact, KULTURE, Ceptre, Mr. Bill, mic black, Teddy Killerz / Astrobear / Mat Zo (prior corpus).

- A **higher body (250–500 Hz)** with a rounded-square shape.
- **Very short**, about a 16th note.
- **Click from a 90° phase start plus a pitch spike.**
- **Noise enters as the body ends.**
- The tail gets **resonance from noise**:
  - ARTFX-DnB: 5–6 max-Q EQ bells above the fundamental, picked by ear, with **the EQ's global gain automated: 0 during the transient, peaking with the noise**.
  - DNB Academy: a convolution with a harmonically rich IR (size ~46%), then a narrow +22.7 dB peak at ~1 kHz.
  - Mr. Bill: Corpus membrane tuned to the body (~235 Hz) or a multiple.
- A subtle room.
- **Waveshaper on the sum** as the glue (DNB Academy neuro).

**Defining DSP**
- Precise tuning.
- Short envelopes.
- A **noise-excited resonator bank whose wet level is enveloped to rise after the transient**.

### 3.6 Hybrid trap / xdm "resonant-peak" snare

**Sources**: SPORTMODE, Zens, Frshmlk, bayesonic (colour-bass variant).

- A **tuned fundamental at 200–800 Hz.** Zens uses two sines matched to the reference's partials, the second quieter.
- SPORTMODE adds a second osc with **FM, 4 octaves up**.
- A "stadium" clap layer, sometimes with breath or glass textures.
- A **resonant-peak layer**: a struck glass, spoon or bell, through **Corpus, then soothe in Delta mode** (only the resonances survive).
  - **Tuned 1, 4 or 7 semitones above the fundamental's pitch class.**
  - **Kept at 1–3 kHz.**
  - **Entering slightly late.**
  - With an **inverse EQ cut on the clap at that frequency.**
- bayesonic's Hollow resonator is tuned **one semitone above the body** (F body, F# ring).

**Defining DSP**
- A resonator at a **musical interval above the body**, in the 1–3 kHz band, delayed by a few ms.
- Lossy/MP3 degradation at low mix (Zens).

### 3.7 Colour-bass / hyperpop "beep" and metallic snares

**Sources**: Convexity, Slime Cinema ×2, bayesonic, plus SOPHIE (dnksaus) and Celestric in the prior corpus.

- **Beep snare**: a tiny click plus **two sines an octave apart, triggered as a flam** (two notes offset by a few ms).
  - Transient shaper.
  - A **disperser chain for a "laser" zap**.
  - OTT (+13 dB, 1000%, 70%), then a hard clip.
  - **Linear-phase HPF after the clipper.**
- **Metallic (dariacore)**:
  - A trap snare body plus a clap with complementary EQ.
  - Two **ring-modulated foley** metal layers. The ring-mod frequency is tuned to complement the snare pitch.
  - Enters just after the clap.
  - Parallel chorus and pitch-shift, then a small room, then a **gate**.
  - Then a convolution using another drum as the IR.
- **Tonal**: real-snare harmonics accentuated with EQ, plus a pitched-down, frequency-shifted bell ("snare coils").

**Defining DSP**
- Audible pitch. The tone is allowed to be melodic here.
- Ring-mod/FM metal.
- Flam timing.
- Dispersion.

---

## 4. Common mistakes: what makes a snare stop sounding like a snare

1. **Stacked transients.** Every layer with its own attack makes "a mush" and steals headroom.
   - The fix is fading in the clap, noise and body under one lead transient.
   - Sources: ARTFX-dubstep, Au5, RPS, Frshmlk, Ceptre, Cure, Letsynthesize (7).
2. **Low end at time zero.** Letsynthesize: "do you see any low frequencies in the first section? I don't."
   - Move the bottom layer a few ms later. W.A. puts a few ms of delay on the bottom.
3. **Noise on top of the transient, or too far behind it.** The first blurs the click; the second "sounds fake" (DNB Academy neuro). The usable window is about 5–25 ms.
4. **Fundamental too loud or too long.** Zens: "you don't want it to sound too melodic; it still has to be a drum."
   - Too many body cycles, or a slow pitch drop, reads as a tom or a kick. Ceptre: one cycle matters.
   - **Too much pitch envelope reads as a laser.** XLNTSOUND and DNB Academy both cut their initial depth by 80–90%.
5. **Plain white noise reads as "just noise."** It needs resonance or metallic content (W.A., ARTFX-DnB, Letsynthesize).
   - An unfiltered noise layer with low end reads as a tom plus a bad clap (Moritz Klein, prior corpus).
   - High-pass the noise.
6. **Layers out of tune or out of phase.** Two bottoms on different notes, or a flipped polarity, cancel the weight.
   - Use one bottom, one transient and one tail (Letsynthesize, Cure, XLNTSOUND).
7. **Cutting too close to the fundamental** loses punch and "messes up the low end." Leave space below the body, not a steep HPF right at it (W.A., Letsynthesize, Art1fact, Cure).
8. **Stacked reverbs.** Several samples each with baked-in reverb pile up (Dripment).
   - Reverb on the low end or the body muddies it. High-pass the reverb and keep the body dry (RPS, Au5, ARTFX, Maulik).
9. **Too long for the tempo.** DnB and dubstep snares are short; long tails eat the space between hits (ARTFX-DnB, Mutrix, Arcade).
10. **Gaps or dips in the middle of the body** (Au5). This is separate from the deliberate under-10 ms post-click dip in §2.2.
11. **Random oscillator phase** gives inconsistent hits; reset the phase (DNB Academy, ARTFX, Fragmental, Au5's noise-looped click).
12. **Transposing a finished snare** changes its length and timbre; use a frequency shifter (Au5).
13. **Too stereo**, or a stereo transient. Keep the click and body mono and the noise partly wide (Zens, Pandorum).
14. **A hi-hat transient on a snare** is too bright. A snare attack should be darker (XLNTSOUND).

---

## 5. Proposed JS architecture and knobs

All values below are [inf] distilled from §2–3.

```
voice:
  click   : body-osc pitch spike (+clickSt, clickMs) + optional seeded noise burst (hold/decay)
  body    : osc(shape: sine→rounded-square morph, phase0=90°) * ampEnv(hold, decay)
            pitch = f0 * 2^((bodySt*exp(-t/bodyTau) + clickSt*exp(-t/clickTau))/12)
            → soft-clip with drive envelope (driveStart→driveEnd)
  noise   : white/pink, stereo-decorrelation 0..1
            ampEnv(delay, attack, decay, flutter[2nd-peak offset, depth])
            HPF(cut, 12/24dB) + LP cutoff falling env (bright→dark), optional noise pitch drop
  clap    : optional 2-4 burst noise (burst spacing 6-12 ms) → BP 1–2 kHz
  ring    : noise/click-excited resonator bank (N modes, ratio preset, T60, Q)
            tuning = body pitch class + interval {0, +1, +4, +7, +12 st}, or fixed key note
            wet env: 0 during click, rises with noise
            → drive
  sum     : formant peak (freq start→end, Q, gain env)  [VR whip]
            mid scoop ~500 Hz, HPF 80–120 Hz
            waveshaper/softclip (glue) → OTT-lite → clipper (ceiling)
  room    : short room/plate, HPF 400–1500 Hz, wet env (gated) or 2–8% static
globals: tune (Hz or note), keytrack on/off, time (scales all env times), velocity → click/noise level + time
```

### Per-family presets (defaults)

| Knob | Brostep | VR-whip | Riddim | Tearout-pan | DnB/neuro | Trap-peak | Colour-beep |
|---|---|---|---|---|---|---|---|
| f0 | 200–230 Hz | 200 Hz | 80–120 Hz (kick click) | 180–220 Hz | 250–450 Hz | 250–600 Hz | 400–900 Hz, octave pair |
| body hold/decay | 45 / 60 ms | 20 / 120 ms | 5 / 30 ms | 30 / 80 ms | 15 / 40 ms | 15 / 50 ms | 10 / 80 ms + flam 8–15 ms |
| body pitch | +7 st, τ 15 ms | +5 st, slow (~150 ms lin) | +24 st, τ 8 ms | +12 st, τ 15 ms | +12 st, τ 12 ms | +7 st, τ 10 ms | 0–3 st |
| click | +36 st, 3 ms; noise burst 8 ms | +24 st, 3 ms | kick click, clipped | +36 st, mono, hard | 90° phase + +36 st, 2 ms | +24 st, 2 ms | sample click / disperser |
| body shape | rounded square, heavily clipped | triangle + tube | clipped | sine → clipped | rounded square | sine + FM ×16 osc | pure sines |
| noise delay / decay | 15 / 250 ms | 5 / 180 ms | 0 / 60 ms (dry clap) | 10 / 200 ms | 8–25 / 90 ms | 10 / 200 ms (stadium clap) | 5 / 60 ms |
| noise HPF | 1.5 kHz | 800 Hz | 600 Hz + formant ~1 kHz | 1 kHz | 1–2 kHz | 1 kHz | 2 kHz |
| ring | off / China-like metal | off | off | **on**: high-Q 600–1500 Hz, T60 300 ms, key-tracked, driven | on: noise→bank, 5 modes above f0, wet rises after click | **on**: +1/+4/+7 st, 1–3 kHz, delayed 5 ms | ring-mod metal optional |
| formant sweep | — | **300 → 1000 Hz peak, +9 dB** | — | static 1.6–1.8 kHz +6 dB | static 1–2 kHz | 1–3 kHz cut on clap | — |
| drive | heavy + limiter | enveloped (high→low) | hard clip | heavy | moderate softclip + waveshaper | soft | hard clip |
| room | HP 1.5 kHz, 0.8–1.5 s, loud | 10–20% | none | 0.8 s, HP 1.5 kHz | 3–5%, very short | short | gated small room |

### Implementation notes from the sources

- **Seed the click noise per voice** (Au5's noise-looped choice). Let the tail noise be random. A Geiger-style sparse noise option gives per-hit variation (VR).
- **The post-click dip**: an optional 6–10 ms amplitude notch to 50% right after the click (KelSounds, QUIVILE).
- **A "time" macro scales every envelope**, and pitch-up should co-shorten. Most sources expose a single length/tightness knob.
- **Velocity**: scale the click and noise level more than the body, and shorten envelope times (Au5 1-min: velocity→time 33%, velocity→volume 66%).
- **Output stage**: aim for clipping. Every bass-music source prints its snare hot into a clipper or limiter. A linear-phase HPF after the clipper controls low-frequency clip artifacts (Convexity).
