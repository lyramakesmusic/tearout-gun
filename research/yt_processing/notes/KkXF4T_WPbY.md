# KkXF4T_WPbY — SERUM TEAR-OUT GUN BASS TUTORIAL [Phaseone/Svdden Death/Marauda]
Wheysted Music · 13:14

**Relevance:** Directly on-topic: tearout gun made in Serum + full post chain + print/resample.

## Chain
Synth (Serum, gun layer):
1. Wavetable: simple "Digital" tables (e.g. "M Splat") work best; noisier/harmonic tables get crazier.
2. Osc A pitched UP one octave because it's FM'd from a lower osc B (~2 octaves lower). FM amount envelope reversed: starts HIGH and falls on trigger (low→high flip of usual direction).
3. LFO1: instant transient spike then decay ("as soon as you shoot it's there") → envelope-shaped amount/timbre.
4. Very fast pitch bend envelope on master tune.
5. Noise osc used as "bed"; sub osc down; high-notch filter adds the "concussive" resonance on the low.
6. Serum FX: Distortion (adds "ring from the sub"), Flanger (small), Compressor with lows and mids boosted (heavy contribution), very short Delay (tunnel/short delay, ~"30") for stereo/space; Reverb off.
7. Macro → master tune + filter; stepping master tune ("3, 5 ...") and resampling yields ~10 different gun tonalities. Rate: ramp LFO at 1/16 (and another at 1/8).
Post (insert, pre-print):
8. OTT, light ("squashing a little bit, very little").
9. Phat rack (VR/ fat rack).
10. Disperser = stack of Ableton EQ Three ("just a ton of EQ3"); amount = number of copies; freq low/high knobs swept.
11. EQ boosting highs.
Sub (separate track): plain sine, down one octave, slight attack transient of mid-highs, pitch bend envelope (essential: "without it it's very static"), Distortion, Compressor with heavy upward + downward ("a lot of upward [and] lower"), lowpass to remove all highs; OTT + fat rack same as gun; Disperser turned off on sub; low-mids cut, small high-end lift.
Print (resample), then on printed audio:
12. EQ boosting low.
13. Valhalla Supermassive (free), "small wide" preset → more metallic + wider.
14. Ableton Hybrid Reverb at 100% "hybrid" (convolution+algo blend), size reduced, "small room" preset.
15. GClip (soft clipper).
16. Low cut. Second variant: Fresh Air (free exciter, very little), EQ out lows + some mids, GClip.
17. Sub: low-cut [sic, probably high-cut], light distortion, summed. Final gun printed again.
18. Mono-compatible; slight widening (Widener).
19. "Sub shifter" rack for rising variation; "super important for tearout".

**Techniques tags:** OTT, upward_comp, disperser/allpass, reverb_on_bass, convolver, softclip, resampling, layering, mono_low/sub_separate, sub_sidechain_or_split, pitch_env/gun_transient, lowmid_scoop, highcut/lowpass_after_dist, phaser/flanger, filter_env

## Quotes worth keeping
- "a gun whenever you shoot it's ... instantly the sound is there ... so the LFO reflects like a really quick transient and then kind of goes down"
- "disperser ... is just a ton of EQ3 ... you can duplicate EQ3 and then you can play with the amount"
- "I'm starting with a higher FM and then as soon as it triggers it goes lower"
- "the reason why I don't use a sub within the patch is because I need it to bend"
