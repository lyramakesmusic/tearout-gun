# /// script
# dependencies = ["numpy", "soundfile"]
# ///
"""Cross-check sub pitch via zero-crossing periods (independent of Hilbert)."""
import numpy as np, soundfile as sf, sys
from pathlib import Path
R = Path("/Users/lyra/Music/samples/KFU GUN KIT VOL 2.1/KFU GUN KIT VOL 2.1 GUN SORT")
for g in sys.argv[1:]:
    f = next((R / g).glob("*SUB*.wav"))
    x, sr = sf.read(f, always_2d=True); x = x.mean(1)
    i0 = np.argmax(np.abs(x) > 0.01 * np.abs(x).max())
    x = x[i0:]
    zc = np.where((x[:-1] < 0) & (x[1:] >= 0))[0]
    zc = zc + (-x[zc]) / (x[zc + 1] - x[zc])  # linear interp crossing
    per = np.diff(zc)
    t_ms = 1000 * zc[1:] / sr
    hz = sr / per
    pick = [0, 1, 2, 3, 5, 8, 12, 16, 20, 25, 30, 35, 40]
    print(g, " | ".join(f"{t_ms[i]:.0f}ms:{hz[i]:.0f}Hz" for i in pick if i < len(hz)))
