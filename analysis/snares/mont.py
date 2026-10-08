# /// script
# dependencies = ["numpy", "scipy", "soundfile", "matplotlib"]
# ///
import glob, os, sys, numpy as np, soundfile as sf, matplotlib; matplotlib.use("Agg")
import matplotlib.pyplot as plt
from scipy.signal import stft
fs = sorted(glob.glob(sys.argv[1])); cols = int(os.environ.get("COLS", 7)); rows = (len(fs) + cols - 1) // cols
fig, ax = plt.subplots(rows, cols, figsize=(cols * 3.4, rows * 2.6), squeeze=False)
for a in ax.flat: a.axis("off")
for a, f in zip(ax.flat, fs):
    x, sr = sf.read(f, always_2d=True); x = x.mean(1)[: int(sr * float(os.environ.get("SECS", 0.4)))]
    fr, t, Z = stft(x, sr, nperseg=1024, noverlap=1024 - 128); S = 20 * np.log10(np.abs(Z) + 1e-7); S -= S.max()
    a.axis("on"); a.pcolormesh(t * 1000, fr, S, vmin=-70, vmax=0, shading="auto", cmap="magma"); a.set_yscale("symlog", linthresh=500); a.set_ylim(40, 20000); a.set_title(os.path.basename(f)[:40], fontsize=8)
plt.tight_layout(); plt.savefig(sys.argv[2], dpi=50)
