# /// script
# dependencies = ["numpy", "soundfile"]
# ///
"""Is PROCESSED a linear (gain-weighted) sum of stems? Least-squares gains + residual energy, per gun."""
import numpy as np, soundfile as sf
from pathlib import Path
R = Path("/Users/lyra/Music/samples/KFU GUN KIT VOL 2.1/KFU GUN KIT VOL 2.1 GUN SORT")
res = []
for g in sorted(R.iterdir()):
    if not g.is_dir(): continue
    get = lambda tag: [f for f in g.glob("*.wav") if f"_{tag}" in f.stem.upper()]
    P = get("PROCESSED"); parts = {t: get(t) for t in ["TR", "BODY", "SUB", "REV"]}
    if len(P) != 1 or any(len(v) != 1 for v in parts.values()): continue
    y = sf.read(P[0], always_2d=True)[0]
    X = [sf.read(v[0], always_2d=True)[0] for v in parts.values()]
    n = min([len(y)] + [len(x) for x in X])
    A = np.stack([x[:n].reshape(-1) for x in X], 1); b = y[:n].reshape(-1)
    w, *_ = np.linalg.lstsq(A, b, rcond=None)
    resid = b - A @ w
    plain = b - A.sum(1)
    db = lambda v: 10 * np.log10(np.sum(v ** 2) / np.sum(b ** 2) + 1e-20)
    res.append((g.name, db(resid), db(plain), w))
for name, r, p, w in res:
    print(f"{name:26s} lstsq resid {r:6.1f} dB  plain-sum resid {p:6.1f} dB  gains {np.round(w,2)}")
rr = np.array([r for _, r, _, _ in res]); print("median lstsq residual dB", np.median(rr).round(1))
