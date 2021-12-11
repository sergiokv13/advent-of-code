import fileinput
from typing import DefaultDict

def print_matrix(m):
  for row in m:
    print(row)
  print()

def getp(matrix, i, j, default = None):
  try:
    if i < 0 or j < 0: return default
    return matrix[i][j]
  except:
    return default

def increment_one(m):
  to_propagate = []
  for j in range(len(m)):
    for i in range(len(m[j])):
      m[i][j] += 1
      if m[i][j] > 9: to_propagate.append((i,j))
  return to_propagate

def propagate(m, to_prop):
  visited = DefaultDict(bool)
  for p in to_prop:
    visited[p] = True
  
  while(to_prop):
    i, j = to_prop.pop()
    # propagate to neighboors
    for pos in [(i+1, j), (i-1, j), (i, j+1), (i, j-1), (i+1, j+1), (i-1, j-1), (i+1, j-1), (i-1, j+1)]:
      if getp(m, pos[0], pos[1]) is not None:
        m[pos[0]][pos[1]] += 1
        if m[pos[0]][pos[1]] > 9 and not visited[pos]:
          to_prop.append(pos)
          visited[pos] = True
  return m

def count_and_reset(m):
  count = 0
  for j in range(len(m)):
    for i in range(len(m[j])):
      if m[i][j] > 9:
        count += 1
        m[i][j] = 0
  return count


def perform_step(m):
  to_propagate = increment_one(m)
  propagate(m, to_propagate)
  count = count_and_reset(m)
  return count, count == len(m) * len(m[0])

def perform_n_steps(m, steps):
  count = 0
  for _i in range(steps):
    step_count, _f = perform_step(m)  
    count += step_count
  return count

oct = []
for line in fileinput.input():
  oct.append([int(el) for el in line.strip()])

# First Star

print(perform_n_steps(oct, 100))

# Second Star

finished = False
steps = 0
while(not finished):
  steps+=1
  _count, finished = perform_step(oct)
print(steps + 100)

