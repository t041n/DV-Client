import zipfile
import os

os.chdir('/mnt/d/projects/dv-client')
release_zip = "dist/DigitalVigital-v1.0.2-windows-x64-portable.zip"

if os.path.exists(release_zip):
    os.remove(release_zip)
with zipfile.ZipFile(release_zip, "w", zipfile.ZIP_DEFLATED) as z:
    if os.path.exists("dv-launcher-app.exe"):
        z.write("dv-launcher-app.exe", "dv-launcher-app.exe")
        z.write("dv-launcher-app.exe", "DigitalVigitalLauncher.exe")
    if os.path.exists("dv_core.dll"):
        z.write("dv_core.dll", "dv_core.dll")
    if os.path.exists("WebView2Loader.dll"):
        z.write("WebView2Loader.dll", "WebView2Loader.dll")
    if os.path.exists("assets/logo.png"):
        z.write("assets/logo.png", "logo.png")
    if os.path.exists("assets/logo.ico"):
        z.write("assets/logo.ico", "logo.ico")
    if os.path.exists("assets/mods/dvclient-fabric-1.0.0.jar"):
        z.write("assets/mods/dvclient-fabric-1.0.0.jar", "assets/mods/dvclient-fabric-1.0.0.jar")
    if os.path.exists("README.md"):
        z.write("README.md", "README.md")
    if os.path.exists("LICENSE"):
        z.write("LICENSE", "LICENSE")

print(f"Created {release_zip}: {os.path.getsize(release_zip)} bytes")
