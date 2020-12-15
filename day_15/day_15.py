import fileinput
import functools
import re
import math
import operator
from collections import deque
import sys

def get_in_pos(pos, memo, starting_set):
  if pos < len(starting_set):
    return starting_set[pos]
  
  prev = get_in_pos(pos - 1, memo, starting_set)
  if not prev in memo:
    memo[prev] = [pos - 1]
    return 0
  else:
    last_show = memo[prev][-1]
    memo[prev].append(pos - 1)
    return (pos - 1) - last_show

def get_in_pos_it(pos, memo, starting_set):
  prev = None
  for i in range(pos):
    if i < len(starting_set):
      prev = starting_set[i]
    else:
      if not prev in memo:
        memo[prev] = [i - 1]
        prev = 0
      else:
        last_show = memo[prev][-1]
        memo[prev].append(i - 1)
        prev = (i - 1) - last_show

  return prev


for line in fileinput.input():
  values = [int(v) for v in line.strip().split(',')]
 
memo = {}
for i, v in enumerate(values):
  memo[v] = [i]

# First Star
 turn = 2020
 print(get_in_pos(turn - 1, memo, values))

memo = {}
for i, v in enumerate(values):
  memo[v] = [i]

# Second Star
turn = 30000000
print(get_in_pos_it(turn, memo, values))
