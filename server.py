#!/usr/bin/env python3
from rl_zoo3 import ALGOS
from http.server import HTTPServer, BaseHTTPRequestHandler
from socketserver import ThreadingMixIn
import threading
import json


model = ALGOS["ppo"].load("model/CartPole-v1.zip")
  
# creating a class for handling 
# basic Get and Post Requests
class Handler(BaseHTTPRequestHandler):
    
    # creating a function for Get Request
    def do_POST(self):
        content_length = int(self.headers.get('Content-Length'))
        body = self.rfile.read(content_length)
        data = json.loads(body.decode('utf-8'))
        action = model.predict([[
            data["cart_position"],
            data["cart_velocity"],
            data["pole_angle"],
            data["pole_angular_velocity"]]]
        )
        # We use .item() so we can decode the float32 to json
        response = {"action": action[0].item()}
        self.send_response(200)
          
        # Type of file that we are using for creating our
        # web server.
        self.send_header('content-type', 'application/json')
        self.end_headers()
          
        # what we write in this function it gets visible on our
        # web-server
        self.wfile.write(json.dumps(response).encode())
  
  
class ThreadingSimpleServer(ThreadingMixIn, HTTPServer):
    """Handle requests in a separate thread."""


if __name__ == "__main__":
  print("Serving...")
  server = ThreadingSimpleServer(('localhost', 5555), Handler)
  server.serve_forever()
