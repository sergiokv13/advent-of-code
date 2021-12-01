import fileinput
import functools
import re
import math
from collections import deque

def move(positions, direction, value):
  if direction == 'N' or direction == 'E':
    positions[direction] += value
  elif direction == 'S':
    positions['N'] -= value
  elif direction == 'W':
    positions['E'] -= value

def rotate(current_dir, degrees):
  directions = ['E','S','W','N']
  current_idx = directions.index(current_dir)
  degrees /= 90
  current_idx += degrees
  if current_idx >= len(directions):
    current_idx -= len(directions)
  return directions[int(current_idx)]

def rotate_vector(point, degrees):
  new_point = {'E': 0, 'N': 0}
  v1 = rotate('N', degrees)
  v2 = rotate('E', degrees)
  move(new_point, v1, point['N'])
  move(new_point, v2, point['E'])
  return new_point

def navigate(positions, instructions):
  current_dir = 'E'
  for instruction in instructions:
    i,v = instruction
    if i in ['N','E','S','W']:
      move(positions, i, v)
    if i == 'F':
      move(positions,current_dir, v)
    if i == 'L':
      current_dir = rotate(current_dir, v * -1)
    if i == 'R':
      current_dir = rotate(current_dir, v)

def navigate_2(positions, instructions):
  waypoint = {'E': 10, 'N': 1}
  for instruction in instructions:
    i,v = instruction
    if i in ['N','E','S','W']:
      move(waypoint, i, v)
    if i == 'F':
      move(positions, 'N', v * waypoint['N'])
      move(positions, 'E', v * waypoint['E'])
    if i == 'R':
      waypoint = rotate_vector(waypoint, v)
    if i == 'L':
      waypoint = rotate_vector(waypoint, v * -1)

def rule_1_dist(positions):
  return abs(positions['N']) + abs(positions['E'])
    
instructions = []
for line in fileinput.input():
  clean_line = line.strip()
  instructions.append((clean_line[0], int(clean_line[1:])))

# First Star
positions = {"N": 0, "E": 0}
navigate(positions, instructions)
print(rule_1_dist(positions))

# Second Star
positions = {"N": 0, "E": 0}
navigate_2(positions, instructions)
print(rule_1_dist(positions))
