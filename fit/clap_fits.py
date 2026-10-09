# /// script
# dependencies = ["numpy", "soundfile", "torch", "transformers", "librosa"]
# ///
"""CLAP cosine distance fit ↔ its own target, per method; plus each target's nearest other real snare for scale."""
import json, os, sys, numpy as np, soundfile as sf, torch, librosa, collections
from transformers import ClapModel, ClapProcessor
dev = "mps"; M = ClapModel.from_pretrained("laion/larger_clap_general").to(dev).eval(); P = ClapProcessor.from_pretrained("laion/larger_clap_general")
def load(f):
    x, sr = sf.read(f, always_2d=True); x = x.mean(1); x = librosa.resample(x, orig_sr=sr, target_sr=48000) if sr != 48000 else x
    pk = np.abs(x).max() + 1e-9; on = int(np.argmax(np.abs(x) > 0.05 * pk)); return (x[max(0, on - 48): on + 48000] / pk).astype(np.float32)
@torch.no_grad()
def emb(fs):
    out = []
    for i in range(0, len(fs), 32):
        e = M.get_audio_features(**P(audio=[load(f) for f in fs[i:i + 32]], sampling_rate=48000, return_tensors="pt").to(dev)); e = getattr(e, "pooler_output", e); out.append(torch.nn.functional.normalize(e, dim=-1).cpu())
    return torch.cat(out)
if __name__ == "__main__":
    L = json.load(open(sys.argv[1])); T = sorted(set(x["target"] for x in L)); ET = dict(zip(T, emb(T))); EF = emb([x["fit"] for x in L])
    d = collections.defaultdict(list)
    for x, e in zip(L, EF): d[x["method"]].append(1 - float(ET[x["target"]] @ e))
    for m, v in d.items(): print(f"  {m:20} CLAP fit↔target median {np.median(v):.3f}  mean {np.mean(v):.3f}  n={len(v)}")
    C = torch.load(os.path.expanduser("~/tearout-gun/analysis/manifold/corpus_clap.pt")); Et = torch.stack([ET[t] for t in T]); D = 1 - Et @ C.T; D[D < 1e-4] = 9
    print(f"  {'(nearest other real)':20} median {D.min(1).values.median():.3f}   — earlier staged fits on other snares: 0.288")
