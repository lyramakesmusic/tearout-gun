# /// script
# dependencies = ["numpy", "soundfile", "torch", "transformers", "librosa"]
# ///
"""CLAP: how close each randomizer's rolls are to unseen real snares (precision), how much of the real set they cover (recall), and their spread."""
import json, os, sys, numpy as np, torch, collections
sys.path.insert(0, os.path.dirname(__file__)); from clap_fits import emb
J = json.load(open(sys.argv[1])); C = torch.load(os.path.expanduser("~/tearout-gun/analysis/manifold/corpus_clap.pt"))[J["held"]]
by = collections.defaultdict(list)
for x in J["items"]: by[x["method"]].append(x["wav"])
g = torch.Generator().manual_seed(0); idx = torch.randperm(len(C), generator=g); R1, R2 = C[idx[:len(C)//2]], C[idx[len(C)//2:]]
D = 1 - R1 @ R2.T; print(f"  {'real vs other real':10} precision {D.min(1).values.median():.3f}")
for m, fs in by.items():
    E = emb(fs); P = (1 - E @ C.T).min(1).values; Rc = (1 - C @ E.T).min(1).values; dv = 1 - E @ E.T
    print(f"  {m:10} precision {P.median():.3f}  recall {Rc.median():.3f}  diversity {dv[~torch.eye(len(E), dtype=bool)].mean():.3f}  n={len(fs)}")
