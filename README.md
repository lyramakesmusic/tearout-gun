# Tearout Gun Synth

A browser synth for tearout guns, snares, chugs and pings. Everything renders offline in Web Workers, so randomize, mutate, stems and reference fitting all run on the page with no server.

**[Open it](https://lyramakesmusic.github.io/tearout-gun/)**

## Using it

- **family** picks what randomize makes: `fitted` and `gun` roll variations of guns fitted to real gun samples; `snare` rolls variations of ~150 snares fitted from a sample library; `chug` and `ping` have their own recipes; `808` and `whoosh` have their own engines (808: sub, FM, noise, post; whoosh: noise through moving filters for whooshes, swishes, crashes, wind, hype loops, impacts and foley).
- **randomize** rolls a new sound; **mutate** nudges the current one. Every group has its own **roll** and **lock**.
- Drop a file on **+ layer** to stack a sample; drop one on **reference** to A/B it, see its tonal balance against yours, and **fit** the current engine to it.
- Click a stem chip to solo it; drag it (or the main wav) into a DAW.
- **save** keeps patches in this browser.

## Engines

- **Gun** (`dsp/engine.js`): sub with pitch dive, noise transient, noisy/modal/FM body, synth layer, spice, samples → crunch chain (disperser, convolver, drive, OTT rack, freq shift) → mids chain with a clean sub path → reverb from synthesized IRs.
- **Snare** (`dsp/snare.js`, `dsp/snare_spec.js`): click, tuned tone, noise with its own drive and crunchy noise types, metal, clap, room → width (symmetric, tail only) → bus with whip formant, OTT, snap and clip.
- **808** (`dsp/e808.js`): sine sub with a fast pitch dive and slow drift, FM with its own decay, noise click → saturation (tanh/hard/fold/tube) → tone → optional clean sub below a crossover → OTT, EQ, clip.
- **Whoosh** (`dsp/whoosh.js`): white/pink/brown/crackle/cymbal-metal source → curved amp envelope → filter sweeping start → peak → end with wobble → rhythmic chop → low body swell → symmetric width and pass-by → reverb, optional reverse → post.

## How the sounds are made

Presets and randomize anchors come from fitting the engines to real sounds: staged CMA-ES on a multi-resolution log-mel + long-term-spectrum + third-octave loss (`fit/`, `dsp/fitstages.js`). Randomize picks a fitted sound, jitters it, and keeps the best of several candidates by a family reference. Design notes from tutorial research are in `research/*/SYNTHESIS.md`; the experiment narrative is in `research_log.md`. `data/preset_map.json` maps preset names to the gun each was fitted to.

## Running locally

```
node serve.js   # http://localhost:8642/web/
```

Any static server works; the page imports the engine from `../dsp/`.
