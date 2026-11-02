p = r"C:\Users\shiva\Windy_App\extracted\assets\public\v\51.3.2.mob.3c81\mobile.css"
with open(p, "r", encoding="utf-8") as f:
    c = f.read()
if 'section.mm-section[data-icon="m"]' not in c:
    c = c.replace(".bottom-section,", 'section.mm-section[data-icon="m"],\n.bottom-section,')
    with open(p, "w", encoding="utf-8") as f:
        f.write(c)

p2 = r"C:\Users\shiva\Windy_App\extracted\assets\public\index.html"
with open(p2, "r", encoding="utf-8") as f:
    c2 = f.read()
if 'section.mm-section[data-icon="m"]' not in c2:
    c2 = c2.replace(".bottom-section,", 'section.mm-section[data-icon="m"],\n.bottom-section,')
    with open(p2, "w", encoding="utf-8") as f:
        f.write(c2)

print("Updated mobile.css and index.html safely!")
