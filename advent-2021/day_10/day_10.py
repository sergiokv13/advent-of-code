import fileinput
import math


def match_close(el):
  if el == '[': return ']'
  if el == '(': return ')'
  if el == '<': return '>'
  if el == '{': return '}'

def points(el):
  if el == ')' : return 3
  if el == ']' : return 57
  if el == '}' : return 1197
  if el == '>' : return 25137

def points2(remaining):
  score = 0
  def p(el):
    if el == ')' : return 1
    if el == ']' : return 2
    if el == '}' : return 3
    if el == '>' : return 4
  
  for el in remaining:
    score *= 5
    score += p(el)

  return score


def is_open(el):
  return el == '[' or el == '(' or el == '<' or el == '{'

def is_corrupted(line):
  stack = []
  for c in line:
    if is_open(c): stack.append(c)
    else:
      if match_close(stack.pop()) != c:
        return c
  return False

def complete_line(line):
  stack = []
  for c in line:
    if is_open(c): stack.append(c)
    else: stack.pop()
  return [match_close(el) for el in stack[::-1]]
    
points_first_star = 0
points_second_star = []
for line in fileinput.input():
  line = line.strip()
  is_corr = is_corrupted(line)

  if is_corr:
    points_first_star += points(is_corr)
  else:
    remaining = complete_line(line)
    points_second_star.append(points2(remaining))

# First Star

print(points_first_star)

# Second Star

print(sorted(points_second_star)[len(points_second_star) // 2])

