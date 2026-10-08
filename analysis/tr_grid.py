# /// script
# dependencies = ["numpy", "soundfile", "matplotlib", "scipy"]
# ///
"""Fine-time spectrograms of reference transient stems, first 160 ms."""
import sys, numpy as np, soundfile as sf, matplotlib; matplotlib.use("Agg")
import matplotlib.pyplot as plt
from pathlib import Path
from scipy import signal
R = Path("/Users/lyra/Music/samples/KFU GUN KIT VOL 2.1/KFU GUN KIT VOL 2.1 GUN SORT")
fs = sorted(f for f in R.rglob("*_TR*.wav") if "TYPE" not in str(f))[: int(sys.argv[2]) if len(sys.argv) > 2 else 24]
cols = 6; rows = (len(fs) + cols - 1) // cols
fig, ax = plt.subplots(rows, cols, figsize=(4 * cols, 2.6 * rows))
for a, f in zip(ax.flat, fs):
    x, sr = sf.read(f, always_2d=True); x = x.mean(1)
    i0 = np.argmax(np.abs(x) > 0.01 * np.abs(x).max()); x = x[i0: i0 + int(0.16 * sr)]
    fq, tt, S = signal.spectrogram(x, sr, nperseg=256, noverlap=240)
    a.pcolormesh(1000 * tt, fq, 10 * np.log10(S + 1e-14), vmin=-110, vmax=-25, shading="auto")
    a.set_yscale("symlog", linthresh=500); a.set_ylim(100, 20000)
    a.set_title(f.stem.replace("KFU_GUN_", "").replace("_TR", ""), fontsize=8); a.tick_params(labelsize=6)
for a in ax.flat[len(fs):]: a.axis("off")
plt.tight_layout(); plt.savefig(sys.argv[1], dpi=55)
