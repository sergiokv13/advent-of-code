import fileinput
import math

match_close = {'[': ']', '(': ')', '<': '>', '{': '}'}
points1 = {')': 3, ']': 57, '}': 1197, '>': 25137}
points2 = {')': 1, ']': 2, '}': 3, '>': 4}

def score2(remaining):
  score = 0
  for el in remaining:
    score *= 5
    score += points2[el]
  return score

def is_open(el):
  return el == '[' or el == '(' or el == '<' or el == '{'

def is_corrupted(line):
  stack = []
  for c in line:
    if is_open(c): stack.append(c)
    else:
      if match_close[stack.pop()] != c:
        return c
  return False

def complete_line(line):
  stack = []
  for c in line:
    if is_open(c): stack.append(c)
    else: stack.pop()
  return [match_close[el] for el in stack[::-1]]
    
points_first_star = 0
points_second_star = []
for line in fileinput.input():
  line = line.strip()
  is_corr = is_corrupted(line)

  if is_corr:
    points_first_star += points1[is_corr]
  else:
    remaining = complete_line(line)
    points_second_star.append(score2(remaining))

# First Star

print(points_first_star)

# Second Star

print(sorted(points_second_star)[len(points_second_star) // 2])

