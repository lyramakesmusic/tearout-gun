# Drum synthesis: written sources, extracted numbers

Compiled 2026-10-07. Every item carries its URL. Tags:
- **[VERIFIED]** read in the primary source text (or a figure/table image of it).
- **[FIG]** read off a plotted figure; approximate.
- **[SECONDARY]** stated by a secondary source; primary not seen.
- **[COMPUTED]** derived here from the governing equation (scipy), as a check on quoted ratios.
- **[UNVERIFIED]** asked about but not found in any source read.

---

## 1. Sound On Sound "Synth Secrets" (Gordon Reid)

Index: https://www.soundonsound.com/series/synth-secrets-sound-sound

### 1.1 Physics of percussion (Part 2, Jun 1999)
https://www.soundonsound.com/techniques/physics-percussion
- [VERIFIED] Circular membrane 0,3 mode ("w03"): 3.6 × fundamental, nodal circles at 27.8 % and 63.8 % of the radius.

### 1.2 Synthesizing Percussion (Nov 2001) — ideal membrane + timpani
https://www.soundonsound.com/techniques/synthesizing-percussion
- [VERIFIED] Table 1, ideal circular membrane, ratio to 0,1:
  0,1 1.00 | 1,1 1.59 | 2,1 2.14 | 0,2 2.30 | 3,1 2.65 | 1,2 2.92 | 4,1 3.16 | 2,2 3.50 | 0,3 3.60 | 5,1 3.65 | 3,2 4.06 | 6,1 4.15
- [COMPUTED] Bessel zeros j_mn / j_01 agree: 1, 1.593, 2.136, 2.295, 2.653, 2.917, 3.155, 3.500, 3.598, 3.647, 4.059, 4.132, (1,3) 4.230, (4,2) 4.601, (2,3) 4.832, (0,4) 4.903.
- [VERIFIED] Table 2, membrane in open air, radial modes relative to 1,1 principal: 1,1 1.00 | 2,1 1.47 (+10 %) | 3,1 1.91 (+15 %) | 4,1 2.36 (+19 %). Ideal values relative to principal: 1.00, 1.34, 1.66, 1.98.
- [VERIFIED] Table 3, kettle drum: 1,1 1.00 | 2,1 1.50 (+11 %) | 3,1 1.98 (+19 %) | 4,1 2.44 (+23 %).
- [VERIFIED] Fundamental (0,1) of timpani ≈ 63 % of principal; principal is 1,1 mode; strong overtones at 3:2 and 2:1. Strike point ≈ 1/4 of the way edge→centre suppresses circular (n,2+, 0,n) modes. Centre strike gives "dull, toneless thump" (circular modes, short-lived).
- [VERIFIED] Air loading shifts radial modes up and toward harmonic; air modes coupling to n,1 modes are always higher in frequency and drag membrane modes up.
- [FIG] Fig 10 decay times: radial modes 1,1 / 2,1 / 3,1 / 4,1 / 5,1 long; 0,1 / 0,2 / 0,3 / 2,2 very short; 1,2 medium. Circular modes give an initial "noise-like" burst, radial modes ring.
- [VERIFIED] Timpani pedal range ≈ half an octave. Harder hits → higher initial pitch (edge stiffness: membrane "appears smaller at higher amplitudes"), much less so in kettle drums.

### 1.3 Practical Percussion Synthesis: Timpani (Dec 2001)
https://www.soundonsound.com/techniques/practical-percussion-synthesis-timpani
- [VERIFIED] Principal of a large kettle drum ≈ 150 Hz. Four modes 1.00 / 1.50 / 1.98 / 2.44 → 150, 225, 297, 366 Hz (sine waves).
- [VERIFIED] Relative decay/release times of these four: ≈ 45 %, 73 %, 91 %, 84 %. Amplitude ratios ≈ 5:4:3:1.
- [VERIFIED] Strike burst: ring modulation of 100 Hz × 87 Hz signals (first 5 harmonics each → 25 partials; ≈46,000 partials over 20 kHz in full), then a high-pass to remove the ~dozen components below 150 Hz, with a short decay.
- [VERIFIED] DX7 algorithm 31 with ratios 1.00 : 1.50 : 1.99 : 2.44. MS-20 version: filter a triangle so partials sit at 1 : 1.5 : 2 : 2.5 (fundamental removed).

### 1.4 Synthesizing Drums: The Bass Drum (Jan 2002)
https://www.soundonsound.com/techniques/synthesizing-drums-bass-drum
- [VERIFIED, table image] Table 1, dual-membrane bass drum, equal tension: 0,1 = 50 & 118 Hz | 1,1 = 86 & 93 Hz | 2,1 = 136 | 3,1 = 182 | 4,1 = 225 | 5,1 = 273 Hz.
- [VERIFIED] Table 2 (dropping 118 and 86): 50, 93, 136, 182, 225, 273 Hz; spacing +43, +43, +46, +43, +48; ratios 1.00, 1.86, 2.72, 3.64, 4.50, 5.46.
- [VERIFIED] Table 3 (minus 7 Hz): 43, 86, 129, 175, 218, 266 → ratios 1, 2, 3, 4.07, 5.07, 6.19. Model: 43 Hz harmonic source + Bode frequency-shifter +7 Hz → 50, 93, 136, 179 Hz.
- [VERIFIED] Table 4, carry head detuned (looser): 0,1 44 | 1,1 91 | 2,1 138 | 3,1 184 | 4,1 232 | 5,1 282 Hz; ratios to 46 Hz: 0.96, 1.98, 3.00, 4.00, 5.04, 6.13 (near-harmonic, no offset; doubled modes vanish). Holed kick heads behave like Table 4.
- [VERIFIED] "The drum generates scores of partials between 250Hz and 1kHz" → modelled with FM pair + band-pass.
- [VERIFIED] Conclusion quote: "The frequency components of a kick drum approximate a harmonic spectrum at low frequencies, with a large number of densely packed enharmonic components at mid and high frequencies."
- [VERIFIED] Decay: texts suggest constant decay rate across important partials → one VCA/EG.
- [VERIFIED] Pitch: tension rises ∝ (displacement)²; 30-inch membrane, 1-inch centre displacement adds only ~1/16 inch to surface length. "the pitch of a typical kick drum can shift by a couple of semitones from start to finish"; "whereas the VCA Gain will change by 100 percent… the pitch should only shift by around 10 percent" → one AR envelope to both, attenuated into pitch CV.
- [VERIFIED] Beater click: "hundreds of short-lived high-frequency partials that exist for just a few milliseconds"; modelled as a short noise burst or a fast AR on an LPF over the FM cluster.
- [VERIFIED] Frequency-shifter box: shifting 100 Hz harmonic series (100/200/300/400) by +25 Hz → 125/225/325/425 = 1 : 1.8 : 2.6 : 3.4 (inharmonic); shifting by +100 Hz → 200/300/400/500 = missing-fundamental 100 Hz.

### 1.5 Practical Bass Drum Synthesis (Feb 2002) — incl. TR-909 / TR-808 kick
https://www.soundonsound.com/techniques/practical-bass-drum-synthesis
- [VERIFIED] Simplification: FM cluster replaced by filtered noise. Monosynth kick = self-oscillating filter (resonance max, cutoff min) swept by ADSR: Axxe env amount ~50 %, A=0, D=R≈50 %, S=0; SH-101 VCF Env ~60 %, D/R ≈ "6". Add VCF keyboard tracking 100 % for the SynDrum "ray gun".
- [VERIFIED] Click: instant VCA attack creates a discontinuity = a short HF burst ("the desirable consequence of truly snappy contour generators and VCAs").
- [VERIFIED] Reid observes mid/high partials are barely audible in these patches and still read as kick (engineers emphasise lows; filter/VCA distortion adds HF).
- [VERIFIED] TR-909 kick (Reid's re-layout): sawtooth VCO whose pitch is set by EG3 (instant attack, slow decay) → waveshaper turning saw into near-sine → VCA (EG1, accent modifies attack amplitude and decay rate). Parallel path: noise → LPF, summed with a short pulse ("click") from a pulse generator → VCA (EG2). Mixer.
  - Note: Robin Whittle's circuit notes (§2.4) say the 909 BD core is a triangle rounded by back-to-back diodes; Reid's text says sawtooth+waveshaper. Treat the core waveform as "triangle/saw → diode-rounded near-sine".
- [VERIFIED] TR-808 kick: bridged-T network kicked by trigger (no VCO/EG); trigger+accent pulse also added into the audio path as the beater click; LPF tone control; VCA. Positive feedback sets decay; too much → endless oscillation. "the TR808 kick drum oscillator goes slightly flat at long decays."

### 1.6 Synthesizing Drums: The Snare Drum (Mar 2002)
https://www.soundonsound.com/techniques/synthesizing-drums-snare-drum
- [VERIFIED] Snare-less snare drum: 9 frequencies from first 7 modes; 0,1 mode is doubled at **≈180 Hz and ≈330 Hz** (this is Reid's real-drum measurement, used for the 909 analogy; it is not a TR-808 oscillator spec).
- [FIG] Fig 1 lines ≈ 180, 280, 330, 342, 400, 440, 510, 550, 618 Hz (0–700 Hz axis).
- [VERIFIED] Remaining 7 split into two quasi-harmonic series: series A spacings 125, 109, 107 Hz; series B spacings 101, 114 Hz.
- [VERIFIED, Fig 4 image] Synth model: triangle oscillator **111 Hz** → two frequency-shifters **+175 Hz** and **+224 Hz**, plus sine oscillators at **180 Hz** and **330 Hz**, mixed into one VCA/AR. (Implied partials: 286, 397, 508, 619… and 335, 446, 557… Hz.)
- [VERIFIED] The 0,1 pair decays "far more quickly than the other partials... sometimes at more than twice the rate" → give the two sines their own faster VCA/AR.
- [VERIFIED] Snare wires act as "some form of band-limited noise generator"; harder hits → more energy, more HF, wider modal peaks, eventually a broad noise spectrum. Model: noise → velocity-controlled LPF → VCA/EG, plus voltage-controlled notch filters for spectral holes, plus velocity-controlled crossfade between modal and noise paths.
- [VERIFIED] No FM/ring-mod carrier:modulator pair found that reproduces the modal distribution.

### 1.7 Practical Snare Drum Synthesis (Apr 2002) — TR-909 / TR-808 snare
https://www.soundonsound.com/techniques/practical-snare-drum-synthesis
- [VERIFIED] TR-909 snare: two oscillators + two waveshapers for the 0,1 pair (shifted series dropped), each with **its own VCA and contour generator** (different amplitudes and decays); a pitch CV with its own contour generator (Reid calls it "anomalous" for a real snare). Noise → LPF → split: (a) HPF → VCA4/CG5 (narrow band), (b) VCA3/CG4 unfiltered; mix → VCA → output. Gives different decay in high and low noise regions.
- [VERIFIED] TR-808 snare: trigger fires a contour generator ("snappy"); attenuated snappy CV is added to the trigger and kicks **two bridged-T oscillators** (pitch + decay fixed by components, fixed mix ratio). Same contour drives a VCA on white noise → **high-pass filter** → mixed with oscillators → output amp.
- [VERIFIED] SH-101 noise snare: noise 10, VCF Freq 10, Env 10, Kybd 10, Res 0; A=0, S=0, D=R="2"; Gate+Trig. "FM snare": LFO at max rate (just reaches audio) modulating 25 % pulse, mixed ≈35 % with noise, played on top G.
- [VERIFIED] FM sideband example: carrier 200 Hz, modulator 30 Hz → 170/200/230 Hz.

### 1.8 Analysing Metallic Percussion (May 2002) — plates/cymbals
https://www.soundonsound.com/techniques/analysing-metallic-percussion
- [VERIFIED, fig images] Flat circular plate, centre-mounted, scaled to 50 Hz lowest mode: [2,0] 50 | [4,0] 145 | [2,1] 210 | [4,1] 445 | [2,2] 515 | [4,2] 865 Hz → ratios 1, 2.90, 4.20, 8.90, 10.3, 17.3.
- [VERIFIED, fig images] Real cymbal (Rossing holograms, rescaled by Reid to 50 Hz): 50 | 165 | 445 | 630 | 780 | 895 Hz → ratios 1, 3.3, 8.9, 12.6, 15.6, 17.9.
- [VERIFIED] Time course: first few ms strong modal peaks at a few hundred Hz; over next few hundred ms energy moves up to a few kHz; HF modes die first so the tail returns to mid frequencies. Hard hits → mode splitting, subharmonics, then chaos (essentially noise).
- [FIG] Fig 3 (log time/log freq): band starts ≈40 Hz–6 kHz at t≈0, dips, rises to ≈1 kHz–20 kHz+ around 0.03–0.1 s, falls to ≈50 Hz–1 kHz by ~5–10 s.
- [VERIFIED] Bending waves propagate "at speeds of up to 4000 miles per hour" (≈1.8 km/s).

### 1.9 Synthesizing Realistic Cymbals (Jun 2002) — Nord Micro Modular ride
https://www.soundonsound.com/techniques/synthesizing-realistic-cymbals
- [VERIFIED] FM source: pulse modulator ≈1 kHz into square carrier ≈2.5 kHz, both pitch-mod and linear FM at max; 7th sideband ≈18.5 kHz.
- [VERIFIED] "Ping" path: 24 dB/oct band-pass, base ≈1 kHz, fast AD sweeping the passband down to 1 kHz over **≈200 ms**; same envelope on amplitude.
- [VERIFIED] "Tail" path: 12 dB/oct high-pass at **2.64 kHz**, AHD opens to max over **≈200 ms**, closes over **3.7 s**. Tail louder than ping. Master AD: instant attack, decay **≈0.75 s**.
- [VERIFIED] Boyk (Caltech): crash cymbal spectrum "shows no sign of running out of energy at 100kHz"; up to 40 % of energy above 20 kHz.

### 1.10 Practical Cymbal Synthesis (Jul 2002) — TR-909 / TR-808 cymbal
https://www.soundonsound.com/techniques/practical-cymbal-synthesis
- [VERIFIED] TR-909 cymbal is a 6-bit ROM sample clocked by a **≈30 kHz** tunable clock; address counter → second DAC → anti-log converter → VCA envelope (decay tracks playback position); LPF to remove quantisation/aliasing.
- [VERIFIED, Roland block diagram image] TR-808 cymbal: **six square oscillators** "tuned enharmonically" → **two band-pass filters**; lower BPF → VCA with user Decay; upper BPF → split into two VCAs with own envelopes (shortest decay, and one mid-range); all three → **high-pass filters** → Tone control (mix of bands) → level.
- [VERIFIED] Korg (Rhythm 55) used multiple **pairs of modulated square oscillators** → "an order of magnitude more complex" metallic tone; Reid's improved patch: three FM pairs of pulse oscillators, detuned, mixed.
- Frequencies of the six 808 oscillators are not given here → see §2.2.

### 1.11 Synthesising Bells (Aug 2002)
https://www.soundonsound.com/techniques/synthesizing-bells
- [VERIFIED] Three phases: enharmonic strike (dies fast) → strike note of a few strong low near-harmonics → long hum an octave below. Strike-note pitch = virtual pitch: partials at 2:3:4 imply "1" (100/150/200 Hz → hear 50 Hz). Near-degenerate mode doublets beat ("warble").
- [VERIFIED, Nord OscSineBank image] Reid's six-sine bell ratios: **2 : 3.0102 : 4.1658 : 5.4317 : 6.7974 : 8.2159** (× base). These match Rossing/Fletcher "actual bell" ratios (§3.4) to 3 decimals.
- [VERIFIED] Clapper: 2-op sine FM, shorter envelope. Hum: two sines ≈ an octave apart, slightly detuned to beat, slow attack, long tail. LFO on one amplitude path for warble.
- [VERIFIED, Roland diagram image] TR-808 hi-hats: same six oscillators → one band-pass → separate OH and CH chains: envelope → VCA → **hi-pass filter** → buffer; OH has a decay timer + "envelope shut-off" that the CH trigger cuts (choke).

### 1.12 Synthesizing Cowbells & Claves (Sep 2002)
https://www.soundonsound.com/techniques/synthesizing-cowbells-claves
- [VERIFIED] TR-808 cowbell: two of the six pulse oscillators → two VCAs (one contour) → band-pass → amp.
- [VERIFIED] Reid's measurement of CR-8000 cowbell: **≈587 Hz and ≈845 Hz**, ratio 1 : 1.44; "even small deviations from these pitches destroy the cowbell illusion". (808 cowbell itself is 540 / 800 Hz, §2.2.)
- [VERIFIED] Roland doc quote on cowbell envelope: "a series of R82 and C34 connected in parallel with C9 forms an envelope having abrupt level decay at the initial trailing edge to emphasise attack effect" → two-stage decay (short loud impact + longer tail).
- [VERIFIED] Nord recreation: triangle waves (pulse too bright), BPF **2.64 kHz**, 12 dB/oct + a little resonance; pinkish noise halo during impact at low level.
- [VERIFIED] One-oscillator trick: replace the 845 Hz oscillator with a filter self-oscillating at 845 Hz, VCA placed before the filter.
- [VERIFIED] TR-808 claves: one bridged-T network kicked by a trigger, straight to output.

### 1.13 Springs, Plates & Buckets (Feb 2001)
https://www.soundonsound.com/techniques/springs-plates-buckets-physical-modelling
- [VERIFIED] Delay-line resonance: spring transit 12.5 ms → round trip 25 ms → reinforces 40 Hz, cancels 60 Hz (comb response). Cavity mode at 500 Hz ↔ 0.34 m with c = 340 m/s; round-trip delay = 2 ms (article prints "two microseconds", a typo).

---

## 2. TR-808 / TR-909 circuit analyses

### 2.1 Werner, Abel, Smith — TR-808 Bass Drum (DAFx-14)
https://dafx14.fau.de/papers/dafx14_kurt_james_werner_a_physically_informed,_ci.pdf
- [VERIFIED] Blocks: trigger logic → **1 ms** pulse → pulse shaper → **bridged-T network (a band-pass filter)** whose ringing is the drum; feedback buffer (high-shelf, set by Decay pot VR6) feeds back into the bridged-T; output = passive LPF (tone, VR5) → divider (level, VR4) → HPF.
- [VERIFIED] Bridged-T centre ≈ **49.5 Hz** (Roland "typical and variable" tuning chart: **56 Hz**). Simple bridged-T: fc = 1 / (2π √(R_eff · R167 · C41 · C42)).
- [VERIFIED] Attack: envelope generator (Q41/Q42) stays high until ≈**5 ms** after trigger falls; while high, Q43 grounds part of R_eff (R165+R166 → R166), raising Q and centre frequency by **more than an octave** for ≈**6 ms** ("less than a single period at the higher frequency"): heard as punch/crispness rather than pitch. A retriggering pulse (C39, R161, D52) then re-excites the bridged-T to prevent an amplitude hole when fc drops back.
- [VERIFIED] "Pitch sigh": leakage through R161; when V_comm drops below ≈ one diode drop below ground, Q43 draws current i_C, lowering R_eff → raising fc. Fit: i_C = (log(1 + e^{α(V_comm − V0)}))^m / α with α = 14.3150, V0 = 0.5560.
- [FIG] Fig 11 instantaneous frequency over first 300 ms: starts ≈ **56–57 Hz**, sighs down toward ≈ **50 Hz** with per-cycle ripple, mostly settled by ~0.15–0.2 s.
- [VERIFIED] Components: ±20 % capacitors, ±5 % resistors → unit-to-unit variation in gain, fc, Q, decay. Overlapping retriggers interact with residual filter state (no "machine gun effect").
- [VERIFIED] Footnote: 808 snare, toms/congas, rimshot/clave all use bridged-T networks the same way (decaying pseudo-sinusoids); handclap, cowbell, cymbal and hats use bridged-T networks as band-pass filters.

### 2.2 Werner, Abel, Smith — The TR-808 Cymbal (ICMC/SMC 2014)
https://speech.di.uoa.gr/ICMC-SMC-2014/images/VOL_2/1453.pdf  (also https://zenodo.org/record/850891)
- [VERIFIED] Six Schmitt-trigger inverter oscillators on one **HD14584** hex Schmitt trigger chip, shared by **Cowbell, Cymbal, Open HH, Closed HH**.
- [VERIFIED] Nominal frequencies: **205.3, 369.6, 304.4, 522.7 Hz** (oscillators #1–4, fixed); **#5 range 359.4–1149.9 Hz, factory-trimmed to 800 Hz**; **#6 range 254.3–627.2 Hz, factory-trimmed to 540 Hz** (#5/#6 = cowbell pair). Serial < 000300 used different #5/#6 parts.
  → Sorted: 205.3, 304.4, 369.6, 522.7, 540, 800 Hz. The commonly quoted list is correct.
- [VERIFIED] Duty cycle **D = 47.98 %**, amplitude 5 V. Period T = RC·ln((V_OH−V_T−)/(V_OH−V_T+)) + RC·ln((V_OL−V_T+)/(V_OL−V_T−)).
- [VERIFIED] Two active band-pass filters (bridged-T-in-op-amp-feedback, 3rd-order denominator): **BPF1 ≈ 3440 Hz, BPF2 ≈ 7100 Hz**; "strongly accentuate the upper overtones of the square waves, while deemphasizing their fundamental frequencies".
- [VERIFIED] Three swing-type VCAs → three Sallen-Key HPFs: HPF1 2nd-order (emitter follower), HPF2 2nd-order non-unity gain, HPF3 3rd-order with resonance near **10.5 kHz**. Tone control mainly attenuates band 3 (fifth-order tone-stage transfer functions).
- [VERIFIED] Attack smoother: one-pole, τ = **0.102 ms** (1.0244·10⁻⁴ s), V_BE = 0.7258 V. Envelope release: dV_B/dt = −V_B/(VR2k·R93·C41) (decay knob sets RC).
- [FIG] Fig 7 (log time): EG#3 (short band) peaks ≈1–3 ms, gone by ≈50 ms; EG#2 peaks ≈30–50 ms; EG#1.2 (decay-controlled band) peaks ≈0.2–0.3 s and decays over ~1 s at the plotted setting.
- [VERIFIED] Simulation at 4× oversampling (176.4 kHz) to tame aliasing of the square waves.

### 2.3 TR-808 instrument frequencies (Roland service-notes figures)
https://scrapbox.io/0b5vr/TR-808 (cites Roland TR-808 service manual)
- [SECONDARY] BD 56 Hz; **SD two oscillators ≈ 238 Hz and 476 Hz** + HP-filtered noise; LT/MT/HT 90/135/185 Hz; LC/MC/HC 185/250/400 Hz; RS 455 & 1667 Hz; CL 2500 Hz; CB 540 & 800 Hz; CY/HH 200–800 Hz range.
- Cross-checks: BD 56 Hz and CB 540/800 Hz match Werner et al. (primary). SD 238/476 not seen in a primary document.
- [VERIFIED] Tiptop SD808 manual (https://www.polynominal.com/sample-packs/TIPTOP-SD808/Tiptop_Audio_SD808_ns.pdf): two "pure sine wave T-Network oscillators", one low and one high; TONE crossfades them; SNAPPY = noise VCA envelope amplitude, also sets accent response.
- **Correction to the brief**: "808 snare ≈ 180 Hz and ≈ 330 Hz" conflates two sources. 180/330 Hz are Reid's measured 0,1-mode pair of an acoustic snare. The 808 SD oscillators are ≈238/476 Hz per the service notes, an exact octave pair [SECONDARY].

### 2.4 TR-909 (Robin Whittle, "TR-909 sound mods")
https://www.firstpr.com.au/rwi/tr-909/TR-909-Sound-Mods.pdf
- [VERIFIED] BD: "Tune" pot sets the **decay time of the pitch envelope** (start high, sweep to normal), not base pitch. BD core = **triangle rounded toward sine by back-to-back diodes** (turn-on ≈0.5–0.6 V). Pitch-sweep depth set by R28 (150 k); doubling to 300 k ≈ 2× normal sweep. Added Pitch pot range 0.43×–4.7× nominal frequency. Drive mod 0.9×–10× into the diodes → toward square.
- [VERIFIED] SD: four elements. VCO1 (triangle → diode-rounded, own crude 1-transistor VCA Q50, envelope C70 whose start voltage ∝ accent); VCO2 "tuned somewhat higher" (own VCA Q51, envelope C79). Both VCO pitches set by one CV = Tune pot + a short rise/fall pulse (ENV1, C56) at note start, same for all notes. Noise path A: filtered noise, short ENV5, quiet. Noise path B (main "snappy"): different filter, VCA Q48, ENV4 start set by accent. Oscillators always fully in the output; Snappy pot sets noise level only.
- [VERIFIED] Stock snappy envelope **holds at max ≈24 ms** before decaying, and decays faster than exponential (discharge through VR7 500k + R254 100k toward a negative rail). Noise HPF capacitor C80 = 0.022 µF.
- [VERIFIED] TR-909 noise is digital (shift registers); TR-808 noise is an amplified noisy transistor (random, "spikier").
- [VERIFIED] Electric Druid (https://electricdruid.net/tr-909-noise-generator/): 909 noise = **31-stage LFSR, taps 31 & 13, clocked ≈300 kHz**, repeat period ≈2 hours; input held high ≈20–30 ms at power-up.
- [SECONDARY, forum snippet, page 403] modwiggler thread "TR-909 Tom and Snare oscillator frequencies": 909 low tom VCO3:VCO1:VCO2 = 1 : 1.5 : 2.77. https://modwiggler.com/forum/viewtopic.php?t=222069
- [UNVERIFIED] Absolute 909 kick start/end frequencies and 909 snare oscillator Hz: no primary written source found. The SOS text gives only the architecture.

---

## 3. Modal / physical-model references

### 3.1 Comb / delay-loop decay (J.O. Smith, PASP)
https://ccrma.stanford.edu/~jos/pasp/Feedback_Comb_Filters.html
https://ccrma.stanford.edu/~jos/pasp/Achieving_Desired_Reverberation_Times.html
- [VERIFIED] Feedback comb: y(n) = x(n) + g·y(n−M); |g| < 1 for stability. Resonances at k·fs/M.
- [VERIFIED] Per-sample attenuation for T60: G = 0.001^(1/N60) = 10^(−3/N60), N60 = T60·fs. T60 = 3 ln(10) τ ≈ 6.91 τ. For a loop of M samples: **g = 10^(−3M / (fs·T60))**. Approximation: G ≈ 1 − 6.91/N60.
- [VERIFIED] EKS (Jaffe & Smith): H_p(z) = (1−p)/(1−p z⁻¹) pick-direction LPF; H_β(z) = 1 − z^(−⌊βN+½⌋) pick-position comb; H_L(z) = (1−R_L)/(1−R_L z⁻¹), R_L = e^(−πLT) dynamic-level LPF (L = bandwidth Hz); first-order tuning allpass H_η(z) = −(η − z⁻¹)/(1 − η z⁻¹), η ∈ [−1/11, 2/3] for 0.2–1.2 sample delays. https://ccrma.stanford.edu/~jos/pasp/Extended_Karplus_Strong_Algorithm.html
- [VERIFIED] Frequency-dependent loop loss: symmetric FIR loop filter with |G| ≤ 1 and Σg(n) = 1 (no loss at DC). https://ccrma.stanford.edu/~jos/pasp/Frequency_Dependent_Damping.html

### 3.2 Karplus–Strong drum (Karplus & Strong patent US4649783; CMJ 7(2) 1983)
https://patents.google.com/patent/US4649783A/en
- [VERIFIED] Drum variant: probabilistic recurrence, Y_t = +½(Y_{t−N} + Y_{t−N−1}) with probability b, −½(Y_{t−N} + Y_{t−N−1}) with probability 1−b. "The parameter b is called the blend factor."
  - b = 1 → plucked string, N sets pitch. b = ½ → drum-like ("requires only a single bit of randomness on each sample"). b = 0 → sign flips every pass: octave down, odd harmonics only (their "harp").
  - For b ≈ ½, "the buffer length does not control the pitch of the tone, as the sound is aperiodic. Instead it controls the decay time of the noise burst. For large N (around 200) and a sampling period of about 50 microseconds [fs ≈ 20 kHz], the effect is that of a snare drum. For small N (around 20), the effect is that of a tom-tom."
  - Buffer may start filled with a constant; the recurrence makes the randomness.
  - Stretch: decay probability d (stretch s = 1/d); decay time ∝ s; period ≈ N + d/2. "For drums, this has the effect of increasing the 'snare' sound, allowing smaller values of N to be used." With d = 0 and b = ½ the output is white noise.
- [VERIFIED] Csound pluck (https://csound.com/manual/opcodes/pluck/) method 3 "Simple Drum": roughness 0→1; "Zero gives the plucked string effect, while 1 reverses the polarity of every sample (octave down, odd harmonics). The setting .5 gives an optimum snare drum." (Csound's roughness = 1 − b.) Method 4 adds a stretch factor ≥ 1.

### 3.3 Bars, plates, Chladni
- [VERIFIED] Chladni's law: f = C (m + 2n)^p, m = nodal diameters, n = nodal circles; p ≈ 2 for flat circular plates; p = 1.4–2.4 for cymbals, handbells, church bells (Rossing & Fletcher, *Principles of Vibration and Sound* pp. 73–74; Fletcher & Rossing, *Physics of Musical Instruments* p. 680). https://en.wikipedia.org/wiki/Chladni%27s_law
- [VERIFIED] Rossing (ASA abstract, via search): ~300 modes observed in a 16-inch cymbal; modified law f_mn = C_n (m + 2n)^{p_n} fits flat and non-flat plates.
- [COMPUTED] Free–free uniform bar (cos βL cosh βL = 1): βL = 4.730, 7.853, 10.996, 14.137, 17.279, 20.420 → **1 : 2.757 : 5.404 : 8.933 : 13.344 : 18.638**.
- [COMPUTED] Clamped–free bar (cantilever): 1 : 6.267 : 17.547 : 34.386.
- [COMPUTED] Clamped circular plate: 1 (0 diam) : 2.081 (1) : 3.414 (2) : 3.893 (0,2nd) : 4.995 (3) : 5.954 : 6.819 : 8.280 : 8.722 : 10.868.
- [VERIFIED] Csound Appendix "Modal Frequency Ratios" (J. Bower / S. Lindroth), https://csound.com/manual/misc/modalfreq/ :
  uniform aluminum bar 1, 2.756, 5.423, 8.988, 13.448, 18.680 | uniform wooden bar 1, 2.572, 4.644, 6.984, 9.723, 12 | xylophone 1, 3.932, 9.538, 16.688, 24.566, 31.147 | vibraphone 1: 1, 3.984, 10.668, 17.979, 23.679, 33.642 | vibraphone 2: 1, 3.997, 9.469, 15.566, 20.863, 29.440 | Chladni plates 62, 107, 360, 460, 863 Hz (1, 1.726, 5.806, 7.419, 13.919) | Tibetan bowl 180 mm: 221, 614, 1145, 1804, 2577, 3456, 4419 Hz (1, 2.778, 5.181, 8.163, 11.661, 15.638, 20.0) | wine glass 1, 2.32, 4.25, 6.63, 9.38 | pot lid 1, 3.2, 6.23, 6.27, 9.92, 14.15 | dahina tabla 1, 2.89, 4.95, 6.99, 8.01, 9.02 | bayan tabla 1, 2.0, 3.01, 4.01, 4.69, 5.63 | small handbell doublets 1312.0/1314.5, 2353.3/2362.9, 3306.5/3309.4, 3923.8/3928.2, 4966.6/4993.7 Hz …
- [SECONDARY, search snippet] Rochester PHY103 (https://www.pas.rochester.edu/~sybenzvi/courses/phy103/2016f/phy103_bars_and_bells.pdf): uniform bar 1 : 2.76 : 5.40 : 8.90; marimba/xylophone bars are arched underneath to lower pitch and retune partials. [UNVERIFIED] the textbook tuning targets "xylophone 1:3:6, marimba 1:4:10" (from memory); the measured Csound row gives xylophone ≈1 : 3.93 : 9.54.

### 3.4 Bells (Fletcher & Rossing via Wikipedia "Strike tone"; Hibbert)
https://en.wikipedia.org/wiki/Strike_tone (table: Fletcher & Rossing 2008 p. 682 citing Ross & Perrin 1987)
- [VERIFIED] Ratio to prime — ideal / equal-tempered / actual bell:
  hum (2,0) 0.500/0.500/0.500 | prime (2,1#) 1/1/1 | tierce (3,1) 1.200/1.189/1.183 | quint (3,1#) 1.500/1.498/1.506 | nominal (4,1) 2/2/2 | major third/deciem (4,1#) 2.500/2.520/2.514 | fourth (2,2) 2.667/2.670/2.662 | twelfth (5,1) 3.000/2.997/3.011 | upper octave (6,1) 4/4/4.166 | upper fourth (7,1) 5.333/5.339/5.433 | upper sixth (8,1) 6.667/6.727/6.796 | triple octave (9,1) 8/8/8.215.
- [VERIFIED] Strike note is a virtual pitch from nominal, twelfth, upper octave ≈ 2:3:4; usually heard near the prime. Chimes: modes 4, 5, 6 ∝ 9²:11²:13² = 81:121:169 ≈ 2:3:4.
- [VERIFIED] Hibbert (https://www.hibberts.co.uk/identifying-bell-partials/): hum two octaves below nominal; prime octave below; tierce a major 6th below nominal (minor 3rd above strike); quint a 4th below nominal; superquint ≈ fifth above nominal; octave nominal ≈ octave above. Nodal counts around the rim: hum 4, prime 4, tierce 6, quint 6, nominal 8, superquint 10, octave nominal 12. Nominal of a measured service bell is a doublet 831.7 / 832.3 Hz (0.6 Hz beat).

### 3.5 STK modal presets (Cook & Scavone, Synthesis ToolKit)
https://github.com/thestk/stk/blob/master/src/ModalBar.cpp , src/BiQuad.cpp, src/Modal.cpp
- [VERIFIED] Two-pole resonator per mode: a2 = r², a1 = −2 r cos(2π f / fs); optional zeros at ±1, b0 = (1 − r²)/2, b2 = −b0. Negative ratio = absolute Hz.
- [VERIFIED] ModalBar presets {ratios}, {pole radii}, {gains}:
  Marimba {1, 3.99, 10.65, 2443 Hz} {0.9996, 0.9994, 0.9994, 0.999} {0.04, 0.01, 0.01, 0.008}
  Vibraphone {1, 2.01, 3.9, 14.37} {0.99995, 0.99991, 0.99992, 0.9999}
  Agogo {1, 4.08, 6.669, 3725 Hz} {0.999 ×4} {0.06, 0.05, 0.03, 0.02}
  Wood1 {1, 2.777, 7.378, 15.377} {0.996, 0.994, 0.994, 0.99}
  Wood2 {1, 1.777, 2.378, 3.377}; Beats {1, 1.004, 1.013, 2.377}; 2Fix {1, 4.0, 1320 Hz, 3960 Hz}; Clump {1, 1.217, 1.475, 1.729}.
  r → T60: T60 = −3 ln 10 / (fs · ln r) (from §3.1); at fs = 44.1 kHz, r = 0.999 ≈ 0.16 s, 0.9996 ≈ 0.39 s, 0.9999 ≈ 1.6 s.

### 3.6 Cook PhISM (CMJ 21:3, 1997)
- [VERIFIED, abstract-level only] Two branches: PhISAM (physically informed control of modal synthesis) and PhISEM (stochastic particle-collision events exciting resonant filters; maracas, chimes, etc.). Full paper equations not retrieved. Sources: https://soundlab.cs.princeton.edu/research/phymod , https://www.cs.princeton.edu/~prc/Pubs8-04.txt
- [UNVERIFIED] Adrien "The missing link: modal synthesis" (1991): not retrieved.

### 3.7 Tension-modulation pitch glide (Avanzini, Marogna, Bank)
- [VERIFIED] Marogna & Avanzini DAFx-09 (https://avanzini.di.unimi.it/downloads/publications/marogna_dafx09.pdf): measured drum-membrane partial glides "concentrated in the first 200 − 300 ms of the sound", larger with impact velocity; air loading detunes and damps the lowest modes (below ≈500–600 Hz). Nonlinear surface tension T_NL ∝ (Eh / (2πR²(1−ν²))) ∫∫ |∇z|² (von Kármán-type).
- [VERIFIED, abstract] JASA 131(1):897–906 (2012): short-time-average tension variation ≈ proportional to system energy → pitch decays exponentially as energy decays (glide follows the amplitude envelope). This matches Reid §1.4 (single envelope driving pitch and amplitude).

---

## 4. Practical kick / 808 / tearout guides (non-academic)

- [VERIFIED] Credland Audio, "BigKick: Some Kick Drum Theory" https://www.credland.net/kick-drum-theory.html — body = sine with sharply decaying pitch envelope settling "(usually between 40 and 60Hz)"; "the range of frequencies that sound kick-like is fairly narrow"; kick-like range ≈35 Hz (typical system roll-off) to ≈65 Hz (above that "stops sounding very much like a kick"); Funktion-One sub −3 dB at 25 Hz; tight kicks settle "within a handful of milliseconds"; 808-style pitch keeps falling slowly after settling; attack: 909 ≈ short white-noise burst, 808 ≈ single-sample impulse; harder attack ≈ 300 Hz low-mid content, slightly longer.
- [VERIFIED] ModeAudio, "Drum Synth Sound Design: Kick & Snare" https://modeaudio.com/magazine/drum-synth-sound-design-kick-snare — kick sine ≈58 Hz, pitch env attack ≈1 ms, decay ≈50 ms, S=R=0; click = noise with "just a few ms" envelope; snare sine ≈200 Hz + enveloped-filter noise; decays 50 ms+ for slower genres; saturation ≤0.20 and EQ cut ≈40 Hz Q≈2 (kick), ≈190 Hz Q≈2 (snare).
- [SECONDARY, search summary] Other kick guides: drop 150 → 48 Hz; pitch mod ~2 octaves; 909-like ≈4–5 octaves with ≈200 ms decay, less steep; pitch env decay ≈40 ms with steep start (https://www.perfectcircuit.com/signal/kick-drum-synthesis , https://modeaudio.com/magazine/using-synths-for-drums-part-1-kicks).
- [VERIFIED] SOS "Designing Kicks In Logic Pro X" https://www.soundonsound.com/techniques/designing-kicks-logic-pro-x — 808-style: amp decay **720 ms**, pitch-env decay **7.6 ms** with modulation depth set to F5 from a C0 base (≈5 octaves); faster decay curve → more HF click, slower → more mid "thump".
- [VERIFIED] arXiv 2502.07524 (ISMIR 2024), "Harmonic and Transposition Constraints Arising from the Use of the Roland TR-808 Bass Drum" https://arxiv.org/html/2502.07524v1 — median f0 of 808 samples **49.48 Hz** (≈G1); median initial sweep range ≈ **one semitone** for 'long' samples; after a ≈**0.4 s** attack the sound is a single low sine; transposing down a fourth costs ≈6.3 dB at the speaker (≈11.8 dB with ear sensitivity).
- [VERIFIED] Attack Magazine, "808 bass with saturation" https://www.attackmagazine.com/technique/synth-secrets/808-bass-with-saturation/ — mono + portamento ≈**300 ms**; Serum tube distortion with HP so the sub bypasses distortion; multiband saturation split at **123 Hz**, much less drive below.
- [SECONDARY] 808 glide 80–150 ms typical, 2× and 3× harmonics from saturation for small-speaker audibility (search summaries of soundtrap / violetrecording pages).
- [VERIFIED, no numbers] EDM Templates, "How To Make Tearout Dubstep Bass In Serum 2" https://edmtemplates.net/blogs/edm-templates-blog/how-to-make-tearout-dubstep-bass-in-serum-2 — gun bass = "short, explosive, pitchy, transient-heavy"; recipe: fast-dropping pitch envelope + short amp envelope + FM movement + distortion/compression/clipping; separate clean mono sub, HP the destroyed layer; chop transient of one render with body of another; macro "Pitch envelope amount, transient layer, decay time, noise burst" for gun shots. No numeric settings given.
- [UNVERIFIED] No written source with numeric settings for tearout "guns" or "chug" basses was found; available material is video or generic prose.
