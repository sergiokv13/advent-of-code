import fileinput
import functools

# position => [x, y]
# movement => [x, y]
def traverse(map_matrix, movement, position, tree_count = 0):
  if (map_matrix[position[1]][position[0]] == '#'):
    tree_count += 1

  if (position[1] == len(map_matrix) - 1):
     return tree_count

  position[1] += movement[1]
  position[0] += movement[0]

  if (position[0] >= len(map_matrix[0])):
    position[0] = position[0] - len(map_matrix[0])

  return traverse(map_matrix, movement, position, tree_count)

map_matrix = []
for line in fileinput.input():
  map_matrix.append(line.strip())

# First Star
print(traverse(map_matrix, [3,1],[0,0]))

# Second Star
movements = [[1,1],[3,1],[5,1],[7,1],[1,2]]
res = functools.reduce((lambda x, y: x * y), [traverse(map_matrix, movement, [0,0]) for movement in movements])
print(res)

