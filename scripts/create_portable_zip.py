import zipfile
import os

os.chdir('/mnt/d/projects/dv-client')
release_zip = "dist/DigitalVigital-v1.0.1-windows-x64-portable.zip"

if os.path.exists(release_zip):
    os.remove(release_zip)
with zipfile.ZipFile(release_zip, "w", zipfile.ZIP_DEFLATED) as z:
    z.write("dist/DigitalVigitalLauncher.exe", "DigitalVigitalLauncher.exe")
    if os.path.exists("dist/dv_core.dll"):
        z.write("dist/dv_core.dll", "dv_core.dll")
    z.write("WebView2Loader.dll", "WebView2Loader.dll")
    z.write("assets/logo.png", "logo.png")
    z.write("assets/logo.ico", "logo.ico")
    z.write("README.md", "README.md")
    z.write("LICENSE", "LICENSE")

print(f"Created {release_zip}: {os.path.getsize(release_zip)} bytes")
