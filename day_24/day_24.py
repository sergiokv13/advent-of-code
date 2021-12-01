import fileinput
import functools
import itertools
import re
import math
from collections import deque
import sys
import numpy as np
import copy

def navigate(direction, initial = (0,0)):
  idx = 0
  while idx < len(direction):
    if direction[idx] == 'e':
      initial = (initial[0]+1, initial[1])
    if direction[idx] == 'w':
      initial = (initial[0]-1, initial[1])

    if direction[idx] == 's':
      if direction[idx:idx+2] == 'se':
        initial = (initial[0], initial[1]+1)
      if direction[idx:idx+2] == 'sw':
        initial = (initial[0]-1, initial[1]+1)
      idx += 1
  
    if direction[idx] == 'n':
      if direction[idx:idx+2] == 'ne':
        initial = (initial[0]+1, initial[1]-1)
      if direction[idx:idx+2] == 'nw':
        initial = (initial[0], initial[1]-1)
      idx += 1

    idx += 1
      
  return initial

def flip_and_count(directions):
  flipped_black = set()
  for direction in directions:
    new_tile = navigate(direction)
    if new_tile in flipped_black:
      flipped_black.remove(new_tile)
    else:
      flipped_black.add(new_tile)
  return len(flipped_black)

def build_floor(directions):
  floor = {}
  for direction in directions:
    new_tile = navigate(direction)
    if new_tile in floor:
      floor[new_tile] = not floor[new_tile]
    else:
      floor[new_tile] = True
  return floor

def get_active_neighboors(position, floor):
  active = 0
  active += floor.get((position[0]+1, position[1]), 0)
  active += floor.get((position[0]-1, position[1]), 0)
  active += floor.get((position[0], position[1]+1), 0)
  active += floor.get((position[0]-1, position[1]+1), 0)
  active += floor.get((position[0]+1, position[1]-1), 0)
  active += floor.get((position[0], position[1]-1), 0)

  return active

def get_neighboors_positions(position, floor):
  positions = []
  positions.append((position[0]+1, position[1]))
  positions.append((position[0]-1, position[1]))
  positions.append((position[0], position[1]+1))
  positions.append((position[0]-1, position[1]+1))
  positions.append((position[0]+1, position[1]-1))
  positions.append((position[0], position[1]-1))

  return positions

def iterate(floor):
  floor_copy = copy.deepcopy(floor)
  # add neighboors to dict
  for pos in floor_copy:
    n_positions = get_neighboors_positions(pos, floor)
    for n_pos in n_positions:
      if n_pos not in floor:
        floor[n_pos] = False
  
  # Perform simulation
  floor_copy = copy.deepcopy(floor)
  for pos in floor:
    act = get_active_neighboors(pos, floor)
    if floor[pos] and (act == 0 or act > 2):
      floor_copy[pos] = False
    elif not floor[pos] and act == 2:
      floor_copy[pos] = True

  return floor_copy

def count_black(floor):
  count = 0
  for pos in floor:
    count += floor[pos]
  return count


directions = []
for line in fileinput.input():
  directions.append(line.strip())

# First Star
# print(flip_and_count(directions))
# Second Star
floor = build_floor(directions)
for i in range (100):
  floor = iterate(floor)
  print(f"Day {i+1}: ", count_black(floor))