import http.server
import socketserver
import urllib.request
import urllib.error
import os

PORT = 8080
PUBLIC_DIR = r"C:\Users\shiva\Windy_App\extracted\assets\public"

class WeatherAppHandler(http.server.SimpleHTTPRequestHandler):
    def __init__(self, *args, **kwargs):
        super().__init__(*args, directory=PUBLIC_DIR, **kwargs)

    def end_headers(self):
        self.send_header('Access-Control-Allow-Origin', '*')
        self.send_header('Access-Control-Allow-Methods', 'GET, POST, OPTIONS')
        self.send_header('Access-Control-Allow-Headers', '*')
        super().end_headers()

    def do_OPTIONS(self):
        self.send_response(200)
        self.end_headers()

    def do_GET(self):
        # Proxy external Windy services locally to eliminate browser CORS errors
        if self.path.startswith('/services/') or self.path.startswith('/metadata/'):
            remote_url = f"https://node.windy.com{self.path}"
            self._proxy(remote_url)
        elif self.path.startswith('/tiles/'):
            remote_url = f"https://tiles.windy.com{self.path}"
            self._proxy(remote_url)
        else:
            super().do_GET()

    def _proxy(self, url):
        try:
            req = urllib.request.Request(url, headers={
                'User-Agent': 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36',
                'Accept': '*/*'
            })
            with urllib.request.urlopen(req, timeout=10) as resp:
                self.send_response(resp.status)
                for header, val in resp.getheaders():
                    if header.lower() not in ['access-control-allow-origin', 'content-length', 'transfer-encoding', 'connection']:
                        self.send_header(header, val)
                data = resp.read()
                self.send_header('Content-Length', str(len(data)))
                self.end_headers()
                self.wfile.write(data)
        except urllib.error.HTTPError as e:
            self.send_response(e.code)
            self.end_headers()
            self.wfile.write(e.read())
        except Exception as e:
            self.send_response(502)
            self.end_headers()
            self.wfile.write(str(e).encode())

if __name__ == '__main__':
    socketserver.TCPServer.allow_reuse_address = True
    with socketserver.TCPServer(("", PORT), WeatherAppHandler) as httpd:
        print(f"Weather app CORS-enabled server running at http://localhost:{PORT}")
        httpd.serve_forever()
