# /// script
# dependencies = ["numpy", "scipy", "soundfile"]
# ///
import json, os, sys, numpy as np, soundfile as sf
from concurrent.futures import ProcessPoolExecutor
sys.path.insert(0, os.path.dirname(__file__)); from embed import embed
def one(p):
    try: x, sr = sf.read(p, always_2d=True); return embed(x.mean(1), sr)
    except Exception: return None
if __name__ == "__main__":
    C = json.load(open(os.path.expanduser('~/tearout-gun/analysis/snares/corpus.json')))
    with ProcessPoolExecutor(4) as ex: E = list(ex.map(one, [r['path'] for r in C], chunksize=32))
    keep = [i for i, e in enumerate(E) if e is not None]
    np.savez_compressed(os.path.expanduser('~/tearout-gun/analysis/manifold/corpus_emb.npz'), E=np.stack([E[i] for i in keep]), paths=np.array([C[i]['path'] for i in keep]))
    print(len(keep), 'embedded')
