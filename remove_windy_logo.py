import os
import zlib
import struct

# 1. Create a 100% transparent PNG to overwrite sprite_64.png
def create_transparent_png(width, height):
    # PNG signature
    png = b'\x89PNG\r\n\x1a\n'
    # IHDR chunk
    ihdr_data = struct.pack('>IIBBBBB', width, height, 8, 6, 0, 0, 0)
    ihdr_crc = struct.pack('>I', zlib.crc32(b'IHDR' + ihdr_data))
    png += struct.pack('>I', len(ihdr_data)) + b'IHDR' + ihdr_data + ihdr_crc
    # IDAT chunk (transparent raw scanlines: filter byte 0 + RGBA 0,0,0,0)
    raw_data = b''.join([b'\x00' + (b'\x00\x00\x00\x00' * width) for _ in range(height)])
    idat_data = zlib.compress(raw_data)
    idat_crc = struct.pack('>I', zlib.crc32(b'IDAT' + idat_data))
    png += struct.pack('>I', len(idat_data)) + b'IDAT' + idat_data + idat_crc
    # IEND chunk
    iend_crc = struct.pack('>I', zlib.crc32(b'IEND'))
    png += struct.pack('>I', 0) + b'IEND' + iend_crc
    return png

sprite_path = r"C:\Users\shiva\Windy_App\extracted\assets\public\img\sprite_64.png"
with open(sprite_path, 'wb') as f:
    f.write(create_transparent_png(1728, 32))
print("sprite_64.png replaced with 100% transparent PNG.")

# 2. Modern AeroCast SVG for all logo variants
aerocast_svg = '''<svg width="267px" height="68px" viewBox="0 0 267 68" version="1.1" xmlns="http://www.w3.org/2000/svg">
    <defs>
        <linearGradient id="aeroGrad" x1="0%" y1="0%" x2="100%" y2="100%">
            <stop offset="0%" stop-color="#38bdf8" />
            <stop offset="100%" stop-color="#0284c7" />
        </linearGradient>
    </defs>
    <g transform="translate(6, 12)">
        <path d="M12 24 C4 24 0 18 0 12 C0 5 6 0 13 0 C19 0 24 4 25 10 C27 9 30 9 32 11 C35 13 36 17 35 20 C34 23 31 24 28 24 Z" fill="url(#aeroGrad)" opacity="0.95"/>
        <path d="M4 30 Q 18 28 32 30 T 44 30" stroke="#38bdf8" stroke-width="3" stroke-linecap="round" fill="none"/>
        <path d="M0 36 Q 14 34 26 36 T 38 36" stroke="#bae6fd" stroke-width="2.5" stroke-linecap="round" fill="none"/>
    </g>
    <text x="62" y="44" font-family="-apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif" font-size="34" font-weight="800" fill="#FFFFFF">
        Aero<tspan fill="#38bdf8">Cast</tspan>
    </text>
</svg>'''

logo_files = [
    r"C:\Users\shiva\Windy_App\extracted\assets\public\img\logo201802\logo-text-windycom-white.svg",
    r"C:\Users\shiva\Windy_App\extracted\assets\public\img\logo-full-windy-gray-v3.svg",
    r"C:\Users\shiva\Windy_App\extracted\assets\public\img\logo-windy-premium.svg",
    r"C:\Users\shiva\Windy_App\extracted\assets\public\img\logo-windycom-outline.svg"
]

for lp in logo_files:
    os.makedirs(os.path.dirname(lp), exist_ok=True)
    with open(lp, 'w', encoding='utf-8') as f:
        f.write(aerocast_svg)
    print(f"Updated logo file: {os.path.basename(lp)}")

# 3. Add explicit CSS in mobile.css and index.html to hide any lingering w-sprite or Windy logo
css_rule = "\n.w-sprite, .animated-windy-logo .w-sprite { display: none !important; width: 0 !important; height: 0 !important; background-image: none !important; }\n"

css_path = r"C:\Users\shiva\Windy_App\extracted\assets\public\v\51.3.2.mob.3c81\mobile.css"
with open(css_path, 'r', encoding='utf-8') as f:
    css = f.read()

if ".w-sprite" not in css:
    css += css_rule
    with open(css_path, 'w', encoding='utf-8') as f:
        f.write(css)
    print("mobile.css updated with .w-sprite hiding rules.")

index_path = r"C:\Users\shiva\Windy_App\extracted\assets\public\index.html"
with open(index_path, 'r', encoding='utf-8') as f:
    html = f.read()

hide_style = "<style>.w-sprite, .animated-windy-logo .w-sprite { display: none !important; width: 0 !important; height: 0 !important; background-image: none !important; }</style>"
if hide_style not in html:
    html = html.replace("</head>", f"{hide_style}</head>")
    with open(index_path, 'w', encoding='utf-8') as f:
        f.write(html)
    print("index.html updated with inline w-sprite hiding rules.")

print("All Windy logos and the red spiral sprite completely eradicated!")
