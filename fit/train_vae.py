# /// script
# dependencies = ["torch", "numpy"]
# ///
"""8-d VAE over normalized synth params of gun-like patches. Exports the decoder as JSON for the browser."""
import glob, json, sys
import numpy as np, torch, torch.nn as nn

THRESH = float(sys.argv[1]) if len(sys.argv) > 1 else 0.9
Z = 8
BETA = float(sys.argv[2]) if len(sys.argv) > 2 else 0.5
FREE = float(sys.argv[3]) if len(sys.argv) > 3 else 0.0
rows = [json.loads(l) for f in glob.glob("out/vae/cand_*.jsonl") for l in open(f)]
rows = [r for r in rows if r["score"] <= THRESH]
X = torch.tensor(np.array([r["x"] for r in rows]), dtype=torch.float32)
torch.manual_seed(0)
perm = torch.randperm(len(X)); nv = len(X) // 10
Xv, Xt = X[perm[:nv]], X[perm[nv:]]
D = X.shape[1]
print(f"{len(X)} patches, {D} params, train {len(Xt)} / val {len(Xv)}")

class VAE(nn.Module):
    def __init__(s):
        super().__init__()
        s.enc = nn.Sequential(nn.Linear(D, 256), nn.GELU(), nn.Dropout(0.1), nn.Linear(256, 128), nn.GELU(), nn.Linear(128, 2 * Z))
        s.dec = nn.Sequential(nn.Linear(Z, 128), nn.GELU(), nn.Linear(128, 256), nn.GELU(), nn.Linear(256, D))
    def forward(s, x):
        mu, lv = s.enc(x).chunk(2, -1)
        z = mu + torch.randn_like(mu) * (0.5 * lv).exp()
        return torch.sigmoid(s.dec(z)), mu, lv

m = VAE(); opt = torch.optim.AdamW(m.parameters(), 2e-3, weight_decay=1e-4)
def losses(x, beta):
    y, mu, lv = m(x)
    rec = ((y - x) ** 2).sum(-1).mean()
    kl_d = (-0.5 * (1 + lv - mu ** 2 - lv.exp())).mean(0)
    return rec, kl_d, rec + beta * torch.clamp(kl_d, min=FREE).sum()
best, best_state, bad = 1e9, None, 0
for ep in range(600):
    m.train(); beta = min(1.0, ep / 150) * BETA  # KL warmup
    for i in range(0, len(Xt), 128):
        xb = Xt[torch.randperm(len(Xt))[:128]]
        _, _, l = losses(xb, beta); opt.zero_grad(); l.backward(); opt.step()
    m.eval()
    with torch.no_grad(): rv, klv, lv_ = losses(Xv, BETA)
    if ep >= 150 and lv_.item() < best - 1e-3: best, best_state, bad = lv_.item(), {k: v.clone() for k, v in m.state_dict().items()}, 0
    elif ep >= 150: bad += 1
    if ep % 50 == 0: print(f"ep {ep} val rec {rv.item():.3f} kl/dim {[round(v, 2) for v in klv.tolist()]}")
    if bad > 60: break
m.load_state_dict(best_state); m.eval()
with torch.no_grad():
    rv, klv, _ = losses(Xv, BETA)
    base = ((Xv - Xt.mean(0)) ** 2).sum(-1).mean()  # mean-patch baseline
print(f"final val rec {rv.item():.3f} (mean-patch baseline {base.item():.3f}); kl per dim {[round(v, 2) for v in klv.tolist()]}")
# export decoder: list of {W, b, act}
layers = []
for mod in m.dec:
    if isinstance(mod, nn.Linear): layers.append({"W": mod.weight.detach().numpy().round(5).tolist(), "b": mod.bias.detach().numpy().round(5).tolist()})
ids = json.load(open("out/vae/ids.json")) if __import__("os").path.exists("out/vae/ids.json") else None
json.dump({"z": Z, "ids": ids, "layers": layers, "act": "gelu", "out": "sigmoid", "kl": klv.tolist()}, open("web/vae.json", "w"))
print("wrote web/vae.json")
