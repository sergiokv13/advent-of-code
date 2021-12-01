import fileinput
import functools
import itertools
import re
import math
from collections import deque
import sys
import numpy as np
import copy

operators = ['+','*']
order = ['(',')']

# Calls this recursively after finding an inner expression
# Works the expression as it finds first level operations
def work_expression(expression):
  in_parenthesis = ""; sub_expression = 0; current_operator = None; val = 0

  for el in expression:
    if sub_expression > 0:
      in_parenthesis += el
    if el == '(':
      sub_expression += 1
    if el == ')':
      sub_expression -= 1
      # First level subexpression ended, then call recursively for that
      if sub_expression == 0:
        inner = work_expression(in_parenthesis[:-1])
        in_parenthesis = ''
        if val and current_operator:
          val = eval(f"{val}{current_operator}{inner}")
        else:
          val = inner
  
    if sub_expression == 0 and el not in order:
      if el in operators:
        current_operator = el
      elif val and current_operator:
        val = eval(f"{val}{current_operator}{el}")
      else:
        val = el

  return val

def arr_get(arr, idx):
  if idx < 0 or idx >= len(arr):
    return None
  else:
    return arr[idx]

# Adding parenthesis on all sums to have the option to use the first star method
# This just works adding parenthesis deep into the expression, it won't work to add other parenthesis after
# Example:
# 1 * 2 + 3 + 4 => 1 * ( 2 + 3) + 4

def add_parenthesis_rec(expression):
  if expression.isnumeric() : return expression
  expressions = []
  sub_expression = 0
  in_parenthesis = ""

  for idx, el in enumerate(expression):
    # First we will separate all first level subexpressions. Ex:
    # 1+2+(3+(1*2)) => [1,+,2,+,(3+(1*2))]
    if sub_expression > 0: in_parenthesis += el
    if el == '(': sub_expression += 1
    if el == ')':
      sub_expression -= 1
      if sub_expression == 0:
        expressions.append(in_parenthesis[:-1])
        in_parenthesis = ''
    
    if sub_expression == 0 and el not in order:
      expressions.append(el)

  # Iterate over subexpressions and add + on close numbers (just if no other + is close) 
  new_expression = [e for e in expressions]
  inserted = 0
  for idx,exp in enumerate(expressions):
    if exp == '+' and (arr_get(expressions,idx-2) == '*' or arr_get(expressions,idx+2) == '*'):
      new_expression.insert(idx-1 + inserted,'(')
      inserted+=1
      new_expression.insert(idx+2 + inserted,')')
      inserted+=1

  new_expression = [f"({add_parenthesis(e)})" if e not in operators and e not in order else e for e in new_expression]
  return ('').join(new_expression)

# As the recursive method had the problem to add parenthesis after adding the first parameter matching a sum
# then we will iterate adding parenthesis to the expression until there are no more parenthesis to add... HACKY
# 1 * 2 + 3 + 4 => 1 * ( 2 + 3) + 4 => 1 * ((2+3) + 4)
def add_parenthesis(expression):
  while expression != add_parenthesis_rec(expression):
    expression = add_parenthesis_rec(expression)
  return expression


expressions = []
for line in fileinput.input():
  expressions.append(line.strip().replace(' ',''))

# First Star
res = 0
for expression in expressions:
 res += work_expression(expression)
print(res)

# Second Star
res = 0
responses = []
for expression in expressions:
  res += work_expression(add_parenthesis(expression))
print(res)

