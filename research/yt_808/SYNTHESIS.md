# 808 synthesis — consensus from 31 YouTube tutorials

Corpus: 31 transcripts (`txt/`), 31 notes (`notes/<id>.md`), low-res frame sheets for 6 of them (`frames/`). Twenty-eight are used for counts below (`IHD1Yth_RFA` has no 808 content; `f2mquMgg8uY` and `AcTrt7LfW_w` only count for level/clipping). That leaves **28 videos from 27 creators**; 23 of them build an 808 from an oscillator. "(OS)" means read off screen, everything else is what the creator said. A count like "n=9" means nine videos state or show that thing.

Out of scope: `Z07NWhI2oQQ`, `H6i55lXAMh0`, `S1PkHMA0cYo` (covered earlier). Transcripts that hit YouTube's 429 rate limit and are missing: kaFS85uzozY (Bunting/Metro), Gl_zx1Tbv5c (Dillon XO), PvfEB5WIxYY (Gunnr), WbYTMamy144 (Kermode dubstep), DrctDPxOHZc (Ben Kestok additive), 84yQuwyghN4, 5WaqAm_kaXg, Bu2C7RO-O0I.

---

## 1. Consensus architecture

```
          ┌────────────── pitch env (fast exp, → root) ───────────────┐
          │                                                           ▼
 [BODY]   sine (± 2nd/3rd partial)  ──► amp env ──┬──────────────────────────► LOW band (≤100–500 Hz) clean, mono
          phase reset, mono/legato                │
                                                  └─► drive (env-modulated) ─► HP/BP ─► HIGH band (harmonics)
 [HIT]    pitch env depth  /  FM pop (ratio 1–2, few-ms index env)  /  fixed-freq sine ~100 Hz
 [CLICK]  noise burst (HP, few ms)  or  kick-sample click with no low end
                                                                              ▼
                                         sum ─► EQ ─► (OTT, light) ─► comp/glue ─► soft clip / limiter (last)
```

| Block | Consensus | Count | IDs |
|---|---|---|---|
| Body oscillator = **sine** | sine, or sine plus a few low partials | 16 pure sine, 8 sine+partials; 3 use a filtered square/saw | sine: sYpX, Y8di, Kqpj, mV1C, pu-JB, c9VQ, gdY4, LaG6, 5_-4, oTTY, jsUF, 2Edt, QkFL, X12V, Dqpz, rRTm · +partials: tHdA, ruMw, FWSE, Dqpz, rRTm, X12V, Kqpj(OS BD_Sin), pu-JB · square/saw: rgSL, _5o4, iW8o |
| Harmonics come from **2nd/3rd partials** placed *before* distortion | additive 2nd+3rd, trapezoid osc, BD_Sin table, FM ratio 1–2 | 8 | Dqpz ("my favourite is the 3rd harmonic"), rRTm, X12V, ruMw (square-ified BD sine = odd), tHdA, FWSE, QkFL (FM), oTTY (keep only 2nd/3rd) |
| **Mono / one voice** | always | 12 | sYpX, rgSL, mV1C, pu-JB, c9VQ, LaG6, rRTm, oTTY, _5o4, pIYu, QkFL, Kqpj |
| **Phase reset** (random phase 0, fixed start phase) | yes, for identical transients | 5 explicit, 1 shows the opposite (free-run → every hit differs → print & pick) | jsUF, X12V, ruMw, zGfi, pu-JB(osc random off) · gdY4 |
| **Unison off** on the body | yes, implicitly everywhere; unison only on an upper/hybrid layer | 3 use unison on a non-sub layer | _5o4 (4 v, det 30), rgSL (osc B +12), zGfi (5 v, det 10) |
| **Pitch envelope** | the "kick in the 808" | 21 | almost all synthesis videos |
| **Glide/legato** | portamento with retrigger off | 6 | sYpX (115 ms), c9VQ (always glide), pIYu, rRTm, QkFL, LaG6 |
| **Amp env** | instant attack (0–0.5 ms OS) or de-click attack; long decay or gate+short release | 20 | see §2 |
| **Transient layer** separate from body | pitch env alone (≈10), or a dedicated hit/click layer | 10 dedicated | sYpX, Y8di, mV1C, pu-JB, FWSE, oTTY, LaG6, AcTr, QkFL, gdY4 |
| **Noise** | low-level texture or a short HP'd burst | 10 | rgSL, iW8o, Y8di, tHdA, c9VQ, gdY4, jsUF, X12V, rRTm, QkFL |
| **FM** | transient pop or bloom-in harmonics, never the main body | 4 | gdY4, QkFL, sYpX, zGfi |
| **Distortion** on the 808 | yes | 24 | nearly all |
| **Distortion drive under envelope/LFO control** | yes | 7 | Kqpj (more at onset), ruMw (less on the transient; dist + comp mix automated *down* over the note), X12V (ramp up then back), jsUF, zGfi, rgSL (env → filter drive), Y8di (env → filter drive) |
| **Split-band** (clean sub, distort only the top) | yes | 10 | rgSL, Y8di, oTTY, 1zzg, jsUF, rRTm, B7iw, FWSE, ruMw, iW8o(M/S) |
| **Mono low end** | yes | 8 (2 dissent) | iW8o, Y8di, FWSE, jsUF, rRTm, ruMw, LaG6, Dqpz(leads) · dissent: AcTr (widened one), rHwa (chorus on lows sometimes OK) |
| **OTT / multiband upward** | light; amount down, time up | 6 | mV1C, c9VQ, Dqpz, zGfi, ruMw, Y8di(Saturn MB) |
| **Final soft clip / limiter** on the 808 or the master | yes, the last stage | 11 | rgSL, iW8o, f2mq, AcTr, gdY4, FWSE, oTTY, ruMw, Y8di, LaG6, c9VQ |
| **Tuner check after processing** | yes | 5 | rgSL, iW8o, pu-JB, pIYu, Y8di |

---

## 2. Consensus numbers

### Pitch envelope (the most important parameter)
- **Start offset: +12 to +24 st.** Stated values: +12 (sYpX at 60% amount ≈ +7 effective; jsUF, then attenuated; 5_-4), +24 (5_-4, rRTm, oTTY then reduced, pu-JB range), "usually 12–24" (rRTm), "+12 or +24 keeps it in tune" (5_-4). The lowest stated: +4 st (sYpX "also works"). FM8 amount 88 (gdY4, units not stated). Serum percentages (not converted, depth range not shown): coarse 11% / 19% (Kqpj), 29% (_5o4), master-tune 68 (rgSL OS).
- **Shape: fast exponential/bowed decay, unipolar, ending exactly on the root** (gdY4 "I really like exponential curves", tHdA "stop the curve early so it rings at its root", ruMw/oTTY unipolar). Every frame sheet showing a pitch LFO/env shows a near-vertical drop and then a long concave tail (rgSL, Kqpj, ruMw, oTTY, gdY4; OS).
- **Time:** almost always given as "short/fast" rather than ms. Stated: 500 ms decay parameter in Operator (sYpX, exponential, so most of the drop happens in the first tens of ms). Regimes: fast = "knock"/modern; slow = "dive bomb" (LaG6), long = old-school jungle "drop bass" (5_-4). Longer envelope when the 808 *is* the kick, shorter when a kick is layered (rgSL).
- **Not stated anywhere:** the start Hz. That needs measuring from audio.

### Amp envelope
- **Attack 0–0.5 ms** (OS 0.5 ms in rgSL, oTTY, ruMw; "attack 0" in pIYu, UZio, _5o4 0.3 ms), raised slightly *only until the click disappears* (iW8o, tHdA, jsUF, 2Edt, pu-JB, mV1C). A deliberately late body is used when a separate transient leads: **20 ms delayed attack** (sYpX), **80 ms attack** (Y8di), Zaytoven-style delayed body (Y8di).
- **Shape A — decay-only:** S = 0, decay sets length: ~**1 s** (rgSL/oTTY/ruMw OS D 1.00 s), **2 s** (pu-JB), **3.4 s** (sYpX), D 500 ms + S −1.5 dB + R 1.8 s (_5o4).
- **Shape B — gate:** hold/sustain full, length = MIDI note, short release: R **15 ms** (ruMw OS), **115 ms** (rgSL OS), 0 (pIYu, UZio), 1.3–1.4 (LaG6). Drill and FL-sampler workflows use B (pIYu, UZio). A long release is "the special sauce" in mV1C, and R ~2–2.9 s in oTTY (OS).
- **End-of-note click** (stopping mid-cycle) gets named twice: fix with release ≥ a few ms (gdY4, mV1C).

### FM (4 videos)
- **Modulator ratio 1:1 then 2:1 to the carrier** (gdY4 OS: ratio 1.0000 → 2.0000); QkFL: carrier coarse 0.5, modulator coarse 1.0 (2× carrier), second modulator also on the carrier. "Higher ratio = higher pop" (gdY4).
- **Index envelope:** either a **few-ms spike** for a transient "pop" (gdY4 OS), or a **delayed/slow attack on the modulator** so harmonics come in *after* the hit (QkFL), plus one envelope on all modulator levels for movement (QkFL).
- **Index kept low enough that the fundamental stays dominant**, and a separate plain-sine sub carrier is added back because FM thins the fundamental (QkFL).
- **Fixed-frequency hit operator:** sine at **100 Hz** fixed, very short decay (QkFL); a fixed triangle used as the "tap" layer or as an FM modulator on the sine (sYpX).
- Click via **self-feedback at max = noise**, very short env (QkFL).

### Noise / click
- **Level:** "just barely audible" pink (Y8di), "just a little" (c9VQ, tHdA). It's there for distortion to grab onto more than to be heard directly.
- **Burst:** HP-filtered noise, very short (gdY4, QkFL); removed for the long-808 version in gdY4.
- **Kick-click layer:** a sample with its low end and tail removed and the fade-in cut (oTTY); LP'd, pitched down, shortened (FWSE); kick crossfaded into the body (mV1C).

### Distortion chain
- **Types in use:** soft clip (n=9), tube/warm (4), asymmetric (ruMw: "even+odd, less top, richer bottom"), diode (Kqpj: Diode 2, drive 79%, mix 75%), amp/cab sims (Dqpz Amp→Amp, rgSL, rHwa, X12V), iZotope Trash 2 (3: rgSL, LaG6 "Deep Wax / Amp Drainer, drive 0.5, mix 50%", rRTm), bitcrush/downsample as a pseudo-noise HF layer (rgSL, iW8o, jsUF Erosion, rRTm Erosion *before* Trash).
- **Order (serial):** harmonics in the source → saturate/drive → **LP or BP after distortion** (c9VQ LP with env after soft clip; Kqpj MG12 LP @ ~300 Hz after diode; oTTY BP around the 2nd/3rd harmonic) → comp → clip/limit.
- **Parallel:** wet/dry 50–75% is common (Kqpj 75%, LaG6 50%, Dqpz partial, 2Edt mix knob).
- **Drive level:** saturator **≈22 dB drive with soft clip** feeding only the high band (oTTY); Vital soft clip drive −12 dB at mix 0.6 (zGfi).
- **Rule:** **the harmonic must not outlast the fundamental, and it sits below the fundamental** (oTTY). The sub stays the loudest peak on the analyser (rRTm).

### Split band (how the sub stays clean while phones still hear it)
- **Crossovers:** **~100 Hz** (rRTm HP on the distorted layer), **250 Hz** (Y8di mono below), **300–400 Hz** (iW8o side cut), **500 Hz** (1zzg), heard band **350 Hz–2 kHz** (B7iw).
- **Variant that matters for an engine:** distort the *full* signal (including the fundamental) but listen to only its HP/BP output, then sum it with the clean sine (oTTY). Harmonics then track the fundamental's phase and envelope.
- **Cost model:** making a sub audible on phones by raising the sub costs ~7 dB of headroom; raising the 350–2k band costs ~1 dB for more perceived loudness (B7iw).
- **Crossover comb filtering** when the two bands overlap or leave a gap: linear-phase or complementary filters (1zzg, Y8di, rgSL).

### OTT / compression
- **OTT:** amount reduced, time increased (Dqpz); Vital multiband attack 75%, release 25% (zGfi); used to *bring out* harmonics, followed by EQ (ruMw).
- **Comp:** −13.8 dB threshold at 4:1 (Kqpj OS); 1176 at 4:1 with ~3 dB GR and medium-fast attack (gdY4); **punch = low threshold + slow attack ~227 ms** (rHwa); upward comp +10–12 dB to raise the body (rHwa).

### EQ moves with frequencies
- API: **+6 dB @ 10 kHz, +2 dB @ 30 Hz, −2 dB @ 200 Hz**; Massive Passive **+8 dB @ 3.9 kHz, −3 dB @ 390 Hz**; Pro-MB **sub −3 dB, ~200 Hz −3..−4 dB** (gdY4).
- **LP at 10 kHz** (or 7.8 k) to stay out of the hats (Y8di).
- **Dynamic dip at ~60 Hz keyed by the kick** (pIYu). Kick fundamental at 80–120 Hz vs 808 energy at 40–60 Hz to separate them (rHwa). A kick at ~40 Hz checked against the 808 (sYpX).
- **No low-end boosts on the 808 itself**, because some notes would peak (LaG6).

### Tuning, key, level, length
- **Keys:** C (iW8o, pIYu, labels kits in C); **E / F / F#** "lowest notes before it gets weird" (mV1C, ruMw, oTTY: F1 ≈ 43 Hz); **G** for club (gdY4). Fundamentals around **30–50 Hz**; below ~35 Hz it's inaudible without overtones (sYpX).
- **Level:** the 808 is the loudest element, driven *into* a soft clipper on the master (f2mq, AcTr: limiter 0.8 dB, 74%, 1.5:1 OS), normalized to 0 dBFS when printed (pu-JB, ruMw, iW8o).
- **Length:** 1–3.4 s for one-shots; render 2–4 bars (rRTm, ruMw); drill uses long 808s gated by note length (pIYu).

---

## 3. Archetypes and the DSP that defines each

| Archetype | Body | Pitch env | Distortion | Other defining features | Sources |
|---|---|---|---|---|---|
| **Clean trap** (Metro/Zay/Spinz) | pure sine, mono | +12 st, short; or none, with the kick layered | light saturation, often split-band only | long decay (1–3 s) or long release; **Zay = delayed body after a kick transient**; Spinz = transient spliced onto a clean tail | sYpX, pu-JB, Y8di, oTTY, 5_-4 |
| **Distorted / rage / "Travis" / underground** | sine + **3rd harmonic** drawn in | quick "kicky" bend | **stacked**: soft clip → amp → amp/Trash → glue soft clip → OTT; drive enveloped; 808 slammed into a master clipper | mono; whole-mix clipping (snares/hats clipped too); OsamaSon = sample + master clipper/limiter; a brass hit layered as the "kick" | Dqpz, LaG6, AcTr, FWSE, rRTm, IHD1 (context), f2mq |
| **Drill (UK/NY) sliding** | long, somewhat distorted sample/sine, mid energy for phones | short | moderate | **mono legato glide, slide time = slide-note length**; slides of +12 st (sometimes +24) at bar ends; root note on the downbeat; HF slides at lower velocity; key-specific sidechain at ~60 Hz; sustain-gate envelope (A 0, R 0) | pIYu, c9VQ, sYpX (115 ms glide), 1zzg (Pop Smoke ref) |
| **Phonk / cowbell-adjacent** | clean-ish 808 | short | 808 moderate; **the cowbell gets crushed**: bitcrush 1 (drive 5 dB, DS 25, mix 25%) → bitcrush 2 (drive 14, DS max, mix 100%) | kick-sidechained cowbell; delay 1/8-dotted, 60% feedback. *Weak evidence: the phonk-808-specific videos are among the 429 misses* | zGfi |
| **Dubstep / riddim / DnB "drop bass"** | sine or BD-sine, oct −2 | **longer, audible downward sweep** (jungle) or short (modern) | diode / zero-square with enveloped drive, LP 300 Hz after, OTT redlined, chorus | keys E/F/F#; used as the first hit of a drop or a transition | Kqpj, 5_-4, mV1C, tHdA, jsUF |
| **Hybrid-trap reese-808** | **saw/square-saw + unison 4–5, detune 10–30** on an upper layer over a sine sub | sharp, on osc A only | tube / soft clip, multiband, Erosion→Trash on the top layer, HP ~100 Hz | negative filter env (−37), LFO → wavetable/bend for motion; **sub layer separate and mono, highs widened** | _5o4, rRTm, Dqpz, zGfi, X12V (pitch LFO −5 st on the harmonic osc only) |
| **Kick-808 hybrid** | sine (+ FM partials) | **deeper/longer** (+24 st), exponential | light; limiter glues | FM pop (ratio 1–2, few-ms env) or fixed 100 Hz hit op; noise click; a kick click with no lows, **no sidechain** (sidechain leaves a hole: click → silence → boom); polarity check vs the kick | gdY4, QkFL, oTTY, UZio, rgSL |

---

## 4. Common mistakes (stated by creators)

1. **Polyphony overlap** makes a mud generator, so set mono/cut-itself (sYpX, mV1C, pIYu + 9 more).
2. **Onset click** from attack 0 + nonzero start phase or a Serum compressor. Fix with a ~0.5–few-ms attack (iW8o, tHdA, jsUF). **End click** from cutting mid-cycle: give release a few ms (gdY4, mV1C).
3. **Random oscillator phase** makes every transient different (ruMw, jsUF, X12V; gdY4 prints and picks).
4. **Pitch env not landing on the root** (bipolar mod, or the curve still moving at the end), so the 808 is out of key (gdY4, tHdA, ruMw). Re-tune after processing as well (rgSL, iW8o, pIYu).
5. **Distorting the sub itself** gives an "overpowering", fuzzy 808 that costs headroom (Y8di, 1zzg, B7iw). Split instead, and keep the sub as the tallest peak (rRTm).
6. **Harmonics louder than or longer than the fundamental** (oTTY).
7. **Crossover comb filtering** in split chains (1zzg, rgSL).
8. **Stereo low end**: Trash and chorus add width, so mono below 100–300 Hz (LaG6, Y8di, iW8o). This one is disputed (AcTr, rHwa).
9. **Sidechain hole** between the click and the body (oTTY). Use a frequency-specific duck instead (pIYu) or delay the body ~20 ms (sYpX).
10. **Lo-fi plugins** that drift pitch or thin the lows (rgSL).
11. **Too much resonance** eats the lows, so put an envelope on the resonance as well as the cutoff, or bypass the sub around the filter (rgSL).

---

## 5. User's reference 808 vs the consensus

Reference: 96 Hz → 38 Hz within ~60 ms, then 38 → 34.6 Hz over ~500 ms; near-flat ~100 ms, −11 dB at 600 ms, −21 dB at 900 ms, gated by ~1 s; H3 −14 dB, H2 −25 dB, crest 5 dB.

- **Pitch target.** 34.6 Hz is **C#1 (34.65 Hz)**, so the "slow drift" is the tail of the pitch envelope still settling onto the root. It's a single envelope approaching C#1, which matches the consensus "exponential, ending exactly on root" shape (gdY4, tHdA, ruMw), with a long tail.
- **Depth.** The start sits at **+17.6 st above the root** (96 Hz vs C#1); 96→38 is a 16.0 st drop. That's inside the stated +12…+24 range and close to its middle. It isn't an octave-multiple start, so it falls outside the "+12/+24 keeps it in tune" rule (5_-4).
- **Curve.** The stated 808s show two time scales. A single exponential in semitones can't produce both: the fast part fits τ ≈ 25 ms (17.6 → 1.6 st in 60 ms), and the last 1.6 st needs τ ≈ 150–200 ms. That is the bowed/power-curve shape seen in every pitch-envelope frame (Serum LFO-envelope curves, FM8 "exponential"), or two stacked exponentials. The engine needs either a curvature parameter or a fast+slow two-stage pitch env, where the slow stage carries ~1–2 st over ~500 ms.
- **Time.** 60 ms for the main drop is the "knock"/modern regime, the opposite of "dive bomb" or jungle drop bass.
- **Amp.** Flat for ~100 ms, then an accelerating decay (−11 dB at 600 ms, −10 more dB in the next 300 ms), then a gate at 1 s. That matches **Shape B (sustain-gate + short release)** layered on a slow decay: the drill/FL-sampler workflow (pIYu, UZio), with a ~1 s envelope like rgSL/oTTY/ruMw (D 1.00 s OS). Most trap one-shots run longer (2–3.4 s).
- **Harmonics.** H3 ≫ H2 (−14 vs −25 dB) means **mostly symmetric saturation with slight asymmetry**. That matches the consensus favourites, soft clip (n=9) plus "3rd harmonic is my favourite" (Dqpz). By computation (scratch, pure sine in): tanh drive k ≈ 2–2.5 gives H3 ≈ −14 to −15.5 dB; a DC bias of ~0.15–0.2 before tanh gives H2 ≈ −23 to −25 dB. Hard clip reaches −14 dB H3 at ~1.8× gain. This is heavier than the clean-trap archetype and lighter than square-wave rage (H3 → −9.5 dB). It also obeys the consensus rule "harmonic below the fundamental".
- **Crest 5 dB.** A steady-state tanh(k=2.3) sine has a crest of ~1.4 dB, so the 5 dB whole-sample crest comes mostly from the envelope (the transient and the decay), which fits a saturated body with a punchier head. The consensus places that head in the pitch drop or in a click layer. It also points to final-stage clipping/limiting (n=11).
- **What the reference shows little of:** a broadband click/noise layer (n=10 use one), an FM pop, and split-band processing (n=10). If H2/H3 come from full-band saturation, the fundamental is saturated too, which the split-band camp warns against. To check, look for fundamental compression (crest at the fundamental vs at H3).

### Engine defaults implied
- Sine body, phase reset at 0, mono legato with glide (slide time = overlap length), optional 2nd/3rd partials at −20…−30 dB before the shaper.
- Pitch env: start +12…+24 st (default ~+17), a curved two-rate decay to the exact root (fast τ 15–30 ms, slow tail of 1–2 st over 300–600 ms), unipolar.
- Amp: 0.5 ms attack, hold/sustain gate with a decay of 0.6–3 s, release 15–115 ms.
- Hit/click: noise burst HP > 2–5 kHz, 3–10 ms, −20 dB; optional FM pop with ratio 1–2 and a 2–10 ms index env; optional fixed ~100 Hz sine at 20–40 ms.
- Shaper: tanh/soft clip with drive k 1.5–4, a small bias for H2, and drive under envelope control (lower on the first ms or higher, selectable). Run it on the full signal, then HP at 100–500 Hz and sum with the clean sine, or run it full-band for the rage archetype.
- Post: LP 10 kHz, light OTT (amount ~30–50%), soft clip/limiter last, mono below ~150 Hz, re-check pitch.
