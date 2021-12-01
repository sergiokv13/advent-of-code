import fileinput
import functools
import itertools
import re
import math
from collections import deque
import sys
import numpy as np
import copy
from time import time

init = time()

def remove_next_n(cups, current, n):
  removed = []
  next_link = None
  local_current = current
  for i in range(n):
    local_current = cups[local_current]
    removed.append(local_current)
    next_link = cups[local_current]

  cups[current] = next_link
  for r in removed:
    del cups[r]

  return removed

def insert_on(cups, current, to_insert):
  next_link = cups[current]
  for el in to_insert:
    cups[current] = el
    current = el

  cups[current] = next_link
  return cups

def simulate_game(cups, current, iterations):
  while iterations:
    print("it: ", iterations)
    excluded = remove_next_n(cups, current, 3)

    destination_label = current - 1
    while(destination_label not in cups):
      destination_label = destination_label - 1
      if destination_label <= 0:
        destination_label = max(cups.values())

    insert_on(cups, destination_label, excluded)
    iterations -= 1
    current = cups[current]

  return cups

def get_cups_string(cups, current):
  visited = set()
  cups_string = ""
  while(current not in visited):
    cups_string += str(current)
    visited.add(current)
    current = cups[current]
  return cups_string

for line in fileinput.input():
  cups_list = [int(el) for el in list(line.strip())]
  cups = {}
  first = int(cups_list[0])
  for idx in range(len(cups_list)):
    if idx + 1 < len(cups_list):
      cups[cups_list[idx]] = cups_list[idx+1]
    else:
      cups[cups_list[idx]] = cups_list[0]

  # First Star
  cups_first_star = simulate_game(cups.copy(), first, 100)
  print(get_cups_string(cups_first_star, 1)[1:])

  # Second Star
  cups_second = cups.copy()
  max_val = max(cups.values())
  last_val = int(cups_list[-1])
  for i in (range(max_val+1, 1000000+1)):
    cups_second[last_val] = i
    last_val = i
  cups_second[last_val] = first

  # Slow but will work in around a minute or so
  cups_second = simulate_game(cups_second, first, 10000000)
  print(cups_second[1] * cups_second[cups_second[1]])

  print(time() - init)