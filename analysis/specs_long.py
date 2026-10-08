# /// script
# dependencies = ["numpy", "soundfile", "matplotlib", "scipy"]
# ///
import sys, numpy as np, soundfile as sf, matplotlib; matplotlib.use("Agg")
import matplotlib.pyplot as plt
from scipy import signal
out, fs = sys.argv[1], sys.argv[2:]
fig, ax = plt.subplots(len(fs), 1, figsize=(18, 3.2 * len(fs)), squeeze=False)
for a, f in zip(ax[:, 0], fs):
    x, sr = sf.read(f, always_2d=True); x = x.mean(1)
    fq, tt, S = signal.spectrogram(x, sr, nperseg=1024, noverlap=896)
    a.pcolormesh(1000 * tt, fq, 10 * np.log10(S + 1e-14), vmin=-110, vmax=-20, shading="auto")
    a.set_yscale("symlog", linthresh=200); a.set_ylim(30, 20000); a.set_title(f.split("/")[-1], fontsize=10)
plt.tight_layout(); plt.savefig(out, dpi=55)
