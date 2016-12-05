#!/user/bin/env python

import unittest
import sys
import socket
import fileinput
import thread
import json
 


class TestClient(unittest.TestCase):

    def test_small(t):
        TCP_IP = '127.0.0.1'
        TCP_PORT = 3000
        BUFFER_SIZE = 1024
        user_to = 'jim'
        
        user = 'jim'
        a = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
        a.connect((TCP_IP, TCP_PORT))
        login_creds = '{"LoginCredentials":{"user":"' + user + '"}}\n'
        a.send(login_creds)
        
        user = 'jones'
        b = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
        b.connect((TCP_IP, TCP_PORT))
        login_creds = '{"LoginCredentials":{"user":"' + user + '"}}\n'
        b.send(login_creds)
        
        message = '"Message"{"to":"' + user_to + '","from":"' + user + '","msg":"' + "hello" + '"}\n'
        b.send(message)
        
        received = a.recv(BUFFER_SIZE)
        decode = json.loads(received)
       # print(decode)
        print("message sent: hello \n message received: " + (decode['Message'])['msg'])

    def test_big(t):
        TCP_IP = '127.0.0.1'
        TCP_PORT = 3000
        BUFFER_SIZE = 1024
        user_to = 'jim'
        
        user = 'jim'
        a = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
        a.connect((TCP_IP, TCP_PORT))
        login_creds = '{"LoginCredentials":{"user":"' + user + '"}}\n'
        a.send(login_creds)
        
        user = 'jones'
        b = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
        b.connect((TCP_IP, TCP_PORT))
        login_creds = '{"LoginCredentials":{"user":"' + user + '"}}\n'
        b.send(login_creds)
        payload = "123456789101112131415161718192021222324252627282930313233343536373839404142434445464748495051525354555657585960616263646566676869707172737475767778798081828384858687888990919293949596979899100"
        message = '"Message"{"to":"' + user_to + '","from":"' + user + '","msg":"' + payload + '"}\n'
        b.send(message)
        
        received = a.recv(BUFFER_SIZE)
        decode = json.loads(received)
       # print(decode)
        print("message sent: " + payload + "\n message received: " + (decode['Message'])['msg'])
        
   


if __name__ == '__main__':
    unittest.main()
