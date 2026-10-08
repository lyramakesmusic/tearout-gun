# /// script
# dependencies = ["numpy", "soundfile", "scipy"]
# ///
import glob, os, json, numpy as np, soundfile as sf
from scipy.signal import butter, sosfiltfilt
from scipy.ndimage import uniform_filter1d
def load(f):
    x, sr = sf.read(f, always_2d=True); x = x.mean(1)
    if sr != 48000: x = np.interp(np.arange(int(len(x) * 48000 / sr)) * sr / 48000, np.arange(len(x)), x)
    pk = np.abs(x).max(); on = int(np.argmax(np.abs(x) > 0.05 * pk)); return x[on:] / (pk + 1e-9)
def fine(x):
    seg = x[int(0.02 * 48000): int(0.12 * 48000)]; seg = np.pad(seg, (0, max(0, 4800 - len(seg))))
    S = 20 * np.log10(np.abs(np.fft.rfft(seg * np.hanning(len(seg)), 16384)) + 1e-9); f = np.fft.rfftfreq(16384, 1 / 48000)
    lf = np.log(f[1:]); grid = np.linspace(np.log(300), np.log(10000), 600); Sg = np.interp(grid, lf, S[1:])
    sm = uniform_filter1d(Sg, 20)  # ~1/3 octave smoothing on log axis
    r = Sg - sm; return r.std(), np.percentile(r, 99) - np.median(r)  # fine-structure spread, prominence of strongest peaks
def rattle(x):
    y = sosfiltfilt(butter(4, [2000, 8000], "band", fs=48000, output="sos"), x)[int(0.03 * 48000): int(0.15 * 48000)]
    env = np.abs(y); env = uniform_filter1d(env, 48)  # 1 ms
    slow = uniform_filter1d(env, 960); m = env / (slow + 1e-9) - 1  # modulation around a 20 ms trend
    return m.std()
fits = sorted(glob.glob(os.path.expanduser("~/tearout-gun/out/sfits/s[0-9][0-9][0-9].json")))
rows = []
for f in fits:
    t = json.load(open(f))["file"]
    try: a, b = load(t), load(f.replace(".json", ".wav"))
    except Exception: continue
    rows.append((*fine(a), rattle(a), *fine(b), rattle(b)))
R = np.array(rows)
names = ["fine-structure std dB", "peak prominence dB", "2–8k rattle depth"]
for i, n in enumerate(names): print(f"{n:22} real median {np.median(R[:, i]):.2f}  [{np.percentile(R[:, i],25):.2f}–{np.percentile(R[:, i],75):.2f}]   fit median {np.median(R[:, i+3]):.2f}  [{np.percentile(R[:, i+3],25):.2f}–{np.percentile(R[:, i+3],75):.2f}]")
# reference: pure white noise burst with the same envelope as itself
w = np.random.default_rng(0).standard_normal(48000) * np.exp(-np.arange(48000) / 4800); print("white-noise burst:", np.round([*fine(w), rattle(w)], 2))
