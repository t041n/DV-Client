import zipfile
import os

os.chdir('/mnt/d/projects/dv-client')
zip_path = "dist/DigitalVigital-Testing-Portable.zip"
if os.path.exists(zip_path):
    os.remove(zip_path)
with zipfile.ZipFile(zip_path, "w", zipfile.ZIP_DEFLATED) as z:
    z.write("dist/testing/DigitalVigitalLauncher.exe", "DigitalVigitalLauncher.exe")
    z.write("dist/testing/dv_core.dll", "dv_core.dll")
    z.write("dist/testing/WebView2Loader.dll", "WebView2Loader.dll")
    z.write("dist/testing/logo.png", "logo.png")
    z.write("dist/testing/logo.ico", "logo.ico")
    z.write("README.md", "README.md")
    z.write("LICENSE", "LICENSE")

print(f"Created {zip_path}: {os.path.getsize(zip_path)} bytes")
