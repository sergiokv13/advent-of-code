import fileinput
import functools
import re
import math
import operator
from collections import deque
import sys
import numpy as np

def get_invalid_nearby(rules, nearby):
  res = 0
  main_set = set()
  for rule in rules.values():
    main_set |= rule

  for n in nearby:
    for v in n:
      if v not in main_set:
        res += v
  return res

def filter_invalid_nearby(rules, nearby):
  valid_nearby = []
  main_set = set()
  for rule in rules.values():
    main_set |= rule

  for n in nearby:
    valid_n = True
    for v in n:
      if v not in main_set:
        valid_n = False
        
    if valid_n:
      valid_nearby.append(n)

  return valid_nearby


def get_star_2_val(classes, my_ticket):
  res = 1
  for i, row_class in enumerate(classes):
    if "departure" in row_class:
      res *= my_ticket[i]
  return res


########################################
# For second star we will buid a matrix of class vs position
# We will get valid solutions checking no overlappings on columns and rows
# It's like the queens problem.. We will also store the paths we already visited
# with the rows_taken + cols_taken + position key
########################################

def build_matrix(rules, nearby):
  m = []
  for rule_key in rules:
    m.append([])
    rule_values = rules[rule_key]
    for pos in range(len(rules)):
      pos_valid = True
      for n in nearby:
        if n[pos] not in rule_values:
          pos_valid = False
      m[-1].append(pos_valid)
  
  m = np.array(m).transpose()
  print("Each row a position")
  for row in m: print(row)
  print("")
  return m

visited = {}
def get_from_matrix(m, row=0, sol=[], cols_taken=set()):
  if len(sol) == len(m):
    print("Sol Positions: ", sol)
    return sol
  
  for idx, class_valid in enumerate(m[row]):
    m_pos = (row, idx)
    valid_pos = m_pos[1] not in cols_taken

    if class_valid and valid_pos:
      new_cols_taken = cols_taken | set([m_pos[1]])

      if (m_pos, str(new_cols_taken)) in visited:
        return

      solution = get_from_matrix(
        m, 
        row + 1, 
        sol + [m_pos], 
        new_cols_taken
      )
      if solution : return solution
      visited[(m_pos, str(new_cols_taken))] = True
  return None

def match_sol_with_cols(sol, rules):
  rule_keys = list(rules.keys())
  cols_order = []
  for v in sol:
    cols_order.append(rule_keys[v[1]])

  return cols_order

# Read input data
rules = {}
my_ticket = []
nearby_tickets = []
input_state = 'rules'
for line in fileinput.input():
  clean = line.strip()
  if not clean : continue

  if "your ticket" in clean:
    input_state = 'my'
    continue

  if "nearby tickets:" in clean:
    input_state = 'nearby'
    continue

  if input_state == 'rules':
    colon_idx = clean.index(':')
    rule_name = clean[:colon_idx]
    rules[rule_name] = set()
    for rule_range in clean[colon_idx + 1:].split('or'):
      range_values = [int(x) for x in rule_range.strip().split('-')]
      min_val = range_values[0]
      max_val = range_values[1]
      for val in range(min_val, max_val + 1):
        rules[rule_name].add(val)
      
  if input_state == 'my':
    my_ticket = [int(x) for x in clean.split(',')]
  if input_state == 'nearby':
    nearby_tickets.append([int(x) for x in clean.split(',')])
  
# First Star
print(get_invalid_nearby(rules, nearby_tickets))

# Second Star
nearby_tickets = filter_invalid_nearby(rules, nearby_tickets)
sol = get_from_matrix(build_matrix(rules, nearby_tickets))
classes = match_sol_with_cols(sol, rules)
print("Classes: ", classes)
print(get_star_2_val(classes, my_ticket))
