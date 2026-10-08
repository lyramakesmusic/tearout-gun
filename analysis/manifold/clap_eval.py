# /// script
# dependencies = ["numpy", "soundfile", "torch", "transformers", "librosa"]
# ///
"""CLAP critic: (a) kNN cosine distance to real snares in CLAP space, (b) zero-shot snare score vs other drum words."""
import glob, os, sys, numpy as np, soundfile as sf, torch, librosa
from transformers import ClapModel, ClapProcessor
S = "/private/tmp/claude-501/-Users-lyra/7e0688b8-cbfc-4465-81af-4a39d6a3d171/scratchpad/"
dev = "mps" if torch.backends.mps.is_available() else "cpu"
M = ClapModel.from_pretrained("laion/larger_clap_general").to(dev).eval(); P = ClapProcessor.from_pretrained("laion/larger_clap_general")
def load(f):
    x, sr = sf.read(f, always_2d=True); x = x.mean(1); x = librosa.resample(x, orig_sr=sr, target_sr=48000) if sr != 48000 else x
    pk = np.abs(x).max() + 1e-9; on = int(np.argmax(np.abs(x) > 0.05 * pk)); return (x[max(0, on - 48): on + 48000] / pk).astype(np.float32)
@torch.no_grad()
def emb(fs, bs=32):
    out = []
    for i in range(0, len(fs), bs):
        xs = [load(f) for f in fs[i:i + bs]]; inp = P(audio=xs, sampling_rate=48000, return_tensors="pt").to(dev)
        e = M.get_audio_features(**inp); e = getattr(e, 'pooler_output', e); out.append(torch.nn.functional.normalize(e, dim=-1).cpu())
    return torch.cat(out)
cache = os.path.expanduser("~/tearout-gun/analysis/manifold/corpus_clap.pt")
if os.path.exists(cache): C = torch.load(cache)
else:
    paths = list(np.load(os.path.expanduser("~/tearout-gun/analysis/manifold/corpus_emb.npz"))["paths"]); C = emb(paths); torch.save(C, cache)
words = ["a snare drum hit", "a kick drum", "a hi-hat", "a hand clap", "a tom drum", "an 808 bass", "a laser zap sound effect", "a synth pluck", "white noise", "a gunshot"]
with torch.no_grad():
    t = M.get_text_features(**P(text=words, return_tensors="pt", padding=True).to(dev)); T = torch.nn.functional.normalize(getattr(t, 'pooler_output', t), dim=-1).cpu()
def score(E, self_ex=False):
    D = 1 - E @ C.T
    if self_ex: D[D < 1e-4] = 9
    d5 = D.topk(5, largest=False).values.mean(1)
    zs = (100 * E @ T.T).softmax(-1)[:, 0]
    return d5.numpy(), zs.numpy()
groups = {"real snares (LOO)": None}
for t in ["kick", "hat", "clap"]: groups[t + "s"] = [os.path.expanduser("~/Music/samples/") + l.strip()[2:] for l in open(S + f"neg_{t}.txt")]
groups["old snare engine"] = glob.glob(os.path.expanduser("~/tearout-gun/out/snare_cur/*.wav"))
groups["lyra exports"] = [os.path.expanduser(f"~/Music/Export/snare {i}.wav") for i in range(1, 7)]
for m in ["uniform", "flavor", "anchored", "anchors", "current"] + sys.argv[1:]: groups[m] = sorted(glob.glob(S + f"man/{m}/*.wav"))
idx = np.random.default_rng(0).choice(len(C), 400, replace=False); res = {}
for n, fs in groups.items(): res[n] = score(C[idx] if fs is None else emb(fs), fs is None)
thr = np.percentile(res["real snares (LOO)"][0], 90)
print(f"CLAP: kNN cosine distance to 5 nearest real snares; on-manifold = under real 90th pct ({thr:.3f}); zero-shot P('snare drum hit' among {len(words)} words)")
for n, (d, z) in res.items(): print(f"  {n:20} median dist {np.median(d):.3f}  on-manifold {100*np.mean(d<thr):5.1f}%  | zero-shot snare median {np.median(z):.2f}  >0.5 {100*np.mean(z>0.5):5.1f}%")
# what does zero-shot call each group?
import collections
print("\nzero-shot top word per group:")
for n, fs in groups.items():
    E = C[idx] if fs is None else emb(fs); top = (E @ T.T).argmax(-1).numpy()
    c = collections.Counter(words[i] for i in top); print(f"  {n:20}", ", ".join(f"{w.replace('a ','').replace('an ','')} {100*k/len(top):.0f}%" for w, k in c.most_common(4)))
# each fit vs its own target
import json
fits = sorted(glob.glob(os.path.expanduser("~/tearout-gun/out/sfits/s[0-9][0-9][0-9].json")))
tg = [json.load(open(f))["file"] for f in fits]; fw = [f.replace(".json", ".wav") for f in fits]
Et, Ef = emb(tg), emb(fw); dpair = 1 - (Et * Ef).sum(-1)
Dt = 1 - Et @ C.T; Dt[Dt < 1e-4] = 9; dnn = Dt.min(1).values
print(f"\nfit vs its own target: median cos dist {dpair.median():.3f}; target vs its nearest other real snare {dnn.median():.3f}; fits closer to own target than nearest real neighbour is: {100*(dpair<dnn).float().mean():.0f}%")
o = dpair.argsort(descending=True)[:6]
print("worst fit pairs:", [(os.path.basename(tg[i])[:40], round(float(dpair[i]), 3)) for i in o])
torch.save({"Et": Et, "Ef": Ef, "files": tg, "fits": fits}, os.path.expanduser("~/tearout-gun/analysis/manifold/fitpairs_clap.pt"))
