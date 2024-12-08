# import sys
# import socket
from time import sleep

# HEADER = 64
# PORT = 6000
# DISCONNECT_MESSAGE = "!DISCONNECT!"
# SERVER = "192.168.1.195"
# ADDR = (SERVER, PORT)

# client = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
# connected = False
# while not connected:
#     try:
#         client.connect(ADDR)
#         connected = True
#     except:
#         PORT += 1
#         ADDR = (SERVER, PORT)

# def send(msg):
#     message = msg.encode('utf-8')
#     msg_length = len(message)
#     send_length = str(msg_length).encode('utf-8')
#     send_length += b' ' * (HEADER - len(send_length))
#     client.send(send_length)
#     client.send(message)

import sys

def main():
    # Disable output buffering
    sys.stdout.reconfigure(line_buffering=True)
    
    while True:
        print("Hello from python", flush=True)
        sleep(1)

if __name__ == "__main__":
    main()
