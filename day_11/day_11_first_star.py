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

def get_empty_change(row, col, m):
  cross = get_el(row-1,col,m) != '#' and get_el(row+1,col,m) != '#' and get_el(row,col-1,m) != '#' and get_el(row,col+1,m) != '#'
  diag =  get_el(row-1,col-1,m) != '#' and get_el(row+1,col+1,m) != '#' and get_el(row+1,col-1,m) != '#' and get_el(row-1,col+1,m) != '#'
  return cross and diag

def get_occupied_change(row, col, m):
  count = 0
  count += get_el(row-1,col,m) == '#' 
  count += get_el(row+1,col,m) == '#' 
  count += get_el(row,col-1,m) == '#' 
  count += get_el(row,col+1,m) == '#'
  count += get_el(row-1,col-1,m) == '#' 
  count += get_el(row+1,col+1,m) == '#' 
  count += get_el(row+1,col-1,m) == '#' 
  count += get_el(row-1,col+1,m) == '#'
  return count >= 4

def change_seat_state(seats_map, row, col):
  val = seats_map[row][col]
  if (val == 'L' and get_empty_change(row, col, seats_map)):
    return '#'
  if (val == '#' and get_occupied_change(row,col, seats_map)):
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