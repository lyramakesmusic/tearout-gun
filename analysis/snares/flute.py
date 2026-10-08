# /// script
# dependencies = ["numpy", "scipy", "soundfile"]
# ///
import glob, os, numpy as np, soundfile as sf
from scipy.signal import stft
D = "/private/tmp/claude-501/-Users-lyra/7e0688b8-cbfc-4465-81af-4a39d6a3d171/scratchpad/flute/"
for f in sorted(os.listdir(D)):
    x, sr = sf.read(D + f, always_2d=True); x = x.mean(1)
    fr, t, Z = stft(x, sr, nperseg=4096, noverlap=4096 - 240); M = np.abs(Z)
    # harmonic track: strongest peak 200–2000 Hz in frames 50–150 ms
    seg = M[:, (t > 0.05) & (t < 0.15)].mean(1); band = (fr > 200) & (fr < 2000); f0 = fr[band][seg[band].argmax()]
    harm = [20 * np.log10(seg[np.argmin(abs(fr - k * f0))] / seg[np.argmin(abs(fr - f0))] + 1e-9) for k in range(1, 7)]
    # noise floor between harmonics 3–8 kHz vs fundamental, over time
    def nz(t0, t1):
        s = M[:, (t >= t0) & (t < t1)].mean(1); b = (fr > 3000) & (fr < 8000); return 20 * np.log10(np.median(s[b]) / seg[np.argmin(abs(fr - f0))] + 1e-12)
    # pitch over time (vibrato): peak near f0 per frame
    fb = (fr > f0 * 0.8) & (fr < f0 * 1.25); tr = fr[fb][M[fb].argmax(0)]
    live = M[fb].max(0) > M[fb].max() * 0.05
    # envelope 2ms of full signal and of HP 3k part
    h = int(sr * 0.002); n = len(x) // h; e = 20 * np.log10(np.sqrt((x[: n * h].reshape(n, h) ** 2).mean(1)) + 1e-9)
    print(f"{f}: f0 {f0:.0f} Hz | harmonics dB {np.round(harm,1)} | noise 3–8k re f0: 0–20ms {nz(0,.02):.1f} 20–60 {nz(.02,.06):.1f} 100–200 {nz(.1,.2):.1f} | pitch range while live {np.percentile(tr[live],5):.0f}–{np.percentile(tr[live],95):.0f} | peak at {e.argmax()*2} ms, -20dB at {np.where(e>e.max()-20)[0][-1]*2} ms")
