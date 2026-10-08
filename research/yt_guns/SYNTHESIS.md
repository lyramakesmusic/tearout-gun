# Tearout gun / machine-gun bass: synthesis of 42 YouTube tutorials + 13 on-screen analyses

## Corpus and method

- **42 videos with transcripts** (notes/<id>.md), from **27 distinct creators**. Three more search hits (hP9v4DMRk1s, Hn44XLGan_A, FBo0BgTLP2E) had no speech and were dropped; x8kJ93TSyMQ is near-empty (arrangement only).
- **On-screen pass on 13 videos** (frames/<id>_<sec>.jpg, 47 frames kept). Nine are in the 42 (etZIR8MP3XU, LN2JgfVNmxA, P55h-jIKinc, KkXF4T_WPbY, vhciw1HgExE, w6q4AQiRR1Y, _yvW7iH_ufk, mJdPGZXxHyU, Zcx64R68uoc). Four come from the already-covered set (IL5I2ME8wZo, sk--QbE_TM0, YvEIKc2Eg9Q, WCN-ILwRTMI). Their notes hold only the on-screen section.
- Counts below are **videos that state or show the technique for the gun**, not keyword hits. Sound design for screeches, growls, and drums is left out unless it was used on the gun.
- **Independence caveat.** Several channels contribute 2–4 videos each: oddprophet (3), Code: Pandorum (2), PHENOMSOUND (3), Rocket Powered Sound (3), DECIEVENCE (4), Silvyr (3), Dripment (2 + WCN), BassTi (2), Synoid Dub (2). Swampy and Code: Pandorum both credit oddprophet's Multipass+convolver method by name. **WCN-ILwRTMI (Dripment) runs Letsynthesize's own "Machine Gun Bass" preset** (ftwBNJP1tgs), so those two are one source. Where it matters, the count is given as videos / distinct creators.

---

## Top-line consensus (ranked)

| # | Finding | Videos | Creators | Representative IDs |
|---|---|---|---|---|
| 1 | Each hit is gated by a **drawn per-note decay** (LFO in envelope/trigger mode, or amp env with sustain −∞). There is no sustain; the rhythm comes from retriggering | ~26 | ~20 | etZIR8MP3XU, LN2JgfVNmxA, P55h-jIKinc, KkXF4T_WPbY, vhciw1HgExE, w6q4AQiRR1Y, Zcx64R68uoc, mJdPGZXxHyU, ftwBNJP1tgs, FeWnYudwCno, ibdbe1nxAhs, V5JLKz82WlE, w7H7P2VMnVs, Vy7hIbdZE4E, JLUvQ1aAoDM, Rapv1lcXagA, P6elrI3VaKs, mxCxFuOokjY |
| 2 | **OTT or multiband up/down compression** on the gun (in-synth multiband or Xfer OTT) | ~30 | ~22 | nearly all synthesis videos |
| 3 | **Allpass dispersion** (Kilohearts Disperser, an "EQ Three ×N" rack, Serum allpass filter, Multiband Delay) for a "lasery" transient and liquid mids | 20 | 15 | 2Ox-rSTms9o, LM5LgEvjNHs, ODdHap_SYM0, nApsgDWSQ7Y, -Z5YunR-upo, _yvW7iH_ufk, KkXF4T_WPbY, w7H7P2VMnVs, twT4wEypWt8, 1033mUjb1oI, P55h-jIKinc, mHMv1M90NKw |
| 4 | A **resonant metallic tail** from a **comb filter** (12 videos; 15 with on-screen) and/or a **10–36 ms delay with feedback** (12). Union: ~20 videos | ~20 | ~16 | comb: ftwBNJP1tgs, mJdPGZXxHyU, Zcx64R68uoc, vhciw1HgExE, Vy7hIbdZE4E, w7H7P2VMnVs, _5t_90zAnHk, bDPyECsNHGk, JLUvQ1aAoDM; delay: FeWnYudwCno, vhciw1HgExE, Zcx64R68uoc, JYeBhtNFdB8, _5t_90zAnHk, 1k4N5kiQINE, P6elrI3VaKs, P55h-jIKinc |
| 5 | **A fast downward pitch blip at onset**, on the gun body (12) and/or the sub (9). Union: 18 | 18 | 15 | etZIR8MP3XU, LN2JgfVNmxA, YoXPfuez3nQ, Rapv1lcXagA, ftwBNJP1tgs, KkXF4T_WPbY, w6q4AQiRR1Y, _yvW7iH_ufk, fJZYQPGFXB4, Vy7hIbdZE4E, ODdHap_SYM0, 1033mUjb1oI |
| 6 | A **separate clean mono sub** under a high-passed gun, usually following the gun's rhythm | ~17 | ~14 | 1033mUjb1oI, nApsgDWSQ7Y, Vy7hIbdZE4E, _yvW7iH_ufk, LM5LgEvjNHs, 2Ox-rSTms9o, fJZYQPGFXB4, twT4wEypWt8, KkXF4T_WPbY, Rapv1lcXagA, P6elrI3VaKs, ftwBNJP1tgs |
| 7 | **Sample sources**: a snare, gunshot, percussion, or bass one-shot used as the oscillator, or layered as the gun | 17 | 13 | P55h-jIKinc, bDPyECsNHGk, JLUvQ1aAoDM, 1033mUjb1oI, 1k4N5kiQINE, qkN3c78U9hE, JYeBhtNFdB8, ODdHap_SYM0, LM5LgEvjNHs, 2Ox-rSTms9o, Rapv1lcXagA, LN2JgfVNmxA, YvEIKc2Eg9Q |
| 8 | **Convolution** as the core colour: short IR, "the most important ingredient" | 16 | 12 | etZIR8MP3XU, LN2JgfVNmxA, YoXPfuez3nQ, QmI2qznHABk, P55h-jIKinc, LM5LgEvjNHs, ODdHap_SYM0, _yvW7iH_ufk, bDPyECsNHGk, JLUvQ1aAoDM, Rapv1lcXagA, P6elrI3VaKs |
| 9 | A **dedicated transient layer** (noise burst, percussion, click), timed exactly on the hit | ~14 | ~12 | nApsgDWSQ7Y, fJZYQPGFXB4, Vy7hIbdZE4E, P6elrI3VaKs, _yvW7iH_ufk, 3ZNN60bTfT4, LM5LgEvjNHs, Rapv1lcXagA, P55h-jIKinc, LN2JgfVNmxA |
| 10 | **FM** in the body, with the modulator an octave or more above the carrier, or a fifth off | 14 | 12 | FeWnYudwCno, vhciw1HgExE, w6q4AQiRR1Y, ftwBNJP1tgs, KkXF4T_WPbY, V5JLKz82WlE, w7H7P2VMnVs, P55h-jIKinc, _5t_90zAnHk, sk--QbE_TM0 |
| 11 | **Frequency shifter**: for variation across a drop, or "atonal, evil" grit | 14 | 11 | fJZYQPGFXB4, LN2JgfVNmxA, etZIR8MP3XU, _5t_90zAnHk, KkXF4T_WPbY, P6elrI3VaKs, Rapv1lcXagA, -Z5YunR-upo, w6q4AQiRR1Y, YoXPfuez3nQ |
| 12 | **Resample/print**, then chop, re-pitch, and cut the tails | 13 | 10 | LM5LgEvjNHs, 2Ox-rSTms9o, nApsgDWSQ7Y, fJZYQPGFXB4, P55h-jIKinc, KkXF4T_WPbY, Rapv1lcXagA |
| 13 | **One modulator drives many destinations**: the same decay curve on level, WT position, filter, FM, drive, and FX mixes | ~12 | ~10 | P55h-jIKinc (16 destinations), vhciw1HgExE (10), WCN-ILwRTMI (8), Rapv1lcXagA, Vy7hIbdZE4E, w6q4AQiRR1Y, _5t_90zAnHk, JLUvQ1aAoDM |
| 14 | **Hard clipping**: the sub and guns clipped together, plus a final clip/limit | ~10 | ~8 | LM5LgEvjNHs, 2Ox-rSTms9o, -Z5YunR-upo, ftwBNJP1tgs, Rapv1lcXagA, P55h-jIKinc, 1033mUjb1oI, twT4wEypWt8 |
| 15 | A **per-hit filter sweep, high → low** ("guns start high and end low, like a kick") | ~10 | ~8 | etZIR8MP3XU, LN2JgfVNmxA, YoXPfuez3nQ, YvEIKc2Eg9Q, Rapv1lcXagA, -Z5YunR-upo, JLUvQ1aAoDM, Vy7hIbdZE4E |
| 16 | A **notch / band-reject filter** on the gun (Serum HN12/BN12/High-Notch, Massive double notch) | 9 incl. on-screen | 7 | KkXF4T_WPbY ("the concussive side of the gun"), fJZYQPGFXB4, w6q4AQiRR1Y, YoXPfuez3nQ, LN2JgfVNmxA, YvEIKc2Eg9Q, nLxHdC0FBwU, mxCxFuOokjY |
| 17 | **Triplet rhythm**: 1/8T or 1/16T bursts, or LFOs at 1/8T | 8 + 2 on-screen | 8 | LM5LgEvjNHs, 2Ox-rSTms9o, w7H7P2VMnVs, vhciw1HgExE, FeWnYudwCno, ibdbe1nxAhs, nLxHdC0FBwU; on-screen _yvW7iH_ufk, sk--QbE_TM0 |

---

## Layer by layer

### 1. Transient (first ~5–50 ms)

**What makes the attack (most to least common):**
- **A pitch drop at onset** (18 videos). This is the most-cited transient mechanism. oddprophet: "pitch bend is really important with guns, it emphasizes the transient" (etZIR8MP3XU). Swampy: "the laser, it really needs it" (Rapv1lcXagA).
  - On-screen shapes: an **exponential plunge from max to ~0 within 10–25% of a 1/8–1/4 note cycle**. That is ~40 ms in LN2JgfVNmxA (1/4 at 140 BPM, ENV mode) and ~50 ms in WCN-ILwRTMI (1/8). Slower outliers: ~140 ms in w6q4AQiRR1Y, ~60 ms for Pandorum's LFO3 (P55h-jIKinc).
  - Amounts: Serum master-tune mod amounts of **23–40** (JLUvQ1aAoDM, vhciw1HgExE, w6q4AQiRR1Y). Sub pitch envelopes of **+15 st** (Vy7hIbdZE4E: "higher sounds goofy"; also P6elrI3VaKs's transient operator), **+19 st coarse** (fJZYQPGFXB4), and Operator Initial +12 / **Peak +36 st × 75% amount** (_yvW7iH_ufk, on-screen).
- **A separate transient layer** (~14). It is a white-noise burst (Vy7hIbdZE4E, JLUvQ1aAoDM), a short percussion sample (nApsgDWSQ7Y, fJZYQPGFXB4, LM5LgEvjNHs, 2Ox-rSTms9o), a Collision "beam" click (_yvW7iH_ufk: decay 139 ms, tune +36 st, noise D 16 ms), or a very short, very high WT plus a noise osc inside the patch (P55h-jIKinc). Two rules recur: **timing must be exactly on the hit** (Vy7hIbdZE4E, P6elrI3VaKs), and the **transient stays out of the heavy group chain so it keeps its punch** (_yvW7iH_ufk, P6elrI3VaKs).
- **Dispersion on the transient** (see Spice/Group). The Disperser is "set high to get a clean lasery phasey effect at the beginning" (LM5LgEvjNHs) and "makes it beefier at the beginning, which is really what a gun is" (2Ox-rSTms9o).
- **A high→low filter sweep per hit** (~10). It gives a bright first few ms (etZIR8MP3XU, LN2JgfVNmxA, YoXPfuez3nQ).
- **Transient shapers** (6): Kilohearts Transient Shaper with clip on (fJZYQPGFXB4), Thorn (3ZNN60bTfT4), "Panics" multiband (nCVEu36UeEc), FL Transient Processor (1k4N5kiQINE, P6elrI3VaKs), and a crazy-gain shaper on a "big gun snare" (Rapv1lcXagA).
- A minority view (KkXF4T_WPbY): give the sub a slight attack because "a gunshot is a hair of mid-highs and then the low end comes in." In the transient band, the highs come before the lows.

### 2. Body (the tonal/grit core)

**Source choices**
- **Wavetables named on screen or in speech:**
  - Serum factory: Monster 1 / Monster 4 (mJdPGZXxHyU, ftwBNJP1tgs, WCN-ILwRTMI, -Z5YunR-upo), Creeper (Zcx64R68uoc), Wraith (vhciw1HgExE), 4088 + DirtySaw (w6q4AQiRR1Y), Trilobyte 3 (FeWnYudwCno), FM_Splat + FlangeSquare (KkXF4T_WPbY), Scout (fJZYQPGFXB4), Dinosaurs (sk--QbE_TM0), Electric Guitar [SL] (P55h-jIKinc), Analog_BD_Sin (YvEIKc2Eg9Q).
  - Cymatics freebies: Get It Noisy (etZIR8MP3XU), Saw Phasery Warp II / Rounded Saw Phase (LN2JgfVNmxA).
  - Vital: Glorkglunk "Clicky Robot" (ibdbe1nxAhs), "Drink the Juice" (IL5I2ME8wZo).
  - Massive: Strontium / Chrome / Modern Talking / Frozen.
  - Several creators say **the WT matters less than the processing** and swap tables for variants. oddprophet calls this the "butterfly effect" (YoXPfuez3nQ). Rocket Powered Sound: "change WT, whole sound changes."
- **Samples as oscillators** (17):
  - Snares in Serum 2's sample/spectral oscillators (P55h-jIKinc: Snare 30 + Snare 48; bDPyECsNHGk; YvEIKc2Eg9Q puts the snare in the noise osc).
  - A shotgun in the spectral osc (JLUvQ1aAoDM).
  - Bass loops dragged in as wavetables (Rapv1lcXagA).
  - Tonal "pops" in the spectral osc, one-shot, sem −3 (LN2JgfVNmxA).
  - Long granular files (3ZNN60bTfT4).
- **Octave:** carriers sit at **−2 to −4** (mode −3: etZIR8MP3XU, LN2JgfVNmxA, -Z5YunR-upo, YvEIKc2Eg9Q; −4 in vhciw1HgExE).
- **FM** (14), with these modulator intervals:
  - **+2 oct +7 st** (FeWnYudwCno)
  - **+2 oct** (vhciw1HgExE)
  - **same oct +7 st** (w6q4AQiRR1Y)
  - **saw +3 oct** (ftwBNJP1tgs / WCN-ILwRTMI)
  - **+1 oct** (w7H7P2VMnVs)
  - a **carrier one octave above the modulator, with the modulator at −7 st** (KkXF4T_WPbY)
  - FM amount ~30–50%, enveloped by the same gate.
- **Warps:** Bend +/- (ftwBNJP1tgs, WCN-ILwRTMI, LN2JgfVNmxA; Massive Bend in Synoid Dub, Cozy Robinson). Bend+ adds "thick low end" (LN2JgfVNmxA, -Z5YunR-upo). Sync with a rising ramp per hit (vhciw1HgExE on-screen). Diode/Tube warps (bDPyECsNHGk, P55h-jIKinc).
- **Unison** (8). The fat-but-percussive trick keeps voices phase-coherent:
  - 16 voices with **detune ~0** (fJZYQPGFXB4) or **very narrow spread** (P55h-jIKinc).
  - **Unison phase-random turned down**, "more percussive" (w7H7P2VMnVs).
  - **Unison only on the FM modulator**, so the carrier stays mono but the output is wide and noisy (ftwBNJP1tgs).
  - Massive "restart via gate" (-4EJhV7Skfs).

**Gate/amp shape: the main finding of the on-screen pass.** Three families appear:

| Family | Shape (x = fraction of one LFO cycle; y = level) | Who | Rate / mode |
|---|---|---|---|
| **A. Spike-then-ramp** | instant max → **straight drop to 20–45% by 5–15% of cycle** → long near-linear ramp to 0 at cycle end | oddprophet (etZIR8MP3XU 2024: drop to ~20% by ~6%; LN2JgfVNmxA 2025: ~40% by 15%, knee at 60%; YvEIKc2Eg9Q: ~30% by 15%), Code: Pandorum GUN 3 (P55h-jIKinc: ~40% by 10%) | **1/4**, ENVELOPE (one-shot) |
| A′. Hold-then-cut | concave drop to ~45% by 20%, plateau ~45–55% to 85%, steep cut to 0 | oddprophet "Basic and Useable" preset (etZIR8MP3XU_582) | 1/4 ENV |
| **B. Exponential decay** | smooth concave decay; ~35% at 30% of cycle, ~10% at 60%, ~0 at end. Variants add a ~3% linear attack (WCN-ILwRTMI) or ~5% hold (vhciw1HgExE) | Wheysted (KkXF4T_WPbY), Pandorum (sk--QbE_TM0, P55h-jIKinc_318), Letsynthesize (IL5I2ME8wZo Vital, WCN-ILwRTMI), PHENOMSOUND (vhciw1HgExE) | 1/8 TRIG, 1/8T RETRIG, 1/4 ENV |
| **C. Convex dome** | starts at max, falls slowly then faster (quarter-cosine), **hits 0 at ~50% of cycle**, silent after | PHENOMSOUND (w6q4AQiRR1Y, 1/4 at 150 BPM ≈ 200 ms audible), Rocket Powered Sound (mJdPGZXxHyU, 1/16 at 102 BPM ≈ 75 ms audible) | TRIG |
| D. ADSR | A 0.5 ms, H 0, **D 116 ms, S −∞, R 52 ms** | Rocket Powered Sound (Zcx64R68uoc) | MIDI-driven |

- **Rate/mode semantics matter for a DSP design.**
  - **ENV mode** (oddprophet, Pandorum, Letsynthesize) fires once per note, so the MIDI pattern sets the rhythm and the LFO rate sets the max hit length. A 1/4 note is ~400–430 ms at 140–150 BPM, though most energy is gone within 15%.
  - **TRIG/RETRIG mode** (Wheysted 1/8, PHENOMSOUND 1/8T and 1/4, RPS 1/16, Pandorum 1/8T) loops while a note is held, so **a single held note becomes an automatic burst** at the LFO rate. Both workflows are common.
- **Filter on the body:**
  - MG Low 12 is the default for oddprophet and Pandorum.
  - Notch types (BN12 / HN12 / High Notch, 9 videos) sweep with the gate. Wheysted: the high notch "adds the concussive side of the gun, really important."
  - Band-pass + LP + HP "shoved together" (etZIR8MP3XU), BP key-tracked at 137 Hz with res 18% (-Z5YunR-upo), MG Low 12 at cutoff 19 Hz + LFO 88 (JLUvQ1aAoDM).
- **Distortion in the synth:**
  - Tube, Soft Clip at 68–80, Overdrive (oddprophet's favorite; Pandorum's 5-stack OD + Asym at 75% mix), Diode, Hard Clip.
  - **The drive is modulated by the same gate** (Vy7hIbdZE4E: "cleaner"; P55h-jIKinc, _5t_90zAnHk, vhciw1HgExE).
  - Serum 2 distortion pre-filters were shown at **425 Hz, Q 1.9** (LN2JgfVNmxA, sk--QbE_TM0), 330 Hz / Q 1.9 (Zcx64R68uoc), and 193 Hz / Q 1.2 (mJdPGZXxHyU).
  - Rocket Powered Sound draws a custom X-Shaper fold curve.

### 3. Sub

- **Separate, clean, mono sub** (~17) vs **sub inside the gun patch** (4: etZIR8MP3XU "you don't even need a separate sub", ftwBNJP1tgs "sub osc adds crunch when distorted", w7H7P2VMnVs triangle direct-out, P55h-jIKinc). Swampy names the split directly: "oddprophet keeps the sub in the gun; I like a cleaner approach."
- **Sub rhythm.** Most gate the sub with the gun's rhythm (copy the MIDI or route it: nLxHdC0FBwU, ftwBNJP1tgs, 2Ox-rSTms9o chopped with short audio fades). Avant/Lux describe the **"sub wall"**: a long sustained sub following kick/snare sidechain, credited to Svdden Death and Marauda (twT4wEypWt8). Only one video says this.
- **Sub sound:** a sine, sometimes + 1 harmonic (_yvW7iH_ufk), + a little high-passed noise (nApsgDWSQ7Y, Rapv1lcXagA, Vy7hIbdZE4E, -Z5YunR-upo).
  - Pandorum's Massive sub stacks sin-squ at **0 / +12 / +19 st** (sk--QbE_TM0).
  - Common chain: **saturate hard then low-pass** (2Ox-rSTms9o: Saturator drive → EQ8 LP; ftwBNJP1tgs: Operator PRD filter drive +21 dB, digital clip, LP ~300 Hz).
  - Corpus "Kick Tight" tuned to the root (_yvW7iH_ufk).
- **Pitch envelope on the sub:** +12…+36 st, decaying in a few tens of ms (see Transient).
- **Level:** "the sub must be the loudest part of a tearout gun" (_yvW7iH_ufk). Dripment targets **~−9 LUFS sub, ~−4 LUFS gun print**, with the lows bus at −3 dB.
- **Mono below 100–120 Hz:** Utility Bass Mono at 120 Hz (IL5I2ME8wZo). Gun groups are low-cut at **100–160 Hz** (LM5LgEvjNHs 100 Hz, 2Ox-rSTms9o 160 Hz).

### 4. Spice / ear candy

- **Metallic resonance (comb filter / short delay / chorus) is the most-repeated spice.** Rocket Powered Sound: "comb filter is pretty much necessary for any machine-gun bass." Silvyr: "comb always works for tearout."
  - **Comb:** resonance high (~98; Zcx64R68uoc), cutoff low (knob 19–93; 425 Hz), damp up. Its cutoff is critical: "any slight change changes the sound dramatically" (vhciw1HgExE). It is often doubled (Zcx64R68uoc, _5t_90zAnHk).
  - **Short delays seen or stated:**
    - 10 / 15 ms ping-pong, feedback 60%, filter 3.2 kHz (FeWnYudwCno)
    - 10.89 / 13.11 ms ping-pong (vhciw1HgExE on-screen)
    - 13.28 / 16.67 ms ping-pong, filter 884 Hz (KkXF4T_WPbY on-screen)
    - ~15 ms (Zcx64R68uoc)
    - **25.95 ms L = R** (P55h-jIKinc on-screen, ≈ 38.5 Hz comb)
    - "tunnel" ~30 (KkXF4T_WPbY)
    - 30–36 ms slapback (w7H7P2VMnVs)
    - "very short delay with a lot of feedback turns an atonal bang into a note" (JYeBhtNFdB8, Virtual Riot / Dodge & Fuski)
    - DaniMusiX tunes the short delay "to the right pitch"
  - **Chorus as a static comb:** rate 0, feedback modulated by the gate (V5JLKz82WlE). Depth up and feedback low "super metallic at the start" (w7H7P2VMnVs). Rate 0.9 Hz, mix 50% (1033mUjb1oI). A Serum phaser with rate/depth/freq 0 for a "guitar-type" comb (Zcx64R68uoc).
- **Frequency shifter** (14):
  - **Automated across the drop for variation** (fJZYQPGFXB4).
  - Fixed: −100 Hz then −35 Hz on a tonal layer (fJZYQPGFXB4); 165 Hz in the mid band (etZIR8MP3XU on-screen).
  - **Serum 2 Bode, direction fully to one side, inside the high band of a splitter** so the sub stays in key (LN2JgfVNmxA).
  - A "no-sub shifter rack" for risers (KkXF4T_WPbY). "Atonal, evil" (P6elrI3VaKs). Ableton Shifter ring mod with high drive on the high band only (_yvW7iH_ufk).
- **Resonator / physical-model layers:** Corpus tracked from the synth's MIDI (K-qtdBWYsqM), Collision tuned +40 / +16 st (_yvW7iH_ufk), Collision as a layer (3ZNN60bTfT4). Metallic bell samples (2Ox-rSTms9o, _5t_90zAnHk).
- **Noise/foley layers in the upper mids and highs only:** glass breaking (1033mUjb1oI, YoXPfuez3nQ), white noise re-recorded through a subwoofer (YoXPfuez3nQ), noise riser routed into the bass bus (Rapv1lcXagA), hats in the same rhythm (qkN3c78U9hE).
  - oddprophet's rule (YoXPfuez3nQ): **one big bass + many small high-frequency details. Never layer in sub/low-mids.**
- **A "glassy/spectral" layer:** Kilohearts Filter Table with a spectral frame automated upward (nCVEu36UeEc). A Formant Filter on Lux's chain (twT4wEypWt8).

### 5. Reverb / convolver

- **The convolver is the signature tearout-gun processor** in the oddprophet lineage (etZIR8MP3XU, LN2JgfVNmxA, YoXPfuez3nQ; copied by QmI2qznHABk, Rapv1lcXagA). oddprophet: "the main component that makes it sound like a tearout gun is the convolver."
- **IR choices:**
  - Kilohearts "Spaces Real" shortened, with fade-out shaped and tone up (etZIR8MP3XU: "Garage Ramp", "Empty Apartment Bedroom").
  - **"Phaser 1"** from the Phase folder, stretch 40%, which "acts like a disperser, adds whip" (etZIR8MP3XU).
  - Serum 2 **"L90 Hall – Vocal Hall"** from the medium category: "too massive just sounds like a reverb" (LN2JgfVNmxA).
  - "Digital Chamber" (P55h-jIKinc). Serum 2 "Growl" / "Growlish" coloration IRs, mix 45–48% (bDPyECsNHGk, JLUvQ1aAoDM).
  - Hybrid Reverb "Car Garage", followed by a **~1.6 kHz boost** (ODdHap_SYM0). Hybrid Reverb "Small Ambience" (_yvW7iH_ufk).
  - **A tearout bass sample used as the IR** (LM5LgEvjNHs).
- **Settings:** **decay way down** ("otherwise it's in a cave"). IR **stretch/size** to tune tone. oddprophet starts the small transient-band convolver **halfway into the IR** because the IR's start is dry.
- **Placement:** high/mid bands only:
  - Multipass bands **<250 | 250 Hz–3 kHz | >3 kHz**, convolver on the mid band (etZIR8MP3XU, QmI2qznHABk, YvEIKc2Eg9Q).
  - The split moved to **424 Hz** (etZIR8MP3XU).
  - Serum 2 Splitter at **210 Hz**, later **546 Hz** (LN2JgfVNmxA).
  - The convolver adds stereo, so it is kept off the low band (etZIR8MP3XU), or the result is collapsed to mono and width re-added with Dimension Expander (Rapv1lcXagA). Convolver mix is automated with the bass (Rapv1lcXagA).
- **Plain reverb** is short when used: plate 5–20% (FeWnYudwCno, vhciw1HgExE), hall 5% / 1 s / 10% mix (w6q4AQiRR1Y), ≤1 s with 600 ms typical (2Ox-rSTms9o), 620 ms with lo-cut 1.24 kHz at 44% (IL5I2ME8wZo), reverb filter in Serum (fJZYQPGFXB4, w7H7P2VMnVs).
  - **Exception:** Dripment's Marauda recipe uses **four stacked convolution reverbs** (30%, 38%, kick-verb 22%) because "Marauda guns have so much reverb, lots of space" (LM5LgEvjNHs).

### 6. Group / master processing

- **The canonical "loud rack" order** (Dripment, LM5LgEvjNHs / 2Ox-rSTms9o): Disperser ×1–3 (moved up in freq) → OTT 81% → Saturator 5 dB → OTT 76% → Saturator 6 dB/100% → EQ8 (+167 Hz, +6.8 kHz) → Glue comp (thr −9 dB, makeup 5 dB) → Utility → **GClip/JST clip to 0 dB**.
- **Sub + guns clipped together** ("Mixology room") with GClip **+5 to +6.3 dB, softness 23%** (Dripment). BassTi glues four layers with **Roar multiband**: low band diode, amount 17%, bias −0.25; 180 Hz / 2 kHz crossovers; 78% wet; then Roar 22% bitcrush (_yvW7iH_ufk). Silvyr: group OTT → Disperser "is already 80% of the work" (-Z5YunR-upo).
- **Multipass** (10 videos; the oddprophet lineage) puts different distortion per band:
  - Phase Distortion ("almost FM feedback"), wave shaper, frequency shifter, and convolver in the mid band; Overdrive post (etZIR8MP3XU on-screen).
  - OTT inside Multipass: "compression is absolute magic, more powerful than any distortion" (QmI2qznHABk).
  - Kilohearts N.O.T.T. (YvEIKc2Eg9Q).
- **Serum-internal endings** (most Serum tutorials): Distortion → (Flanger/Phaser/Chorus) → **Delay (short)** → **Compressor multiband, gain 10–15 dB** (FeWnYudwCno 15, vhciw1HgExE 11.5 on-screen, w6q4AQiRR1Y 11) → Filter (Combs or High Notch) → small Reverb. Pandorum's GUN 3: Convolve → EQ (210 Hz shelf +7.4 dB) → Delay 25.95 ms → Reverb → Dist → Dist → Comp (very aggressive, gain 13) → Utility (mono lows), then **soft clip in the mixer**.
- **Final stage:** clipper → limiter (Pro-L, Ozone Maximizer, Maximus). "Most of my mixing is clipper then limiter" (-Z5YunR-upo).

### 7. Pattern, burst, retrigger, variation

- **Tempo:** 130–150 BPM, half-time drums. Reported: 130 (etZIR8MP3XU, 2Ox-rSTms9o), 138 (_5t_90zAnHk, "Marauda's favorite"), 140 (LN2JgfVNmxA, YoXPfuez3nQ, sk--QbE_TM0, vhciw1HgExE), 145 (_yvW7iH_ufk, twT4wEypWt8, w7H7P2VMnVs), 150 (KkXF4T_WPbY, w6q4AQiRR1Y).
- **Subdivisions:**
  - **Triplet grids dominate where rhythm is discussed** (8 transcript + 2 on-screen). Marauda's "Casket VIP" rhythm is "all triplets, 1/8T and 1/16T, fast but spaced out: 1-2-3 1-2-3 1 1-2-3" (LM5LgEvjNHs). Two-bar call/response phrases in a 16-bar drop, with the second 2 bars faster and triplets mixed with straight 8ths/16ths (2Ox-rSTms9o).
  - On-screen MIDI in _yvW7iH_ufk: **16th-triplet runs (1/24 notes) of 3–5 hits** on the root, with 1–7 st steps forming short staircases and single-step rests between groups.
  - IL5I2ME8wZo's clip: bursts of straight 16ths of ½, 1, and 1½ beats separated by ~½-beat gaps.
  - Gate rates: 1/8T (FeWnYudwCno, vhciw1HgExE, sk--QbE_TM0), 1/16 (mJdPGZXxHyU at 102 BPM, nLxHdC0FBwU), 1/8 (KkXF4T_WPbY, WCN-ILwRTMI).
- **Retrigger mechanism:** see the ENV vs TRIG note above. Both MIDI-note-per-hit and held-note looping LFOs are used.
- **Per-hit variation** (no one does strict repetition for long):
  - Notes stepping a few semitones (_yvW7iH_ufk, twT4wEypWt8 pulls single hits down).
  - **Master-tune / LFO2 sweeps** for "10 different gunshots from one patch" (KkXF4T_WPbY).
  - Frequency-shift automation (fJZYQPGFXB4, LN2JgfVNmxA). Alternating two copies with slightly different LFO shapes (w6q4AQiRR1Y). Macro-morphing the LFO shape (Rapv1lcXagA). Spectral scan rate (LN2JgfVNmxA). Grain position in a long file (3ZNN60bTfT4). Swapping WTs post-chain (YoXPfuez3nQ).
  - A LFO2 rising ramp → osc B sync, so each hit sweeps sync (vhciw1HgExE on-screen).
- **Hit length:** longer gate = stab → sustain (etZIR8MP3XU). Lengthening the LFO decay softens the cut (twT4wEypWt8). Hit length and audio-fade length shape the groove (2Ox-rSTms9o).
- **Tails:** choke them with Fruity Balance automation (Rapv1lcXagA), or resample and cut because reverb/compression tails bleed into the next hit (P55h-jIKinc; the bleed is sometimes kept on purpose).
- **Swing:** one source says perfectly quantized guns "sound abrupt" and hand-placed off-grid notes are essential (Lux via twT4wEypWt8). No other video corroborates this; the triplet grid is the mainstream answer.

---

## Consensus numeric ranges (for synth defaults)

| Parameter | Range seen | Center |
|---|---|---|
| Hit length (gate cycle) | 1/16 – 1/4 note (≈75–430 ms); audible energy mostly in first 15–50% | 1/4-note ENV with spike-ramp; or 1/8 TRIG exponential |
| Gate shape (spike-ramp family) | drop to 20–45% within 5–15% of cycle, then linear to 0 | ~35% at 10% |
| Pitch blip | exponential decay to 0 in ~25–140 ms; +12 to +36 st on the sub; Serum mod 23–40 on master tune (bipolar 48-st range) | ~+15–24 st, ~40–60 ms |
| Carrier octave | −2 to −4 | −3 |
| FM modulator | +1 to +3 oct, or +7 st | +2 oct |
| Comb / short delay | 10–36 ms (≈28–100 Hz fundamental), feedback 30–60%, ping-pong L/R offset 2–4 ms | 10–16 ms L/R with offset |
| Band split | low/mid 210–546 Hz; mid/high 3 kHz | 250 Hz / 3 kHz |
| Gun low-cut (with separate sub) | 100–160 Hz | 120 Hz |
| Multiband/OTT | Serum MB comp gain 10–15 dB; OTT 12–81% | OTT ~30–50% |
| Convolver | short decay, mix 30–48% on mids/highs | ~40% |
| Plain reverb | 0.6–1 s, mix 5–44% | ≤1 s, 10–20% |
| Group clip | +4 to +6.3 dB into GClip; final ceiling 0 dB | +5 dB |
| Tempo | 130–150 BPM | 140–145 |

---

## Notable disagreements

1. **Sub in the gun vs separate sub.** oddprophet: "you don't even need a separate sub." Letsynthesize: the in-patch sub is what crunches under distortion. Swampy, BassTi, Silvyr, RYZEN, DRIZKO, Dripment, and MOONBOY use a clean separate sub, often clipped together with the gun afterwards. Hybrid: keep a sub in the patch for distortion texture, high-pass the gun at 100–160 Hz, and add a clean sine sub (Wheysted: "I need it to bend").
2. **Sub follows the gun rhythm vs a "sub wall."** Most gate the sub to the gun. Avant/Lux claim Svdden Death and Marauda run a sustained sub ducked by kick/snare. That claim has a single source.
3. **Gate shape families** (on-screen): spike-then-ramp (oddprophet, Pandorum) vs smooth exponential (Wheysted, Letsynthesize, PHENOMSOUND vhci) vs convex dome (PHENOMSOUND w6q4, RPS). The dome sounds rounder and "subby." The spike-ramp gives a click plus a body tail.
4. **ENV (one-shot) vs TRIG (looping) LFO.** This is a workflow split, not a sound split. It decides whether rhythm lives in the MIDI or in the LFO rate.
5. **How much reverb.** Dripment: four convolution reverbs, "Marauda has lots of space." Most others: tiny (5–20%) or a short convolver only on the mids/highs.
6. **Disperser vs EQ-Three stacks.** XLNTSOUND prefers the "poor man's disperser" (EQ3s) "for control." Wheysted's "JK disperser" is EQ3s. Ghosthack modulates the EQ3 split points per note with a MIDI envelope and says the movement is what matters.
7. **Synthesis vs samples.** Wheysted usually Frankensteins gun samples. Dripment is pure sample collage. oddprophet and Pandorum load drum samples into synth oscillators. RPS, PHENOMSOUND, and Letsynthesize are pure synthesis. All of them converge on the same post-chain.
8. **Where the transient lives.** BassTi and Silvyr keep it out of group processing so it stays punchy. MOONBOY and Dripment process everything together and resample.
9. **Distortion drive.** Silvyr modulates drive with the gate ("full static drive sounds worse in the mix"). Others crank it statically. RPS: Tube "sounds like garbage," and he uses a custom X-Shaper.
10. **Pitch-blip size.** Silvyr: +15 st max, "higher sounds goofy." BassTi's Operator reaches ~+27 st effective. DRIZKO: +19 coarse.
11. **Layering philosophy.** oddprophet: one big bass + small high details. Dripment: many full-range layers clipped together. Silvyr warns that over-processing makes the bass "too big to layer anything else."

---

## Rare but interesting tricks

- **Reversed FM sweep:** FM index starts high and falls on trigger, rather than rising (KkXF4T_WPbY).
- **IR start offset:** a small transient-band convolver with the IR start set halfway, skipping the dry IR attack (etZIR8MP3XU).
- **A "Phaser 1" IR as a disperser** in Multipass Pre-FX (etZIR8MP3XU).
- **A tearout bass one-shot as the convolution IR** (LM5LgEvjNHs).
- **Chorus at 0 Hz as a tuned comb**, with feedback enveloped per hit (V5JLKz82WlE). A static Serum phaser for "guitar" coloration (Zcx64R68uoc).
- **Unison phase-random down** for percussive unison (w7H7P2VMnVs). **Unison only on the FM modulator** (ftwBNJP1tgs). **Massive "restart via gate" + detune 0.30/0.70 sweet spots** (-4EJhV7Skfs).
- **Bode frequency shifter:** direction fully one-sided, in the splitter's high band; Blur "softens the transient like another convolver" (LN2JgfVNmxA).
- **A disperser whose split frequencies follow a per-note MIDI envelope** (ODdHap_SYM0).
- **Collision/Corpus tuned to the note** as a ringing layer (_yvW7iH_ufk +40 st, K-qtdBWYsqM, 3ZNN60bTfT4).
- **Massive sub with sin-squ at 0/+12/+19 st** (sk--QbE_TM0).
- **Dubstepifier in parallel** for top-end texture (YoXPfuez3nQ).
- **Glass-break SFX layered**, sidechained (1033mUjb1oI, YoXPfuez3nQ).
- **17 ms pure delay (0 feedback) on the whole gun** to sit with the drums (_yvW7iH_ufk).
- **Clap volume automated to fake sidechain** against the kick (_5t_90zAnHk).
- **Ableton Roar** multiband diode with bias as the "glue" stage (_yvW7iH_ufk).
- **Real guns recorded with subsonic vs supersonic ammo:** subsonic for subtle transients, supersonic for snap (JYeBhtNFdB8).
- **A per-hit rising sync ramp** (LFO2 saw → osc B sync), so every hit chirps upward inside a falling amp gate (vhciw1HgExE).

---

## Implications for the DSP synth (inferred from the above)

- Core voice: **one "hit" envelope generator with drawable/parametric shapes covering families A–C**, routed by default to amp, WT/morph position, filter cutoff, FM index, and drive. A second, **faster pitch-blip envelope** (exponential, 25–140 ms, +12…+36 st) goes to master pitch.
- Retrigger modes: **one-shot per note** and **looping at a tempo-synced rate (1/16, 1/8T, 1/8, 1/4)** while held.
- Built-in **notch/BP filter swept high→low per hit**, a **tunable comb / 10–36 ms feedback delay** (ping-pong offset), and **allpass dispersion** with a per-note-modulatable center.
- **Band-split FX** (default 250 Hz / 3 kHz) with a short convolver, phase distortion, and a frequency shifter on the mid/high bands only. The low band stays clean, with a separate sine sub that can be clipped together with the body.
- **A sample/spectral source slot** (snare, gunshot, one-shot) alongside the wavetable, since 17 videos build guns from drum or gun samples.
- **A per-hit variation engine:** random/step offsets for pitch (±1–7 st), shifter Hz, WT position, and gate length; plus a triplet-aware burst sequencer (1/16T runs of 3–5 hits, 2-bar call/response).
