import http.server
import socketserver
import os
import requests
from requests.adapters import HTTPAdapter
from urllib3.util.retry import Retry
from collections import OrderedDict
import threading

PORT = 8080
PUBLIC_DIR = r"C:\Users\shiva\Windy_App\extracted\assets\public"

# Persistent HTTP Session with connection pool & retry logic
session = requests.Session()
retries = Retry(total=3, backoff_factor=0.2, status_forcelist=[500, 502, 503, 504])
adapter = HTTPAdapter(pool_connections=100, pool_maxsize=100, max_retries=retries)
session.mount('https://', adapter)
session.mount('http://', adapter)

# Fast thread-safe LRU Cache for map & weather tiles
class TileLRUCache:
    def __init__(self, capacity=3000):
        self.capacity = capacity
        self.cache = OrderedDict()
        self.lock = threading.Lock()

    def get(self, key):
        with self.lock:
            if key in self.cache:
                self.cache.move_to_end(key)
                return self.cache[key]
            return None

    def put(self, key, value):
        with self.lock:
            if key in self.cache:
                self.cache.move_to_end(key)
            self.cache[key] = value
            if len(self.cache) > self.capacity:
                self.cache.popitem(last=False)

tile_cache = TileLRUCache(capacity=3000)

class ThreadedHTTPServer(socketserver.ThreadingMixIn, socketserver.TCPServer):
    allow_reuse_address = True
    daemon_threads = True

class WeatherAppHandler(http.server.SimpleHTTPRequestHandler):
    protocol_version = "HTTP/1.1"

    def __init__(self, *args, **kwargs):
        super().__init__(*args, directory=PUBLIC_DIR, **kwargs)

    def log_message(self, format, *args):
        # Clean custom logging without cluttering console on routine 200s
        if args and str(args[1]) in ['200', '304']:
            return
        super().log_message(format, *args)

    def do_OPTIONS(self):
        self.send_response(200)
        self.send_header('Access-Control-Allow-Origin', '*')
        self.send_header('Access-Control-Allow-Methods', 'GET, POST, OPTIONS')
        self.send_header('Access-Control-Allow-Headers', '*')
        self.end_headers()

    def do_GET(self):
        # 1. Routing for all proxies
        if self.path.startswith('/rdr/'):
            remote_url = f"https://rdr.windy.com{self.path[4:]}"
            self._proxy(remote_url, is_tile=False)
        elif self.path.startswith('/sat/'):
            remote_url = f"https://sat.windy.com{self.path[4:]}"
            self._proxy(remote_url, is_tile=False)
        elif self.path.startswith('/ims/'):
            remote_url = f"https://ims.windy.com{self.path[4:]}"
            self._proxy(remote_url, is_tile=True)
        elif self.path.startswith('/tiles/'):
            remote_url = f"https://tiles.windy.com{self.path}"
            self._proxy(remote_url, is_tile=True)
        elif (self.path.startswith('/services/') or 
              self.path.startswith('/metadata/') or 
              self.path.startswith('/maptile/') or 
              self.path.startswith('/tc/') or 
              self.path.startswith('/users/')):
            remote_url = f"https://node.windy.com{self.path}"
            self._proxy(remote_url, is_tile=False)
        else:
            try:
                super().do_GET()
            except (ConnectionResetError, ConnectionAbortedError, BrokenPipeError):
                pass

    def _proxy(self, url, is_tile=False):
        cache_key = self.path
        if is_tile:
            cached = tile_cache.get(cache_key)
            if cached:
                status, headers, content = cached
                try:
                    self.send_response(status)
                    for k, v in headers.items():
                        self.send_header(k, v)
                    self.end_headers()
                    self.wfile.write(content)
                    return
                except (ConnectionResetError, ConnectionAbortedError, BrokenPipeError):
                    return

        try:
            resp = session.get(url, headers={
                'User-Agent': 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/130.0.0.0 Safari/537.36',
                'Accept': '*/*'
            }, timeout=12)

            # Headers to pass through
            send_headers = {
                'Access-Control-Allow-Origin': '*',
                'Access-Control-Allow-Methods': 'GET, POST, OPTIONS',
                'Access-Control-Allow-Headers': '*',
                'Content-Length': str(len(resp.content))
            }

            if is_tile:
                send_headers['Cache-Control'] = 'public, max-age=86400'
            else:
                send_headers['Cache-Control'] = 'no-cache'

            for h, v in resp.headers.items():
                if h.lower() in ['content-type', 'etag', 'last-modified']:
                    send_headers[h] = v

            if is_tile and resp.status_code == 200:
                tile_cache.put(cache_key, (resp.status_code, send_headers, resp.content))

            self.send_response(resp.status_code)
            for k, v in send_headers.items():
                self.send_header(k, v)
            self.end_headers()
            self.wfile.write(resp.content)

        except (ConnectionResetError, ConnectionAbortedError, BrokenPipeError):
            pass
        except Exception as e:
            try:
                self.send_response(502)
                self.send_header('Access-Control-Allow-Origin', '*')
                self.send_header('Content-Type', 'text/plain')
                self.end_headers()
                self.wfile.write(str(e).encode())
            except Exception:
                pass

if __name__ == '__main__':
    with ThreadedHTTPServer(("", PORT), WeatherAppHandler) as httpd:
        print(f"Bhartiya Marut Kendra High-Performance Server running at http://localhost:{PORT}")
        httpd.serve_forever()
