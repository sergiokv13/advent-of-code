import fileinput
from os import wait

def print_matrix(m):
  for row in m:
    print(row)
  print()

def matrix_get(m, i,j):
    if i >= len(m): i = 0
    if j >= len(m[i]): j = 0
    return m[i][j], i , j

def move_cucumber(m, pos, direction):
    if direction == '>':
        next_pos, new_i, new_j = matrix_get(m, pos[0], pos[1] + 1)
        if next_pos == '.':
            return new_i,new_j
    if direction == 'v':
        next_pos, new_i, new_j = matrix_get(m, pos[0] + 1, pos[1])
        if next_pos == '.':
            return new_i,new_j
        
    return False

def simulate_step(m):
    moved = False

    for direction in ['>', 'v']:
        original_mat = [row[:] for row in m]
        for i in range(len(original_mat)):
            for j in range(len(original_mat[i])):
                if original_mat[i][j] == direction:
                    new_pos = move_cucumber(original_mat, (i,j), direction)
                    if new_pos:
                        m[i][j] = '.'
                        m[new_pos[0]][new_pos[1]] = direction
                        moved = True

    return m, moved


def wait_until_stop(m):
    moved = True
    steps_count = 0
    while(moved):
        m, moved = simulate_step(m)
        steps_count += 1

    return steps_count

    

seamap = []
for idx, line in enumerate(fileinput.input()):
    seamap.append(list(line.strip()))

# First Star

print(wait_until_stop(seamap))

# # Second Star
