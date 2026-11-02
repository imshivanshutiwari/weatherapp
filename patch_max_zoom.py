import os
import re

js_path = r"C:\Users\shiva\Windy_App\extracted\assets\public\v\51.3.2.mob.3c81\mobile.js"

with open(js_path, 'r', encoding='utf-8') as f:
    js = f.read()

# 1. Patch Jb (country zoom restriction for India/Vietnam: 11 -> 19)
# Old: const qb=e=>e===`vn`||e===`in`,Jb=qb(N.get(`country`))?11:16
old_jb = "qb=e=>e===`vn`||e===`in`,Jb=qb(N.get(`country`))?11:16"
new_jb = "qb=e=>!1,Jb=19"
if old_jb in js:
    js = js.replace(old_jb, new_jb)
    print("Patched Jb: maxZoom is now unlocked to level 19 (Street / Deep City level)!")
else:
    # Alternative regex match
    js, n = re.subn(r'Jb=qb\(N\.get\(`country`\)\)\?11:16', 'Jb=19', js)
    print(f"Patched Jb via regex: {n} replacements.")

# 2. Patch labels layer maxZoom so city names and labels don't vanish past zoom 11
old_labels = "minZoom:3,maxZoom:11,pane:`markerPane`,className:`labels-layer`"
new_labels = "minZoom:3,maxZoom:19,pane:`markerPane`,className:`labels-layer`"
if old_labels in js:
    js = js.replace(old_labels, new_labels)
    print("Patched labels layer: labels will stay visible up to zoom level 19!")
else:
    js, n = re.subn(r'minZoom:3,maxZoom:11,pane:`markerPane`', 'minZoom:3,maxZoom:19,pane:`markerPane`', js)
    print(f"Patched labels maxZoom via regex: {n} replacements.")

# 3. Patch webgl-land maxZoomSource so terrain and land render at high zoom
old_land = "{minZoomSource:0,maxZoomSource:11,url:e,variant:Cb[t]}"
new_land = "{minZoomSource:0,maxZoomSource:19,url:e,variant:Cb[t]}"
if old_land in js:
    js = js.replace(old_land, new_land)
    print("Patched webgl-land: land textures now render up to zoom 19!")
else:
    js, n = re.subn(r'maxZoomSource:11,url:e,variant:Cb\[t\]', 'maxZoomSource:19,url:e,variant:Cb[t]', js)
    print(f"Patched webgl-land via regex: {n} replacements.")

# 4. Patch satellite maxTileZoom from 11 to 19
js, n_sat = re.subn(r'maxTileZoom:\{free:7,premium:11\}', 'maxTileZoom:{free:19,premium:19}', js)
print(f"Patched satellite maxTileZoom: {n_sat} replacements.")

with open(js_path, 'w', encoding='utf-8') as f:
    f.write(js)

print("All zoom restrictions unlocked successfully to MAX ZOOM (Level 19)!")
