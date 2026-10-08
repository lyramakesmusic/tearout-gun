# /// script
# dependencies = ["numpy", "soundfile", "matplotlib", "scipy"]
# ///
import numpy as np, soundfile as sf, matplotlib; matplotlib.use("Agg")
import matplotlib.pyplot as plt
from pathlib import Path
from scipy import signal
R = Path("/Users/lyra/Music/samples/KFU GUN KIT VOL 2.1/KFU GUN KIT VOL 2.1 GUN SORT")
fs = sorted(R.rglob("*SUB*.wav"))[:24:2]
fig, ax = plt.subplots(4, 6, figsize=(24, 12))
for i, f in enumerate(fs):
    x, sr = sf.read(f, always_2d=True); x = x.mean(1)
    a = np.abs(x); i0 = np.argmax(a > 0.02 * a.max()); x = x[i0:i0 + int(0.3 * sr)]
    zc = np.where((x[:-1] < 0) & (x[1:] >= 0))[0]
    zc = zc + (-x[zc]) / (x[zc + 1] - x[zc] + 1e-12)
    ax[0 if i < 6 else 2, i % 6].semilogy(1000 * zc[1:] / sr, sr / np.diff(zc), ".", ms=3)
    ax[0 if i < 6 else 2, i % 6].set_title(f.stem.replace("KFU_GUN_", ""), fontsize=9)
    ax[0 if i < 6 else 2, i % 6].set_ylim(20, 20000)
    fq, tt, S = signal.spectrogram(x, sr, nperseg=256, noverlap=224)
    ax[1 if i < 6 else 3, i % 6].pcolormesh(1000 * tt, fq, 10 * np.log10(S + 1e-12), vmin=-120, shading="auto")
    ax[1 if i < 6 else 3, i % 6].set_yscale("symlog", linthresh=100); ax[1 if i < 6 else 3, i % 6].set_ylim(20, 20000)
plt.tight_layout(); plt.savefig("subs.png", dpi=60)
