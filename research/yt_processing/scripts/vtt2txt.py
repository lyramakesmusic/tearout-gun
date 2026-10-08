import re, sys, glob, os
src, dst = sys.argv[1], sys.argv[2]
files = {}
for f in glob.glob(os.path.join(src, "*.vtt")):
    base = os.path.basename(f)
    vid, lang = base.split(".")[0], ".".join(base.split(".")[1:-1])
    pri = {"en": 0, "en-orig": 1}.get(lang, 2)
    if vid not in files or pri < files[vid][0]:
        files[vid] = (pri, f)
for vid, (_, f) in files.items():
    out, prev = [], None
    for line in open(f, encoding="utf-8"):
        line = line.strip()
        if not line or "-->" in line or line.startswith(("WEBVTT", "Kind:", "Language:", "NOTE")) or line.isdigit():
            continue
        line = re.sub(r"<[^>]+>", "", line).replace("&nbsp;", " ").replace("&amp;", "&").strip()
        if not line or line == prev:
            continue
        if out and line in out[-3:]:
            continue
        out.append(line); prev = line
    text = " ".join(out)
    text = re.sub(r"\s+", " ", text)
    # wrap ~ 100 words per line
    w = text.split(" ")
    with open(os.path.join(dst, vid + ".txt"), "w") as fh:
        fh.write("\n".join(" ".join(w[i:i+100]) for i in range(0, len(w), 100)))
    print(vid, len(w))
