##################
# Very ineffincient way of solving the second problem. It requires some time to finish (around 5min)
# We could improve it by (TODO):
# 1. Don't use a preset cube, just add dimmensions to it as needed (Only in the places that is needed), we could use a hash for this
# 2. Don't check the 80 neighboors everytime we check the state, there has to be a way to do it on the fly
##################

import fileinput
import functools
import re
import math
import operator
from collections import deque
import sys
import numpy as np
import copy

optionsz = ['z0','z1','z-1']
optionsy = ['y0','y1','y-1']
optionsx = ['x0','x1','x-1']
optionsw = ['w0','w1','w-1']

# Getting all the possibilities of getting a position neighboor
combined = []
for w in optionsw:
  for z in optionsz:
    for y in optionsy:
      for x in optionsx:
        if (x == 'x0' and y == 'y0' and z == 'z0' and w == 'w0'):
          continue
        else:
          combined.append([w,z,y,x])

print("Combined options ready :", len(combined) )

# Get the state for the cube position by the rules
def define_state(w,z,y,x,cube):
  # Killing borders to forget about index out of range
  if (z-1 < 0 or z+1 >= len(cube)) : return '.'
  if (x-1 < 0 or x+1 >= len(cube)) : return '.'
  if (y-1 < 0 or y+1 >= len(cube)) : return '.'
  if (w-1 < 0 or w+1 >= len(cube)) : return '.'

  active_neighboors = 0
 
  # Check neighboors state
  for operations in combined:
    pos = {'w': w, 'z': z, 'y': y, 'x': x}
    for operation in operations:
      pos[operation[0]] = pos[operation[0]] + int(operation[1:])
    if cube[pos['w']][pos['z']][pos['y']][pos['x']] == '#':
      active_neighboors += 1

  if cube[w][z][y][x] == '#' and (active_neighboors != 2 and active_neighboors != 3):
    return '.'
  if cube[w][z][y][x] == '.' and active_neighboors == 3:
    return '#'
  return cube[w][z][y][x]

def perform_iteration(cube):
  newcube = copy.deepcopy(cube)
  for w,_v in enumerate(cube):
    for z,_v in enumerate(cube[w]):
      for y,_v in enumerate(cube[z]):
        for x,_v in enumerate(cube[y]):
          newcube[w][z][y][x] = define_state(w,z,y,x,cube)
  return newcube

def count_active(cube):
  active = 0
  for w,_v in enumerate(cube):
    for z,_v in enumerate(cube[w]):
      for y,_v in enumerate(cube[z]):
        for x,_v in enumerate(cube[y]):
          if cube[w][z][y][x] == '#':
            active += 1
  return active

# Adding the first layer of the cube at the center of an 25*25*25*25 cube (MORE time and memory inefficient)
def build_init_cube(lines, size = 25):
  cube = [[[['.' for _x in range(0,size)] for _y in range(0,size)] for _z in range(0,size)] for _w in range(0,size)]
  mid = size // 2
  
  mid_slice = [['.' for _x in range(0,size)] for _y in range(0,size)]

  start = mid - len(lines) // 2
  for i in range(start, start + len(lines)):
      for i2 in range(start, start + len(lines)):
        mid_slice[i][i2] = lines[i-start][i2-start]

  cube[mid][mid] = mid_slice
  return cube

def print_cube(cube):
  for w,_v in enumerate(cube):
    for z,_v in enumerate(cube):
      print(f"w: {w}, z: {z}")
      for y,_v2 in enumerate(cube[z]):
        print(cube[w][z][y])
      print()
    print()

lines = []
for line in fileinput.input():
  clean = line.strip()
  lines.append(clean)

cube = build_init_cube(lines)
for i in range(0,6):
  cube = perform_iteration(cube)
  print(f"Turn {i+1} ready")
  if i == 5:
    print(f"Turn {i+1}: ", count_active(cube))

