# "by lyraaaa", drawn as pen strokes (cubic curves on a baseline at y=100, x-height 72, ascender 22,
# descender to ~140), stroked with a slanted nib. Writes handsig.svg, handsig.png and handsig.rgba (+ size).
import pathlib, subprocess
here = pathlib.Path(__file__).parent

def a(x):  # one cursive a, entering and leaving on the baseline
    return (f"C{x+6},100 {x+12},86 {x+20},74 "
            f"C{x+14},68 {x+5},76 {x+5},88 C{x+5},100 {x+15},98 {x+20},86 "
            f"C{x+20},80 {x+21},76 {x+21},72 C{x+20},84 {x+19},98 {x+28},100 ")
def b(x):
    return (f"C{x+8},96 {x+18},60 {x+18},34 C{x+18},20 {x+8},22 {x+8},40 "
            f"C{x+8},62 {x+7},84 {x+8},100 C{x+12},86 {x+22},74 {x+26},80 C{x+30},88 {x+24},100 {x+14},98 "
            f"C{x+22},98 {x+30},94 {x+36},88 ")
def y(x, tail=False):
    return (f"C{x+4},84 {x+6},76 {x+8},72 C{x+6},86 {x+8},98 {x+16},98 C{x+22},98 {x+26},84 {x+28},72 "
            f"C{x+26},96 {x+24},122 {x+16},136 C{x+10},146 {x+2},142 {x+8},128 C{x+14},116 {x+30},106 {x+42},100 ")
def l(x):
    return (f"C{x+10},90 {x+24},40 {x+22},26 C{x+20},14 {x+8},24 {x+10},54 C{x+11},74 {x+12},94 {x+20},100 ")
def r(x):
    return (f"C{x+4},92 {x+6},80 {x+8},72 C{x+10},70 {x+14},76 {x+19},72 C{x+18},84 {x+18},96 {x+24},100 ")

by = "M0,100 " + b(0) + y(36)
# lyraaaa, written as one stroke with a long rising tail
x0 = 74
sig = f"M{x0 - 6},100 " + l(x0) + y(x0 + 20) + r(x0 + 62)
x = x0 + 86
for _ in range(4):
    sig += a(x); x += 28
sig += f"C{x + 14},100 {x + 34},90 {x + 56},70"
W = x + 62
def svg(ink, w=2.6):
    return f'''<svg xmlns="http://www.w3.org/2000/svg" viewBox="-6 10 {W + 12} 140" width="{(W + 12) * 2}" height="280">
<g fill="none" stroke="{ink}" stroke-linecap="round" stroke-linejoin="round" transform="skewX(-12) translate(18,0)">
<path d="{by}" stroke-width="{w * 1.1}" transform="translate(14,36) scale(0.64)"/>
<path d="{sig}" stroke-width="{w}"/>
<path d="{sig}" stroke-width="{w * 0.45}" transform="translate(0.7,-0.5)"/>
</g></svg>
'''
for name, ink in [('handsig', '#000000'), ('handsig-light', '#ffffff')]:
    (here / f'{name}.svg').write_text(svg(ink))
    subprocess.run(['rsvg-convert', '-h', '280', '-o', str(here / f'{name}.png'), str(here / f'{name}.svg')], check=True)
subprocess.run(['magick', str(here / 'handsig-light.png'), '-depth', '8', 'rgba:' + str(here / 'handsig.rgba')], check=True)
w, h = subprocess.run(['magick', 'identify', '-format', '%w %h', str(here / 'handsig-light.png')], check=True, capture_output=True, text=True).stdout.split()
(here / 'handsig.size').write_text(f'{w} {h}\n')
print('size', w, h)
