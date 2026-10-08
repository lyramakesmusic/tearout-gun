# /// script
# dependencies = ["numpy", "soundfile", "matplotlib", "scipy"]
# ///
import sys, numpy as np, soundfile as sf, matplotlib; matplotlib.use("Agg")
import matplotlib.pyplot as plt
from scipy import signal
fs = sys.argv[2:]
fig, ax = plt.subplots(len(fs), 1, figsize=(16, 2.8 * len(fs)), squeeze=False)
for a, f in zip(ax[:, 0], fs):
    x, sr = sf.read(f, always_2d=True); x = x.mean(1)
    for lo, hi, col, lab in [(30, 120, "#888", "sub 30–120"), (300, 3000, "#ff7a1a", "mids 0.3–3k"), (3000, 16000, "#59a8ff", "highs 3–16k")]:
        sos = signal.butter(4, [lo, min(hi, sr / 2 - 100)], "bandpass", fs=sr, output="sos"); y = signal.sosfilt(sos, x)
        w = int(0.003 * sr); e = np.sqrt(np.convolve(y ** 2, np.ones(w) / w, "same")); t = np.arange(len(e)) / sr * 1000
        a.plot(t, 20 * np.log10(e / e.max() + 1e-6), color=col, lw=1.2, label=lab)
    a.set_ylim(-40, 2); a.set_title(f.split("/")[-1], fontsize=10); a.legend(fontsize=8, loc="upper right"); a.grid(alpha=.2)
plt.tight_layout(); plt.savefig(sys.argv[1], dpi=60)
