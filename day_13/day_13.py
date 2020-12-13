import fileinput
import functools
import re
import math
import operator
from collections import deque

def get_earliest_by_minutes(goal, buses):
  min_timestamp = float('inf')
  selected_bus = None
  for bus in buses:
    div = math.ceil(goal / bus)
    if (bus * div < min_timestamp):
      min_timestamp = bus * div
      selected_bus = bus

  return selected_bus * (min_timestamp - goal)

# https://en.wikipedia.org/wiki/Chinese_remainder_theorem
def crt(buses, differences):
  res = 0
  m = functools.reduce(operator.mul, buses, 1)
  for idx in range(len(buses)):
    partial_product = m // buses[idx]
    inv_m = pow(partial_product, -1, buses[idx])
    remainder = buses[idx] - differences[idx] if differences[idx] != 0 else 0
    res += partial_product * inv_m * remainder

  return res % m
      
goal = None
differences = []
for line in fileinput.input():
  if goal is None:
    goal = int(line.strip())
  else:
    buses = line.strip().split(',')
    for idx in range(len(buses)):
      if buses[idx] != 'x':
        differences.append(idx)

    # remove x's and set as ints
    buses = [int(x) for x in list(filter(lambda x: x != 'x', buses))]
    
# First Star
print(get_earliest_by_minutes(goal, buses))

# Second Star
print(crt(buses, differences))