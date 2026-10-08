# /// script
# dependencies = ["numpy", "scipy", "soundfile"]
# ///
"""Zero-crossing pitch track of every SUB stem + fit f(t) = fe + (f0-fe) / (1 + t/tau)^p."""
import json, numpy as np, soundfile as sf
from pathlib import Path
from scipy.optimize import least_squares
ROOTS = [Path("/Users/lyra/Music/samples/KFU GUN KIT VOL 2.1/KFU GUN KIT VOL 2.1 GUN SORT"),
         Path("/Users/lyra/Music/samples/KFU BUG TOOLS/GUNS"),
         Path("/Users/lyra/Music/samples/KAIFU GUN KIT 1.6/KAIFU GUN KIT VOL 1.6")]

def track(f):
    x, sr = sf.read(f, always_2d=True); x = x.mean(1)
    a = np.abs(x); i0 = np.argmax(a > 0.02 * a.max()); x = x[i0:]
    zc = np.where((x[:-1] < 0) & (x[1:] >= 0))[0]
    zc = zc + (-x[zc]) / (x[zc + 1] - x[zc] + 1e-12)
    t = zc[1:] / sr; hz = sr / np.diff(zc)
    m = t < 0.4
    return t[m], hz[m]

def model(p, t):
    f0, fe, tau, pw = p
    return fe + (f0 - fe) / (1 + t / tau) ** pw

rows = []
for R in ROOTS:
    for f in sorted(R.rglob("*SUB*.wav")):
        if "TYPESORT" in str(f) or "TYPE SORT" in str(f): continue
        t, hz = track(f)
        if len(t) < 10: continue
        lt = np.log(hz)
        res = lambda p: np.log(np.maximum(model(p, t), 1)) - lt
        x0 = [np.clip(hz[:3].mean(), 21, 19999), np.clip(np.median(hz[-20:]), 16, 299), 0.003, 1.0]
        fit = least_squares(res, x0,
                            bounds=([20, 15, 1e-5, 0.2], [20000, 300, 0.5, 6]), loss="soft_l1")
        f0, fe, tau, pw = fit.x
        # time to reach within 1 semitone of fe, from the fit
        tt = np.linspace(0, 0.4, 4001); ff = model(fit.x, tt)
        t1 = tt[np.argmax(12 * np.log2(ff / fe) < 1)]
        t_half = tt[np.argmax(ff < np.sqrt(f0 * fe))]  # half-way in log-pitch
        rows.append(dict(file=f.name, f0=f0, fe=fe, tau_ms=1000 * tau, p=pw,
                         rms_st=float(np.sqrt(np.mean((12 / np.log(2) * res(fit.x)) ** 2))),
                         t_logmid_ms=1000 * t_half, t_settle_ms=1000 * t1,
                         first_hz=float(hz[:3].mean())))
json.dump(rows, open("sub_fits.json", "w"), indent=1)
import statistics as st
for k in ["first_hz", "f0", "fe", "tau_ms", "p", "t_logmid_ms", "t_settle_ms", "rms_st"]:
    v = sorted(r[k] for r in rows)
    q = lambda a: v[int(a * (len(v) - 1))]
    print(f"{k:12s} p10={q(.1):9.2f}  p50={q(.5):9.2f}  p90={q(.9):9.2f}")
print(len(rows), "subs")
