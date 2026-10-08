# /// script
# dependencies = ["numpy", "scipy", "soundfile", "torch"]
# ///
import glob, os, sys, numpy as np, soundfile as sf, torch
sys.path.insert(0, os.path.dirname(__file__)); from embed import embed
S = "/private/tmp/claude-501/-Users-lyra/7e0688b8-cbfc-4465-81af-4a39d6a3d171/scratchpad/"
Z = np.load(os.path.expanduser("~/tearout-gun/analysis/manifold/corpus_emb.npz")); E = torch.tensor(Z["E"])
dev = "mps" if torch.backends.mps.is_available() else "cpu"; Eg = E.to(dev)
def nn(X, k=5, exclude_self=False):
    X = torch.tensor(np.asarray(X), dtype=torch.float32).to(dev)
    # energy-weighted |dB| difference: cells where either sound is near the floor count little
    D = torch.empty(len(X), len(Eg), device=dev)
    for i in range(0, len(X), 16):
        a = X[i:i + 16, None, :]; w = (torch.maximum(a, Eg[None]) + 60) / 60
        D[i:i + 16] = (w * (a - Eg[None]).abs()).sum(-1) / w.sum(-1)
    if exclude_self: D[D < 0.05] = 1e9         # drop the item itself / exact dupes
    d, _ = D.topk(k, largest=False); return d[:, 0].cpu().numpy(), d.mean(1).cpu().numpy()
def emb_files(fs):
    out = []
    for f in fs:
        try: x, sr = sf.read(f, always_2d=True); out.append(embed(x.mean(1), sr))
        except Exception: pass
    return np.stack(out)
rng = np.random.default_rng(0); idx = rng.choice(len(E), 400, replace=False)
groups = {"real snares (LOO)": None}
for t in ["kick", "hat", "clap"]:
    groups[t + "s"] = [os.path.expanduser("~/Music/samples/") + l.strip()[2:] for l in open(S + f"neg_{t}.txt")]
groups["old snare engine"] = glob.glob(os.path.expanduser("~/tearout-gun/out/snare_cur/*.wav"))
groups["lyra exports"] = [os.path.expanduser(f"~/Music/Export/snare {i}.wav") for i in range(1, 7)]
for m in ["uniform", "flavor", "anchored", "anchors", "current"] + sys.argv[1:]: groups[m] = sorted(glob.glob(S + f"man/{m}/*.wav"))
res = {}
for name, fs in groups.items():
    X = E[idx].numpy() if fs is None else emb_files(fs)
    d1, d5 = nn(X, exclude_self=fs is None); res[name] = d5
thr = np.percentile(res["real snares (LOO)"], 90)
print(f"distance = mean |dB| to the 5 nearest of {len(E)} real snares (lower = more snare-like); on-manifold = under the real-snare 90th pct ({thr:.2f} dB)")
for name, d in res.items(): print(f"  {name:20} n={len(d):4}  median {np.median(d):5.2f}  p25 {np.percentile(d,25):5.2f}  p75 {np.percentile(d,75):5.2f}  on-manifold {100*np.mean(d<thr):5.1f}%")
