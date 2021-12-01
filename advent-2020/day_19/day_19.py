import fileinput
import functools
import itertools
import re
import math
from collections import deque
import sys
import numpy as np
import copy

# our graph will have keys, and for each key will have the possible "branches"
def build_graph(rules):
  my_graph = {}
  leafs = {}
  for rule in rules:
    node, inner_rules = rule.split(': ')
    if '"' in inner_rules:
      charac = inner_rules.replace('"', ' ').strip()
      leafs[node] = charac
      continue

    for inner_rule in inner_rules.split('|'):
      current_node = node
      if current_node in my_graph:
        my_graph[current_node].append(list(inner_rule.strip().split(' ')))
      else: 
        my_graph[current_node] = [list(inner_rule.strip().split(' '))]

  return my_graph, leafs

def find(target, my_graph, leafs, to_explore):
  if not target or not to_explore:
    if not target : return True
    return False

  if len(to_explore) > len(target):
    return False

  node = to_explore[0]
  if node in leafs:
    if target[0] == leafs[node]:
      # First char of target ready
      return find(target[1:], my_graph, leafs, to_explore[1:])
  else:
    for branch in my_graph[node]:
      # Call for each branch and explore the branch first (order of the string)
      if find(target, my_graph, leafs, branch + to_explore[1:]):
        return True

  return False

rules = []
messages = set()
reading_rules = True
for line in fileinput.input():
  clean_line = line.strip()
  if not clean_line:
    reading_rules = False
    continue

  if reading_rules : rules.append(clean_line)
  else : messages.add(clean_line)

my_graph, leafs = build_graph(rules)

count = 0
for message in messages:
  found = find(message, my_graph, leafs, my_graph['0'][0])
  count += found

# First Start
print(count)

# Second Star
my_graph['8'] = [['42'],['42','8']]
my_graph['11'] = [['42','31'],['42','11','31']]

count = 0
for message in messages:
  found = find(message, my_graph, leafs, my_graph['0'][0])
  count += found

print(count)