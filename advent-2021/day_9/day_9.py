import fileinput
from typing import DefaultDict
import math

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

def is_low(matrix, i, j):
  val = matrix[i][j]
  return (
    val < getp(matrix, i+1, j, float('inf')) and
    val < getp(matrix, i-1, j, float('inf')) and
    val < getp(matrix, i, j+1, float('inf')) and
    val < getp(matrix, i, j-1, float('inf'))
  )

def get_basin_size(matrix, i, j):
  visited = DefaultDict(int)
  return get_basin_size_rec(matrix, i, j, visited)

def get_basin_size_rec(matrix, i, j, visited):
  if visited[(i,j)]: return 0

  visited[(i,j)] = True
  basin_count = 1
  val = matrix[i][j]

  p = getp(matrix, i+1, j, None)
  if p and p != 9 and val < p:
    basin_count += get_basin_size_rec(matrix, i+1, j, visited)

  p = getp(matrix, i-1, j, None)
  if p and p != 9 and val < p:
    basin_count += get_basin_size_rec(matrix, i-1, j, visited)

  p = getp(matrix, i, j+1, None)
  if p and p != 9 and val < p:
    basin_count += get_basin_size_rec(matrix, i, j+1, visited)

  p = getp(matrix, i, j-1, None)
  if p and p != 9 and val < p:
    basin_count += get_basin_size_rec(matrix, i, j-1, visited)

  return basin_count

def get_level_values(matrix):
  risk = 0
  basins = []
  for i in range(len(matrix)):
    for j in range(len(matrix[i])):
      if is_low(matrix, i, j):
        risk += 1 + matrix[i][j]
        basins.append(get_basin_size(matrix, i, j))  
  return risk, math.prod(sorted(basins)[-3:])

mat = []
for line in fileinput.input():
  mat.append([int(x) for x in line.strip()])


risk, basins = get_level_values(mat)

# # First Star

print(risk)

# # Second Star

print(basins)

