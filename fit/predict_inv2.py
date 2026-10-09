# /// script
# dependencies = ["numpy", "torch"]
# ///
"""Run inverse model v2 on mel + pitch files → normalized knob vectors (JSON). usage: predict_inv2.py <model.pt> <mel.u8> <pitch.u8> <out.json>"""
import json, sys, numpy as np, torch, torch.nn as nn
ck = torch.load(sys.argv[1], weights_only=False); meta, cont, ints = ck["meta"], ck["cont"], ck["ints"]
FR, BD, PB, D = meta["frames"], meta["bands"], meta["pbins"], len(meta["knobs"])
class Net(nn.Module):
    def __init__(s):
        super().__init__()
        def blk(a, b): return nn.Sequential(nn.Conv2d(a, b, 3, padding=1), nn.BatchNorm2d(b), nn.GELU(), nn.Conv2d(b, b, 3, padding=1), nn.BatchNorm2d(b), nn.GELU())
        s.f = nn.Sequential(blk(1, 32), nn.MaxPool2d(2), blk(32, 64), nn.MaxPool2d(2), blk(64, 128), nn.MaxPool2d(2), blk(128, 192), nn.AdaptiveAvgPool2d((4, 3)))
        s.p = nn.Sequential(nn.Conv1d(1, 32, 9, padding=4), nn.GELU(), nn.Conv1d(32, 32, 9, padding=4), nn.GELU(), nn.MaxPool1d(3), nn.Conv1d(32, 64, 7, padding=3), nn.GELU(), nn.Flatten(), nn.Linear(64 * (PB // 3), 256), nn.GELU())
        s.h = nn.Sequential(nn.Linear(192 * 12 + 256, 768), nn.GELU(), nn.Dropout(0.1), nn.Linear(768, 768), nn.GELU())
        s.cont = nn.Linear(768, len(cont)); s.cls = nn.ModuleList([nn.Linear(768, n) for _, n in ints])
    def forward(s, x, h):
        z = s.h(torch.cat([s.f(x).flatten(1), s.p(h)], 1)); return torch.sigmoid(s.cont(z)), [c(z) for c in s.cls]
net = Net(); net.load_state_dict(ck["state"]); net.eval()
M = np.fromfile(sys.argv[2], np.uint8).reshape(-1, 1, FR, BD); H = np.fromfile(sys.argv[3], np.uint8).reshape(-1, 1, PB)
with torch.no_grad(): yc, yl = net(torch.tensor(M).float().div(255), torch.tensor(H).float().div(255))
out = np.zeros((len(M), D), np.float32); out[:, cont] = yc.numpy()
for (i, n), l in zip(ints, yl): out[:, i] = l.argmax(1).numpy() / max(1, n - 1)
json.dump({"knobs": [k["id"] for k in meta["knobs"]], "pred": out.tolist()}, open(sys.argv[4], "w")); print(len(out), "predictions")
