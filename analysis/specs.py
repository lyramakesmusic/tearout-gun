# /// script
# dependencies = ["numpy", "soundfile", "matplotlib", "scipy"]
# ///
"""Side-by-side spectrograms (+ RMS) of arbitrary wavs. usage: specs.py out.png a.wav b.wav ..."""
import sys, numpy as np, soundfile as sf, matplotlib; matplotlib.use("Agg")
import matplotlib.pyplot as plt
from scipy import signal
out, fs = sys.argv[1], sys.argv[2:]
fig, ax = plt.subplots(1, len(fs), figsize=(5 * len(fs), 3.6), squeeze=False)
for a, f in zip(ax[0], fs):
    x, sr = sf.read(f, always_2d=True); x = x.mean(1)[: int(0.7 * sr)]
    fq, tt, S = signal.spectrogram(x, sr, nperseg=1024, noverlap=896)
    a.pcolormesh(1000 * tt, fq, 10 * np.log10(S + 1e-14), vmin=-110, vmax=-20, shading="auto")
    a.set_yscale("symlog", linthresh=200); a.set_ylim(30, 20000)
    a.set_title(f"{f.split('/')[-1][:34]} rms={20*np.log10(np.sqrt(np.mean(x**2))):.1f}", fontsize=9)
plt.tight_layout(); plt.savefig(out, dpi=60)
