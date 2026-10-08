# /// script
# dependencies = ["numpy", "scipy", "soundfile"]
# ///
"""Audio fingerprint for "is this near a real snare": log-mel, 40 bands 60 Hz–16 kHz, 20 ms hop, 0–480 ms
(24 frames), dB re the sound's own peak, floored at −60 → 960 dims. Onset-aligned, level-invariant."""
import numpy as np
SR = 48000; NFFT = 2048; HOP = 960; FR = 24; NB = 40
def _mel():
    hz2m = lambda f: 2595 * np.log10(1 + f / 700); m2hz = lambda m: 700 * (10 ** (m / 2595) - 1)
    pts = m2hz(np.linspace(hz2m(60), hz2m(16000), NB + 2)); fr = np.arange(NFFT // 2 + 1) * SR / NFFT
    W = np.zeros((NB, len(fr)))
    for i in range(NB):
        a, c, d = pts[i], pts[i + 1], pts[i + 2]
        W[i] = np.clip(np.minimum((fr - a) / (c - a), (d - fr) / (d - c)), 0, None)
    return W / (W.sum(1, keepdims=True) + 1e-12)
MEL = _mel(); WIN = np.hanning(NFFT)
def to48(x, sr):
    if sr == SR: return x
    t = np.arange(int(len(x) * SR / sr)) * sr / SR; return np.interp(t, np.arange(len(x)), x)
def embed(x, sr=SR):
    x = to48(np.asarray(x, float), sr); pk = np.abs(x).max() + 1e-12
    on = int(np.argmax(np.abs(x) > pk * 0.05)); x = x[max(0, on - 48):]
    x = np.concatenate([x, np.zeros(NFFT + FR * HOP)])
    F = np.stack([np.abs(np.fft.rfft(x[i * HOP: i * HOP + NFFT] * WIN)) ** 2 for i in range(FR)])
    S = 10 * np.log10(F @ MEL.T + 1e-14); S -= S.max()
    return np.maximum(S, -60).astype(np.float32).ravel()
