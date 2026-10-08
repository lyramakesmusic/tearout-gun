# /// script
# dependencies = ["numpy", "soundfile", "torch", "transformers", "librosa", "scipy"]
# ///
"""Ceiling test: resynthesize each real snare from (a) its full STFT magnitude with random phase, (b) its 64-band mel
envelope only (what the fit loss sees), (c) mel envelope at 1024/256 + 256/64 resolutions combined. CLAP distance to the original."""
import glob, os, json, numpy as np, soundfile as sf, torch, librosa
from transformers import ClapModel, ClapProcessor
dev = "mps"; M = ClapModel.from_pretrained("laion/larger_clap_general").to(dev).eval(); P = ClapProcessor.from_pretrained("laion/larger_clap_general")
def prep(x):
    pk = np.abs(x).max() + 1e-9; on = int(np.argmax(np.abs(x) > 0.05 * pk)); return (x[max(0, on - 48): on + 48000] / pk).astype(np.float32)
@torch.no_grad()
def E(xs):
    e = M.get_audio_features(**P(audio=[prep(x) for x in xs], sampling_rate=48000, return_tensors="pt").to(dev)); e = getattr(e, "pooler_output", e)
    return torch.nn.functional.normalize(e, dim=-1).cpu()
rng = np.random.default_rng(0)
def randphase(x, n_fft=1024, hop=256):
    S = np.abs(librosa.stft(x, n_fft=n_fft, hop_length=hop)); ph = np.exp(2j * np.pi * rng.random(S.shape))
    y = librosa.istft(S * ph, hop_length=hop, length=len(x))
    for _ in range(30): ph = np.exp(1j * np.angle(librosa.stft(y, n_fft=n_fft, hop_length=hop))); y = librosa.istft(S * ph, hop_length=hop, length=len(x))  # Griffin-Lim
    return y
def melonly(x, n_mels=64, n_fft=1024, hop=256):
    S = np.abs(librosa.stft(x, n_fft=n_fft, hop_length=hop)) ** 2
    mel = librosa.filters.mel(sr=48000, n_fft=n_fft, n_mels=n_mels, fmin=30, fmax=16000)
    Mm = mel @ S; Sh = np.sqrt(np.maximum(np.linalg.pinv(mel) @ Mm, 0))  # smear back to linear bins
    noise = librosa.stft(rng.standard_normal(len(x)), n_fft=n_fft, hop_length=hop); noise /= np.abs(noise) + 1e-9
    return librosa.istft(Sh * noise, hop_length=hop, length=len(x))
fits = sorted(glob.glob(os.path.expanduser("~/tearout-gun/out/sfits/s[0-9][0-9][0-9].json")))[:60]
X = []
for f in fits:
    x, sr = sf.read(json.load(open(f))["file"], always_2d=True); x = x.mean(1); X.append(librosa.resample(x, orig_sr=sr, target_sr=48000) if sr != 48000 else x)
E0 = E(X)
for name, fn in [("full STFT magnitude, phase rebuilt", randphase), ("64-band mel envelope only", melonly), ("mel envelope, 256-pt (fast) STFT", lambda x: melonly(x, 32, 256, 64))]:
    d = 1 - (E0 * E([fn(x.astype(np.float64)) for x in X])).sum(-1); print(f"  {name:36} median {d.median():.3f}  p90 {d.quantile(0.9):.3f}")
F = []
for f in fits: y, sr = sf.read(f.replace(".json", ".wav"), always_2d=True); F.append(y.mean(1))
d = 1 - (E0 * E(F)).sum(-1); print(f"  {'our fits':36} median {d.median():.3f}  p90 {d.quantile(0.9):.3f}")
