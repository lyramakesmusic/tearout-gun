# Vy7hIbdZE4E — "Fl studio, NIMDA Tearout gun - Tutorial" — Silvyr (10:36)

FL + Serum (gun) + Vital (sub). 3 layers: tail/body, sub, transient. Good structure, few numbers.

## Body ("tail of the gun", Serum)
- Any **bass** wavetable, **-2 octaves**.
- LFO1 → **volume + filter** (cutoff, res, "frequency") + **distortion drive** (drive follows the LFO so it's "cleaner"; full static drive sounds worse in mix).
- LFO2 → **WT position**; LFO3 → **white noise** level (adds tone/brightness, aggression).
- FX: distortion → multiband comp → **comb filter** ("comb always works for tearout") → stereo effect.
- Insert chain: EQ (low cut) → OTT → distortion → small convolver/reverb → EQ → **multipass distortion** → **disperser** → reverb (decay/size very low, lows removed) → EQ → OTT → EQ → **Soothe** (remove resonances) → multipass distortion → EQ → wave shaper → EQ. "Remove low end, keep all the high end."

## Sub (Vital)
- Envelope → **pitch +15 st** at onset "to give a kick" ("if too high it sounds goofy").
- Little white noise; filter **band** to control crunch (Nimda "spams" this).
- Distortion; EQ low-mid cut (**linear phase** on subs); distortion with **bias 30%**; remove some high end (gun already has highs); **clipper**.
- **Grit layer** (optional): duplicate sub patch, band filter up, noise up, no pitch env, distortion **bias all the way down**, low cut, quiet.

## Transient
- **White noise only**, LFO → volume maxed — a short noise burst. Timing alignment with the gun "really important". EQ make loud → **Vocalex**? (captions; "make it zappy", params maxed/min) → EQ removing only sub. Don't let it drown the gun.

## Group
- Waveshaper + **Saturn** (multiband) for high end: distortion amounts: sub band 0?, **mid 50%**, **high 25%**, mix **50%**, linear phase.
- Master: FabFilter distortion on whole song **30%** linear phase.
