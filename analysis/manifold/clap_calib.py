# /// script
# dependencies = ["numpy", "soundfile", "torch", "transformers", "librosa", "scipy"]
# ///
"""How far does CLAP move under small, perceptually minor edits vs the fit gap?"""
import glob, os, json, numpy as np, soundfile as sf, torch, librosa
from scipy.signal import butter, sosfilt
from transformers import ClapModel, ClapProcessor
dev = "mps"; M = ClapModel.from_pretrained("laion/larger_clap_general").to(dev).eval(); P = ClapProcessor.from_pretrained("laion/larger_clap_general")
def prep(x, sr):
    x = librosa.resample(x, orig_sr=sr, target_sr=48000) if sr != 48000 else x
    pk = np.abs(x).max() + 1e-9; on = int(np.argmax(np.abs(x) > 0.05 * pk)); return (x[max(0, on - 48): on + 48000] / pk).astype(np.float32)
@torch.no_grad()
def E(xs):
    e = M.get_audio_features(**P(audio=xs, sampling_rate=48000, return_tensors="pt").to(dev)); e = getattr(e, "pooler_output", e)
    return torch.nn.functional.normalize(e, dim=-1).cpu()
fits = sorted(glob.glob(os.path.expanduser("~/tearout-gun/out/sfits/s[0-9][0-9][0-9].json")))[:40]
edits = {
  "gain −6 dB (should be 0)": lambda x: x * 0.5,
  "tilt EQ ±2 dB": lambda x: x + 0.25 * sosfilt(butter(1, 2000, "hp", fs=48000, output="sos"), x),
  "lowpass 12 kHz": lambda x: sosfilt(butter(4, 12000, fs=48000, output="sos"), x),
  "pitch +1 st": lambda x: librosa.effects.pitch_shift(x, sr=48000, n_steps=1),
  "noise floor −40 dB": lambda x: x + 0.01 * np.random.default_rng(0).standard_normal(len(x)),
  "tail −30% (faster decay)": lambda x: x * np.exp(-np.arange(len(x)) / (0.25 * 48000)),
  "soft clip +6 dB": lambda x: np.tanh(2 * x) / np.tanh(2),
}
X = []
for f in fits:
    t = json.load(open(f))["file"]; x, sr = sf.read(t, always_2d=True); X.append(prep(x.mean(1), sr))
E0 = E(X)
for name, fn in edits.items():
    Ee = E([prep(fn(x.astype(np.float64)), 48000) for x in X]); d = 1 - (E0 * Ee).sum(-1)
    print(f"  {name:28} median {d.median():.3f}  p90 {d.quantile(0.9):.3f}")
F = []
for f in fits: y, sr = sf.read(f.replace(".json", ".wav"), always_2d=True); F.append(prep(y.mean(1), sr))
d = 1 - (E0 * E(F)).sum(-1); print(f"  {'(our fit of the same snare)':28} median {d.median():.3f}  p90 {d.quantile(0.9):.3f}")
# equalize tails: both cut where the 10 ms envelope first stays under −45 dB re peak (same length for the pair), 5 ms fade
def gate(x, n=None):
    h = 480; k = len(x) // h; e = 20 * np.log10(np.sqrt((x[: k * h].reshape(k, h) ** 2).mean(1)) + 1e-9); e -= e.max()
    last = np.where(e > -45)[0]; end = (last[-1] + 1) * h if len(last) else len(x)
    return end if n is None else np.concatenate([x[:n] * np.r_[np.ones(max(0, n - 240)), np.linspace(1, 0, min(n, 240))], np.zeros(max(0, len(x) - n))])
G0, G1 = [], []
for a, b in zip(X, F):
    n = min(gate(a), gate(b)); G0.append(gate(a, n).astype(np.float32)); G1.append(gate(b, n).astype(np.float32))
d = 1 - (E(G0) * E(G1)).sum(-1); print(f"  {'fit vs target, tails gated':28} median {d.median():.3f}  p90 {d.quantile(0.9):.3f}")
d = 1 - (E(G0) * E(X)).sum(-1); print(f"  {'target gated vs target raw':28} median {d.median():.3f}  p90 {d.quantile(0.9):.3f}")
