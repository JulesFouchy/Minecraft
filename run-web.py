import os
from pathlib import Path
import shutil

os.system("wasm-pack build --target web")
shutil.copy("src-web/index.html", "pkg/index.html")
os.chdir(Path(__file__).parent / "pkg")
os.system("http-server -o")
