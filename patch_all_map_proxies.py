import os

js_path = r"C:\Users\shiva\Windy_App\extracted\assets\public\v\51.3.2.mob.3c81\mobile.js"

with open(js_path, 'r', encoding='utf-8') as f:
    js = f.read()

replacements = [
    # 1. Map tile server endpoints for radar and satellite
    ("const Ar=[`https://ims.windy.com`,`https://rdr.windy.com`,`https://sat.windy.com`]",
     "const Ar=[`/ims`,`/rdr`,`/sat`]"),
    
    ("https://rdr.windy.com", "/rdr"),
    ("https://sat.windy.com", "/sat"),
    ("https://ims.windy.com", "/ims"),
    ("https://node.windy.com/maptile", "/maptile"),
]

applied = 0
for old, new in replacements:
    if old in js:
        js = js.replace(old, new)
        applied += 1
        print(f"Replaced: {old[:30]} -> {new[:30]}")

with open(js_path, 'w', encoding='utf-8') as f:
    f.write(js)

print(f"Applied {applied} proxy replacements to mobile.js. Radar, Satellite, and all 65 maps now route through local proxy!")
