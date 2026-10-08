# /// script
# dependencies = ["numpy", "scipy", "soundfile", "matplotlib"]
# ///
import os, numpy as np, soundfile as sf, matplotlib; matplotlib.use("Agg")
import matplotlib.pyplot as plt
from scipy.signal import stft
R = os.path.expanduser("~/Music/samples/CamaCon Snaretober 2023 Pack/")
fs = ["Layers/camaSnare - ST23 - Day 6 Pre-transient.wav", "Layers/camaSnare - ST23 - Day 6 Snare - G.wav", "camaSnare - ST23 - Day 6 - G-D.wav",
      "Layers/camaSnare - ST23 - Day 13 Pre-transient.wav", "Layers/camaSnare - ST23 - Day 13 Snare - C#.wav", "camaSnare - ST23 - Day 13 - C#.wav"]
fig, ax = plt.subplots(2, 3, figsize=(18, 8))
for a, f in zip(ax.flat, fs):
    x, sr = sf.read(R + f, always_2d=True); x = x.mean(1)[: int(sr * 0.5)]
    fr, t, Z = stft(x, sr, nperseg=1024, noverlap=1024 - 64); S = 20 * np.log10(np.abs(Z) + 1e-7); S -= S.max()
    a.pcolormesh(t * 1000, fr, S, vmin=-70, vmax=0, shading="auto", cmap="magma"); a.set_yscale("symlog", linthresh=500); a.set_ylim(40, 20000); a.set_title(f.split("/")[-1][13:], fontsize=9)
    print(f, len(x) / sr)
plt.tight_layout(); plt.savefig("pre.png", dpi=60)
