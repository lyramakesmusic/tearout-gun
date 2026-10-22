# Draws the "TEAROUT FARM" wordmark: blocky slab capitals with flared serifs, chamfered edges lit from the
# top left, an orange face. Writes title.svg and, with rsvg-convert, title.png next to this script.
import math, random, subprocess, pathlib

G = {  # glyphs: rings of (x, y) in a 100-high box, outer ring first, then holes
    'T': (90, [[(0,0),(90,0),(90,30),(76,30),(72,22),(57,22),(57,82),(66,86),(66,100),(24,100),(24,86),(33,82),(33,22),(18,22),(14,30),(0,30)]]),
    'E': (72, [[(0,0),(72,0),(72,28),(62,28),(58,21),(32,21),(32,41),(52,41),(52,59),(32,59),(32,79),(58,79),(62,72),(72,72),(72,100),(0,100),(0,88),(8,84),(8,16),(0,12)]]),
    'A': (88, [[(30,0),(58,0),(82,86),(88,89),(88,100),(56,100),(56,89),(60,86),(57,75),(31,75),(28,86),(32,89),(32,100),(0,100),(0,89),(6,86)],
               [(36,57),(52,57),(44,28)]]),
    'R': (86, [[(0,0),(58,0),(74,6),(82,16),(82,40),(74,50),(64,55),(80,84),(86,88),(86,100),(56,100),(56,89),(59,86),(46,60),(32,60),(32,84),(40,88),(40,100),(0,100),(0,88),(8,84),(8,16),(0,12)],
               [(32,20),(52,20),(58,26),(58,35),(52,41),(32,41)]]),
    'O': (84, [[(18,0),(66,0),(84,18),(84,82),(66,100),(18,100),(0,82),(0,18)],
               [(33,22),(51,22),(59,30),(59,70),(51,78),(33,78),(25,70),(25,30)]]),
    'U': (86, [[(0,0),(38,0),(38,12),(31,16),(31,70),(37,77),(49,77),(55,70),(55,16),(48,12),(48,0),(86,0),(86,12),(78,16),(78,82),(60,100),(26,100),(8,82),(8,16),(0,12)]]),
    'F': (72, [[(0,0),(72,0),(72,28),(62,28),(58,21),(32,21),(32,43),(52,43),(52,61),(32,61),(32,84),(42,88),(42,100),(0,100),(0,88),(8,84),(8,16),(0,12)]]),
    'M': (108, [[(0,0),(36,0),(54,40),(72,0),(108,0),(108,12),(100,16),(100,84),(108,88),(108,100),(72,100),(72,88),(78,84),(78,42),(60,78),(48,78),(30,42),(30,84),(36,88),(36,100),(0,100),(0,88),(8,84),(8,16),(0,12)]]),
}
TEXT, SPACE, TRACK, BEVEL = 'TEAROUT FARM', 34, 7, 6.5
# stencil bridges: gaps (x, y, w, h) cut through each glyph where a stencil holds its counters
GAP = 4.5
CUTS = {
    'T': [(0, 22 - GAP / 2, 90, GAP)],
    'E': [(32 - GAP / 2, 0, GAP, 100)],
    'F': [(32 - GAP / 2, 0, GAP, 100)],
    'A': [(44 - GAP / 2, 0, GAP, 28)],
    'R': [(32 - GAP / 2, 0, GAP, 60)],
    'O': [(42 - GAP / 2, 0, GAP, 22), (42 - GAP / 2, 78, GAP, 22)],
    'U': [(43 - GAP / 2, 77, GAP, 23)],
    'M': [(30 - GAP / 2, 0, GAP, 100), (78 - GAP / 2, 0, GAP, 100)],
}

def area(r): return sum(r[i][0] * r[(i + 1) % len(r)][1] - r[(i + 1) % len(r)][0] * r[i][1] for i in range(len(r))) / 2

def inset(ring, d, solid_inside):
    # offset every edge by d toward the material and meet neighbours at their line intersection
    s = 1 if (area(ring) > 0) == solid_inside else -1
    n = len(ring)
    lines = []
    for i in range(n):
        (x0, y0), (x1, y1) = ring[i], ring[(i + 1) % n]
        dx, dy = x1 - x0, y1 - y0; L = math.hypot(dx, dy)
        nx, ny = -dy / L * s, dx / L * s  # into the material
        lines.append(((x0 + nx * d, y0 + ny * d), (dx, dy), (-nx, -ny)))
    pts = []
    for i in range(n):
        (p, u, _), (q, v, _) = lines[i - 1], lines[i]
        den = u[0] * v[1] - u[1] * v[0]
        if abs(den) < 1e-9: pts.append(q); continue
        t = ((q[0] - p[0]) * v[1] - (q[1] - p[1]) * v[0]) / den
        pts.append((p[0] + u[0] * t, p[1] + u[1] * t))
    return pts, [l[2] for l in lines]

def mix(a, b, t): return '#%02x%02x%02x' % tuple(round(a[k] + (b[k] - a[k]) * t) for k in range(3))
DARK, LIGHT = (70, 26, 0), (255, 214, 150)
light = (-0.55, -0.83)

parts, x = [], 0.0
for ch in TEXT:
    if ch == ' ': x += SPACE; continue
    w, rings = G[ch]
    faces, facets = [], []
    for k, ring in enumerate(rings):
        ring = [(px + x, py) for px, py in ring]
        inner, normals = inset(ring, BEVEL, solid_inside=(k == 0))
        faces.append(inner)
        for i in range(len(ring)):
            j = (i + 1) % len(ring)
            nx, ny = normals[i]
            t = max(0.0, min(1.0, 0.5 + 0.5 * (nx * light[0] + ny * light[1])))
            quad = [ring[i], ring[j], inner[j], inner[i]]
            facets.append((quad, mix(DARK, LIGHT, t)))
    outline = ' '.join('M' + ' L'.join(f'{px:.2f},{py:.2f}' for px, py in [(p[0] + x, p[1]) for p in r]) + 'Z' for r in rings)
    face = ' '.join('M' + ' L'.join(f'{px:.2f},{py:.2f}' for px, py in r) + 'Z' for r in faces)
    cuts = ''.join(f'<rect x="{cx + x:.2f}" y="{cy - 4:.2f}" width="{cw:.2f}" height="{ch_ + 8:.2f}" fill="black"/>' for cx, cy, cw, ch_ in CUTS.get(ch, []))
    parts.append(f'<mask id="m{len(parts)}" maskUnits="userSpaceOnUse" x="-10" y="-10" width="2000" height="130"><rect x="-10" y="-10" width="2000" height="130" fill="white"/>{cuts}</mask><g mask="url(#m{len(parts)})">')
    parts.append(f'<path d="{outline}" fill="#2a1000" fill-rule="evenodd" stroke="#1a0a00" stroke-width="3" stroke-linejoin="round"/>')
    for quad, col in facets:
        parts.append(f'<path d="M{" L".join(f"{px:.2f},{py:.2f}" for px, py in quad)}Z" fill="{col}" stroke="{col}" stroke-width="0.6" stroke-linejoin="round"/>')
    parts.append(f'<path d="{face}" fill="url(#face)" fill-rule="evenodd"/>')
    parts.append(f'<path d="{face}" fill="url(#brush)" fill-rule="evenodd" opacity="0.35"/>')
    parts.append('</g>')
    x += w + TRACK
W = x - TRACK
random.seed(4)
strokes = ''.join(f'<line x1="{random.uniform(-20, W):.1f}" y1="{y:.1f}" x2="{random.uniform(0, W + 20):.1f}" y2="{y + random.uniform(-0.6, 0.6):.1f}" stroke="{random.choice(["#fff3dc", "#5a2200"])}" stroke-width="{random.uniform(0.3, 0.9):.2f}" opacity="{random.uniform(0.15, 0.5):.2f}"/>' for y in [random.uniform(0, 100) for _ in range(160)])
svg = f'''<svg xmlns="http://www.w3.org/2000/svg" viewBox="-4 -4 {W + 8:.1f} 108" width="{(W + 8) * 1.2:.0f}" height="{108 * 1.2:.0f}">
<defs>
<linearGradient id="face" x1="0" y1="0" x2="0" y2="1"><stop offset="0" stop-color="#ffb347"/><stop offset="0.5" stop-color="#f28a12"/><stop offset="1" stop-color="#c25a00"/></linearGradient>
<pattern id="brush" patternUnits="userSpaceOnUse" width="{W + 8:.1f}" height="108">{strokes}</pattern>
</defs>
{chr(10).join(parts)}
</svg>
'''
here = pathlib.Path(__file__).parent
(here / 'title.svg').write_text(svg)
subprocess.run(['rsvg-convert', '-h', '120', '-o', str(here / 'title.png'), str(here / 'title.svg')], check=True)
print('wrote', here / 'title.png', 'aspect', round((W + 8) / 108, 3))
# raw RGBA for the plugin, which embeds it without an image decoder
subprocess.run(['magick', str(here / 'title.png'), '-depth', '8', 'rgba:' + str(here / 'title.rgba')], check=True)
w, h = subprocess.run(['magick', 'identify', '-format', '%w %h', str(here / 'title.png')], check=True, capture_output=True, text=True).stdout.split()
(here / 'title.size').write_text(f'{w} {h}\n')
print('rgba', w, h)
