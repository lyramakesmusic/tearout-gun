# /// script
# dependencies = ["numpy", "soundfile", "matplotlib", "scipy"]
# ///
import numpy as np, soundfile as sf, matplotlib, sys; matplotlib.use("Agg")
import matplotlib.pyplot as plt
from pathlib import Path
from scipy import signal
R = Path("/Users/lyra/Music/samples/KFU GUN KIT VOL 2.1/KFU GUN KIT VOL 2.1 GUN SORT")
guns = sys.argv[1:]
roles = ["TR", "BODY", "SUB", "REV", "PROCESSED"]
fig, ax = plt.subplots(len(guns), 5, figsize=(25, 3.2 * len(guns)))
for gi, g in enumerate(guns):
    for ri, r in enumerate(roles):
        fs = [f for f in (R / g).glob("*.wav") if f"_{r}" in f.stem.upper()]
        if not fs: continue
        x, sr = sf.read(fs[0], always_2d=True); x = x.mean(1)[: int(0.6 * sr)]
        fq, tt, S = signal.spectrogram(x, sr, nperseg=1024, noverlap=896)
        a = ax[gi, ri]
        a.pcolormesh(1000 * tt, fq, 10 * np.log10(S + 1e-14), vmin=-110, vmax=-20, shading="auto")
        a.set_yscale("symlog", linthresh=200); a.set_ylim(30, 20000)
        a.set_title(f"{g.replace('KFU_GUN_','')} {r}  rms={20*np.log10(np.sqrt(np.mean(x**2))+1e-9):.0f}dB", fontsize=9)
plt.tight_layout(); plt.savefig("layers.png", dpi=55)
