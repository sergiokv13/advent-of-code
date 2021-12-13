import fileinput
from typing import DefaultDict
import math

def print_matrix(m):
  for row in m:
    print(['#' if x == True else '.' for x in row])
  print()

# need to have same size
def logical_or(arr1, arr2): 
    return [arr1[idx] or arr2[idx] for idx in range(len(arr1))]

def fliplr(mat):
    mat2 = []
    for i in range(len(mat[0])):
        mat2.append([row[i] for row in mat][::-1])
    return mat2

def fliprl(mat):
    mat2 = []
    for i in range(len(mat[0])):
        mat2.append([row[i] for row in mat])
    return mat2[::-1]


def fold_y(mat, p):
    mat_a = mat[:p]
    mat_b = mat[p+1:]

    mat_b = mat_b[::-1]

    size = max([p, len(mat) - p]) - 1
    new_mat = []
    
    for i in range(size):
        if i < len(mat_a) and i < len(mat_b):
            new_mat.append(logical_or(mat_a[i], mat_b[i]))
        elif i < len(mat_a):
            new_mat.append(mat_a[i])
        else:
            new_mat.append(mat_b[i])
    
    return new_mat

def fold_x(mat, p):
    new_mat = fliplr(mat)
    new_mat = fold_y(new_mat, p)
    return fliprl(new_mat)

def flatten(t):
    return [item for sublist in t for item in sublist]

def count_mat(mat):
    return sum(flatten(mat))

positions = []
instructions = []
reading_positions = True
for line in fileinput.input():
    if reading_positions:
        if line == '\n': reading_positions = False
        else: positions.append([int(el) for el in line.split(',')])
    else:
        instruction_axis = 'y' if 'y' in line else 'x'
        instruction_value = int(line.replace("fold along y=", '').replace('fold along x=', ''))
        instructions.append((instruction_axis, instruction_value))

# Build matrix
max_x = max([p[0] for p in positions]) + 1
max_y = max([p[1] for p in positions]) + 1
mat = [[False for x in range(max_x)] for y in range(max_y)] 

for pos in positions:
    mat[pos[1]][pos[0]] = True


# # First Star

for i, ins in enumerate(instructions):
    if (ins[0] == 'y'):
        mat = fold_y(mat, ins[1])
        print(f"Instruction #{i+1}: ", count_mat(mat))
    else:
        mat = fold_x(mat, ins[1])
        print(f"Instruction #{i+1}: ", count_mat(mat))

# # Second Star

print_matrix(mat)



