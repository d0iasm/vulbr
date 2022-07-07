import http.server
import socketserver

class HttpRequestHandler(http.server.SimpleHTTPRequestHandler):
    def do_GET(self):
        print("Server started at localhost:" + str(PORT))

        # redirect
        if self.path == '/work/4-3.py':
            self.send_response(302)
            self.send_header("Location", "http://example.com")
            self.end_headers()
            html = f"<html><head></head><body><p>should redirect this page!</p></body></html>"
            self.wfile.write(bytes(html, "utf8"))
            return

        return http.server.SimpleHTTPRequestHandler.do_GET(self)

handler = HttpRequestHandler

PORT = 8888
server = socketserver.TCPServer(("", PORT), handler)

server.serve_forever()
