# The "lyraaaa" signature as glyph outlines (Mrs Saint Delafield, OFL): signature.svg, plus PNGs in light and
# dark ink on transparent backgrounds.
import pathlib, subprocess
from fontTools.ttLib import TTFont
from fontTools.pens.svgPathPen import SVGPathPen
from fontTools.pens.boundsPen import BoundsPen
here = pathlib.Path(__file__).parent
f = TTFont(here / 'MrsSaintDelafield-Regular.ttf')
gs, cmap, hmtx = f.getGlyphSet(), f.getBestCmap(), f['hmtx']
text = 'lyraaaa'
paths, x = [], 0
bp = BoundsPen(gs)
for ch in text:
    g = cmap[ord(ch)]
    pen = SVGPathPen(gs)
    gs[g].draw(pen)
    paths.append(f'<path transform="translate({x},0)" d="{pen.getCommands()}"/>')
    from fontTools.pens.transformPen import TransformPen
    gs[g].draw(TransformPen(bp, (1, 0, 0, 1, x, 0)))
    x += hmtx[g][0]
x0, y0, x1, y1 = bp.bounds
pad = 40
w, h = x1 - x0 + 2 * pad, y1 - y0 + 2 * pad
for name, ink in [('signature', '#000000'), ('signature-light', '#ffffff')]:
    svg = f'<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 {w:.0f} {h:.0f}" width="{w:.0f}" height="{h:.0f}"><g fill="{ink}" transform="translate({pad - x0:.0f},{y1 + pad:.0f}) scale(1,-1)">{"".join(paths)}</g></svg>\n'
    (here / f'{name}.svg').write_text(svg)
    subprocess.run(['rsvg-convert', '-h', '400', '-o', str(here / f'{name}.png'), str(here / f'{name}.svg')], check=True)
print('wrote', [p.name for p in sorted(here.glob('signature*'))])
