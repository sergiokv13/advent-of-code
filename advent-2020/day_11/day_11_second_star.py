import fileinput
import functools
import re
import math
from collections import deque

def get_el(row, col, m):
  if row < 0 or col < 0:
    return False
  if row >= len(m) or col >= len(m[0]):
    return False
  return m[row][col]

def navigate(row, col, m, expected, agg):
  r = None; l = None; b = None; t = None
  tl = None; tr = None; bl = None; br = None
  finished = 0
  it = 1
  while(finished is not 8):
    if b is None and get_el(row+it,col,m) != '.':
      b = get_el(row+it,col,m) == expected
      finished += 1
    if t is None and get_el(row-it,col,m) != '.':
      t = get_el(row-it,col,m) == expected
      finished += 1
    if r is None and get_el(row,col+it,m) != '.':
      r = get_el(row,col+it,m) == expected
      finished += 1
    if l is None and get_el(row,col-it,m) != '.':
      l = get_el(row,col-it,m) == expected
      finished += 1
    if tl is None and get_el(row-it,col-it,m) != '.':
      tl = get_el(row-it,col-it,m) == expected
      finished += 1
    if tr is None and get_el(row-it,col+it,m) != '.':
      tr = get_el(row-it,col+it,m) == expected
      finished += 1
    if br is None and get_el(row+it,col+it,m) != '.':
      br = get_el(row+it,col+it,m) == expected
      finished += 1
    if bl is None and get_el(row+it,col-it,m) != '.':
      bl = get_el(row+it,col-it,m) == expected
      finished += 1
    it += 1

  if agg == 'sum':
    return r+l+b+t+tl+tr+bl+br
  else:
    return r or l or b or t or tl or tr or bl or br

  
def change_seat_state(seats_map, row, col):
  val = seats_map[row][col]
  if (val == 'L' and not navigate(row, col, seats_map, '#', 'or')):
    return '#'
  if (val == '#' and navigate(row, col, seats_map, '#', 'sum') >= 5):
    return  'L'
  return False


# assuming we have at least one row and one column
def simulate_iteration(seats_map):
  initial_state = [row[:] for row in seats_map]
  changes = 0
  for row in range(len(seats_map)):
    for col in range(len(seats_map[0])):
      new_val = change_seat_state(initial_state, row, col)
      if new_val:
        changes += 1
        seats_map[row][col] = new_val
  return changes == 0

def count_empty(seats_map):
  count = 0
  for row in seats_map:
    for seat in row:
      if seat == '#':
        count += 1
  return count

seats_map = []
for line in fileinput.input():
  seats_map.append(list(line.strip()))


stabilized = False
while(stabilized is False):
  stabilized = simulate_iteration(seats_map)

print(count_empty(seats_map))