# Tearout / heavy-dubstep bass & gun processing: synthesis

Corpus: **44 videos** from **23 channels**, all of them a heavy-dubstep, tearout, riddim or deathstep producer doing sound-design processing on a bass or gun. Per-video notes are in `notes/<id>.md` and clean transcripts in `txt/<id>.txt`.

Some channels contributed several videos: oddprophet 7, Dripment 6, Wheysted 5, Bunting 4, Swampy 2, XLNTSOUND 2, AscianUK 2. The rest contributed one each. Counts below are given as **videos / distinct channels**, because 7 oddprophet videos are not 7 independent opinions.

Excluded as generic mixing, mastering or loudness content, or as off-topic. Their notes and transcripts are kept but not counted here:
- In `txt_excluded/`: i3V1sPUSQfE, n9fMJmsndiw, X91AwPIP3go, cokzhfEoYKI, 4RMIgKGF2Zg, jhU-VZUUVvw, VhLB5GoLBLM, 9X-zeOVPn3c, rrwb5qGrqvs, 8ZW74t0nw-c, 4tbGXJJuviw.
- Notes written but excluded: QSN7nUSH04I (mostly mixing), GIlXe59UCUQ (sponsored arrangement), Ya_Rn9mvU-0 (psytrance Multipass), u4spsStac9c (editing workflow), l6um2IpsMMk (arrangement), eNCrC0veeEs (pack promo).

Caveat on the numbers: auto-captions garble numbers, and most creators show settings on screen without saying them. Treat each number as "one producer said this once" unless a count is given.

---

## 1. Canonical chain order

This is the consensus spine, assembled from the ~20 videos that give a full ordered chain. Everything in brackets is optional.

```
SOURCE
  sine sub (pitch env)  +  top: perc one-shot / spectral-osc perc / FM wavetable  (+ noise)
  │
  ├─ [pre-distortion EQ: cut sub + low-mid mud, cut hiss; peak boost 500–820 Hz; optional LPF]
  │
  ├─ DISTORTION STAGE 1  (stacked overdrive / soft or hard clip / tube; often LFO on drive)
  │
  ├─ BAND SPLIT  ~200–315 Hz  → low band: clean sine or separately-distorted sub, mono
  │                           → high band continues ↓
  ├─ DISPERSION  (Disperser / EQ-Three stack / allpass / phaser-at-rate-0 / chirp IR)   ← "early"
  ├─ CONVOLVER   (short / trimmed IR, decay ↓, IR gain ↑, mix high or parallel 100% wet)
  ├─ DISTORTION STAGE 2  (shaper / multipass band distortion / saturator)
  ├─ OTT  (3-band up+down)  ⇄  SATURATOR/CLIP   repeated 2–3×  ("loud rack")
  ├─ CHARACTER: comb / resonator / frequency shifter / static phaser / short delay
  ├─ EQ cleanup (low-mid scoop, high boost, mids notch for "screamy" peak)
  ├─ width on highs only (Haas / chorus / Dimension), lows mono
  ├─ [short parallel reverb or convolution, low-cut, 100% wet in parallel]
  └─ CLIPPER at 0 dBFS (GClip / JST Clip / Saturator soft-clip, 2× oversampled)
  │
  PRINT / RESAMPLE → 2nd pass (disperser again, freq shift, warp, chop, tape-stop) → clip
```

### Why this order

The videos either state these reasons or they follow directly from the DSP.

- **EQ before the first distortion sets which harmonics and intermods are created.**
  - Op1nyQxm540: "before the distortion find your pain point… always the low mids."
  - IL5I2ME8wZo boosts ~500 Hz heavily before an Analog Clip saturator: "this is where the crunch happens."
  - HuO05jyAVUI states the rule outright: a low-mid notch *before* distortion gives pronounced tonal movement, and a high notch *after* gives a sweep.
  - FGXP_8URE5w cuts sub before distortion ("distorting too much sub → muddy").
  - 75IzG6B_Uz8 and sk--QbE_TM0 put a lowpass before more drive ("removing some of the high frequencies allows you to distort more").
- **Split the band after the first distortion, not before.**
  - keq0WT_pcuU says distortion must sit before the band filters, because "saturation creates frequencies which weren't there before."
  - The Serum 2 chains do the same: overdrive → splitter → convolver (LN2JgfVNmxA, LRQFCUXqXr4, Xgr39T5MXwg, xtaeEvROUFw, OwQDpCtbNTM).
- **The disperser goes early so later nonlinearities react to the smeared phase.**
  - WDGDzquQcFA: "earlier on in my processing chain so other things can affect it."
  - [inferred] In DSP terms, allpass dispersion changes the crest factor and transient shape, so the next clipper produces a different spectrum. Dispersion after the last clip only smears the result.
- **The convolver goes after distortion 1 and before distortion 2 or compression.**
  - The input is already harmonically dense, so a short IR adds many resonances.
  - The following clip and OTT then turn that colouration into crunch.
  - sk--QbE_TM0 is the outlier: convolver *before* any distortion "to give it more tone."
- **OTT goes after distortion, and is followed by more saturation or clipping.**
  - OTT's upward band makes low-level hash and noise loud ("bring those artifacts back up", joYYP0ckXT4).
  - OTT's output gain (+6 to +14 dB in these videos) then drives the next saturator. That is the "loud rack" loop.
- **A clipper is always last**, so the next stage (or the bus) never sees overs. 2× oversampling is used in 1c4O66nkid0 and vRH629oUamk.

---

## 2. Techniques ranked by independent mentions

Count format: videos / channels, with the IDs. A technique counts only if it is actually used in the bass or gun chain.

| # | Technique | Count | IDs |
|---|---|---|---|
| 1 | **OTT / 3-band upward+downward compression** (incl. Serum/Vital "multiband" comp used OTT-style) | **32 / 20** | -pqAEyQwkho 1c4O66nkid0 1S_8K8Wc1QA 3KtaTBKGIh4 75IzG6B_Uz8 7km8ayST3u8 azZr_2GzQtQ fSN0X15AzuY HuO05jyAVUI ibdbe1nxAhs iE0BRh23xJo IL5I2ME8wZo j0ljWHUNKeI joYYP0ckXT4 KkXF4T_WPbY KVeHUiQ5UNk LRQFCUXqXr4 n7PWr_YwP3k nApsgDWSQ7Y NRfK34KZIf8 OiDnhaPnNy8 opVPCKHsKMQ OwQDpCtbNTM QtpT7PlkWCs Rapv1lcXagA riHraXWFi44 vRH629oUamk Vy7hIbdZE4E w7H7P2VMnVs WDGDzquQcFA x90WuBTn-Rc xtaeEvROUFw |
| 2 | **Sub kept clean or separate** (direct-out, separate track, split band, or mono lows) | **22 / 16** | 1S_8K8Wc1QA FGXP_8URE5w fSN0X15AzuY j0ljWHUNKeI KkXF4T_WPbY KVeHUiQ5UNk LN2JgfVNmxA LRQFCUXqXr4 OwQDpCtbNTM QtpT7PlkWCs riHraXWFi44 sk--QbE_TM0 vRH629oUamk Vy7hIbdZE4E w7H7P2VMnVs x90WuBTn-Rc xtaeEvROUFw YvEIKc2Eg9Q 75IzG6B_Uz8 ODdHap_SYM0 keq0WT_pcuU Rapv1lcXagA |
| 3 | **Dispersion / allpass cascade** (Disperser, EQ-Three stack, Serum allpass, phaser-as-disperser, chirp IR, MFreeformPhase) | **21 / 15** | 75IzG6B_Uz8 8wrUMD4aeCM FGXP_8URE5w fSN0X15AzuY IL5I2ME8wZo j0ljWHUNKeI joYYP0ckXT4 KkXF4T_WPbY nApsgDWSQ7Y ODdHap_SYM0 OiDnhaPnNy8 OwQDpCtbNTM Rapv1lcXagA riHraXWFi44 vRH629oUamk Vy7hIbdZE4E w7H7P2VMnVs WDGDzquQcFA x90WuBTn-Rc Xgr39T5MXwg YvEIKc2Eg9Q |
| 4 | **Convolution with a short or trimmed IR on the bass** | **19 / 9** | -pqAEyQwkho 3KtaTBKGIh4 75IzG6B_Uz8 D9Hbz0Hze_0 FGXP_8URE5w iE0BRh23xJo KkXF4T_WPbY LN2JgfVNmxA LRQFCUXqXr4 ODdHap_SYM0 OwQDpCtbNTM Rapv1lcXagA sk--QbE_TM0 vRH629oUamk Vy7hIbdZE4E WDGDzquQcFA Xgr39T5MXwg xtaeEvROUFw YvEIKc2Eg9Q |
| 5 | **Two or more distortion stages in series** (often with EQ, comp or OTT between them) | **19 / 12** | 1c4O66nkid0 1S_8K8Wc1QA 3KtaTBKGIh4 75IzG6B_Uz8 keq0WT_pcuU LRQFCUXqXr4 xtaeEvROUFw Vy7hIbdZE4E HuO05jyAVUI SNWU8aghQ40 n7PWr_YwP3k FGXP_8URE5w Rapv1lcXagA IL5I2ME8wZo opVPCKHsKMQ -pqAEyQwkho NRfK34KZIf8 vRH629oUamk x90WuBTn-Rc |
| 6 | **Comb filter or tuned resonator** (Serum Comb, Corpus/Resonator, short-delay comb, Serum "Reverb" filter) | **19 / 14** | 7km8ayST3u8 azZr_2GzQtQ DBhKpnAXbz8 FGXP_8URE5w IL5I2ME8wZo NRfK34KZIf8 OiDnhaPnNy8 opVPCKHsKMQ OwQDpCtbNTM QtpT7PlkWCs riHraXWFi44 sk--QbE_TM0 Vy7hIbdZE4E w7H7P2VMnVs x90WuBTn-Rc xtaeEvROUFw KVeHUiQ5UNk 1c4O66nkid0 -pqAEyQwkho |
| 7 | **Band-split processing** (Multipass, Serum 2 Splitter, Patcher, manual 3-band) | **16 / 10** | 75IzG6B_Uz8 8wrUMD4aeCM Rapv1lcXagA riHraXWFi44 Vy7hIbdZE4E YvEIKc2Eg9Q keq0WT_pcuU SNWU8aghQ40 LN2JgfVNmxA LRQFCUXqXr4 Xgr39T5MXwg xtaeEvROUFw OwQDpCtbNTM j0ljWHUNKeI FGXP_8URE5w vRH629oUamk |
| 8 | **Fast downward pitch envelope on the gun or sub** | **15 / 11** | 3KtaTBKGIh4 DBhKpnAXbz8 IL5I2ME8wZo ibdbe1nxAhs KkXF4T_WPbY LN2JgfVNmxA LRQFCUXqXr4 ODdHap_SYM0 Rapv1lcXagA sk--QbE_TM0 Vy7hIbdZE4E xtaeEvROUFw YvEIKc2Eg9Q OwQDpCtbNTM nApsgDWSQ7Y |
| 9 | **Resampling / print-and-reprocess** | **14 / 11** | 8wrUMD4aeCM D9Hbz0Hze_0 KkXF4T_WPbY KVeHUiQ5UNk keq0WT_pcuU nApsgDWSQ7Y OiDnhaPnNy8 opVPCKHsKMQ QtpT7PlkWCs Rapv1lcXagA riHraXWFi44 joYYP0ckXT4 WDGDzquQcFA SNWU8aghQ40 |
| 10 | **Frequency shifter** | **13 / 11** | 75IzG6B_Uz8 8wrUMD4aeCM FGXP_8URE5w iE0BRh23xJo IL5I2ME8wZo joYYP0ckXT4 KVeHUiQ5UNk LN2JgfVNmxA OiDnhaPnNy8 Op1nyQxm540 Rapv1lcXagA riHraXWFi44 YvEIKc2Eg9Q |
| 11 | **Clipper as the final stage** (GClip, JST Clip, Saturator soft-clip) | **12 / 7** | -pqAEyQwkho 1c4O66nkid0 fSN0X15AzuY KkXF4T_WPbY NRfK34KZIf8 opVPCKHsKMQ Rapv1lcXagA vRH629oUamk Vy7hIbdZE4E WDGDzquQcFA x90WuBTn-Rc xtaeEvROUFw |
| 12 | **Percussion sample as the gun's top source** (snare, clap, ride or door slam in spectral/noise osc, audio or wavetable) | **12 / 9** | 3KtaTBKGIh4 LRQFCUXqXr4 LN2JgfVNmxA xtaeEvROUFw YvEIKc2Eg9Q ODdHap_SYM0 KVeHUiQ5UNk FGXP_8URE5w joYYP0ckXT4 nApsgDWSQ7Y QtpT7PlkWCs Rapv1lcXagA |
| 13 | **Repeated OTT ⇄ saturation loop** (a "loud rack" or "fat rack") | **12 / 9** | 1c4O66nkid0 NRfK34KZIf8 vRH629oUamk n7PWr_YwP3k -pqAEyQwkho Vy7hIbdZE4E nApsgDWSQ7Y KVeHUiQ5UNk 75IzG6B_Uz8 Rapv1lcXagA riHraXWFi44 j0ljWHUNKeI |
| 14 | **Multiple OTTs in series** (2–3) | **12 / 10** | -pqAEyQwkho 1c4O66nkid0 fSN0X15AzuY j0ljWHUNKeI n7PWr_YwP3k NRfK34KZIf8 riHraXWFi44 vRH629oUamk Vy7hIbdZE4E joYYP0ckXT4 KVeHUiQ5UNk nApsgDWSQ7Y |
| 15 | **Low-mid scoop / mud cut** (on the sub or after distortion) | ~**12 / 9** | Op1nyQxm540 fSN0X15AzuY HuO05jyAVUI KkXF4T_WPbY Vy7hIbdZE4E x90WuBTn-Rc SNWU8aghQ40 riHraXWFi44 3KtaTBKGIh4 FGXP_8URE5w Rapv1lcXagA vRH629oUamk |
| 16 | **EQ deliberately placed before distortion** (pre-shaping) | **9 / 8** | IL5I2ME8wZo Op1nyQxm540 fSN0X15AzuY HuO05jyAVUI FGXP_8URE5w 75IzG6B_Uz8 sk--QbE_TM0 xtaeEvROUFw j0ljWHUNKeI |
| 17 | **Phaser** (mostly static or frozen, low-mid, mono) | 9 / 7 | 7km8ayST3u8 fSN0X15AzuY x90WuBTn-Rc YvEIKc2Eg9Q opVPCKHsKMQ iE0BRh23xJo keq0WT_pcuU 75IzG6B_Uz8 NRfK34KZIf8 |
| 18 | **Short algorithmic reverb after the crunch, low-cut** | ~9 / 7 | KkXF4T_WPbY WDGDzquQcFA joYYP0ckXT4 n7PWr_YwP3k ibdbe1nxAhs w7H7P2VMnVs opVPCKHsKMQ HuO05jyAVUI Vy7hIbdZE4E |
| 19 | **EQ Three stack as a DIY disperser** | 8 / 7 | fSN0X15AzuY IL5I2ME8wZo KkXF4T_WPbY nApsgDWSQ7Y ODdHap_SYM0 w7H7P2VMnVs WDGDzquQcFA x90WuBTn-Rc |
| 20 | **Explicit mid boost** (500 Hz–1.6 kHz) | 7 / 7 | IL5I2ME8wZo xtaeEvROUFw ODdHap_SYM0 -pqAEyQwkho azZr_2GzQtQ opVPCKHsKMQ Rapv1lcXagA |
| 21 | **Noise layer fed into the crunch** | 7 / 6 | 1S_8K8Wc1QA DBhKpnAXbz8 nApsgDWSQ7Y ibdbe1nxAhs Vy7hIbdZE4E Rapv1lcXagA HuO05jyAVUI |
| 22 | **Transient shaping after the crunch** (transient shaper, gate, slow-attack comp) | 7 / 7 | IL5I2ME8wZo azZr_2GzQtQ vRH629oUamk FGXP_8URE5w Rapv1lcXagA KVeHUiQ5UNk OiDnhaPnNy8 |
| 23 | **Very short delay as a comb or metallic doubling** (≈20–36 ms, or "tunnel") | 6 / 6 | QtpT7PlkWCs KkXF4T_WPbY w7H7P2VMnVs sk--QbE_TM0 xtaeEvROUFw OiDnhaPnNy8 |
| 24 | **Erosion** (noise/sine-modulated micro-delay) for top-end hash | 5 / 4 | 1S_8K8Wc1QA KVeHUiQ5UNk n7PWr_YwP3k fSN0X15AzuY -pqAEyQwkho |
| 25 | **Several dispersers stacked or placed at different centres** | 5 / 5 | Rapv1lcXagA WDGDzquQcFA x90WuBTn-Rc FGXP_8URE5w joYYP0ckXT4 |
| 26 | **Tuned-to-key resonator** (Corpus / Resonator) | 5 / 4 | 1c4O66nkid0 -pqAEyQwkho FGXP_8URE5w KVeHUiQ5UNk riHraXWFi44 |
| 27 | **LFO or envelope on distortion drive** | 4 / 4 | Vy7hIbdZE4E HuO05jyAVUI opVPCKHsKMQ 75IzG6B_Uz8 |
| 28 | **Bitcrush / downsample** | 4 / 4 | 8wrUMD4aeCM FGXP_8URE5w ibdbe1nxAhs nApsgDWSQ7Y |
| 29 | **Serum 2 Overdrive stacked** (explicitly 2+ instances) | 2 / 2 | LRQFCUXqXr4 xtaeEvROUFw. 3KtaTBKGIh4 and LN2JgfVNmxA use Overdrive as the main distortion. |
| 30 | Formant/vowel via BP + inverted LFO on cutoff/frequency | 1 | NRfK34KZIf8 |

**Not found in this corpus:** Fruity Waveshaper by name (Vy7hIbdZE4E uses an unnamed "waveshaper"; xtaeEvROUFw mentions it as an alternative), Trash 2 (one passing mention, riHraXWFi44), and FabFilter Saturn (one, Vy7hIbdZE4E). Formant filters are nearly absent; the vowel character comes from notch or band-pass sweeps and combs.

---

## 3. Consensus numeric ranges

### OTT (Ableton Multiband Dynamics "OTT" preset / Xfer OTT)

| Parameter | Values seen | Consensus |
|---|---|---|
| Depth / Amount | 100% (1c4O66nkid0); 81% and 76%, "usually 20–50%" (NRfK34KZIf8); ~80% (joYYP0ckXT4); 64 / 33 / 28% across three stacked instances (vRH629oUamk, captions garbled); 50% (j0ljWHUNKeI, fSN0X15AzuY); 32% (opVPCKHsKMQ); "light", "not 100" (KkXF4T_WPbY, WDGDzquQcFA, KVeHUiQ5UNk) | **30–60% per instance when stacking. 80–100% on a single resample pass.** fSN0X15AzuY: "two half-way OTTs sound cleaner than one full." |
| Time | 1000% max (1c4O66nkid0, j0ljWHUNKeI, joYYP0ckXT4); "up" (fSN0X15AzuY); ~310% and ~479% (vRH629oUamk); "wacky" (KVeHUiQ5UNk) | **Turned up, often to max.** Longer attack and release give fewer pumping and "artifacty" sounds and keep the punch. Nobody lowers it. |
| Output gain | +6 (1c4O66nkid0), +10 (joYYP0ckXT4), +11 to +14 (vRH629oUamk), +12 (opVPCKHsKMQ, NRfK34KZIf8) | **+6 to +14 dB, and it drives the next saturator.** |
| Band trick | Low-band output −24 dB on the high layer (vRH629oUamk); low band off (iE0BRh23xJo) | OTT doubles as a band killer. |

**DSP translation.** These preset figures are from memory of the commonly-documented Ableton OTT preset. Verify them against the device before hard-coding.
- **Bands.** 3 bands with crossovers at about **88 Hz and 2.5 kHz**.
- **Downward compression.** Ratio about **66:1** (effectively a limiter) above a threshold of roughly **−30 to −36 dB** per band.
- **Upward compression.** Ratio about **1:4** below a threshold of roughly **−36 to −41 dB**.
- **Time constants.** Attack roughly **13 ms (high), 22 ms (mid), 48 ms (low)**. Release roughly **130–280 ms**.
- **How the controls map:**
  - *Time* is a multiplier on all attack and release times.
  - *Depth/Amount* is a global dry/wet: NRfK34KZIf8 says "OTT depth = dry/wet". In Ableton it scales the compression amount.
- **Net effect.** Every band is pinned to a narrow level window around −33 dB pre-makeup. Quiet hash, noise floor and tails come up by 10–20 dB, and peaks come down. That is why OTT "adds noise" (n7PWr_YwP3k), and why people clip right after it.

### Distortion / clipping

- **Drive values seen:**
  - Saturator: +1 dB (vRH629oUamk), +5 / +6 dB (1c4O66nkid0, NRfK34KZIf8), +13 dB (1S_8K8Wc1QA).
  - Serum Distortion: soft clip at drive 62, 100% wet (1S_8K8Wc1QA); Tube at max drive and **20% mix** for parallel crisp (7km8ayST3u8).
  - Overdrive: 30% wet, drive 13 (1S_8K8Wc1QA).
  - Amp, Dual mode: 37% wet (opVPCKHsKMQ).
  - Distortion bias 30% for asymmetric crunch (Vy7hIbdZE4E).
  - Saturn multiband: mid 50%, high 25%, mix 50% (Vy7hIbdZE4E).
- **Rule of thumb** (fSN0X15AzuY, and implied by others): push until it breaks, then back off slightly. A cleaner alternative is an LFO on drive instead of drive at max (Vy7hIbdZE4E).
- **Curves named:**
  - Soft clip / Analog Clip: 1c4O66nkid0, IL5I2ME8wZo, vRH629oUamk, 1S_8K8Wc1QA.
  - Hard / Digital clip: Op1nyQxm540, x90WuBTn-Rc, HuO05jyAVUI.
  - Soft Sine at 100% (sinusoidal foldback): riHraXWFi44.
  - Diode: opVPCKHsKMQ.
  - Tube: 7km8ayST3u8, DBhKpnAXbz8, nApsgDWSQ7Y.
  - Pedal Overdrive: ODdHap_SYM0, 1S_8K8Wc1QA.
  - Guitar amp: riHraXWFi44.
  - CamelCrusher at 50: j0ljWHUNKeI.
- **Final clipper:** ceiling **0 dBFS**, **2× oversampling** (1c4O66nkid0, vRH629oUamk), driven roughly 5 dB into clip (1c4O66nkid0).

### Convolver

- **IR type.** Short, medium or plate-like. A big hall "just sounds like reverb" (LN2JgfVNmxA). IRs named:
  - Plates and rooms: "Tight Plate" (xtaeEvROUFw), "rock plates" (LRQFCUXqXr4), "vocal hall" (medium, LN2JgfVNmxA), "Digital Rich Plate" (vRH629oUamk), "car garage" (ODdHap_SYM0), "small room" (KkXF4T_WPbY).
  - The middle section of the "Art Museum" IR (75IzG6B_Uz8).
  - "Growlish" at **28% mix** (iE0BRh23xJo).
  - One-shot percussion as the IR (clap or snare, using its front transient; WDGDzquQcFA, D9Hbz0Hze_0).
  - Random Kilohearts "Real" IRs, faded short (YvEIKc2Eg9Q).
  - Kilohearts "Disperser" IR with feedback (FGXP_8URE5w).
  - A self-made chirp IR (Xgr39T5MXwg).
- **Settings consensus:**
  - **Decay down / short:** 8 videos (LN2JgfVNmxA, LRQFCUXqXr4, OwQDpCtbNTM, D9Hbz0Hze_0, YvEIKc2Eg9Q, WDGDzquQcFA, ODdHap_SYM0, Xgr39T5MXwg).
  - **IR gain up:** 5 (LN2JgfVNmxA, LRQFCUXqXr4, OwQDpCtbNTM, Xgr39T5MXwg, YvEIKc2Eg9Q).
  - **Mix high:** 100% in series (Xgr39T5MXwg: "very important"), or a 100%-wet parallel chain under the dry (D9Hbz0Hze_0, WDGDzquQcFA).
  - **Size/stretch knob** used as a tone control (LN2JgfVNmxA, YvEIKc2Eg9Q, Xgr39T5MXwg).
- **Placement.** 7 videos put it on the **high band only**, above a 210–300 Hz split (LN2JgfVNmxA, LRQFCUXqXr4, xtaeEvROUFw, Xgr39T5MXwg, OwQDpCtbNTM, YvEIKc2Eg9Q, D9Hbz0Hze_0). "No reverb on the sub. That's what they teach you in dubstep school day one." (OwQDpCtbNTM)
- **Stereo handling.** Collapse to mono before the convolver and re-widen after (Rapv1lcXagA: "convolvers are chaotic"). Alternatively, widen *only* the convolved path (WDGDzquQcFA).
- **DSP.**
  - It is an FIR filter with a short, dense, effectively random impulse.
  - Its magnitude response is a jagged random comb (OwQDpCtbNTM: "creates combing and phasing").
  - Its phase response smears the transient.
  - *Stretch/size* resamples the IR in time, which scales every resonance down in frequency and every delay up.
  - *Gain up plus decay down* means a loud early part and a truncated tail: colouration, not space.

### Dispersion

- **Kilohearts Disperser:** amount at **max** (j0ljWHUNKeI); 2 instances, one low-centred and one high-centred (WDGDzquQcFA); up to 4 stacked (x90WuBTn-Rc); 1–3 on snare-to-gun (Rapv1lcXagA).
- **Serum 2 allpass ×3:** resonance ≈30, cutoff ≈19 in macro units (joYYP0ckXT4).
- **Serum allpass:** resonance 50% with a low cutoff, for a "delayed, metallic" sound (riHraXWFi44).
- **Ableton EQ Three stack:** "10+" copies with all cutoffs on one macro (nApsgDWSQ7Y, fSN0X15AzuY). Moving the split frequencies with a MIDI envelope is "the trick" (ODdHap_SYM0).
- **Phaser as disperser:** Multipass pre-FX (YvEIKc2Eg9Q); Serum phaser at rate 0 and ~20 Hz (7km8ayST3u8).
- **Placement:** early in 7 videos (FGXP_8URE5w, x90WuBTn-Rc, WDGDzquQcFA, 8wrUMD4aeCM, Rapv1lcXagA, YvEIKc2Eg9Q, j0ljWHUNKeI). Late or at the end in about 3 (fSN0X15AzuY, KkXF4T_WPbY, vRH629oUamk). x90WuBTn-Rc re-adds a disperser after the multiband comp "to restore moisture."
- **DSP:**
  - Disperser is a cascade of N second-order allpass sections at a centre frequency f₀, with Q set by "pinch." The result is group delay peaked at f₀: content near f₀ is delayed relative to the rest, so a click becomes a chirp ("laser", "whip", "bubbly", "moist").
  - An EQ Three is a 3-band crossover summed at unity. Each copy therefore acts as an allpass at its two crossover frequencies, and 10 copies make a 20-section allpass cascade.
  - A phaser at rate 0 is a static allpass cascade mixed with dry, which gives fixed notches.

### Band splits / mono

- **Splits used:**
  - High band split at **210 Hz**, moved to **300 Hz** if shifting thins the lows (LN2JgfVNmxA).
  - **~315 Hz** low/mid and **~2.5 kHz** mid/high (keq0WT_pcuU, SNWU8aghQ40).
  - High layer low-cut at **350 Hz** (vRH629oUamk).
  - Sub enhancer ≤**100 Hz** (FGXP_8URE5w).
- **Mono below 130 Hz** (xtaeEvROUFw). Low-end stereo is removed with M/S or Slice EQ in 75IzG6B_Uz8, YvEIKc2Eg9Q and LRQFCUXqXr4.
- **Consensus:** the crunch chain runs above roughly **200–350 Hz**, and below that the sub is mono.

### Gun envelope / source

- **Pitch envelope:**
  - **+15 semitones** falling (Vy7hIbdZE4E: "too high sounds goofy").
  - ~**8%** of the main-tuning range (xtaeEvROUFw).
  - LFO to master tune at **−7** (DBhKpnAXbz8).
  - Unipolar, not bipolar (YvEIKc2Eg9Q).
  - "This pitch fall is crucial for the sound of a machine gun" (sk--QbE_TM0).
- **Spectral-osc percussion:** −1 oct +7 st, scan ≈118 (xtaeEvROUFw); −1 oct (LRQFCUXqXr4); sub sine at −3 oct (YvEIKc2Eg9Q).
- **FM index envelope:** high at onset, decaying (KkXF4T_WPbY, IL5I2ME8wZo).
- **Tempo:** 140–150 BPM. Rhythm is triplet or 1/16 (sk--QbE_TM0, KkXF4T_WPbY).

### Mid boost / low-mid cut

- **Peak boosts:** ~500 Hz heavy, before the clipper (IL5I2ME8wZo); **822 Hz** narrow peak, "done a lot by tearout producers" (xtaeEvROUFw); ~**1.6 kHz** (ODdHap_SYM0); a parallel overdrive band of **600 Hz–4 kHz peaking at 1 kHz** (-pqAEyQwkho).
- **Moving notches** at ~750 Hz and ~1.14 kHz (keq0WT_pcuU).
- **Low-mid cut / mud:** around **200–300 Hz** (SNWU8aghQ40 at 200 Hz, riHraXWFi44 at ~300 Hz), "just above the sub" (Op1nyQxm540).
- **Consensus:** boost something in **500 Hz–1.6 kHz**, usually before or into a clipper, and cut **~150–350 Hz** mud.

### Frequency shifter

- **Small shift, 50% wet:** **3.7 Hz** at **50% wet** gives "phase interference," a barber-pole comb (iE0BRh23xJo).
- **Mid band:** **−25 Hz** (YvEIKc2Eg9Q).
- **Large shift on a resample:** down **2.2–2.5 kHz** (range 20 kHz) on a resampled perc, then OTT to raise the artifacts (joYYP0ckXT4).
- **High band only:** keep it off the sub, because it "shifts the sub out of key" (LN2JgfVNmxA).
- **Feedback mode:** feedback ~**70**, delay **20–30 ms** (OiDnhaPnNy8). This is a spiral comb with 33–50 Hz spacing.
- **Shift-filter-shift:** shift down → filter → shift up (FGXP_8URE5w).
- **DSP:** single-sideband modulation via a Hilbert transform, y = Re{(x + jH[x]) · e^{j2πΔft}}. Every partial moves by a fixed Δf, so harmonics become inharmonic. Mixed 50/50 with dry it gives beating at Δf on every partial. Bode "Direction" mixes the upper and lower sidebands.

### Loudness (sound-design context only)

- Bass and track at about **−4 to −3 LUFS**; "−2 is ear-piercing" (1c4O66nkid0).
- Peak at about **−5 dBFS** before the loud rack, and −8 → −6 → −4/−3 as stages are added (NRfK34KZIf8).
- Mastering and master-bus practice are out of scope.

---

## 4. Disagreements

1. **Sub: separate vs crunched together.**
   - The majority (22 videos) keep the sub clean or processed separately.
   - Op1nyQxm540, IL5I2ME8wZo, nApsgDWSQ7Y and DBhKpnAXbz8 deliberately **sum sub + top into one clipper**. Op1nyQxm540: "it takes low-end energy to get distorted to get this fat sound"; sub level into the clipper sets the grit, via intermodulation.
   - YvEIKc2Eg9Q: an in-synth sub is fine and a separate one isn't needed.
   - These are compatible if you **crunch sub+top together for character, then re-split and put a clean mono sine under it** (x90WuBTn-Rc does roughly this).
2. **OTT depth:** 100% (1c4O66nkid0) vs ~50% (j0ljWHUNKeI, fSN0X15AzuY) vs 32% (opVPCKHsKMQ) vs "very light" (KkXF4T_WPbY, WDGDzquQcFA). Everyone agrees on *Time up*.
3. **Disperser placement:** early in about 7 videos vs at the end in about 3. Also, Kilohearts Disperser vs a stacked-EQ "poor man's disperser": w7H7P2VMnVs prefers the EQ stack "for control."
4. **Convolver before or after the first distortion:** before in sk--QbE_TM0; after in the Serum 2 split-chain group.
5. **Low cut before or after distortion:** sk--QbE_TM0 cuts *after* to keep "low-end crackle." FGXP_8URE5w and Op1nyQxm540 cut sub or low-mid *before*.
6. **Notch position:** before distortion gives movement and fatness (fSN0X15AzuY, HuO05jyAVUI); after gives a sweep (HuO05jyAVUI); "saturate after notches to bring the meat back" (SNWU8aghQ40).
7. **Comb: static vs swept.** Parking it at a fixed spot works better than automating (NRfK34KZIf8), vs modulated resonance or cutoff (w7H7P2VMnVs, opVPCKHsKMQ).
8. **Hard vs soft final clip:** hard or digital (x90WuBTn-Rc, Op1nyQxm540) vs soft (Rapv1lcXagA: "soft because it's not the main bass") vs Saturator soft-clip (-pqAEyQwkho).

---

## 5. Rare but interesting tricks (1–2 mentions)

- **Chirp IR "whip"** (Xgr39T5MXwg). Make the IR from a Dirac → multiband delay with more delay on higher bands → linear phase → render. In the convolver at 100% mix this is a custom-curve disperser. The Size knob scales the group delay. For a synth, this is directly a parametric allpass/chirp FIR.
- **Ring mod of the signal by itself, high band only** (FGXP_8URE5w). Squaring gives an octave-up and even harmonics: a "polished, nasty high end."
- **Convolver with feedback**, using the Disperser IR (FGXP_8URE5w). This produces resonant comb peaks.
- **Downward frequency shift of 2–2.5 kHz followed by OTT** (joYYP0ckXT4). Content folds through 0 Hz into inharmonic metal, and OTT raises the residue.
- **Frequency shifter at 50% wet with a 3.7 Hz shift** (iE0BRh23xJo). A barber-pole phaser.
- **Frequency shifter with a 20–30 ms feedback delay** (OiDnhaPnNy8). A spiral comb.
- **Sine + sine 4 octaves up, into one digital clip:** "the classic dubstep synth" (Op1nyQxm540).
- **Odd-harmonic sub layer:** Square 64 → guitar-amp fuzz → cut ~300 Hz, under a clean sine (riHraXWFi44).
- **Glue compressor with a very long attack after heavy distortion** to rebuild the transient (IL5I2ME8wZo). **Transient-shaper release very short** to cut tails (azZr_2GzQtQ).
- **Static phaser** at rate 0, depth 0, ~20 Hz: "guitar amp" low-end fill (7km8ayST3u8). A frozen chorus with feedback gives fixed combs (x90WuBTn-Rc, w7H7P2VMnVs).
- **Unison random phase = 0** so voices start coherent, which sounds more percussive (w7H7P2VMnVs, xtaeEvROUFw).
- **Asymmetric distortion bias of 30%** on the sub for crunch, which adds even harmonics (Vy7hIbdZE4E).
- **Disperser split frequencies driven by a per-note envelope** (ODdHap_SYM0).
- **Step the master tune** (by 3, 5…) and print about 10 gun variants from one patch (KkXF4T_WPbY).
- **Tape-stop the whole drop and loop tiny snippets** as new hits (8wrUMD4aeCM, Rapv1lcXagA).
- **Texture-warp resample at −12 st with grain size tweaked** (opVPCKHsKMQ). A **999 BPM stretch** with a ±4-octave pitch ramp, then 3 OTTs (riHraXWFi44).
- **One-shot → 256-frame crossfade-morph wavetable**, normalized (QtpT7PlkWCs).
- **Serum "Reverb" filter as the metallic core** of riddim (azZr_2GzQtQ, w7H7P2VMnVs). Serum comb at 7149 Hz with drive 35% (opVPCKHsKMQ).
- **Route the gun's MIDI to the sub** so the sub follows the gun rhythm (1S_8K8Wc1QA).

---

## 6. DSP-tool takeaways for the synth

These are inferences from the counts above.

1. **Topology:** sine sub (mono, pitch env) ‖ top voice (perc sample or FM, pitch env, noise) → pre-EQ (mud cut, 500–820 Hz peak) → clip₁ → crossover at ~250 Hz. High band then goes through:
   - an allpass cascade (N, f₀, Q)
   - short-IR convolution (IR choice, stretch, gain, decay, mix)
   - clip₂ or waveshaper
   - [3-band up/down comp → soft clip] × 2–3
   - comb / freq-shift insert
   - EQ
   - width

   Then sum with the sub → oversampled clipper at 0 dBFS.
2. **Expose controls for:**
   - drive and its modulation
   - allpass stage count, centre and envelope
   - IR stretch
   - OTT depth, time and makeup
   - crossover frequency
   - comb frequency and feedback
   - frequency-shift Δf and wet
3. **"Heaviness" levers the corpus agrees on:**
   - multiple nonlinear stages with gain-raising compression between them
   - short-IR convolution on the highs
   - dispersion before a clipper
   - a mono, clean, pitch-enveloped sub underneath
