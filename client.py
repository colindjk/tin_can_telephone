#!/usr/bin/env python
# -*- coding: utf-8 -*-

import sys
import socket
import fileinput
import thread
import json
 
TCP_IP = '127.0.0.1'
TCP_PORT = 3000
BUFFER_SIZE = 1024
user = ''
user_to = ''

def receive():
    #print ("home")
    while True:
        #print("waiting to receive")
        received = s.recv(BUFFER_SIZE)
        #print (received)
        #print("received")
        decode = json.loads(received)
        print ("\n" + (decode['Message'])['from'] + " : " + (decode['Message'])['msg'])
        sys.stdout.write (user + " > ")
        sys.stdout.flush()

def login():
    global user
    global user_to
    user = raw_input ("enter user name: ")
    print ("user name is: " + user)
    user_to = raw_input ("enter destination name: ")
    print ("destination name is:" + user_to)
    login_creds = '{"LoginCredentials":{"user":"' + user + '"}}\n'
    #print ("creds" + login_creds)
    s.send(login_creds)
    
def switch():
    global user_to
    user_to = raw_input ("enter destination name: ")
    print ("destination name is: " + user_to)

s = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
s.connect((TCP_IP, TCP_PORT))
login()
thread.start_new_thread(receive, ())
while True:
    line = raw_input (user + " > ")
    #print ("original line" + line)
    # Insert code to handle switching users that we're talking to
    if line == '!switch':
        switch()
    else:
        line = line.strip()
        line = line.decode('utf-8', 'ignore').encode('utf-8')
        message = '"Message"{"to":"' + user_to + '","from":"' + user + '","msg":"' + line + '"}\n'
        s.send(message)


