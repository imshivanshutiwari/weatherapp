import os

menu_path = r"C:\Users\shiva\Windy_App\extracted\assets\public\v\51.3.2.mob.3c81\plugins\menu.js"
with open(menu_path, "r", encoding="utf-8") as f:
    text = f.read()

idx1 = text.find("function Nt(e,t){")
idx2 = text.find("var Pt=B(")

if idx1 != -1 and idx2 != -1:
    old_nt = text[idx1:idx2]
    new_nt = "function Nt(e,t){if(new.target)return T(Y({component:Nt},e));var i={$set:R,$on:(e,n)=>I(t,e,n)};return H(i)}"
    text = text[:idx1] + new_nt + text[idx2:]
    with open(menu_path, "w", encoding="utf-8") as f:
        f.write(text)
    print("Successfully patched Nt in menu.js!")
else:
    print("Failed to find boundaries:", idx1, idx2)

# Also update mobile.css and index.html
css_rule = """
.bottom-section, 
.build-info, 
.app-stores, 
.svelte-mf1wo8,
[class*="bottom-section"], 
[class*="build-info"],
[class*="app-stores"],
section.mm-section:has(.bottom-section) {
    display: none !important;
    visibility: hidden !important;
    height: 0 !important;
    padding: 0 !important;
    margin: 0 !important;
    overflow: hidden !important;
}
"""

css_path = r"C:\Users\shiva\Windy_App\extracted\assets\public\v\51.3.2.mob.3c81\mobile.css"
with open(css_path, "r", encoding="utf-8") as f:
    css = f.read()

if ".bottom-section" not in css:
    css += css_rule
    with open(css_path, "w", encoding="utf-8") as f:
        f.write(css)
    print("Added CSS hide rule to mobile.css")

index_path = r"C:\Users\shiva\Windy_App\extracted\assets\public\index.html"
with open(index_path, "r", encoding="utf-8") as f:
    html = f.read()

if ".bottom-section" not in html:
    html = html.replace("</head>", f"<style>{css_rule}</style></head>")
    with open(index_path, "w", encoding="utf-8") as f:
        f.write(html)
    print("Added CSS hide rule to index.html")

print("All unwanted footer & info elements removed successfully!")
