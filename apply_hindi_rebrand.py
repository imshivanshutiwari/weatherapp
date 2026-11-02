import os
import re

hindi_title = "भारतीय मारुत केन्द्र"
full_title = "भारतीय मारुत केन्द्र - मौसम एवं वायु मानचित्र"

# 1. Generate crisp Devanagari SVG logo
bmk_svg = '''<svg width="280px" height="68px" viewBox="0 0 280 68" version="1.1" xmlns="http://www.w3.org/2000/svg">
    <defs>
        <linearGradient id="marutGrad" x1="0%" y1="0%" x2="100%" y2="100%">
            <stop offset="0%" stop-color="#38bdf8" />
            <stop offset="100%" stop-color="#0284c7" />
        </linearGradient>
    </defs>
    <g transform="translate(6, 12)">
        <path d="M12 24 C4 24 0 18 0 12 C0 5 6 0 13 0 C19 0 24 4 25 10 C27 9 30 9 32 11 C35 13 36 17 35 20 C34 23 31 24 28 24 Z" fill="url(#marutGrad)" opacity="0.95"/>
        <path d="M4 30 Q 18 28 32 30 T 44 30" stroke="#38bdf8" stroke-width="3" stroke-linecap="round" fill="none"/>
        <path d="M0 36 Q 14 34 26 36 T 38 36" stroke="#bae6fd" stroke-width="2.5" stroke-linecap="round" fill="none"/>
    </g>
    <text x="56" y="42" font-family="'Noto Sans Devanagari', 'Mangal', 'Nirmala UI', 'Kohinoor Devanagari', 'Segoe UI', sans-serif" font-size="22" font-weight="700" fill="#FFFFFF">
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

# 2. Update index.html title
index_path = r"C:\Users\shiva\Windy_App\extracted\assets\public\index.html"
with open(index_path, 'r', encoding='utf-8') as f:
    html = f.read()

html = re.sub(r'<title>.*?</title>', f'<title>{full_title}</title>', html)

with open(index_path, 'w', encoding='utf-8') as f:
    f.write(html)

# 3. Update capacitor.config.json
cap_path = r"C:\Users\shiva\Windy_App\extracted\assets\capacitor.config.json"
if os.path.exists(cap_path):
    with open(cap_path, 'r', encoding='utf-8') as f:
        cap = f.read()
    cap = re.sub(r'"appName":\s*".*?"', f'"appName": "{hindi_title}"', cap)
    cap = re.sub(r'"appId":\s*".*?"', '"appId": "in.bmk.weather"', cap)
    with open(cap_path, 'w', encoding='utf-8') as f:
        f.write(cap)

print("Devanagari rebrand successfully completed.")
