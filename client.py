#!/usr/bin/env python
# -*- coding: utf-8 -*-

import socket
import fileinput

TCP_IP = '127.0.0.1'
TCP_PORT = 3000
BUFFER_SIZE = 1024

user = "colin"

login_creds = '{"LoginCredentials":{"user":"' + user + '"}}\n'

s = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
s.connect((TCP_IP, TCP_PORT))
s.send(login_creds)

user_to = "someone"

for line in fileinput.input():
    # Insert code to handle switching users that we're talking to
    line = line.strip()
    line = line.decode('utf-8', 'ignore').encode('utf-8')

    message = '"Message"{"to":"' + user_to + '","from":"' + user + '","msg":"' + line + '"}\n'

    s.send(message)

