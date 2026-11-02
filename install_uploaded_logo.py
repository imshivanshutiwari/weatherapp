import os
import base64
from io import BytesIO
from PIL import Image

src_img = r"C:\Users\shiva\.gemini\antigravity-cli\brain\ce0edd7c-e683-497d-ba16-12faf6638a4f\.user_uploaded\uploaded_media_1793624659792.png"
dest_dir = r"C:\Users\shiva\Windy_App\extracted\assets\public\img"
os.makedirs(dest_dir, exist_ok=True)

im = Image.open(src_img).convert("RGBA")

# 1. Save high-res PNG for direct web use
bmk_png_path = os.path.join(dest_dir, "bmk_logo.png")
im_resized = im.resize((512, 512), Image.Resampling.LANCZOS)
im_resized.save(bmk_png_path, "PNG", optimize=True)
print("Saved 512x512 bmk_logo.png")

# 2. Save favicon.ico
fav_path = os.path.join(dest_dir, "favicon.ico")
fav_img = im.resize((64, 64), Image.Resampling.LANCZOS)
fav_img.save(fav_path, format="ICO")
print("Saved favicon.ico")

# 3. Create base64 string for SVG embedding (256x256 for crisp rendering without bloat)
emb_img = im.resize((256, 256), Image.Resampling.LANCZOS)
buf = BytesIO()
emb_img.save(buf, format="PNG", optimize=True)
b64_data = base64.b64encode(buf.getvalue()).decode("utf-8")
data_uri = f"data:image/png;base64,{b64_data}"

# 4. Build SVG with circular emblem on left + bold Devanagari typography on right
svg_content = f'''<svg width="340px" height="70px" viewBox="0 0 340 70" version="1.1" xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink">
    <defs>
        <filter id="logoShadow" x="-10%" y="-10%" width="120%" height="120%">
            <feDropShadow dx="0" dy="2" stdDeviation="2" flood-color="#000000" flood-opacity="0.6"/>
        </filter>
    </defs>
    <!-- Circular Bhartiya Marut Kendra Official Emblem -->
    <g filter="url(#logoShadow)">
        <image href="{data_uri}" x="4" y="3" width="64" height="64" preserveAspectRatio="xMidYMid meet"/>
    </g>
    <!-- Big Bold Hindi Devanagari Typography -->
    <text x="78" y="46" font-family="'Noto Sans Devanagari', 'Mangal', 'Nirmala UI', 'Kohinoor Devanagari', 'Segoe UI', sans-serif" font-size="27" font-weight="900" letter-spacing="0.5" fill="#FFFFFF" filter="url(#logoShadow)">
        भारतीय <tspan fill="#38bdf8">मारुत केंद्र</tspan>
    </text>
</svg>'''

svg_files = [
    r"C:\Users\shiva\Windy_App\extracted\assets\public\img\logo201802\logo-text-windycom-white.svg",
    r"C:\Users\shiva\Windy_App\extracted\assets\public\img\logo-full-windy-gray-v3.svg",
    r"C:\Users\shiva\Windy_App\extracted\assets\public\img\logo-windy-premium.svg",
    r"C:\Users\shiva\Windy_App\extracted\assets\public\img\logo-windycom-outline.svg"
]

for sp in svg_files:
    os.makedirs(os.path.dirname(sp), exist_ok=True)
    with open(sp, 'w', encoding='utf-8') as f:
        f.write(svg_content)
    print(f"Updated SVG logo: {os.path.basename(sp)}")

# 5. Update mobile.css scaling so the emblem and text look big and prominent
css_path = r"C:\Users\shiva\Windy_App\extracted\assets\public\v\51.3.2.mob.3c81\mobile.css"
with open(css_path, 'r', encoding='utf-8') as f:
    css = f.read()

import re
css_rule = """
.mobile-calendar__logo, #device-mobile .mobile-calendar__logo, #device-tablet .mobile-calendar__logo {
    transform: scale(1.4) !important;
    transform-origin: left bottom !important;
    left: 10px !important;
    top: -56px !important;
}
.mobile-calendar__logo img, .animated-windy-logo img {
    height: 42px !important;
    width: auto !important;
    filter: drop-shadow(0 2px 6px rgba(0,0,0,0.6)) !important;
}
.w-sprite, .animated-windy-logo .w-sprite {
    display: none !important;
    width: 0 !important;
    height: 0 !important;
    background-image: none !important;
}
"""

if ".mobile-calendar__logo" in css:
    css = re.sub(r'\.mobile-calendar__logo.*?\}\n', '', css, flags=re.DOTALL)
css += css_rule

with open(css_path, 'w', encoding='utf-8') as f:
    f.write(css)

# Update index.html inline override as well
index_path = r"C:\Users\shiva\Windy_App\extracted\assets\public\index.html"
with open(index_path, 'r', encoding='utf-8') as f:
    html = f.read()

style_override = f"<style>{css_rule}</style>"
if style_override not in html:
    html = html.replace("</head>", f"{style_override}</head>")
    with open(index_path, 'w', encoding='utf-8') as f:
        f.write(html)

print("Official Emblem integrated everywhere successfully!")
