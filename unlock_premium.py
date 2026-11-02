import os

js_path = r"C:\Users\shiva\Windy_App\extracted\assets\public\v\51.3.2.mob.3c81\mobile.js"

with open(js_path, 'r', encoding='utf-8') as f:
    content = f.read()

# 1. ju=()=>N.get(`subscription`)!==null -> ju=()=>true
target1 = "ju=()=>N.get(`subscription`)!==null"
assert target1 in content, "target1 not found"
content = content.replace(target1, "ju=()=>true")
print("Target 1 replaced: ju() now always returns true (Full 10-day forecast & 1-hour step enabled).")

# 2. Default subscription store value
target2 = "subscription:{def:null,allowed:e=>!0,save:!0,nativeSync:!0}"
assert target2 in content, "target2 not found"
content = content.replace(target2, 'subscription:{def:{tier:"premium",validUntil:4102444800000},allowed:e=>!0,save:!0,nativeSync:!0}')
print("Target 2 replaced: default subscription store is now active premium.")

# 3. Initial xr flag
target3 = "xr=!!N.get(`subscription`),xr||N.once(`subscription`,e=>xr=!!e);"
assert target3 in content, "target3 not found"
content = content.replace(target3, "xr=true;")
print("Target 3 replaced: xr premium flag initialized to true.")

with open(js_path, 'w', encoding='utf-8') as f:
    f.write(content)

# 4. Patch _shared-premium-only-wrapper.js so click disabler never blocks
wrapper_path = r"C:\Users\shiva\Windy_App\extracted\assets\public\v\51.3.2.mob.3c81\plugins\_shared-premium-only-wrapper.js"
with open(wrapper_path, 'r', encoding='utf-8') as f:
    w_content = f.read()

# Replace pointer-events condition: D()?`auto`:`none` -> `auto`
if "D()?`auto`:`none`" in w_content:
    w_content = w_content.replace("D()?`auto`:`none`", "`auto`")
    print("Premium wrapper click disabler updated to allow all clicks.")

with open(wrapper_path, 'w', encoding='utf-8') as f:
    f.write(w_content)

# 5. Patch mobile.css for font 404
css_path = r"C:\Users\shiva\Windy_App\extracted\assets\public\v\51.3.2.mob.3c81\mobile.css"
with open(css_path, 'r', encoding='utf-8') as f:
    css_content = f.read()

old_font = 'src:url(iconfont.woff2)format("woff2"),url(iconfont.woff)format("woff"),url(iconfont.ttf)format("truetype")'
if old_font in css_content:
    css_content = css_content.replace(old_font, 'src:url(iconfont.ttf)format("truetype")')
    with open(css_path, 'w', encoding='utf-8') as f:
        f.write(css_content)
    print("mobile.css updated to eliminate 404 font errors.")

print("All features unlocked successfully!")
