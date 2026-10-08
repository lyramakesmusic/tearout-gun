# /// script
# dependencies = ["numpy", "scipy", "soundfile", "matplotlib"]
# ///
import glob, os, re, numpy as np, soundfile as sf, matplotlib; matplotlib.use("Agg")
import matplotlib.pyplot as plt
from scipy.signal import stft
R = os.path.expanduser("~/Music/samples")
groups = {}
for f in sorted(glob.glob(f"{R}/CamaCon Snaretober 202*/Layers/*.wav")):
    b = os.path.basename(f); m = re.match(r"(.*?(?:Day \d+|\d+\.)[^A-Z]*?)\s", b)
    key = re.sub(r"(ST2\d - (?:Day )?\d+).*", r"\1", b); groups.setdefault(key, []).append(f)
def env(x, sr):
    h = int(sr/1000); n = len(x)//h; return 20*np.log10(np.sqrt((x[:n*h].reshape(n,h)**2).mean(1))+1e-9)
keep = [k for k in groups if not re.search(r"Pan|Chord", " ".join(groups[k]))]
for k in keep:
    print("==", k)
    for f in groups[k]:
        x, sr = sf.read(f, always_2d=True); x = x.mean(1); e = env(x, sr)
        on = int(np.argmax(e > e.max()-30)); pk = int(e.argmax()); end = int(np.where(e > e.max()-40)[0][-1])
        print(f"   {os.path.basename(f)[-48:]:>48}  peak {20*np.log10(np.abs(x).max()+1e-9):6.1f}dB  onset {on:4d}ms  peak@{pk:4d}ms  end40 {end:5d}ms")
# montage: main snares
mains = sorted(glob.glob(f"{R}/CamaCon Snaretober 202*/*.wav"))
rng = np.random.default_rng(0); pick = rng.choice(len(mains), 24, replace=False)
fig, ax = plt.subplots(4, 6, figsize=(24, 13))
for a, i in zip(ax.flat, pick):
    x, sr = sf.read(mains[i], always_2d=True); x = x.mean(1)[: int(sr*0.4)]
    f, t, Z = stft(x, sr, nperseg=1024, noverlap=1024-128)
    S = 20*np.log10(np.abs(Z)+1e-7); S -= S.max()
    a.pcolormesh(t*1000, f, S, vmin=-70, vmax=0, shading="auto", cmap="magma"); a.set_yscale("symlog", linthresh=500); a.set_ylim(40, 20000)
    a.set_title(os.path.basename(mains[i])[10:52], fontsize=8)
plt.tight_layout(); plt.savefig("montage.png", dpi=55)
