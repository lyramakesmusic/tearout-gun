# /// script
# dependencies = ["numpy", "scipy", "soundfile"]
# ///
"""Measure KFU gun stems: layer levels, envelopes, sub pitch dive, spectra, tonal balance."""
import json
import re
import sys
from pathlib import Path

import numpy as np
import soundfile as sf
from scipy import signal

ROOTS = [
    Path("/Users/lyra/Music/samples/KFU GUN KIT VOL 2.1/KFU GUN KIT VOL 2.1 GUN SORT"),
    Path("/Users/lyra/Music/samples/KFU BUG TOOLS/GUNS"),
    Path("/Users/lyra/Music/samples/KAIFU GUN KIT 1.6/KAIFU GUN KIT VOL 1.6"),
]
OUT = Path(__file__).parent / "kfu_features.jsonl"
NOTE = {n: i for i, n in enumerate("C C# D D# E F F# G G# A A# B".split())}


def role_of(name):
    u = name.upper()
    for key, role in [("PROCESSED", "PROC"), ("_SUB", "SUB"), ("_TR", "TR"), ("_BODY", "BODY"),
                      ("_FX", "BODY"), ("_REV", "REV")]:
        if key in u:
            return role
    return None


def db(x):
    return 20 * np.log10(np.maximum(x, 1e-12))


def env_rms(x, sr, win_ms=2.0):
    w = max(1, int(sr * win_ms / 1000))
    return np.sqrt(np.convolve(x ** 2, np.ones(w) / w, mode="same"))


def envelope_feats(mono, sr):
    e = env_rms(mono, sr)
    pk = e.max() + 1e-12
    i_pk = int(e.argmax())
    edb = db(e / pk)
    above = np.where(edb > -40)[0]
    end40 = above[-1] if len(above) else i_pk
    above20 = np.where(edb > -20)[0]
    end20 = above20[-1] if len(above20) else i_pk
    on = np.where(edb > -30)[0]
    onset = on[0] if len(on) else 0
    return dict(
        onset_ms=1000 * onset / sr,
        attack_ms=1000 * (i_pk - onset) / sr,
        len20_ms=1000 * (end20 - onset) / sr,
        len40_ms=1000 * (end40 - onset) / sr,
    )


def spectral_feats(mono, sr):
    f, P = signal.welch(mono, sr, nperseg=4096)
    P = P + 1e-20
    cen = float((f * P).sum() / P.sum())
    flat = float(np.exp(np.mean(np.log(P[f > 50]))) / np.mean(P[f > 50]))
    # 1/3-octave band energies 31.5 Hz..16k
    centers = 1000 * 2 ** (np.arange(-15, 13) / 3)
    bands = []
    for c in centers:
        lo, hi = c / 2 ** (1 / 6), c * 2 ** (1 / 6)
        m = (f >= lo) & (f < hi)
        bands.append(float(10 * np.log10(P[m].sum() + 1e-20)) if m.any() else -200.0)
    return dict(centroid_hz=cen, flatness=flat, third_oct_db=bands)


def sub_pitch_track(mono, sr, root_midi):
    """Instantaneous frequency of the (near-sinusoidal) sub via Hilbert on a lowpassed copy."""
    sos = signal.butter(4, 1500, "lp", fs=sr, output="sos")
    y = signal.sosfiltfilt(sos, mono)
    a = signal.hilbert(y)
    ph = np.unwrap(np.angle(a))
    inst = np.diff(ph) * sr / (2 * np.pi)
    amp = np.abs(a)[1:]
    # smooth over 1.5 ms, sample at 1 ms
    w = int(sr * 0.0015)
    inst = np.convolve(inst, np.ones(w) / w, mode="same")
    step = sr // 1000
    t = np.arange(0, len(inst), step)
    fr = inst[t]
    am = amp[t] / (amp.max() + 1e-12)
    valid = am > 0.05
    fr = np.where(valid, fr, np.nan)
    track = fr[: 400].tolist()  # first 400 ms
    root_hz = 440 * 2 ** ((root_midi - 69) / 12) if root_midi is not None else None
    v = fr[~np.isnan(fr)]
    out = dict(sub_track_hz_1ms=[None if np.isnan(x) else round(float(x), 1) for x in track])
    if len(v) > 20:
        start = float(np.nanmedian(fr[:3])) if np.isfinite(np.nanmedian(fr[:3])) else float(v[0])
        settle = float(np.nanmedian(fr[60:200])) if np.isfinite(np.nanmedian(fr[60:200])) else float(v[-1])
        out.update(sub_start_hz=start, sub_settle_hz=settle,
                   sub_dive_semitones=12 * np.log2(max(start, 1) / max(settle, 1)))
        # time to fall within 1 semitone of settle
        ok = np.where(np.abs(12 * np.log2(np.maximum(fr, 1) / settle)) < 1.0)[0]
        out["sub_dive_ms"] = float(ok[0]) if len(ok) else None
    out["root_hz"] = root_hz
    return out


def stereo_feats(x):
    if x.ndim == 1 or x.shape[1] == 1:
        return dict(side_mid_db=-120.0)
    m = (x[:, 0] + x[:, 1]) / 2
    s = (x[:, 0] - x[:, 1]) / 2
    return dict(side_mid_db=float(db(np.sqrt(np.mean(s ** 2))) - db(np.sqrt(np.mean(m ** 2)))))


def parse_root(folder):
    m = re.search(r"_([A-G]#?)(?:$|[+ ])", folder.strip())
    if not m:
        return None
    return 36 + NOTE[m.group(1)]  # octave unknown; used only for pitch class


def main():
    rows = []
    for root in ROOTS:
        for gdir in sorted(p for p in root.iterdir() if p.is_dir()):
            if gdir.name in ("TYPESORT", "NON_GUN", "Ableton Folder Info"):
                continue
            wavs = [w for w in gdir.glob("*.wav")]
            if not wavs:
                continue
            root_midi = parse_root(gdir.name)
            for w in sorted(wavs):
                role = role_of(w.stem)
                if role is None:
                    continue
                x, sr = sf.read(w, always_2d=True)
                mono = x.mean(axis=1)
                row = dict(kit=root.name, gun=gdir.name, file=w.name, role=role, sr=sr,
                           dur_ms=1000 * len(mono) / sr,
                           peak_db=float(db(np.abs(x).max())),
                           rms_db=float(db(np.sqrt(np.mean(mono ** 2)))))
                row["crest_db"] = row["peak_db"] - row["rms_db"]
                row.update(envelope_feats(mono, sr))
                row.update(spectral_feats(mono, sr))
                row.update(stereo_feats(x))
                if role == "SUB":
                    row.update(sub_pitch_track(mono, sr, root_midi))
                rows.append(row)
    with OUT.open("w") as f:
        for r in rows:
            f.write(json.dumps(r) + "\n")
    print(f"wrote {len(rows)} rows to {OUT}", file=sys.stderr)


if __name__ == "__main__":
    main()
