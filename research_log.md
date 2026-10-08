# tearout gun synth — research log

## Framing
**Question:** what DSP produces convincing tearout "guns" (short, metallic, heavily processed bass hits fired in bursts), and how do we expose it as a knob-driven synth with a good global randomizer?

**Data:** Lyra's sample library, specifically KFU (Kaifu) gun kits — ~48 guns in `KFU GUN KIT VOL 2.1/… GUN SORT` with per-gun stems TR / BODY / SUB / REV + PROCESSED (the final bounce), labeled with root key; ~25 more in `KAIFU GUN KIT VOL 1.6` (TR / FX / SUB / PROCESSED); a few in `KFU BUG TOOLS/GUNS`. Ground-truth layer decomposition from a pro producer.

**Prior knowledge (from ~14 YouTube tutorial transcripts, 2026-10-07):** consensus recipe is layered —
- SUB: sine, pitch envelope dive ("the pitch fall is crucial" — Code: Pandorum), distorted, low-mid notch, mono.
- BODY/mid: wavetable or sample (snare/perk/ride via Serum 2 spectral osc) + FM (often octave+ up, ratio tweak = color), bandpass/notch or comb filter with a "gun shape" envelope (open fast → close), pitch env on master tune, stacked overdrive, multiband comp/OTT, convolver (short plate/metallic IR, ~25–30% wet) on highs, disperser/allpass chains, frequency shifter, heavy ~500 Hz / ~800 Hz mid boost.
- TRANSIENT: white noise burst, compressed+distorted, sometimes comb'd; or pitched-up resampled body slice.
- REV: reverb/convolution as its own layer (Letsynthesize: "the reverb is the main point of the guns", 100% wet).
- Group/glue: waveshaper, clipper (G-clip/soft clip), Saturn; sub mono < ~130 Hz.
- Burst/pattern: velocity-mapped filter "open/close" per hit (oddprophet: avoid "MIDI spam" sameness); round-robin variation (Raddland); triplet staccato for machine-gun bass (Code: Pandorum).

**Success:** guns from our synth that Lyra would put in a track; quantitatively, stem-level features (envelopes, sub dive curves, spectra) landing in the KFU distribution.

---

## 2026-10-07 — KFU stem measurements (analysis/analyze_kfu.py, fit_sub2.py, plots)

**Why:** tutorials are vibes; the KFU kits give per-layer stems + final bounce, sample-aligned (48k float, ~0.9 s). Measure what a pro gun actually is so the synth's knob ranges and randomizer distributions come from data.

**Sub dive — the big surprise.** I guessed "2–4 octaves over 5–60 ms" from tutorial lore. Hilbert IF (lowpassed 1.5k) first said 30–68 st; that lowpass truncated the start. Zero-crossing tracking says the SUB stem starts at **~6 kHz median (p10 400 Hz, p90 12 kHz)** and lands at 35–90 Hz: a **~59 semitone (5 octave) median dive**. First naive power-law fit had 14 st median error — plots showed why: distorted subs have harmonic stacks, so ZC tracks get stray high points. Keeping only the monotone branch fixes it. Model `st(t) = D·exp(-(t/τ)^k)` fits at median 0.65 st error with **k ≈ 1.03** → plain exponential-in-pitch is right; **τ median 17 ms (p10 1.7, p90 40)**, D median 59 st (p10 10, p90 92). ~10% of tracks still have bad tail estimates (end_hz p90 = 6 kHz is garbage) — doesn't change medians.
Interpretation: the sub layer is a 909-style kick sweep; its pitch sweep *is* the zap/click. The "gun" pitch-fall Code: Pandorum calls crucial lives here.

**Spectrograms (layers.png):** BODY = broadband noise bed + many *stationary* inharmonic partials (horizontal lines) → noise through resonators/comb/convolver; LASE has curved descending partials (pitch-dropping resonance = laser variant). Lyra independently: "body is mostly a noise layer", "guns are highly atonal", "basically very bassy thicc snares". Consistent. **Hard gates everywhere** — TR of BONGER is a rectangle cut at ~110 ms; bodies cut at 300–500 ms. Envelopes are hold→gate, not natural exponential tails. SUB tail heavily clipped (harmonics to ~1 kHz).

**Envelopes (time from onset to -20 dB, median [p10–p90]):** TR 69 ms [30–190], BODY 253 [138–405], SUB 450 [298–612], REV 298 [189–477], PROC 467.

**Levels rel. to PROCESSED RMS (median):** SUB −0.6 dB (sub carries the RMS), TR −8.5, BODY −9.9, REV −18. BODY/TR/REV sit ≥40 dB down below 150 Hz — hard highpassed.

**Tonal balance of PROCESSED (1/3-oct, rel. peak):** 39 Hz 0, 50 −3, 62 −11, 99 −14, 157 −20, **250 −26 (dip)**, 500 −23, 1k −19, **1.6–8 kHz flat plateau −16…−18**, 12.7k −20, 16k −23. Sub spike + scooped low-mids + flat bright plateau. Side/mid −9 dB, crest 12.8 dB.

**Design consequences:** layers TR / BODY / SUB / SPICE (Lyra: comb, metallic samples, random sine bleeps, noise types) / REV as a convolver with synthesized IRs (Lyra's idea; plate, modal metal, chirp "whip", spring). Hold+gate envelopes. Randomizer samples from the distributions above. Show the KFU tonal-balance curve as an overlay on our render. Same engine covers 808s / chugs as parameter regions (Lyra).

**Queued:** deconvolve REV against BODY+TR to recover the IRs KFU used.

## 2026-10-07 — engine v1, fitter, processing consensus, band envelopes

**Engine** (`dsp/engine.js`): pure JS, deterministic; layers SUB (exp-in-pitch dive), TR (colored noise + click sweep), BODY (noise → modal bank / comb / FM, open-close SVF, laser drop), SPICE, REV (convolver with synthesized IRs: plate/metal/whip/spring/room), crunch chain on tops, bus/master (OTT, mono-low, clip). 50–110 ms per render.

**Fitter** (`fit/`): CMA-ES on multi-res log-mel (+ LTAS, + pitch track for sub). Lessons, in order:
- Mel loss NaN'd — 256-pt FFT has 187 Hz bins, low mel bands empty. fmin now ≥ 3·sr/nfft.
- Sub dive depth vs dive time is degenerate under mel (dive happens in first few ms, frames too coarse): truth 40 st/6 ms fit as 22 st/21 ms. Adding a zero-crossing pitch-track term *naively* made it worse: CMA converged prematurely and attack (hiding the dive from the tracker) + a 24-st missing-point penalty carved plateaus. **Staged fitting fixed it**: pitch params on clean renders vs pitch track → envelope/drive on mel → joint polish. Recovers all sub params (dive 38/40, τ 6.4/6, hold 255/250, drive exact).
- TR fit loss 0.151 vs seed-to-seed floor 0.150–0.158: at the noise floor. BODY 0.762 vs floor 0.74–0.76: also at floor — but mechanism unidentifiable (rebuilt modal metal from FM+drop+comb). Mel can't see partial positions finely. Added LTAS (8192-pt, averaged → noise-robust): floor ~1.05, 5% mode-freq shift → 1.46, wrong mechanism → 1.50, so it discriminates. Mode jitter was seed-dependent (design bug) → fixed table. Still lands on comb instead of modes even with mechanism-prior restarts + grid prelude over mode_f×metal. Parked: body identifiability is a needle problem; matched-spectrum is what we actually need. Revisit if real-data body fits look wrong.

**Processing consensus** (subagent, 44 tearout-specific videos / 23 channels, research/yt_processing/SYNTHESIS.md): OTT nearly universal (32/20), time up, 30–60% per stacked instance, +6–14 dB out into next clip; sub clean+mono, crunch above 200–350 Hz split (22/16); dispersion stage early (21/15); short-IR convolver on highs, decay ↓ gain ↑ mix high (19/9); ≥2 distortion stages (19/12); comb/resonators (19/14); clipper last at 0 dBFS. → engine gained a crunch chain on tops: mud cut + scream peak → clip → allpass disperser → short convolver → clip → [OTT→clip]×N → SSB freq shift.

**PROCESSED ≠ sum of stems**: least-squares stem gains leave median −7.6 dB residual; WALTER/WARP/TORPEDO ≈ 0 dB (nothing linear survives). KFU has heavy bus processing → fit our bus by pushing KFU's own stems through it vs PROCESSED (`fit/fit_bus.js`). BONGER: fitted 3.54 vs plain sum 4.58 vs our default master 6.03 — default master moves *away* from KFU.

**Band envelopes** (`analysis/band_adsr.py`, octave bands, 2.7 ms hop): TR = rectangle (1–8 kHz, t-6 15–19 ms, hard gate 13–23 dB/15 ms). SUB's 31–62 Hz band peaks ~170 ms in (the dive lands late → delayed low bloom). BODY tilted bright (1–8k ≈ −2…−3 dB, 250–500 −14), bands decay together. PROC gates softer (2–6 dB/15 ms). Clustering PROC band features: no elbow (inertia 1380→983 smooth for k=2..6) → continuum mostly along length; k=4 gives chop (21) / long-sub (14) / ringers (12, high bands to ~700 ms: LASE, WARP, WALTER) / sustained (GAINSPLIT). Don't oversell "types".

## 2026-10-07 — real-data sub fits, UI, presets, kill test, gun-tutorial frames

**Sub fits on KFU were bad at first** (mel 1–13 vs ~0.1 synthetic). Two causes. (1) SUB stems carry a hiss bed to 16 kHz crunched with the sine; the sine-only model contorted its envelope to shrink HF mismatch (DEX: drive 0, hold 20 ms). (2) The *target* pitch tracks were garbage: raw ZC on ARSON reads ~3 kHz throughout (tracking hiss); CANISTER raw tail 74 Hz vs lowpassed 37 Hz (octave error from clipping crossings). **Correction to the earlier sub-dive analysis: `end_hz` median 66 Hz was contaminated** — robust tracking puts every tail on the labeled note in octave 1 (ARSON E1 41, CANISTER D1 37, DEX C1 34, BONGER 37, GAZER 32). Dive starts (5–12 kHz) are real (visible as the bright descending curve in spectrograms).
Fixes: multi-cutoff zero-phase-lowpass tracker (pick most-lowpassed copy with cutoff ≥ 2.5× estimate, monotone-from-tail), and a `sub_noise` generator crunched through the sub drive (the Letsynthesize "distorted sub with top noise, crunched together" recipe). ARSON 5.25 → 3.25, pitch err 6 → 0.4–0.6 st.

**Kill test (fit/validate.js, 21 guns at the time):** render each assembled preset through the full engine, score vs every gun's PROCESSED. Own-preset rank 1 for 17/21 (chance ≈ 1/21); median own loss 5.59 vs other guns' presets 10.72 vs default engine 10.15. Not pitch-only: C guns mostly still rank 1 among each other. Gap: own-preset 5.6 vs bus-fit-from-real-stems ~2.4 → synthesis is where the remaining error lives.
Odd: bus fits want almost no OTT/drive (median 0/0) and beat plain stem sum only modestly (2.39 vs 2.63) — our master topology may not resemble KFU's bus. Unresolved.

**Gun-tutorial agent** (42 transcripts / 27 creators + 47 on-screen frames from 13 videos; research/yt_guns/SYNTHESIS.md): per-hit gate in ~26/20; frames show three gate-shape families — spike-then-ramp (oddprophet, Code: Pandorum: falls to 20–45% in first 5–15% of the cycle then linear to 0), exponential (Wheysted, Letsynthesize), dome (PHENOMSOUND, RPS). Metallic tail from comb or 10–36 ms feedback delay, often ping-pong with 2–4 ms L/R offset (on-screen 10.9/13.1, 13.3/16.7 ms). Pitch drop to 0 in ~40–140 ms; sub pitch env +12…+36 st in Serum terms (KFU measured far bigger: Serum's master-tune ranges understate it, or KFU goes further). Clip sub+guns together into GClip at +5–6 dB (~10). Caveat: Dripment's machine-gun video runs Letsynthesize's preset — counted as one source.

**Engine additions (defaults bit-identical to prior engine, verified max diff 0):** per-layer gate shape exp/spike/dome; body partial tables stiff/membrane/bar/plate/cymbal/bell/808 (from the drum-synthesis agent's verified ratios); spice type 808 metal (six squares 205.3–800 Hz → BP 3.44k/7.1k); crunch ping-pong delay; sub fast click term (two-speed pitch env, Astrobear/Noisia recreation + 808 analysis). sub_dive range → 120 st (KFU fits pinned at 96).

## 2026-10-07 — full 47-gun fit, level-assembly bug, mix stage

All 47 guns fitted (sub/tr/body/bus). Kill test on 47: own-preset rank 1 for 21/47 (chance ≈ 1/47), median rank 2; own 6.16 vs others 10.52 vs default 9.22. **This is the semi-held-out number**: layers fit to stems, scored vs PROCESSED never seen by any fit.
Failure tail (TORPEDO 44, SHOWER 34, OAAA 33) traced to **level assembly**: layer levels came from bus-fit stem gains, and bus fits went degenerate on guns whose PROCESSED barely contains the stems linearly (TORPEDO lstsq weights TR≈−0.05, BODY≈0) → tops at −44/−60 dB, drive maxed, preset = bare sub. The "KFU bus uses no OTT" result was this artifact.
Fix: `fit/fit_mix.js` fits layer levels + master on *our own rendered layers* vs PROCESSED. TORPEDO 5.16 → 3.20, and the fitted mix now uses heavy OTT (0.78, upward 1.0). All guns: rank 1 for 40/47, median own loss 4.82. **In-sample** (mix tuned on the scored target) — expected to inflate; the 21/47 above is the defensible identity number. Remaining misses: JAW 24, SHOWER 12, WARP 10.

## 2026-10-07 — Lyra's ear: "blown out kick, not a gun", "not enough high end"

Measured instead of eyeballing: median preset tonal balance matched KFU (high 2–8k −21.4 vs −22.9 dB re loudest band), fresh randomizes brighter (−15.8). But 13/47 presets >6 dB dark vs their own gun (WET −30, JAW −23, CANISTER −22). Cause: **the mix fit exploited a model mismatch** — reverb was precomputed from the base tops and scaled independently, so CMA killed the dry tops (body −46 dB) and filled highs with reverb wash; in the engine the reverb is fed by the tops, so the preset rendered as a bare sub. Coupled reverb to its feeding layers in fit_mix → 9/47 dark, rank-1 43/47 (in-sample).

Lyra's direction (overrides matching-KFU-loss as the objective where they conflict): mids are the gun; the sub lies clean underneath; less post-processing on the sub. Also matches tutorial consensus (22/16 keep the sub out of the crunch). **Topology change:** master chain (OTT → drive/clip, loudness-preserving) on tops+reverb only; sub added after it; final stage = shelves, mono lows, light soft ceiling. `mst_sub_in` (default 0) routes sub into the chain for the glued sound. KFU mixes refit under the new topology.

**Archetypes** (dsp/archetypes.js) per Lyra: "a gun should sound like a gun, or snare, or metallic ping with giga noise, or a really clean punchy chug." Hand-built base + varied knob ranges per family; randomize rolls within the family; KFU-anchored randomize kept as one option. First rolls: mids carry energy, sub clean, −17…−19 dB RMS (quieter than KFU's ~−9; loudness now comes from mids drive, not a clipped sub).

## 2026-10-07 — why presets were dark: the loss was blind to balance

Clean-sub topology made KFU presets darker (median −3.8 dB highs, 16/47 dark). Letting the mix fit choose `mst_sub_in` (median 0.21) barely helped (−2.5, 14 dark) → routing wasn't it.
Probe (fit/probe_tops.js, CANISTER): raising tops toward the correct high-band level made *every* loss worse (total 4.23 → 7.03 at +18 dB), yet each layer alone matched its KFU stem spectrally (TR near band-for-band). So: synthesis fine, **loss wrong**. Mel and LTAS remove a gain offset taken from the *median* cell/bin; most cells are mids/highs, so the offset aligns the highs and the sub-vs-mids balance rides on a few low cells — the loss is nearly blind to exactly what Lyra heard.
Fix: tonal-balance term — mean |dB| over 27 third-octave bands, each side normalized to its loudest band (equal weight per band ≈ log-frequency, ear-like), weight 0.3 in fit_mix. Result: **0/47 dark** (worst −1 dB, median +0.3), identity ≈ unchanged (rank-1 38/47, mel 4.89 vs 4.81).
Lesson for the skill file: gain-invariant spectral losses with median offsets silently ignore tonal balance; add an explicit log-band balance term whenever relative band levels matter.

## 2026-10-07 — "too much noise, harsh and flat"; "random makes too much high end; presets are great"

Quantified "harsh and flat" (fit/character.js): noisiness (spectral flatness 400 Hz–10 kHz), peakiness (LTAS partial prominence), movement (centroid range over the hit, octaves), crest, harshness (2.5–6k minus 0.5–2k). KFU medians: noisy 0.23, peaky 1.26, move 2.09, crest 8.56, harsh −1.1. Gun archetype was move 0.71 (static noise block, then gate), crest 12. **"Flat" = no spectral motion** — KFU centroid sweeps ~2 octaves bright→dark per hit.
Fixes: gun body = fully-wet LP sweep 14k→~900 Hz (τ 30–200 ms) + spike gate, less noise, more partials, one rack pass; spice events confined to the first 60% of their window with a fade (bleeps ran far past the gun); mutate can no longer wake a layer at −60 (it crossed the render threshold and upward OTT dragged it up 20+ dB); **auto-balance on every roll**: render, measure 1–8 kHz vs target (KFU −17.4; ping −15; chug −24), trim tr/body/spice, ≤3 passes.
After: gun noisy 0.10 / peaky 1.16 / move 3.09 / crest 8.39 / harsh −1.8 / 1–8k −17.4; kfu-anchored randomize 0.27 / 0.93 / 1.88 / 8.90 / −1.23 / −18.1 (was p90 −13.6, "too bright"). Chug can't reach −24 (LP'd FM body has no top to raise; median −30) — left as "clean", flagged.
Lyra: fitted presets "are great" → the fit pipeline is the trustworthy path; randomizer quality now anchored to the same measured targets.

## 2026-10-07 — gun-ness, transient study, filament B, synth layer, stems

Lyra: randomize = "blown out psystyle kicks every time"; presets "great". Built `dsp/gunness.js`: 8 features (noisy, peaky, move, crest, harsh, subMid, midSustain, midLate), robust-z distance to the fitted presets' distribution. Sanity: presets 0.80 (in-sample), a synthetic psy kick 1.52, gun rolls 1.24 → gun rolls really were kick-ward. **Surprise: not sub level** (gun subMid 6.8 vs presets 12 — sub relatively quieter). The kick-ness was **too tonal + too short**: peaky +1.9σ, noisy 0.13 vs 0.35, mid sustain 117 vs 219 ms. My earlier "less noise" (after "too much noise, harsh and flat") overcorrected texture; the harshness was the 1–8k *level* wall. Retuned gun family (noise back, partials down, longer/brighter bodies, cleaner sub, noise transients) + best-of-5 by gun-ness in the worker: gun 1.05 → 0.84 → 0.73 after transient work; fitted mode 0.58.

**Filament B (Lyra: "study why this one works")** — fitted from WALTER: slow sub dive (τ 57 ms, 3× median → audible sweep = "sub-sweepy"), long dark noise body with fast exp decay under a 1 s hold (fades, not gated) + onset FM burst (idx 30 decaying 7 ms) + 16 st laser drop; transient quiet (−18), dark, crushed (drive 40), soft gate (80 ms); **sub mostly into the chain (0.85)** with master drive ~20; crunch chain off. Clean-sub isn't universal — it's per-gun.

**Transient study** (48 reference TR stems, fine-time spectrograms): mostly broadband noise 40–150 ms, hard gates; dominant shape is **triangular — highs end first** (closing lowpass); some pitched downward zaps (LASE, BIGE: harmonic sweep ~10k → few hundred Hz in 30–50 ms); some stutters (SHOWER, SPOON, CRICKET); many reach to ~100 Hz. Added tr close-to/close-time (time-varying LP), zap (polyBLEP saw sweep), stutter (retriggered envelope); defaults bit-identical. Gun family now: noise transients with closing LP, soft gates to 80 ms, zap 30%, stutter 15%, sub τ to 70 ms, sub-into-chain 0–0.9, crunch optional, synth layer 35% (polyBLEP saw/square/supersaw/pulse, unison, pitch dive, filter sweep).

**Stems**: each layer soloed through the chain with the full mix's gain stages locked (mids pre/post, final pre/post) → relative levels preserved. Sum-of-stems ≠ mix (filament B residual −8.3 dB) because layers drive into shared clippers — expected for nonlinear chains; each stem = "that layer through the chain".

**Presets renamed** (rivet E, splinter C, …); no reference-pack names in shipped files; private map at out/preset_map.json.

**VAE (Lyra's idea, "8 gun-ness knobs")**: 47 presets far too few. Plan: grow a gun manifold — ~10k candidates (fitted 40 / gun 35 / snare 10 / ping 10 / chug 5 %), keep gun-ness ≤ preset level, train 8-d VAE on 141 normalized params. Caveat: learns my sampler + my score's idea of a gun.

## 2026-10-07 — gun-space VAE (8 macro knobs)

Dataset (fit/gen_manifold.js): 10,200 candidates (fitted 40 / gun 35 / snare 10 / ping 10 / chug 5 %), scored by gun-ness. Kept ≤ 0.9: 3,561 (fitted 1,616, gun 1,339, snare 605; ping 1, chug 0 — different families, correctly excluded). 141 normalized params.
VAE (fit/train_vae.py; 141→256→128→8, mirror decoder, sigmoid out, MSE recon, KL warmup, early stop on 10% val): β=0.5 → **posterior collapse, 3/8 dims KL≈0**. β=0.2 + free bits 0.35 nats/dim → all 8 alive (KL 0.33–1.23), val recon 1.57 vs mean-patch baseline 4.86.
**Does the space produce guns?** Prior samples z~N(0,1): gun-ness median 0.82 (fitted presets 0.80, unfiltered rolls ~1.0, psy kick 1.52), 68% ≤ 0.9; z~N(0,1.5²): 0.79, 83%. Caveat: circular — the same score filtered the training data; this shows the decoder learned that region, not independent validation. Lyra's ear is the external test.
Traversals (−2→+2, effect in reference-spread σ): z1 snap (click↔synth, subMid +4σ, harsh +4σ, peaky −4σ — strongest), z3 bite (harsh −3.6σ; sign flipped in UI), z4 metal (peaky +2.9σ, BP body), z2 ring, z7 zap (tr_zap on, move −2σ), z6 wash (long tails), z0 smooth, z5 air. Entangled (each moves several things) but distinct. UI: "gun space" row of bipolar knobs; note/burst/len/seed kept when turning; "space" randomize samples z~N(0,1.2²).

## 2026-10-07 — reference-driven families, honest knobs, mutate drift, transients

**Mutate drift ("SUPER long pitchdown into sustained noise").** First hypothesis (random walk) not supported by single-press data; second (crunch-mix ratchet) killed directly (cr_mix 0.05/0.2 → no length change). Reproduced at scale: 10-press chains on all 47 presets → 21/47 gain ≥ +6 dB energy after 500 ms (p90 +33). Culprits: tail generators — ping-pong feedback ratcheting up from 0 (params at their minimum can only move up under clamped noise), reverb size random-walking 350 → 1.6 s. Fixes: knobs at minimum stay; tail params mutate at 0.4×; mutate = variations of an anchor (last roll/preset/hand edit), not a chain; **hard guard**: candidates whose late energy exceeds anchor + 3 dB (and > −25 dB) are redrawn. Result: 0/188 audible-tail violations, median late change −0.2 dB.

**Transient.** Onset punch (HF first 20 ms vs next 100 ms): reference 5.4 dB, presets 5.1, gun rolls 2.9; transient layer's share of onset HF: presets 51%, gun rolls **11%** — Lyra's "is there even a noise transient" was right. Tied tr_lvl to body (+4…+14 dB), soft body attack, shorter/harder transients, new `snap` transient shaper (fast-vs-slow envelope, ≤ 12 dB) on the mids → rolls 5.3 dB punch (≈ reference), share 39–46%.

**Snare / chug (Lyra: "i dont think you understand what a chug is… or a snare").** Correct: built from her library (her own pack + heavy-pack snares, KFU/Knoir/Disciple chugs). Snare refs (n=40): subMid −35 dB, mids die in ~100 ms (midLate −10.7), noise core 1–3 kHz, shell thump 150–250 Hz. Chug refs (n=10): distorted harmonic-dense pitched note (reese/saw), subMid +7, sustained mids (midLate +9), motion 2.3. Also found: best-of-K scored snare/chug against *gun* features, pulling them toward guns → per-family references (web/family_refs.json), per-family balance targets from the refs (snare −11.4, chug −12.6 dB 1–8k). Rebuilt families: snare score vs snare refs 0.40–0.44; chug 0.64 — still too static (0.7 vs 2.3 motion) and dark; real chugs move via chopping/growl motion, needs a moving vowel. n=10 chug refs: stopped tuning to avoid fitting noise.

**Gun-space knobs (Lyra: "those vae knobs do not do what you label them").** Correct: labels came from one traversal through an entangled latent. Replaced with a **conditional VAE** (params | 8 measured features + 4-d latent); decoder strongly uses the condition (val recon 1.75 true features vs 5.77 shuffled). Verification by render sweeps (−1.5…+1.5σ, 3 latent seeds, Spearman + span): open-loop passes only metal/length/tail. Added **closed-loop steering** (render → measure → correct condition, ≤ 4 passes) with per-knob loop gain: passes noise ρ1.00, motion .93, punch .93, bite .90, sub .90, tail .87 (|target−measured| ≈ 0.3–0.5σ). Metal and length fail under every loop config (correcting others knocks them over) → **shipped 6 verified knobs**, dropped 2. Lesson: verify a knob by measuring its claimed effect, never by naming a latent after one sweep.

**UI.** Tabs synth | gun space; save/load presets (localStorage, inline name field); init = best of 60 seeded gun rolls (gun-ness 0.54); per-group roll; reference controls (play, level-match, clear); "+ layer" sample slots (3) with pitch/key-track/start/length/filters/route; tonal: key-tracked partials+comb, synth interval voice, vowel formants; stems; select padding.

## 2026-10-07 — Lyra's labeled examples; staggered layers; chug = ^ not \

Lyra exported 10 labeled sounds (6 snares, a chug/gun borderline, snare+3 chugs, 2 pings). Her examples overrode pack-derived refs:
- **Snare** (hers): denser (crest 8.9), darker, more thump (subMid −24 vs packs −35), longer mids than pack snares; fast onset dive (snare 1), ring partials ~1–2 kHz (snares 1, 2). Lyra: "the transient-body-wash in snares are staggered" → per-layer onset delay (sub/tr/body/synth, 0–60 ms).
- **Chug** (hers): NOT a bright reese (the pack chugs misled me). Lyra: "a chug has the same distorted noise vibe as a gun, but… chugs in instead of slamming… more ^ shaped than \". Band envelopes confirm: gun (BONGER) every band instant at 0 then decays; her chugs' mids/highs ramp up over tens of ms, sub arrives late (~40 ms) and swells. Time-to-peak alone was inconclusive (5–65 ms chugs vs 5–30 ms guns; crude segment cuts) — the envelope plot was what showed it.
- **Ping** (hers): short (150–250 ms), clean (noisy 0.07), extremely partial-dominated (peaky 2.3), bright, ~no sub.
**Metrics vs ear**: snare scored "close" (0.57) on the 8 features and Lyra: "these snares dont sound like snares". 8 summary features are too coarse for family identity. Switched to what worked for guns: direct full-mix fits of the engine to each of her examples (fit/fit_full.js, staged CMA over groups incl. a stagger stage, mel+LTAS+tob loss); family rolls anchor on those fits.
**Why chugs (and many guns) read as boxes**: envelopes sit upstream of 12–32 dB drive + clipper, which pumps any decay back to full scale. Added per-layer `swell` (post-drive ramp-in) and a post-crush mids envelope (`post swell`/`post fall`). Chug family rebuilt: gun texture, swell 35–110 ms, delayed swelling sub, no slam transient, no snap, little OTT/rack → renders now show a real ^. Engine defaults bit-identical throughout (verified each change).

---

## 2026-10-08 — Snares get their own engine

**Why.** Lyra: "your snares need some work BADLY", "these do not sound like snares". The old snare family was the gun engine with the sub retuned up. Measured against 178 CamaCon Snaretober snares (analysis/snares/analyze.py), our rolls were a small chug: loudest moment at 31 ms vs 4 ms, shell at 160–240 Hz ringing 107 ms vs ~390 Hz dying in 37 ms, 2–6 kHz at −12 dB vs −5, 6k+ at −20 vs −10. Snaretober bodies sit exactly on the labelled note (72/111 tonal snares, octave 3–4).

**What the layers look like.** The Snaretober "Layers" stems show the stagger directly: snare noise peaks at 1–5 ms, metal at 10–20, snap at 17–32, ping at 30–75, foley ~100 ms. "Pre-transient" stems are ghost hits — 2–8 short copies of the attack ~50 ms apart, crescendo into the downbeat (Day 6, Day 13) — not reverse swells; my first pre layer guessed wrong and Lyra caught it by ear.

**Research.** Two agents. yt_snares (45 tutorials / 38 creators, 11 with frame reads): one osc + noise core; body 180–250 Hz (dubstep), 250–500 (DnB); 90° start phase; rounded-square shell; two pitch envelopes (+12 st τ12–20 ms body, +36 st 2–4 ms click); noise peaks 5–25 ms after the hit; 1–2 kHz is the identity band (VR's peak sweeping up to 1 kHz); tuned narrow resonance distinguishes tearout "pan" and trap families; failure modes listed in §4. yt_snaretober: CamaCon's dailies aren't on YouTube (only two long 2025 streams); OCR of FL's hint bar gives pitch env +48–70 st with 90% of the drop in 6–12 ms, body decay 54–77 ms ("long bodies are the most common mistake"), noise attack 6–9 ms.

**Engine (dsp/snare.js, dsp/snare_spec.js).** Layers pre (ghosts/roll/reverse) → click (click/bleeps/zap/burst/snap) → tone → noise (white/pink/geiger/crackle/crushed/digital/ring, own drive, flutter, tuned ring) → metal (modal/comb/unison/808/bell) → clap → room → bus (time macro, whip formant, post-click dip, OTT, snap, drive, ceiling). All mono — Lyra: "are you randomly panning? because no". `engine` param switches render; gun output is bit-identical to before (max diff 0 on a fitted preset).

**Mistakes on the way.** (1) Tone drive after the envelope: the clipper flattened the decay into a tom; envelope now follows the clipper. (2) A scare that the flavors had too much sub: the python band-envelope metric reads the onset transient through filtfilt ringing; energy share below 150 Hz is −26…−29 dB for flavors vs −28.5 median Snaretober — fine. Measure with the metric that matches the claim.

**Corpus.** 4615 files named *snare* in ~/Music/samples → 4030 unique single hits. Snare gun-ness reference and target curve now come from 671 of them instead of Lyra's 7. 212 diverse ones (farthest-point on band/length features, ≤8 per pack, plus Lyra's exports) are being fitted with fit/fit_snare.js to become randomize anchors — the thing that made gun randomize good ("hitting randomize on fitted is amazing").

**Chugs.** Lyra: "chugs dont really have a transient or if they do its well inset from the start". The fitted chug anchors had sub clicks up to +31 st and 1–5 dB of snap. Chug rolls now go through chugify(): no sub click/zap/snap, transient delayed 18–43 ms, master swell 12–40 ms. First-5 ms level vs peak went from ~0 dB to −7…−24 dB (refs: −7, −11).

**Snare fits (2026-10-08).** 212 one-shots fitted (fit/fit_snare.js, ~9k evals each, 12 workers niced). Loss p50 5.43; the worst 15% dropped → 179 anchors in family_fits.snare_eng, 20 of Lyra's snares in "your sounds", 161 library snares in a "snares" preset group. Spectrogram pairs (analysis/snares/pairs*.png) match well — body pitch, wash shape, length, tonal lines. The 8-feature gun-ness score disagrees (anchors 1.03 vs hand-built flavors 0.86 against a 671-snare corpus reference; anchors read less "noisy"/"move"). Same lesson as the earlier snare episode: that score is not a snare detector; trust the pairs and Lyra's ears. Early fits showed the shell ringing too long (OTT upward lifting the tail); a joint body-timing stage (tone hold/decay/gate/drive + OTT + room + noise level/decay) fixed most; 24 fits made before it were redone.

**Gun rolls: sub-locked buzz (2026-10-08).** Lyra's screenshot of a fitted roll vs KFU Ambivalence: hers had fuzz riding the sub peaks; KFU's mids are a dense block that, if anything, dips at the sub peaks. Measured over 47 KFU processed guns: corr(>300 Hz envelope, |sub|) over 20–200 ms median −0.44 [−0.77..0.06], mids/sub peak −2.3 dB [−5.3..3.1]. Fitted rolls reached +0.39 and −10.7 dB — the "mids" there were the sub's own clipping harmonics (they sit at the clipped peaks), i.e. a blown kick. No single knob causes it (sub_tau −0.42, transient/body level −0.3). Best-of-K now adds a penalty for am > −0.1 and mids < −6 dB re sub; fitted rolls after: am ≤ 0.06, mids ≥ −5.4 dB — inside the KFU range.

## 2026-10-08 — Is the fitter any good? (CLAP critic, ceiling and recovery tests)

**Question.** Lyra asked for a model of "on-manifold vs off-manifold" patches. Before building one I needed a critic I trust, and that led somewhere more important.

**Critic validation.** Log-mel kNN to the 4030-snare corpus separates kicks/hats only after energy-weighting (floors made silence look similar); it still calls the old gun-engine snares Lyra rejected 100% snare-like. CLAP (laion/larger_clap_general) kNN cosine separates better: kicks 9% on-manifold, hats 18%, uniform-random patches 6%, real LOO 90% by construction. Zero-shot text labels are useless (real snares → "kick drum" 52%).

**The finding.** CLAP distance fit ↔ its own target: median 0.29. Target ↔ its nearest *other* real snare: 0.09. Only 6% of fits are closer to their own target than an unrelated real snare is. Calibration (CLAP moves 0.001 for ±2 dB tilt, 0.01 LP 12k, 0.014 soft clip, 0.02 faster decay, 0.04 for +1 st, but 0.23 for a −40 dB noise floor across the window). Gating both tails identically only moves the gap 0.29 → 0.26, so it's the body, not the tail. Fine spectral structure / rattle statistics of real vs fit are identical (5.1 vs 5.1 dB, 0.26 vs 0.26) — hypothesis "real wash isn't white noise" killed.

**Ceiling test.** Resynthesize real snares from (a) full STFT magnitude, random phase → 0.013; (b) 64-band mel envelope only (what the loss sees) → 0.050; (c) mel from a 256-pt STFT → 0.163. A perfect mel-loss fit would land ~0.05. The loss is adequate; the fits are nowhere near its optimum.

**Recovery test.** 16 targets rendered *by the engine itself* (exact solution exists, loss 0). Staged CMA-ES (~9k evals) reaches median loss ~4.6 on them vs 5.45 on real snares. **The optimizer is the bottleneck, not the engine and not the loss.** This reframes everything downstream: anchors are mediocre approximations of their sources, which is consistent with Lyra hearing fits as "not quite".

**Next (when on power).** Amortized inverse model (Synplant-2-style "genopatch"): train audio→params on millions of self-rendered patches drawn near the real-sound prior, warm-start CMA from its prediction; measure with the recovery test first (it has ground truth), then CLAP fit↔target on real snares.

## 2026-10-08 (battery, single-core) — Fitting-lab: the loss, not just the optimizer

Harness: fit/lab.js (targets, loss variants v1–v4, fingerprint kNN bank), fit/exp_opt.js (bake-off), fit/exp_landscape.js, fit/exp_snr.js. Backup of the whole project before this: ~/tearout-gun-backup-2026-10-08.

**Bake-off at 800 renders/fit (v1 loss), median.** synthetic: staged 6.55, flat CMA 6.63, nearest-bank-patch 8.72, nn+refine 5.09, nn3 5.27. real: staged 7.11, flat 7.64, nn 8.54, nn+refine **5.75**, nn3 6.00. nn+refine at 800 ≈ staged at 9k (5.45). Caveats: the bank contains the old fits of these same real files (leakage), and nn+refine picks the best of 8 seeded candidates (seed lottery, see below). Not yet a clean win.

**Recovery test was mis-measured.** Loss at the exact answer was 0 for t0 but 2.1 (t3) and 3.6 (t6): targets get onset-trimmed, renders didn't. Fixed in lab loss v2 (renders trimmed identically); truth → 0.00 for all. The earlier "optimizer is the bottleneck" claim (recovery fits at 4.6) was partly this artifact.

**Seed floor.** With truth knobs and only the seed changed, loss is 2–5 (v1). Clap burst-timing jitter and metal excitation were seeded; made deterministic (research says fixed phase/excitation anyway). Still 2–4: the seed also sets shell-mode phases, noise-mod, bleep placement, room IR, and — biggest — the pattern of crushed/digital(LFSR)/ring noise, which is spectral structure, not texture. Time/band smoothing of the loss (v3, v4: ±25–50 ms, ±1–2 mel bands after 30 ms; fine resolution only on the attack) barely moves it (≈2.5). Bus/layer drives are not the cause (all drives off: 2.4–3.1).

**Loss SNR.** Rise for a 10% knob move (same seed) ÷ change from a seed swap: ~0.1 for every variant (v1 0.04–0.79 but the high ones were the alignment artifact). Knob errors sit an order of magnitude under seed effects. For real targets (no matching seed exists) this means much of a fit's loss value is irreducible, and comparisons between methods need many targets/seeds.

**Seed co-adaptation.** On 10 real fits, other seeds with the same knobs are +0.5 worse (median); best of 16 seeds doesn't beat the fitted seed (−0.12). The knobs bent toward seed 1's realization — moderate overfitting.

**Reflections.** I made the "optimizer bottleneck" claim too fast from one number; the recovery test needed its own floor check first (loss at truth, and at truth with another seed). Always measure the loss's own noise floor before reading a fit loss.

**Next.** (1) Seed-robust objective: average loss over 2–3 seeds per eval (cost ×2–3) or a fixed seed bank shared by target-fitting and rendering; compare knob recovery (synthetic, known truth) rather than loss. (2) Shell phases deterministic. (3) Rerun the bake-off with loss v2, bank without leakage, knob-recovery error as the metric.

**Knob recovery (5 engine targets, 800 renders, loss v2).** knob err (mean |Δu| over live knobs) / sound err (v2 loss vs truth rendered at the fit's seed):
nearest bank patch 0.083 / 7.45 · flat CMA 1 seed 0.302 / 6.05 · flat 2-seed avg 0.318 / 7.47 · nn+refine 1 seed 0.162 / **4.98** · nn+refine 2-seed 0.142 / 5.22.
Reading: the kNN start dominates (flattered — targets are jitters of bank anchors, so nn often finds the parent; err 0.083 ≈ the 0.08σ jitter). Refinement lowers sound error but doubles knob error: CMA wanders in insensitive knobs (≈20/73 live knobs move loss < 0.05 for a 10% move — exp_landscape t0), so unweighted knob error overstates harm; sensitivity-weighted error is the better knob metric next time. 2-seed averaging doesn't pay at this budget (it halves iterations). Even at the truth seed, refined fits sit ~5 where 0 is reachable: the landscape is hard to search from a merely-close start. Strongest lever: a better start → learned inverse model (genopatch) once on power.

**Refinement from the same kNN start (5 engine targets, 800 renders, sound err vs truth at same seed).** CMA all live 4.98 (3.38 4.82 7.23 6.56 4.98) · CMA top-30 sensitive 5.08 · **coordinate descent 4.36** (1.98 3.87 5.01 5.03 4.36) — wins 5/5 despite spending ~150 renders on the sensitivity scan. Near a good start, knobs act mostly independently; CMA at λ≈16 in ~70 dims can't learn its covariance in 800 renders. n=5 — enough to change the default, not enough to call it robust. Next: CD with sensitivity re-scan per cycle, CD→CMA hybrid, and the same comparison on real snares with CLAP fit↔target as the judge (needs GPU/power).
