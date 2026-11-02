import os
import re

hindi_title = "भारतीय मारुत केन्द्र"
full_title = "भारतीय मारुत केन्द्र - मौसम एवं वायु मानचित्र"

# 1. Generate much bigger, bolder Devanagari SVG logo (font size 34px, thick strokes)
bmk_svg = '''<svg width="380px" height="75px" viewBox="0 0 380 75" version="1.1" xmlns="http://www.w3.org/2000/svg">
    <defs>
        <linearGradient id="marutGrad" x1="0%" y1="0%" x2="100%" y2="100%">
            <stop offset="0%" stop-color="#38bdf8" />
            <stop offset="100%" stop-color="#0284c7" />
        </linearGradient>
        <filter id="shadow" x="-10%" y="-10%" width="120%" height="120%">
            <feDropShadow dx="0" dy="2" stdDeviation="2" flood-color="#000000" flood-opacity="0.6"/>
        </filter>
    </defs>
    <!-- Weather & Wind Vortex Icon -->
    <g transform="translate(10, 10)" filter="url(#shadow)">
        <path d="M18 36 C6 36 0 27 0 18 C0 8 9 0 19 0 C28 0 36 6 37 15 C41 14 45 14 48 16 C53 19 54 25 53 29 C51 34 46 36 42 36 Z" fill="url(#marutGrad)" opacity="0.95"/>
        <path d="M6 45 Q 27 42 48 45 T 66 45" stroke="#38bdf8" stroke-width="4.5" stroke-linecap="round" fill="none"/>
        <path d="M0 54 Q 20 51 38 54 T 56 54" stroke="#bae6fd" stroke-width="4" stroke-linecap="round" fill="none"/>
    </g>
    <!-- Big Bold Hindi Typography -->
    <text x="82" y="50" font-family="'Noto Sans Devanagari', 'Mangal', 'Nirmala UI', 'Kohinoor Devanagari', 'Segoe UI', sans-serif" font-size="34" font-weight="900" letter-spacing="0.5" fill="#FFFFFF" filter="url(#shadow)">
        भारतीय <tspan fill="#38bdf8">मारुत केन्द्र</tspan>
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
        f.write(bmk_svg)

# 2. Add CSS scaling to make the logo larger on screen
css_rule = """
.mobile-calendar__logo, #device-mobile .mobile-calendar__logo, #device-tablet .mobile-calendar__logo {
    transform: scale(1.35) !important;
    transform-origin: left bottom !important;
    left: 12px !important;
    top: -52px !important;
}
.mobile-calendar__logo img, .animated-windy-logo img {
    height: 38px !important;
    width: auto !important;
    filter: drop-shadow(0 2px 4px rgba(0,0,0,0.5)) !important;
}
"""

css_path = r"C:\Users\shiva\Windy_App\extracted\assets\public\v\51.3.2.mob.3c81\mobile.css"
with open(css_path, 'r', encoding='utf-8') as f:
    css = f.read()

# Replace or append rule
if ".mobile-calendar__logo" in css:
    css = re.sub(r'\.mobile-calendar__logo.*?\}\n', '', css, flags=re.DOTALL)
css += css_rule

with open(css_path, 'w', encoding='utf-8') as f:
    f.write(css)

# Also update index.html with inline style override so it takes effect instantly
index_path = r"C:\Users\shiva\Windy_App\extracted\assets\public\index.html"
with open(index_path, 'r', encoding='utf-8') as f:
    html = f.read()

style_override = f"<style>{css_rule}</style>"
if style_override not in html:
    html = html.replace("</head>", f"{style_override}</head>")
    with open(index_path, 'w', encoding='utf-8') as f:
        f.write(html)

print("Logo enlarged to 34px with 1.35x scale successfully.")
