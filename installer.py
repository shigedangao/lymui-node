import requests
import platform
import sys, os
import zipfile
import io, shutil

_platform = platform.system()
_version = "1.2.1"
_bin_path = "lymui/bin"

def update_progress(progress):
  sys.stdout.write("\rProgress: [{0:50s}] {1:.1f}%".format('#' * int(progress * 50), progress * 100))

if _platform.lower() == "darwin":
  o = "osx"
elif _platform.lower() == "linux":
  o = "linux"
else:
  sys.exit("platform", _platform, " is not supported")

update_progress(0.1)
link = "https://github.com/MarcInthaamnouay/lymui/releases/download/v" + _version + "-" + o +"/liblymui-"+ o +"-"+ _version +".zip"

# Get the zip
r = requests.get(link, stream=True)
update_progress(0.3)

zfile = zipfile.ZipFile(io.BytesIO(r.content))
zfile.extractall()
update_progress(0.6)

# Check if the folder bin exist
if os.path.isdir(_bin_path):
  shutil.rmtree(_bin_path)

# Copy the file to the bin folder of lymui
shutil.copytree("bin/include", "lymui/bin/include")
shutil.copy("bin/liblymui.a", "lymui/bin")
update_progress(1)
print("\nDownload liymui lib v "+_version+ " for: "+o)
