import sys, re, glob, os
src, dst = sys.argv[1], sys.argv[2]
os.makedirs(dst, exist_ok=True)
files = {}
for f in glob.glob(os.path.join(src, "*.vtt")):
    b = os.path.basename(f)
    vid = b.split(".en")[0]
    # prefer manual/en over en-orig
    files.setdefault(vid, []).append(f)
for vid, fs in files.items():
    fs.sort(key=lambda p: ("orig" in p))
    f = fs[0]
    lines = []
    for line in open(f, encoding="utf-8"):
        line = line.strip()
        if not line or "-->" in line or line.startswith(("WEBVTT", "Kind:", "Language:", "NOTE")) or line.isdigit():
            continue
        line = re.sub(r"<[^>]+>", "", line).strip()
        line = line.replace("&gt;", ">").replace("&lt;", "<").replace("&amp;", "&").replace("&nbsp;", " ")
        if not line: continue
        if lines and line == lines[-1]: continue
        lines.append(line)
    # remove rolling duplicates: drop line if it's contained at start of next etc.
    out = []
    for l in lines:
        if out and l.startswith(out[-1]):
            out[-1] = l; continue
        if out and out[-1].endswith(l): continue
        out.append(l)
    text = " ".join(out)
    text = re.sub(r"\s+", " ", text)
    # wrap
    words = text.split(" "); para=[]; cur=[]
    for w in words:
        cur.append(w)
        if len(cur) >= 120: para.append(" ".join(cur)); cur=[]
    if cur: para.append(" ".join(cur))
    open(os.path.join(dst, vid + ".txt"), "w").write("\n\n".join(para) + "\n")
    print(vid, len(words))
