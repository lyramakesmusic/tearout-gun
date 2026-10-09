# /// script
# dependencies = ["numpy", "torch", "onnx", "onnxscript"]
# ///
"""Export inverse model v2 to ONNX for the browser: inputs mel [1,1,F,B] and pitch [1,1,P] in [0,1]; output one vector of normalized knobs.
usage: export_inv2.py <model.pt> <out.onnx> <out.json>"""
import json, sys, torch, torch.nn as nn
sys.argv, args = sys.argv[:1] + [sys.argv[1], "", "", ""], sys.argv[1:]
src = open(__file__.replace("export_inv2.py", "predict_inv2.py")).read().split("net = Net()")[0]
exec(src)
net = Net(); net.load_state_dict(ck["state"]); net.eval()
class Wrap(nn.Module):
    def __init__(s, n): super().__init__(); s.n = n
    def forward(s, x, h):
        yc, yl = s.n(x, h); return torch.cat([yc] + [l for l in yl], 1)
w = Wrap(net); x = torch.zeros(1, 1, FR, BD); h = torch.zeros(1, 1, PB)
torch.onnx.export(w, (x, h), args[1], input_names=["mel", "pitch"], output_names=["out"], opset_version=17, dynamo=False)
json.dump({"knobs": meta["knobs"], "cont": cont, "ints": ints, "frames": FR, "bands": BD, "pbins": PB}, open(args[2], "w"))
print("exported", args[1])
