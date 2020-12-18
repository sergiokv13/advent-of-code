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
def work_expression(expression):
  in_parenthesis = ""; sub_expression = 0; current_operator = None; val = 0

  for idx, el in enumerate(expression):
    if sub_expression > 0:
      in_parenthesis += el
    if el == '(':
      sub_expression += 1
    if el == ')':
      sub_expression -= 1
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

def add_parenthesis_rec(expression):
  if expression.isnumeric() : return expression
  expressions = []
  sub_expression = 0
  in_parenthesis = ""

  for idx, el in enumerate(expression):
    if sub_expression > 0:
      in_parenthesis += el
    if el == '(':
      sub_expression += 1
    if el == ')':
      sub_expression -= 1
      if sub_expression == 0:
        expressions.append(in_parenthesis[:-1])
        in_parenthesis = ''
    
    if sub_expression == 0 and el not in order:
      expressions.append(el)


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

