# /// script
# dependencies = ["numpy", "torch"]
# ///
"""Run the inverse model on a mel file → normalized knob vectors (JSON). usage: predict_inv.py <mel.u8> <out.json>"""
import json, os, sys, numpy as np, torch, torch.nn as nn
ck = torch.load(os.path.expanduser("~/tearout-gun/out/gp/inv.pt"), weights_only=False); meta, cont, ints = ck["meta"], ck["cont"], ck["ints"]
FR, BD, D = meta["frames"], meta["bands"], len(meta["knobs"])
class Net(nn.Module):
    def __init__(s):
        super().__init__()
        def blk(a, b): return nn.Sequential(nn.Conv2d(a, b, 3, padding=1), nn.BatchNorm2d(b), nn.GELU(), nn.Conv2d(b, b, 3, padding=1), nn.BatchNorm2d(b), nn.GELU())
        s.f = nn.Sequential(blk(1, 32), nn.MaxPool2d(2), blk(32, 64), nn.MaxPool2d(2), blk(64, 128), nn.MaxPool2d(2), blk(128, 192), nn.AdaptiveAvgPool2d((4, 3)))
        s.h = nn.Sequential(nn.Flatten(), nn.Linear(192 * 12, 768), nn.GELU(), nn.Dropout(0.1), nn.Linear(768, 768), nn.GELU())
        s.cont = nn.Linear(768, len(cont)); s.cls = nn.ModuleList([nn.Linear(768, n) for _, n in ints])
    def forward(s, x):
        z = s.h(s.f(x)); return torch.sigmoid(s.cont(z)), [c(z) for c in s.cls]
net = Net(); net.load_state_dict(ck["state"]); net.eval()
M = np.fromfile(sys.argv[1], np.uint8).reshape(-1, 1, FR, BD)
with torch.no_grad(): yc, yl = net(torch.tensor(M).float().div(255))
out = np.zeros((len(M), D), np.float32); out[:, cont] = yc.numpy()
for (i, n), l in zip(ints, yl): out[:, i] = l.argmax(1).numpy() / max(1, n - 1)
json.dump(out.tolist(), open(sys.argv[2], "w")); print(len(out), "predictions")
