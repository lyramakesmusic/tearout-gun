import sys, re, glob, os
sub, vid = sys.argv[1], sys.argv[2]
maxn = int(sys.argv[3]) if len(sys.argv) > 3 else 14
fs = sorted(glob.glob(f"subs/{vid}.en*.vtt"), key=lambda p: "orig" in p)
txt = open(fs[0], encoding="utf-8").read()
cues = re.findall(r"(\d+):(\d+):(\d+)\.\d+ --> [^\n]*\n(.*?)(?:\n\n|\Z)", txt, re.S)
pri = [r"\blfo ?(1|one|2|two)?\b.*(like this|looks like|shape)", r"(envelope|shape).*(like this|looks like)", r"looks like (this|that)", r"like this", r"set (it )?(to|at)", r"right here", r"\bdrag", r"this (envelope|lfo|shape|curve)", r"\b(convolver|multipass|splitter|comb|filter)\b"]
hits = []
seen_text = set()
for h, m, s, body in cues:
    t = int(h)*3600+int(m)*60+int(s)
    b = re.sub(r"<[^>]+>", "", body).lower().replace("\n", " ")
    for rank, p in enumerate(pri):
        if re.search(p, b):
            hits.append((rank, t, b[:90])); break
hits.sort()
chosen = []
for rank, t, b in hits:
    if all(abs(t - c[1]) > 12 for c in chosen):
        chosen.append((rank, t, b))
    if len(chosen) >= maxn: break
for rank, t, b in sorted(chosen, key=lambda x: x[1]):
    print(t, rank, b)
