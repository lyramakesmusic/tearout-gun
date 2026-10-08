# /// script
# dependencies = ["numpy", "scipy", "soundfile", "matplotlib", "scikit-learn"]
# ///
"""Per-octave-band envelope (ADSR-ish) analysis across KFU guns and layers, plus gun-type clustering.

For each stem: STFT (1024, hop 128 = 2.7 ms) -> octave-band energy envelopes (dB).
Per band: peak level (rel. stem peak), attack (onset->peak), t-6 (hold-ish), t-20 (decay), gate slope
(dB lost in the 15 ms before the band falls below -40), plus the full normalized envelope for plotting.
"""
import json
import numpy as np, soundfile as sf, matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt
from pathlib import Path
from scipy import signal
from sklearn.cluster import KMeans
from sklearn.preprocessing import StandardScaler

R = Path("/Users/lyra/Music/samples/KFU GUN KIT VOL 2.1/KFU GUN KIT VOL 2.1 GUN SORT")
OUT = Path(__file__).parent
EDGES = [31.25 * 2 ** k for k in range(10)]  # 31..16k: 9 octave bands
BANDS = [f"{int(EDGES[i])}-{int(EDGES[i+1])}" for i in range(9)]
HOP, NFFT, TMAX = 128, 1024, 0.8
ROLES = {"PROCESSED": "PROC", "_SUB": "SUB", "_TR": "TR", "_BODY": "BODY", "_REV": "REV"}


def band_env(x, sr):
    f, t, Z = signal.stft(x, sr, nperseg=NFFT, noverlap=NFFT - HOP, boundary=None)
    P = np.abs(Z) ** 2
    E = np.stack([P[(f >= EDGES[i]) & (f < EDGES[i + 1])].sum(0) for i in range(9)])
    E = 10 * np.log10(E + 1e-14)
    nt = int(TMAX * sr / HOP)
    if E.shape[1] < nt: E = np.pad(E, ((0, 0), (0, nt - E.shape[1])), constant_values=-140)
    return E[:, :nt], HOP / sr


def feats(E, dt):
    gmax = E.max()
    rows = []
    for b in range(E.shape[0]):
        e = E[b]; pk = e.max(); ip = int(e.argmax())
        above = np.where(e > pk - 30)[0]; on = above[0] if len(above) else ip
        def t_below(th):
            idx = np.where(e[ip:] < pk + th)[0]
            return (idx[0] if len(idx) else len(e) - ip) * dt * 1000
        t40 = t_below(-40)
        end = ip + int(t40 / 1000 / dt)
        k = max(ip, end - int(0.015 / dt))
        gate = (e[k] - e[min(end, len(e) - 1)]) if end < len(e) else 0.0
        rows.append(dict(level=pk - gmax, attack=(ip - on) * dt * 1000, t6=t_below(-6), t20=t_below(-20), t40=t40, gate_db15=gate))
    return rows


def main():
    data = []
    for g in sorted(p for p in R.iterdir() if p.is_dir()):
        for w in g.glob("*.wav"):
            u = w.stem.upper(); role = next((v for k, v in ROLES.items() if k in u), None)
            if role is None or "BODY2" in u or "BODY3" in u or "REV2" in u: continue
            x, sr = sf.read(w, always_2d=True); x = x.mean(1)
            E, dt = band_env(x, sr)
            data.append(dict(gun=g.name, role=role, file=w.name, feats=feats(E, dt), env=(E - E.max()).round(1).tolist()))
    json.dump(data, open(OUT / "band_adsr.json", "w"))
    dt = HOP / 48000

    # --- per role: median normalized band envelopes (time x band heatmap) + feature table
    roles = ["PROC", "SUB", "BODY", "TR", "REV"]
    fig, ax = plt.subplots(1, 5, figsize=(26, 4.5))
    summary = {}
    for a, role in zip(ax, roles):
        envs = np.array([d["env"] for d in data if d["role"] == role])
        med = np.median(envs, 0)
        t = np.arange(med.shape[1] + 1) * dt * 1000
        a.pcolormesh(t, np.arange(10), med, vmin=-60, vmax=0, shading="flat", cmap="magma")
        a.set_yticks(np.arange(9) + 0.5, BANDS, fontsize=7); a.set_title(f"{role} median band env (n={len(envs)})"); a.set_xlabel("ms")
        F = [d["feats"] for d in data if d["role"] == role]
        summary[role] = {BANDS[b]: {k: [round(float(np.percentile([f[b][k] for f in F], q)), 1) for q in (25, 50, 75)]
                                    for k in ["level", "attack", "t6", "t20", "t40", "gate_db15"]} for b in range(9)}
    plt.tight_layout(); plt.savefig(OUT / "band_env_by_role.png", dpi=60)
    json.dump(summary, open(OUT / "band_adsr_summary.json", "w"), indent=1)

    # --- cluster guns by their PROCESSED band features
    P = [d for d in data if d["role"] == "PROC"]
    X = np.array([[f[b][k] for b in range(9) for k in ["level", "t6", "t20", "t40"]] for f in [d["feats"] for d in P]])
    Xs = StandardScaler().fit_transform(X)
    inert = []
    for k in range(2, 7):
        km = KMeans(k, n_init=20, random_state=0).fit(Xs); inert.append((k, km.inertia_))
    K = 4
    km = KMeans(K, n_init=50, random_state=0).fit(Xs)
    fig, ax = plt.subplots(1, K, figsize=(6 * K, 4.5))
    clusters = {}
    for c in range(K):
        mem = [P[i] for i in range(len(P)) if km.labels_[i] == c]
        med = np.median(np.array([m["env"] for m in mem]), 0)
        t = np.arange(med.shape[1] + 1) * dt * 1000
        ax[c].pcolormesh(t, np.arange(10), med, vmin=-60, vmax=0, shading="flat", cmap="magma")
        ax[c].set_yticks(np.arange(9) + 0.5, BANDS, fontsize=7)
        names = [m["gun"].replace("KFU_GUN_", "") for m in mem]
        ax[c].set_title(f"type {c} (n={len(mem)}): " + ", ".join(names[:6]) + ("…" if len(names) > 6 else ""), fontsize=8)
        clusters[c] = names
    plt.tight_layout(); plt.savefig(OUT / "band_env_clusters.png", dpi=60)
    json.dump({"inertia": inert, "clusters": clusters}, open(OUT / "band_clusters.json", "w"), indent=1)

    # print compact tables
    for role in ["PROC", "SUB", "BODY", "TR"]:
        print(f"\n{role}: median [p25–p75]  level dB | attack ms | t-6 ms | t-20 ms | t-40 ms | gate dB/15ms")
        for b in BANDS:
            s = summary[role][b]
            fmt = lambda k: f"{s[k][1]:6.0f} [{s[k][0]:.0f}–{s[k][2]:.0f}]"
            print(f"  {b:>11s} " + " | ".join(fmt(k) for k in ["level", "attack", "t6", "t20", "t40", "gate_db15"]))
    print("\ninertia", [(k, round(i)) for k, i in inert])
    for c, n in clusters.items(): print(f"type {c}: {', '.join(n)}")


if __name__ == "__main__":
    main()
