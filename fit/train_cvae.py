# /// script
# dependencies = ["torch", "numpy"]
# ///
"""Conditional VAE: params | (measured features, small latent). The knobs are the features themselves.
Features are robust-standardized with the reference medians/MADs, so a knob value is "σ from a typical gun"."""
import glob, json, sys
import numpy as np, torch, torch.nn as nn

THRESH, ZD, BETA, FREE = 0.9, 4, 0.25, 0.25
KEYS = ["noisy", "peaky", "move", "crest", "harsh", "subMid", "midSustain", "midLate"]
ref = json.load(open("web/gunness.json"))
rows = [json.loads(l) for f in glob.glob("out/vae/cand_*.jsonl") for l in open(f)]
rows = [r for r in rows if r["score"] <= THRESH]
X = torch.tensor(np.array([r["x"] for r in rows]), dtype=torch.float32)
C = torch.tensor(np.array([[(r["f"][k] - ref["median"][k]) / (1.4826 * ref["mad"][k] + 1e-9) for k in KEYS] for r in rows]), dtype=torch.float32).clamp(-4, 4)
torch.manual_seed(0)
perm = torch.randperm(len(X)); nv = len(X) // 10
iv, it = perm[:nv], perm[nv:]
D, CD = X.shape[1], C.shape[1]
print(f"{len(X)} patches, {D} params, {CD} feature knobs; feature sd in data: {[round(v, 2) for v in C.std(0).tolist()]}")

class CVAE(nn.Module):
    def __init__(s):
        super().__init__()
        s.enc = nn.Sequential(nn.Linear(D + CD, 256), nn.GELU(), nn.Dropout(0.1), nn.Linear(256, 128), nn.GELU(), nn.Linear(128, 2 * ZD))
        s.dec = nn.Sequential(nn.Linear(ZD + CD, 128), nn.GELU(), nn.Linear(128, 256), nn.GELU(), nn.Linear(256, D))
    def forward(s, x, c):
        mu, lv = s.enc(torch.cat([x, c], -1)).chunk(2, -1)
        z = mu + torch.randn_like(mu) * (0.5 * lv).exp()
        return torch.sigmoid(s.dec(torch.cat([z, c], -1))), mu, lv

m = CVAE(); opt = torch.optim.AdamW(m.parameters(), 2e-3, weight_decay=1e-4)
def losses(idx, beta):
    x, c = X[idx], C[idx]
    y, mu, lv = m(x, c)
    rec = ((y - x) ** 2).sum(-1).mean()
    kl = (-0.5 * (1 + lv - mu ** 2 - lv.exp())).mean(0)
    return rec, kl, rec + beta * torch.clamp(kl, min=FREE).sum()
best, state, bad = 1e9, None, 0
for ep in range(800):
    m.train(); beta = min(1.0, ep / 150) * BETA
    for _ in range(len(it) // 128):
        b = it[torch.randint(len(it), (128,))]
        _, _, l = losses(b, beta); opt.zero_grad(); l.backward(); opt.step()
    m.eval()
    with torch.no_grad(): rv, klv, lv_ = losses(iv, BETA)
    if ep >= 150 and lv_.item() < best - 1e-3: best, state, bad = lv_.item(), {k: v.clone() for k, v in m.state_dict().items()}, 0
    elif ep >= 150: bad += 1
    if ep % 100 == 0: print(f"ep {ep} val rec {rv.item():.3f} kl {[round(v, 2) for v in klv.tolist()]}")
    if bad > 80: break
m.load_state_dict(state); m.eval()
with torch.no_grad():
    rv, klv, _ = losses(iv, BETA)
    # how much does the decoder use the condition? reconstruct with shuffled conditions
    x, c = X[iv], C[iv]
    mu, _ = m.enc(torch.cat([x, c], -1)).chunk(2, -1)
    y_ok = torch.sigmoid(m.dec(torch.cat([mu, c], -1)))
    y_sh = torch.sigmoid(m.dec(torch.cat([mu, c[torch.randperm(len(c))]], -1)))
    print(f"val rec {rv.item():.3f}; with true features {((y_ok - x) ** 2).sum(-1).mean():.3f} vs shuffled features {((y_sh - x) ** 2).sum(-1).mean():.3f}; kl {[round(v, 2) for v in klv.tolist()]}")
layers = [{"W": l.weight.detach().numpy().round(5).tolist(), "b": l.bias.detach().numpy().round(5).tolist()} for l in m.dec if isinstance(l, nn.Linear)]
ids = json.load(open("out/vae/ids.json"))
json.dump({"z": ZD, "cond": KEYS, "ids": ids, "layers": layers, "act": "gelu", "out": "sigmoid"}, open("web/cvae.json", "w"))
print("wrote web/cvae.json")
