# /// script
# dependencies = ["numpy", "scipy", "soundfile"]
# ///
"""Robust sub-dive fit. Keep the monotone main branch of the zero-crossing track, then fit
   A) stretched-exp in log-pitch: st(t) = D * exp(-(t/tau)^k)
   B) power-law in Hz:           f = fe + (f0-fe)/(1+t/tau)^p
"""
import json, numpy as np, soundfile as sf
from pathlib import Path
from scipy.optimize import least_squares
ROOTS = [Path("/Users/lyra/Music/samples/KFU GUN KIT VOL 2.1/KFU GUN KIT VOL 2.1 GUN SORT"),
         Path("/Users/lyra/Music/samples/KFU BUG TOOLS/GUNS"),
         Path("/Users/lyra/Music/samples/KAIFU GUN KIT 1.6/KAIFU GUN KIT VOL 1.6")]

def track(f):
    x, sr = sf.read(f, always_2d=True); x = x.mean(1)
    a = np.abs(x); i0 = np.argmax(a > 0.02 * a.max()); x = x[i0:i0 + int(0.35 * sr)]
    zc = np.where((x[:-1] < 0) & (x[1:] >= 0))[0]
    zc = zc + (-x[zc]) / (x[zc + 1] - x[zc] + 1e-12)
    t = zc[1:] / sr; hz = sr / np.diff(zc)
    # settle = median of last 40% of points; walk backward keeping a monotone (non-increasing in time) branch
    keep = np.zeros(len(hz), bool); cur = np.inf
    # forward pass: accept if <= 1.15*cur (allow jitter) and not wildly below the tail
    tail = np.median(hz[int(0.6 * len(hz)):])
    for i in range(len(hz)):
        if hz[i] <= 1.15 * cur and hz[i] > 0.6 * tail:
            keep[i] = True; cur = hz[i]
    return t[keep], hz[keep], tail

out = []
for R in ROOTS:
    for f in sorted(R.rglob("*SUB*.wav")):
        if "TYPE" in str(f): continue
        t, hz, tail = track(f)
        if len(t) < 12: continue
        st = 12 * np.log2(hz / tail)
        # A
        ra = lambda p: p[0] * np.exp(-(t / p[1]) ** p[2]) - st
        A = least_squares(ra, [max(st[0], 1), 0.01, 0.7], bounds=([0, 1e-4, 0.2], [120, 1, 3]), loss="soft_l1", f_scale=2)
        # B
        def rb(p):
            f0, fe, tau, pw = p
            return 12 * np.log2(np.maximum(fe + (f0 - fe) / (1 + t / tau) ** pw, 1) / tail) - st
        B = least_squares(rb, [np.clip(hz[0], 30, 19000), np.clip(tail, 16, 290), 0.003, 1.0],
                          bounds=([20, 15, 1e-5, 0.2], [20000, 300, 0.5, 6]), loss="soft_l1", f_scale=2)
        e = lambda r: float(np.median(np.abs(r)))
        out.append(dict(file=f.name, n=len(t), start_hz=float(hz[0]), end_hz=float(tail),
                        dive_st=float(st[0]), A=A.x.tolist(), A_err=e(ra(A.x)), B=B.x.tolist(), B_err=e(rb(B.x))))
json.dump(out, open("sub_fits2.json", "w"), indent=1)
def q(k, f=lambda r, k: r[k]):
    v = sorted(f(r, k) for r in out); g = lambda a: v[int(a * (len(v) - 1))]
    return f"p10={g(.1):9.3f} p50={g(.5):9.3f} p90={g(.9):9.3f}"
print("N", len(out))
for k in ["start_hz", "end_hz", "dive_st", "A_err", "B_err"]: print(f"{k:9s}", q(k))
print("A.D   ", q(0, lambda r, k: r["A"][0]))
print("A.tau_ms", q(1, lambda r, k: 1000 * r["A"][1]))
print("A.k   ", q(2, lambda r, k: r["A"][2]))
