#!/usr/bin/python3
 
from multipledispatch import dispatch
import configparser
 
@dispatch()
def product():
        print("Product not defined")
 
@dispatch(int, int)
def product(a, b):
        p = a + b
        print("Product a + b:", p)
 
@dispatch(int, int, int)
def product(a, b, c):
        p = a + b + c
        print("Product a + b + c:", p)
 
@dispatch(int, int, str)
def product(a, b, c):
        p = a + b
        print("Product '", c, "':", p)
 
#
# START
#
# Configuration
# parsowanie konfiga
#
klient="klient_1"
for klient in ['klient_1', 'klient_2', 'klient_3', 'jakisdefault']:
        if klient == 'klient_1':
                print("Klient: ", klient)
                product(1,5)
        elif klient == 'klient_2':
                print("Klient: ", klient)
                product(1,5,4)
        elif klient == 'klient_3':
                print("Klient: ", klient)
                product(2,4, "skrzynka")
        else:
                print("Klient: ", klient)
                product()
# eof