# /// script
# dependencies = ["numpy", "scipy", "soundfile"]
# ///
# Snare one-shot corpus: dedupe, keep single hits 60 ms–1.5 s, per-file features, write corpus.json
import hashlib, json, os, sys, numpy as np, soundfile as sf
from concurrent.futures import ProcessPoolExecutor
from scipy.signal import butter, sosfiltfilt
ROOT = os.path.expanduser("~/Music/samples/")
BANDS = [(20, 150), (150, 500), (500, 2000), (2000, 6000), (6000, 18000)]
def one(p):
    try:
        info = sf.info(p)
        if not (0.06 <= info.duration <= 3.0): return None
        x, sr = sf.read(p, always_2d=True); x = x.mean(1)
        if sr < 22050: return None
        h = hashlib.md5(np.round(x[: sr // 2] * 1e4).astype(np.int32).tobytes()).hexdigest()
        pk = np.abs(x).max()
        if pk < 1e-4: return None
        x = x / pk; hop = int(sr * 0.002); n = len(x) // hop
        e = 10 * np.log10((x[: n * hop].reshape(n, hop) ** 2).mean(1) + 1e-12); ep = e.max()
        on = int(np.argmax(e > ep - 30)); ipk = int(e.argmax())
        # single hit: after the first decay below -20 dB, nothing climbs back above -8 dB
        after = e[ipk:]; dn = np.where(after < ep - 20)[0]
        if len(dn) and (after[dn[0]:] > ep - 8).any(): return None
        end = np.where(e > ep - 40)[0][-1]; L40 = (end - on) * 2
        if L40 > 1500 or L40 < 40: return None
        if (ipk - on) * 2 > 60: return None  # starts with a swell: not a snare one-shot
        f = {"len40": int(L40), "peak_ms": int((ipk - on) * 2)}
        x2 = x[on * hop:]
        tot = (x2 ** 2).sum()
        for lo, hi in BANDS:
            sos = butter(4, [lo, min(hi, sr / 2 - 200)], btype="band", fs=sr, output="sos"); y = sosfiltfilt(sos, x2)
            f[f"b{lo}"] = float(10 * np.log10((y ** 2).sum() / tot + 1e-12))
        return {"path": p, "hash": h, "sr": sr, "f": f}
    except Exception as ex:
        return None
if __name__ == "__main__":
    paths = [ROOT + l.strip()[2:] for l in open(sys.argv[1])]
    with ProcessPoolExecutor(12) as ex: rows = [r for r in ex.map(one, paths, chunksize=16) if r]
    seen, out = set(), []
    for r in rows:
        if r["hash"] in seen: continue
        seen.add(r["hash"]); out.append(r)
    print(len(paths), "->", len(rows), "single hits ->", len(out), "unique")
    json.dump(out, open(os.path.expanduser("~/tearout-gun/analysis/snares/corpus.json"), "w"))
    F = np.array([[r["f"][k] for k in ["len40", "peak_ms", "b20", "b150", "b500", "b2000", "b6000"]] for r in out])
    print("median len40 peak b20 b150 b500 b2k b6k:", np.median(F, 0).round(1))
    print("p10:", np.percentile(F, 10, 0).round(1)); print("p90:", np.percentile(F, 90, 0).round(1))
