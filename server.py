import http.server
import socketserver
import urllib.request
import urllib.error
import os

PORT = 8080
PUBLIC_DIR = r"C:\Users\shiva\Windy_App\extracted\assets\public"

# Multi-threaded HTTP Server for fast concurrent tile and parameter loading
class ThreadedHTTPServer(socketserver.ThreadingMixIn, socketserver.TCPServer):
    allow_reuse_address = True
    daemon_threads = True

class WeatherAppHandler(http.server.SimpleHTTPRequestHandler):
    protocol_version = "HTTP/1.1"

    def __init__(self, *args, **kwargs):
        super().__init__(*args, directory=PUBLIC_DIR, **kwargs)

    def end_headers(self):
        self.send_header('Access-Control-Allow-Origin', '*')
        self.send_header('Access-Control-Allow-Methods', 'GET, POST, OPTIONS')
        self.send_header('Access-Control-Allow-Headers', '*')
        self.send_header('Cache-Control', 'no-cache, no-store, must-revalidate')
        self.send_header('Pragma', 'no-cache')
        self.send_header('Expires', '0')
        super().end_headers()

    def do_OPTIONS(self):
        self.send_response(200)
        self.end_headers()

    def do_GET(self):
        # Full proxy mapping for all 65+ weather maps, radar, satellite & parameters
        if self.path.startswith('/rdr/'):
            remote_url = f"https://rdr.windy.com{self.path[4:]}"
            self._proxy(remote_url)
        elif self.path.startswith('/sat/'):
            remote_url = f"https://sat.windy.com{self.path[4:]}"
            self._proxy(remote_url)
        elif self.path.startswith('/ims/'):
            remote_url = f"https://ims.windy.com{self.path[4:]}"
            self._proxy(remote_url)
        elif self.path.startswith('/tiles/'):
            remote_url = f"https://tiles.windy.com{self.path}"
            self._proxy(remote_url)
        elif (self.path.startswith('/services/') or 
              self.path.startswith('/metadata/') or 
              self.path.startswith('/maptile/') or 
              self.path.startswith('/tc/') or 
              self.path.startswith('/users/')):
            remote_url = f"https://node.windy.com{self.path}"
            self._proxy(remote_url)
        else:
            try:
                super().do_GET()
            except (ConnectionResetError, ConnectionAbortedError, BrokenPipeError):
                pass

    def _proxy(self, url):
        try:
            req = urllib.request.Request(url, headers={
                'User-Agent': 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36',
                'Accept': '*/*'
            })
            with urllib.request.urlopen(req, timeout=15) as resp:
                self.send_response(resp.status)
                for header, val in resp.getheaders():
                    if header.lower() not in ['access-control-allow-origin', 'content-length', 'transfer-encoding', 'connection', 'cache-control']:
                        self.send_header(header, val)
                data = resp.read()
                self.send_header('Content-Length', str(len(data)))
                self.end_headers()
                self.wfile.write(data)
        except urllib.error.HTTPError as e:
            try:
                self.send_response(e.code)
                self.end_headers()
                self.wfile.write(e.read())
            except Exception:
                pass
        except (ConnectionResetError, ConnectionAbortedError, BrokenPipeError):
            # Client closed connection (normal when fast scrolling/zooming)
            pass
        except Exception as e:
            try:
                self.send_response(502)
                self.end_headers()
                self.wfile.write(str(e).encode())
            except Exception:
                pass

if __name__ == '__main__':
    with ThreadedHTTPServer(("", PORT), WeatherAppHandler) as httpd:
        print(f"Bhartiya Marut Kendra Multi-Threaded Server running at http://localhost:{PORT}")
        httpd.serve_forever()
