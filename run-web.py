import http.server
import socketserver
import os
from pathlib import Path
import webbrowser
import shutil

os.system("wasm-pack build --target web")
shutil.copy("src-web/index.html", "pkg/index.html")


class CustomHTTPRequestHandler(http.server.SimpleHTTPRequestHandler):
    def end_headers(self):
        # Set proper headers for modules
        if self.path.endswith(".js"):
            self.send_header("Content-Type", "application/javascript")
        super().end_headers()


# Configure the server
PORT = 8000
DIR = Path(__file__).parent / "pkg"  # Ensure this points to your 'pkg' directory

import os

os.chdir(DIR)

# Set up the server with the custom handler
Handler = CustomHTTPRequestHandler
with socketserver.TCPServer(("", PORT), Handler) as httpd:
    print(f"Serving {DIR} at http://localhost:{PORT}")
    httpd.serve_forever()
