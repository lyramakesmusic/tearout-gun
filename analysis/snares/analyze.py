# /// script
# dependencies = ["numpy", "scipy", "soundfile", "matplotlib"]
# ///
import glob, json, re, sys, os
import numpy as np, soundfile as sf
from scipy.signal import stft, butter, sosfiltfilt
ROOT = os.path.expanduser("~/Music/samples")
files = sorted(glob.glob(sys.argv[1] if len(sys.argv) > 1 else f"{ROOT}/CamaCon Snaretober 202*/*.wav"))
NOTES = {n: i for i, n in enumerate("C C# D D# E F F# G G# A A# B".split())}
NOTES.update({"Db": 1, "Eb": 3, "Gb": 6, "Ab": 8, "Bb": 10})
BANDS = [(20, 150), (150, 500), (500, 2000), (2000, 6000), (6000, 18000)]
def note_of(name):
    m = re.search(r" - ([A-G][b#]?)(min)?(-[A-G])?\.wav$", name)
    return NOTES[m.group(1)] if m else None
def env_db(x, sr, hop_ms=1):
    h = int(sr * hop_ms / 1000); n = len(x) // h
    e = np.sqrt(np.mean(x[: n * h].reshape(n, h) ** 2, 1) + 1e-12)
    return 20 * np.log10(e)
rows = []
for f in files:
    x, sr = sf.read(f, always_2d=True); x = x.mean(1)
    if len(x) < sr * 0.02: continue
    pk = np.max(np.abs(x)); x = x / (pk + 1e-9)
    e = env_db(x, sr); ep = e.max(); ipk = int(e.argmax())
    above = np.where(e > ep - 40)[0]; L40 = above[-1] if len(above) else 0
    above20 = np.where(e > ep - 20)[0]; L20 = above20[-1]
    r = {"file": os.path.basename(f), "year": (f.split("Snaretober ")+["x"])[1][:4], "len40": int(L40), "len20": int(L20), "peak_ms": ipk}
    # band envelopes
    bands = {}
    for lo, hi in BANDS:
        sos = butter(4, [lo, min(hi, sr / 2 - 100)], btype="band", fs=sr, output="sos")
        eb = env_db(sosfiltfilt(sos, x), sr)
        bp = eb.max(); ib = int(eb.argmax())
        a20 = np.where(eb[ib:] > bp - 20)[0]; d20 = int(a20[-1]) if len(a20) else 0
        bands[f"{lo}"] = {"lvl": round(float(bp), 1), "peak_ms": ib, "dec20_ms": d20}
    r["bands"] = bands
    # tonal peak track: strongest bin 80-1200 Hz per 5ms frame (4096 fft zero-padded)
    nfft = 8192; win = int(sr * 0.025); hop = int(sr * 0.005)
    fr, tt, Z = stft(x, sr, nperseg=win, noverlap=win - hop, nfft=nfft)
    M = np.abs(Z); band = (fr > 80) & (fr < 1200)
    track = fr[band][M[band].argmax(0)]
    tonal = M[band].max(0) / (np.median(M[band], 0) + 1e-9)
    def at(ms):
        i = min(int(ms / 5), len(track) - 1); return float(track[i]), float(tonal[i])
    r["f0_track"] = {ms: at(ms) for ms in (5, 15, 30, 60, 100, 150)}
    # spectral flatness 2k-12k at 40-80ms (noise-ness of the wash)
    hb = (fr > 2000) & (fr < 12000); seg = slice(8, 16)
    Ms = M[hb][:, seg] + 1e-9
    r["flat_hi"] = float(np.exp(np.mean(np.log(Ms))) / np.mean(Ms))
    cen = (fr[:, None] * M ** 2).sum(0) / ((M ** 2).sum(0) + 1e-12)
    r["centroid"] = {ms: float(cen[min(int(ms / 5), len(cen) - 1)]) for ms in (5, 20, 60, 120)}
    n = note_of(r["file"]); r["note"] = n
    rows.append(r)
json.dump(rows, open(sys.argv[2] if len(sys.argv) > 2 else "snaretober.json", "w"), indent=0)
A = lambda k: np.array([r[k] for r in rows])
print(len(rows), "snares")
for k in ("len40", "len20", "peak_ms"): print(k, np.percentile(A(k), [10, 25, 50, 75, 90]).round())
for lo, _ in BANDS:
    lv = np.array([r["bands"][str(lo)]["lvl"] for r in rows]); pk = np.array([r["bands"][str(lo)]["peak_ms"] for r in rows]); d = np.array([r["bands"][str(lo)]["dec20_ms"] for r in rows])
    print(f"band {lo:>5}: lvl {np.percentile(lv,[25,50,75]).round(1)}  peak_ms {np.percentile(pk,[25,50,75]).round()}  dec20 {np.percentile(d,[25,50,75]).round()}")
for ms in (5, 15, 30, 60, 100, 150):
    f0 = np.array([r["f0_track"][ms][0] for r in rows]); tn = np.array([r["f0_track"][ms][1] for r in rows])
    print(f"f0 @{ms}ms: {np.percentile(f0,[25,50,75]).round()} tonalness {np.percentile(tn,[25,50,75]).round(1)}")
print("flat_hi", np.percentile(A("flat_hi"), [25, 50, 75]).round(2))
for ms in (5, 20, 60, 120):
    c = np.array([r["centroid"][ms] for r in rows]); print(f"centroid @{ms}: {np.percentile(c,[25,50,75]).round()}")
# f0 vs labeled note: semitone offset mod 12 at 60ms
offs = []
for r in rows:
    if r["note"] is None: continue
    f = r["f0_track"][60][0]; st = 12 * np.log2(f / 16.3516); offs.append(round((st - r["note"]) % 12))
print("f0@60 minus labeled note (semitones mod 12) histogram:", np.bincount(np.array(offs) % 12, minlength=12))
