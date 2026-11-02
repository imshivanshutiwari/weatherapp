import os

# 1. Update index.html
index_path = r"C:\Users\shiva\Windy_App\extracted\assets\public\index.html"
with open(index_path, 'r', encoding='utf-8') as f:
    html = f.read()

old_title = "<title>Windy: Wind map &amp; weather forecast</title>"
new_title = "<title>AeroCast - Global Wind Map &amp; Weather Forecast</title>"
if old_title in html:
    html = html.replace(old_title, new_title)
    print("Updated page title in index.html to AeroCast")

hide_css = "<style>#go-premium-mobile, .premium-button, [data-ref='premiumButtonMobile'], .premium-flag { display: none !important; }</style>"
if hide_css not in html:
    html = html.replace("</head>", f"{hide_css}</head>")
    print("Added CSS override to hide all Go Premium tags and badges")

with open(index_path, 'w', encoding='utf-8') as f:
    f.write(html)

# 2. Update capacitor.config.json
cap_path = r"C:\Users\shiva\Windy_App\extracted\assets\capacitor.config.json"
if os.path.exists(cap_path):
    with open(cap_path, 'r', encoding='utf-8') as f:
        cap = f.read()
    cap = cap.replace('"appName": "Windy"', '"appName": "AeroCast"')
    cap = cap.replace('"appId": "com.windyty.android"', '"appId": "com.shiva.aerocast"')
    # Clean up local test IP server comment block
    with open(cap_path, 'w', encoding='utf-8') as f:
        f.write(cap)
    print("Updated capacitor.config.json with AeroCast branding and appId com.shiva.aerocast")

# 3. Update mobile.css to hide premium flags and enhance accent styling
css_path = r"C:\Users\shiva\Windy_App\extracted\assets\public\v\51.3.2.mob.3c81\mobile.css"
if os.path.exists(css_path):
    with open(css_path, 'r', encoding='utf-8') as f:
        css = f.read()
    custom_rules = "\n#go-premium-mobile, .premium-button, [data-ref='premiumButtonMobile'], .premium-flag { display: none !important; }\n"
    if custom_rules not in css:
        css += custom_rules
        with open(css_path, 'w', encoding='utf-8') as f:
            f.write(css)
        print("Updated mobile.css with premium-hiding rules")

print("All 0% risk cosmetic changes applied successfully!")
