# /// script
# dependencies = ["numpy", "torch"]
# ///
"""Inverse model for the snare engine: mel spectrogram → knobs. Continuous knobs regress in [0,1] (masked when their
layer is off); switch knobs and the note's pitch class are classified."""
import glob, json, math, os, sys, time, numpy as np, torch, torch.nn as nn, torch.nn.functional as F
torch.manual_seed(0); np.random.seed(0)
ROOT = os.path.expanduser("~/tearout-gun/out/gp/")
meta = json.load(open(sorted(glob.glob(ROOT + "c*.meta.json"))[0])); K = meta["knobs"]; D = len(K); FR, BD = meta["frames"], meta["bands"]
P = np.concatenate([np.fromfile(f, np.float32).reshape(-1, D) for f in sorted(glob.glob(ROOT + "c*.params.f32"))])
M = np.concatenate([np.fromfile(f, np.uint8).reshape(-1, FR, BD) for f in sorted(glob.glob(ROOT + "c*.mel.u8"))])
N = len(P); print(N, "samples", D, "knobs")
cont = [i for i, k in enumerate(K) if not k["int"]]
ints = [(i, k["max"] - k["min"] + 1) for i, k in enumerate(K) if k["int"]]
# mask: a layer's knobs only count when its level knob is above "off" (−59 dB → normalized ≈ 0.015 on a −60..6 dB range)
LVL = {"sn_click": "sn_c_lvl", "sn_metal": "sn_m_lvl", "sn_clap": "sn_k_lvl", "sn_room": "sn_r_lvl", "sn_tone": "sn_t_lvl", "sn_noise": "sn_n_lvl"}
ids = [k["id"] for k in K]; group = {}
for i, k in enumerate(K):
    for g, lv in LVL.items():
        pre = lv.split("_")[1]  # c, m, k, r, t, n
        if k["id"].startswith(f"sn_{pre}_") and k["id"] != lv: group[i] = ids.index(lv)
off_thr = 1.0 / 66 + 1e-3
def masks(p):
    m = torch.ones_like(p)
    for i, j in group.items(): m[:, i] = (p[:, j] > off_thr).float()
    return m
dev = "mps" if torch.backends.mps.is_available() else "cpu"
perm = np.random.permutation(N); nv = 4000; iv, it = perm[:nv], perm[nv:]
Pt, Mt = torch.tensor(P), torch.tensor(M)
class Net(nn.Module):
    def __init__(s):
        super().__init__()
        def blk(a, b): return nn.Sequential(nn.Conv2d(a, b, 3, padding=1), nn.BatchNorm2d(b), nn.GELU(), nn.Conv2d(b, b, 3, padding=1), nn.BatchNorm2d(b), nn.GELU())
        s.f = nn.Sequential(blk(1, 32), nn.MaxPool2d(2), blk(32, 64), nn.MaxPool2d(2), blk(64, 128), nn.MaxPool2d(2), blk(128, 192), nn.AdaptiveAvgPool2d((4, 3)))
        s.h = nn.Sequential(nn.Flatten(), nn.Linear(192 * 12, 768), nn.GELU(), nn.Dropout(0.1), nn.Linear(768, 768), nn.GELU())
        s.cont = nn.Linear(768, len(cont)); s.cls = nn.ModuleList([nn.Linear(768, n) for _, n in ints])
    def forward(s, x):
        z = s.h(s.f(x)); return torch.sigmoid(s.cont(z)), [c(z) for c in s.cls]
net = Net().to(dev); print(sum(p.numel() for p in net.parameters()) / 1e6, "M params")
EP = int(os.environ.get("EPOCHS", 14)); BS = 256; steps = EP * (len(it) // BS)
opt = torch.optim.AdamW(net.parameters(), 2e-3, weight_decay=1e-4); sched = torch.optim.lr_scheduler.OneCycleLR(opt, 2e-3, total_steps=steps, pct_start=0.1)
def batch(idx):
    x = Mt[idx].float().div(255).unsqueeze(1).to(dev); p = Pt[idx].to(dev); return x, p
def losses(x, p):
    yc, yl = net(x); m = masks(p)[:, cont]; lc = ((yc - p[:, cont]).abs() * m).sum() / m.sum()
    li = sum(F.cross_entropy(l, (p[:, i] * (n - 1)).round().long()) for (i, n), l in zip(ints, yl)) / len(ints)
    return lc, li
t0 = time.time(); step = 0
for ep in range(EP):
    net.train(); np.random.shuffle(it)
    for b in range(len(it) // BS):
        x, p = batch(it[b * BS:(b + 1) * BS]); lc, li = losses(x, p); loss = lc + 0.1 * li
        opt.zero_grad(); loss.backward(); opt.step(); sched.step(); step += 1
    net.eval(); vc = vi = 0
    with torch.no_grad():
        for b in range(0, nv, 512): x, p = batch(iv[b:b + 512]); lc, li = losses(x, p); vc += lc.item() * len(x); vi += li.item() * len(x)
    print(f"ep {ep} val knob MAE {vc/nv:.4f}  switch CE {vi/nv:.3f}  {time.time()-t0:.0f}s", flush=True)
# per-knob val error, to see what's identifiable
net.eval(); errs = np.zeros(len(cont)); cnt = np.zeros(len(cont)); acc = np.zeros(len(ints))
with torch.no_grad():
    for b in range(0, nv, 512):
        x, p = batch(iv[b:b + 512]); yc, yl = net(x); m = masks(p)[:, cont]
        errs += ((yc - p[:, cont]).abs() * m).sum(0).cpu().numpy(); cnt += m.sum(0).cpu().numpy()
        for j, ((i, n), l) in enumerate(zip(ints, yl)): acc[j] += (l.argmax(1) == (p[:, i] * (n - 1)).round().long()).float().sum().item()
E = errs / np.maximum(cnt, 1); o = np.argsort(E)
print("best-recovered:", ", ".join(f"{ids[cont[i]].replace('sn_','')} {E[i]:.3f}" for i in o[:12]))
print("worst-recovered:", ", ".join(f"{ids[cont[i]].replace('sn_','')} {E[i]:.3f}" for i in o[-12:]))
print("switch accuracy:", ", ".join(f"{ids[i].replace('sn_','')} {acc[j]/nv:.2f}" for j, (i, n) in enumerate(ints)))
torch.save({"state": net.state_dict(), "cont": cont, "ints": ints, "meta": meta}, ROOT + "inv.pt")
print("saved", ROOT + "inv.pt")
