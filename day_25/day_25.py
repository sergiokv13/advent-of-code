import fileinput
import functools
import itertools
import re
import math
from collections import deque
import sys
import numpy as np
import copy

def get_loop_size(public_key):
  val = 1
  loop = 0
  while(val != public_key):
    loop += 1
    val *= 7
    val %= 20201227
  
  return loop

def use_key(key, loop_size):
  val = 1
  for i in range(loop_size):
    val *= key
    val %= 20201227
  
  return val


public_keys = []
for line in fileinput.input():
  public_keys.append(line.strip())
card_pk = int(public_keys[0])
door_pk = int(public_keys[1])

loop_card = get_loop_size(card_pk)
loop_door = get_loop_size(door_pk)

print(use_key(door_pk, loop_card))
print(use_key(card_pk, loop_door))